//! Types for the running-models endpoint (`GET /api/ps`).
//!
//! The response from [`OllamaClient::ps()`](crate::OllamaClient::ps) is
//! deserialized into a [`PsResponse`].

use serde::{Deserialize, Serialize};

use crate::types::common::ModelDetails;

/// Response from the `GET /api/ps` endpoint.
///
/// Contains a list of models currently loaded in memory on the Ollama server.
#[derive(Debug, Serialize, Deserialize)]
pub struct PsResponse {
    /// The currently loaded models.
    pub models: Vec<RunningModel>,
}

/// A model that is currently loaded and running on the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct RunningModel {
    /// The model name including tag (e.g., `"llama3:latest"`).
    pub name: String,
    /// The model identifier.
    pub model: String,
    /// Total model size in bytes.
    pub size: u64,
    /// The SHA-256 digest of the model.
    pub digest: String,
    /// Detailed model metadata.
    pub details: ModelDetails,
    /// ISO 8601 timestamp of when the model will be unloaded from memory.
    pub expires_at: String,
    /// Amount of VRAM the model occupies, in bytes.
    pub size_vram: u64,
    /// The context length configured for this model instance.
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
