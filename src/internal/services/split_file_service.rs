use async_trait::async_trait;
use validator::Validate;

use crate::internal::{
    interfaces::{
        file_reader::FileReader, split_file_service::SplitFileServiceInterface,
        svc_file_chunks_uploader::FileChunksUploaderInterface,
        svc_recon_tasks_handler::ReconTasksHandlerInterface, transformer::TransformerInterface,
    },
    models::view_models::{
        requests::split_file_request::SplitFileRequest,
        responses::split_file_response::SplitFileResponse,
    },
    shared_reconciler_rust_libraries::models::entities::recon_tasks_models::ReconFileType,
};

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{
    AppError, AppErrorKind,
};

pub struct SplitFileService {
    pub file_reader: Box<dyn FileReader>,
    pub transformer: Box<dyn TransformerInterface>,
    pub file_chunks_uploader: Box<dyn FileChunksUploaderInterface>,
    pub recon_tasks_handler: Box<dyn ReconTasksHandlerInterface>,
}

#[async_trait]
impl SplitFileServiceInterface for SplitFileService {
    /**
    reconstructs a file from file chunks that have finished reconciliations

    # Errors

    This function will return an error if the request fails validation or fails to be uploaded.
    */
    async fn split_file_into_chunks(
        &self,
        request: SplitFileRequest,
    ) -> Result<SplitFileResponse, AppError> {
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
        let mut file = request.file;

        //read the records in the file
        let file_that_has_been_read = self.file_reader.read_file(&file).await?;

        if file.upload_request_id.is_empty() {
            //since this is a new recon task, we create the recon task
            let upload_request_id = self.recon_tasks_handler.create_recon_task(&file).await?;

            //we set the recon task id
            file.upload_request_id = upload_request_id;

            //then we attach the file to the recon task
            //depending on the file type
            match file.file_type {
                ReconFileType::PrimaryFile => {
                    let _ = self
                        .recon_tasks_handler
                        .attach_primary_file_to_task(&file_that_has_been_read)
                        .await?;
                }
                ReconFileType::ComparisonFile => {
                    let _ = self
                        .recon_tasks_handler
                        .attach_comparison_file_to_task(&file_that_has_been_read)
                        .await?;
                }
            }
        }

        //group the records into file chunks
        let file_chunks = self
            .transformer
            .group_rows_into_file_chunks(&file_that_has_been_read, 200);

        //upload each chunk
        for chunk in file_chunks {
            let _ = self.file_chunks_uploader.upload_file_chunk(&chunk).await?;
        }

        //return success
        return Ok(SplitFileResponse {
            upload_request_id: file.upload_request_id,
        });
    }
}
