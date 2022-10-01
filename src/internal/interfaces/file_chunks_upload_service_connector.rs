use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppError;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;

#[automock]
#[async_trait]
pub trait FileChunksUploadHandlerServiceConnectorInterface: Send + Sync {
    async fn upload_file_chunk(&self, request: &UploadFileChunkRequest) -> Result<(), AppError>;
}
