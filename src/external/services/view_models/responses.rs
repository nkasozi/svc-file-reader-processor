use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct CreateReconTaskResponse {
    pub task_id: String,
}

#[derive(Default, Serialize, PartialEq, Clone, Eq, Deserialize, Debug)]
pub struct AttachFileResponse {}
