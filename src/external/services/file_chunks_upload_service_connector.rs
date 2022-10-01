use async_trait::async_trait;

use crate::internal::{
    interfaces::file_chunks_upload_service_connector::FileChunksUploadHandlerServiceConnectorInterface,
    shared_reconciler_rust_libraries::models::entities::app_errors::AppError,
};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::interfaces::file_upload_handler_microservice::FileChunksUploadHandlerMicroserviceClientInterface;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;

pub struct FileChunksUploadHandlerServiceConnector {
    file_upload_chunk_microservice_client: Box<dyn FileChunksUploadHandlerMicroserviceClientInterface>,
}

#[async_trait]
impl FileChunksUploadHandlerServiceConnectorInterface for FileChunksUploadHandlerServiceConnector {
    async fn upload_file_chunk(
        &self,
        file_upload_chunk: &UploadFileChunkRequest,
    ) -> Result<(), AppError> {
        let _ = self.file_upload_chunk_microservice_client.upload_file_chunk(file_upload_chunk).await?;
        return Ok(());
    }
}

impl FileChunksUploadHandlerServiceConnector {
    pub(crate) fn new(microservice_client: Box<dyn FileChunksUploadHandlerMicroserviceClientInterface>) -> FileChunksUploadHandlerServiceConnector {
        return FileChunksUploadHandlerServiceConnector {
            file_upload_chunk_microservice_client: microservice_client
        };
    }
}
