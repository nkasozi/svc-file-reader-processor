use mockall::automock;

use crate::internal::{
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::entities::file::FileThatHasBeenRead,
};

#[automock]
pub trait TransformerInterface: Send + Sync {
    fn group_rows_into_file_chunks(
        &self,
        file_that_has_been_read: &FileThatHasBeenRead,
        max_group_size: i64,
    ) -> Vec<UploadFileChunkRequest>;
}
