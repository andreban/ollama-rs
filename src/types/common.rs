use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Think {
    Bool(bool),
    Level(ThinkLevel),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkLevel {
    High,
    Medium,
    Low,
}
