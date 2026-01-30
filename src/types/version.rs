//! Types for the server version endpoint (`GET /api/version`).
//!
//! The response from [`OllamaClient::version()`](crate::OllamaClient::version)
//! is deserialized into a [`VersionResponse`].

use serde::{Deserialize, Serialize};

/// Response from the `GET /api/version` endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    /// The Ollama server version string (e.g., `"0.6.2"`).
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize() {
        let json = json!({"version": "0.6.2"});
        let response: VersionResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.version, "0.6.2");
    }

    #[test]
    fn round_trip() {
        let response = VersionResponse {
            version: "0.6.2".to_string(),
        };
        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json, json!({"version": "0.6.2"}));
    }
}
