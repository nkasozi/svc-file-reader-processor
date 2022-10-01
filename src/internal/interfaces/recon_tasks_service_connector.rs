use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError,
    file::FileThatHasBeenRead,
};

#[automock]
#[async_trait]
pub trait ReconTasksServiceConnectorInterface: Send + Sync {
    async fn create_recon_task(&self, file: &FileThatHasBeenRead) -> Result<String, AppError>;

    async fn attach_primary_file_to_task(&self, file: &FileThatHasBeenRead)
                                         -> Result<String, AppError>;

    async fn attach_comparison_file_to_task(
        &self,
        file: &FileThatHasBeenRead,
    ) -> Result<String, AppError>;
}
