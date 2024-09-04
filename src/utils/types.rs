use serde::{Deserialize, Serialize};
use zksync_ethers_rs::types::zksync::L1BatchNumber;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBatchResponse {
    pub batch_file: String,
    pub request_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostSubmitProofRequest {
    pub request_id: u32,
    pub proof_data: String,
    pub proving_time: u64,
    pub cost: u64,
    pub price: u64,
    pub deployment_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBatchInfo {
    pub request_id: u32,
    pub batch_number: L1BatchNumber,
}
