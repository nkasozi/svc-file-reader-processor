use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError, file::File,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait FileRetriever: Send + Sync {
    async fn retrieve_file(&self, file: File) -> Result<File, AppError>;
}
