use async_trait::async_trait;
use mockall::automock;

use crate::internal::{
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::entities::app_errors::AppError,
};

#[automock]
#[async_trait]
pub trait FileChunksUploaderInterface: Send + Sync {
    async fn upload_file_chunk(&self, request: &UploadFileChunkRequest) -> Result<(), AppError>;
}
