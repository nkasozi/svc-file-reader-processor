use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    file_row::FileRow, file_upload_chunk::FileUploadChunkSource,
};

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct UploadFileChunkRequest {
    #[validate(length(min = 1, message = "please supply an upload_request_id"))]
    pub upload_request_id: String,

    #[validate(range(min = 1))]
    pub chunk_sequence_number: i64,

    pub chunk_source: FileUploadChunkSource,

    #[validate]
    pub chunk_rows: Vec<FileRow>,

    pub is_last_chunk: bool,
}
