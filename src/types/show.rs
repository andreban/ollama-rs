//! Types for the show model information endpoint (`POST /api/show`).
//!
//! Use [`ShowModelRequest`] to construct a request and pass it to
//! [`OllamaClient::show_model`](crate::OllamaClient::show_model).

use serde::{Deserialize, Serialize};

use super::common::ModelDetails;

/// A request to the show model endpoint (`POST /api/show`).
#[derive(Debug, Serialize)]
pub struct ShowModelRequest {
    /// The name of the model to retrieve information for.
    pub name: String,
    /// Whether to return full model details (e.g. system prompt, template).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

impl ShowModelRequest {
    /// Creates a new request for the given model name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            verbose: None,
        }
    }

    /// Enables verbose output, returning full model details.
    pub fn verbose(mut self) -> Self {
        self.verbose = Some(true);
        self
    }
}

/// A response from the show model endpoint.
#[derive(Debug, Deserialize)]
pub struct ShowModelResponse {
    /// The license of the model.
    pub license: Option<String>,
    /// The modelfile content.
    pub modelfile: Option<String>,
    /// The model parameters (e.g., stop tokens, temperature defaults).
    pub parameters: Option<String>,
    /// The prompt template.
    pub template: Option<String>,
    /// The system prompt.
    pub system: Option<String>,
    /// Detailed model metadata.
    pub details: Option<ModelDetails>,
    /// Additional model information.
    pub model_info: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_new_serialized() {
        let request = ShowModelRequest::new("llama3".to_string());
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json, json!({"name": "llama3"}));
    }

    #[test]
    fn request_verbose_serialized() {
        let request = ShowModelRequest::new("llama3".to_string()).verbose();
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json, json!({"name": "llama3", "verbose": true}));
    }

    #[test]
    fn response_deserialization() {
        let json = json!({
            "license": "MIT",
            "modelfile": "FROM llama3",
            "parameters": "temperature 0.7",
            "template": "Please answer:",
            "system": "You are helpful",
            "details": {
                "format": "gguf",
                "family": "llama",
                "families": ["llama"],
                "parameter_size": "8B",
                "quantization_level": "Q4_0"
            }
        });
        let response: ShowModelResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.license, Some("MIT".to_string()));
        assert_eq!(response.modelfile, Some("FROM llama3".to_string()));
        assert_eq!(response.parameters, Some("temperature 0.7".to_string()));
        assert_eq!(response.template, Some("Please answer:".to_string()));
        assert_eq!(response.system, Some("You are helpful".to_string()));
        
        let details = response.details.unwrap();
        assert_eq!(details.format, "gguf");
        assert_eq!(details.family, "llama");
    }
}
