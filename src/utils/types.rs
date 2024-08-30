use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBatchResponse {
    pub batch_file: String,
    pub request_id: u32,
}
