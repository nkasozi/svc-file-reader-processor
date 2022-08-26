use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReconstructFileFromChunksResponse {
    pub file_chunk_id: String,
}
