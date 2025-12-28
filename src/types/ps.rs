use serde::{Deserialize, Serialize};

use crate::types::common::ModelDetails;

#[derive(Debug, Serialize, Deserialize)]
pub struct PsResponse {
    pub models: Vec<RunningModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunningModel {
    pub name: String,
    pub model: String,
    pub size: usize,
    pub digest: String,
    pub details: ModelDetails,
    pub expires_at: String,
    pub size_vram: usize,
    pub context_length: u32,
}
