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
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
    pub expires_at: String,
    pub size_vram: u64,
    pub context_length: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_ps_response() {
        let json = json!({
            "models": [{
                "name": "llama3:latest",
                "model": "llama3:latest",
                "size": 4_700_000_000u64,
                "digest": "abc123",
                "details": {
                    "format": "gguf",
                    "family": "llama",
                    "parameter_size": "8B",
                    "quantization_level": "Q4_0"
                },
                "expires_at": "2024-01-01T01:00:00Z",
                "size_vram": 4_700_000_000u64,
                "context_length": 8192
            }]
        });
        let response: PsResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.models.len(), 1);
        assert_eq!(response.models[0].name, "llama3:latest");
        assert_eq!(response.models[0].size_vram, 4_700_000_000);
        assert_eq!(response.models[0].context_length, 8192);
    }

    #[test]
    fn deserialize_empty_models() {
        let json = json!({"models": []});
        let response: PsResponse = serde_json::from_value(json).unwrap();
        assert!(response.models.is_empty());
    }
}
