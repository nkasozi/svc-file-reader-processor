use async_trait::async_trait;
use validator::Validate;

use crate::internal::{
    interfaces::{
        pubsub::PubSubInterface, recon_results_repository::ReconResultsRepositoryInterface,
        reconstruct_file_service::ReconstructFileServiceInterface,
        transformer::TransformerInterface,
    },
    models::view_models::{
        requests::reconstruct_file_from_chunks_request::ReconstructFileFromChunksRequest,
        responses::reconstruct_file_from_chunks_response::ReconstructFileFromChunksResponse,
    },
    shared_reconciler_rust_libraries::models::entities::reconstructed_file::ReconstructedFile,
};

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{
    AppError, AppErrorKind,
};

pub struct ReconstructFileService {
    pub recon_results_repository: Box<dyn ReconResultsRepositoryInterface>,
    pub pubsub: Box<dyn PubSubInterface>,
    pub transformer: Box<dyn TransformerInterface>,
}

#[async_trait]
impl ReconstructFileServiceInterface for ReconstructFileService {
    /**
    reconstructs a file from file chunks that have finished reconciliations

    # Errors

    This function will return an error if the request fails validation or fails to be uploaded.
    */
    async fn rebuild_file(
        &self,
        request: ReconstructFileFromChunksRequest,
    ) -> Result<ReconstructFileFromChunksResponse, AppError> {
        //validate request
        match request.validate() {
            Ok(_) => (),
            Err(e) => {
                return Err(AppError::new(
                    AppErrorKind::BadClientRequest,
                    e.to_string().replace("\n", " , "),
                ));
            }
        }

        //get a handle to the underlying file chunk
        let mut file_upload_chunk = request.file_upload_chunk;

        //get reconstructed file
        let probable_reconstructed_file = self
            .recon_results_repository
            .get_reconstructed_file(&file_upload_chunk.id)
            .await?;

        let reconstructed_file: ReconstructedFile;

        //handle the probable_reconstructed_file
        match probable_reconstructed_file {
            Some(mut file) => {
                //since we already have a reconstructed file for this file,
                //we simply insert this file chunk in its correct location within that file
                reconstructed_file = self
                    .transformer
                    .put_file_chunk_in_correct_position_in_reconstructed_file(
                        &mut file_upload_chunk,
                        &mut file,
                    );
            }

            None => {
                //since there is no exisiting reconstructed_file,
                //we create one
                reconstructed_file = self
                    .transformer
                    .create_new_reconstructed_file(&mut file_upload_chunk);
            }
        }

        //save the reconstructed_file
        let _ = self
            .recon_results_repository
            .save_reconstructed_file(&reconstructed_file)
            .await?;

        if self.is_reconstructed_file_complete(&reconstructed_file) {
            self.pubsub
                .publish_to_reconciled_files_queue(&reconstructed_file)
                .await?;
        }

        //handle the save result
        Ok(ReconstructFileFromChunksResponse {
            file_chunk_id: file_upload_chunk.id,
        })
    }
}

impl ReconstructFileService {
    //loops thru the reconstructed_file and
    //first of all determines if we have recieved the last file chunk in this file
    //if so it checks to make sure there are no missing file chunks
    fn is_reconstructed_file_complete(&self, file: &ReconstructedFile) -> bool {
        let probable_last_file_chunk_seq_number = self.get_last_file_chunk_sequence_number(file);
        match probable_last_file_chunk_seq_number {
            None => {
                return false;
            }

            Some(last_file_chunk_seq_number) => {
                for chunk_sequence_number in 1..last_file_chunk_seq_number {
                    let probable_value = file
                        .file_chunk_id_to_chunk_details_map
                        .get(&chunk_sequence_number);
                    match probable_value {
                        Some(_) => continue,
                        None => return false,
                    }
                }
                return true;
            }
        }
    }

    //returns the chunk_sequence_number of the last file chunk
    fn get_last_file_chunk_sequence_number(&self, file: &ReconstructedFile) -> Option<i64> {
        for chunk in file.file_chunk_id_to_chunk_details_map.values() {
            if chunk.is_last_chunk {
                return Some(chunk.chunk_sequence_number);
            }
        }
        return Option::None;
    }
}
