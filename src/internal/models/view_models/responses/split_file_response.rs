use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SplitFileResponse {
    pub upload_request_id: String,
}
