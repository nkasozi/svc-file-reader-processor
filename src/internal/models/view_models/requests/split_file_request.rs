use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::internal::shared_reconciler_rust_libraries::models::entities::file::File;

#[derive(Serialize, Deserialize, Clone, Validate, Debug)]
pub struct SplitFileRequest {
    pub file: File,
}

impl SplitFileRequest {
    pub fn is_metadata_required_for_new_recon_job(&self) -> bool {

        //if this is the second file in an existing upload job then we dont need metadata
        if !self.file.upload_request_id.is_none() {
            return false;
        }

        //otherwise we need some info in the metadata
        //before we can even begin to start
        return match self.clone().file.file_metadata {
            None => { true }
            Some(metadata) => {
                match metadata.comparison_pairs {
                    None => { true }
                    Some(_) => {
                        false
                    }
                }
            }
        };
    }
}
