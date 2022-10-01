use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::file::FileThatHasBeenRead;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;

#[automock]
pub trait TransformerInterface: Send + Sync {
    fn group_rows_into_file_chunks(
        &self,
        file_that_has_been_read: &FileThatHasBeenRead,
        max_group_size: i64,
    ) -> Vec<UploadFileChunkRequest>;
}
