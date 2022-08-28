use crate::internal::{
    models::view_models::{
        requests::split_file_request::SplitFileRequest,
        responses::split_file_response::SplitFileResponse,
    },
    shared_reconciler_rust_libraries::models::entities::app_errors::AppError,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait SplitFileServiceInterface: Send + Sync {
    async fn split_file_into_chunks(
        &self,
        file: SplitFileRequest,
    ) -> Result<SplitFileResponse, AppError>;
}
