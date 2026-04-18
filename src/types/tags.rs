// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

//! Types for the model listing endpoint (`GET /api/tags`).
//!
//! The response from [`OllamaClient::tags()`](crate::OllamaClient::tags) is
//! deserialized into a [`TagsResponse`].

use serde::{Deserialize, Serialize};

use crate::types::common::ModelDetails;

/// Response from the `GET /api/tags` endpoint.
///
/// Contains a list of all models available on the Ollama server.
#[derive(Debug, Serialize, Deserialize)]
pub struct TagsResponse {
    /// The available models.
    pub models: Vec<Model>,
}

/// An available model on the Ollama server.
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// The model name including tag (e.g., `"llama3:latest"`).
    pub name: String,
    /// The model identifier.
    pub model: String,
    /// ISO 8601 timestamp of when the model was last modified.
    pub modified_at: String,
    /// Total model size in bytes.
    pub size: u64,
    /// The SHA-256 digest of the model.
    pub digest: String,
    /// Detailed model metadata.
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
