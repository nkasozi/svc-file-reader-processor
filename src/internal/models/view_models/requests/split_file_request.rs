use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::internal::shared_reconciler_rust_libraries::models::entities::file::File;

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct SplitFileRequest {
    pub file: File,
}
