use serde::{Deserialize, Serialize};

use crate::types::common::ModelDetails;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: usize,
    pub digest: String,
    pub details: ModelDetails,
}
