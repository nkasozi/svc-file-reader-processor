use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError, reconstructed_file::ReconstructedFile,
};

#[automock]
#[async_trait]
pub trait PubSubInterface: Send + Sync {
    async fn publish_to_reconciled_files_queue(
        &self,
        queue: &ReconstructedFile,
    ) -> Result<bool, AppError>;
}
