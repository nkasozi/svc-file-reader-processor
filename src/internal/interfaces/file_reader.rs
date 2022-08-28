use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError,
    file::{File, FileThatHasBeenRead},
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait FileReader: Send + Sync {
    async fn read_file(&self, file: &File) -> Result<FileThatHasBeenRead, AppError>;
}
