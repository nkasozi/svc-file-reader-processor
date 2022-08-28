use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SplitFileResponse {
    pub upload_request_id: String,
}
