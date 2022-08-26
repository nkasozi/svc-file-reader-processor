use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError, reconstructed_file::ReconstructedFile,
};

#[automock]
#[async_trait]
pub trait ReconResultsRepositoryInterface: Send + Sync {
    async fn get_reconstructed_file(
        &self,
        file_id: &String,
    ) -> Result<Option<ReconstructedFile>, AppError>;

    async fn save_reconstructed_file(
        &self,
        reconstructed_file: &ReconstructedFile,
    ) -> Result<bool, AppError>;
}
