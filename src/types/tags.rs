use serde::{Deserialize, Serialize};

use crate::types::common::ModelDetails;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsResponse {
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_tags_response() {
        let json = json!({
            "models": [{
                "name": "llama3:latest",
                "model": "llama3:latest",
                "modified_at": "2024-01-01T00:00:00Z",
                "size": 4_700_000_000u64,
                "digest": "abc123",
                "details": {
                    "format": "gguf",
                    "family": "llama",
                    "parameter_size": "8B",
                    "quantization_level": "Q4_0"
                }
            }]
        });
        let response: TagsResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.models.len(), 1);
        assert_eq!(response.models[0].name, "llama3:latest");
        assert_eq!(response.models[0].size, 4_700_000_000);
        assert_eq!(response.models[0].details.family, "llama");
    }

    #[test]
    fn deserialize_empty_models() {
        let json = json!({"models": []});
        let response: TagsResponse = serde_json::from_value(json).unwrap();
        assert!(response.models.is_empty());
    }
}
