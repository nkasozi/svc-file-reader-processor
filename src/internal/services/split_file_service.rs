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
use crate::internal::shared_reconciler_rust_libraries::common::utils::{app_error, app_error_with_msg};
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{
    AppError, AppErrorKind,
};
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::FileThatHasBeenRead;

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
    async fn read_and_split_file_into_chunks(
        &self,
        request: SplitFileRequest,
    ) -> Result<SplitFileResponse, AppError> {

        //validate request
        match request.validate() {
            Ok(_) => (),
            Err(e) => {
                return app_error(AppErrorKind::BadClientRequest, Box::new(e));
            }
        }

        if request.is_metadata_required_for_new_recon_job() {
            return app_error_with_msg(AppErrorKind::BadClientRequest, "please supply comparison pairs if no upload_request_id supplied");
        }

        //get a handle to the underlying file
        let file = request.file;

        //read the records in the file
        let mut file_that_has_been_read = self.file_reader.read_file(&file).await?;

        match file_that_has_been_read.upload_request_id {
            None => {

                //since this is a new recon task, we create the recon task
                let upload_request_id = self.recon_tasks_handler.create_recon_task(&file_that_has_been_read).await?;

                //we set the recon task id
                file_that_has_been_read.upload_request_id = Some(upload_request_id);

                //then we attach the file to the recon task
                //depending on the file type
                self.attach_file_to_task(&mut file_that_has_been_read).await?
            }

            Some(_) => {
                //we attach the file to the recon task
                //depending on the file type
                self.attach_file_to_task(&mut file_that_has_been_read).await?
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
            upload_request_id: file_that_has_been_read.upload_request_id.unwrap_or("".to_string()),
        });
    }
}

impl SplitFileService {
    async fn attach_file_to_task(&self, file_that_has_been_read: &mut FileThatHasBeenRead) -> Result<(), AppError> {
        //then we attach the file to the recon task
        //depending on the file type
        match file_that_has_been_read.file_type {
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

        return Ok(());
    }
}
