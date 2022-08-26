use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::internal::shared_reconciler_rust_libraries::models::entities::file_upload_chunk::FileUploadChunk;

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct ReconstructFileFromChunksRequest {
    pub file_upload_chunk: FileUploadChunk,
}
