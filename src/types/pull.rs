// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

//! Types for the model pull (download) endpoint (`POST /api/pull`).
//!
//! Use [`PullRequest::builder()`] to construct a request and pass it to
//! [`OllamaClient::pull()`](crate::OllamaClient::pull). The response is
//! streamed as a sequence of [`PullResponse`] status updates.
//!
//! # Examples
//!
//! ```no_run
//! # use ollama_rs::OllamaClient;
//! # use ollama_rs::types::pull::PullRequest;
//! # use futures_util::StreamExt;
//! # async fn run() -> ollama_rs::error::OllamaResult<()> {
//! let client = OllamaClient::default();
//! let request = PullRequest::builder("llama3").build();
//!
//! let mut stream = client.pull(request);
//! while let Some(chunk) = stream.next().await {
//!     println!("{}", chunk?.status);
//! }
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

/// A request to download a model from the Ollama registry (`POST /api/pull`).
///
/// Construct via [`PullRequest::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::pull::PullRequest;
///
/// let request = PullRequest::builder("llama3")
///     .stream(true)
///     .build();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    /// The model name to pull (e.g., `"llama3"`, `"llama3:latest"`).
    pub model: String,
    /// Allow insecure (HTTP) connections to the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// Whether to stream status updates. When `None`, the server default applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl PullRequest {
    /// Returns a [`PullRequestBuilder`] for the given model name.
    pub fn builder<M: Into<String>>(model: M) -> PullRequestBuilder {
        PullRequestBuilder {
            pull_request: PullRequest {
                model: model.into(),
                insecure: None,
                stream: None,
            },
        }
    }
}

/// A builder for constructing a [`PullRequest`].
///
/// Obtain a builder via [`PullRequest::builder()`].
pub struct PullRequestBuilder {
    pull_request: PullRequest,
}

impl PullRequestBuilder {
    /// Sets whether to stream status updates.
    pub fn stream(mut self, stream: bool) -> Self {
        self.pull_request.stream = Some(stream);
        self
    }

    /// Allows insecure (HTTP) connections to the model registry.
    pub fn insecure(mut self, insecure: bool) -> Self {
        self.pull_request.insecure = Some(insecure);
        self
    }

    /// Consumes the builder and returns the configured [`PullRequest`].
    pub fn build(self) -> PullRequest {
        self.pull_request
    }
}

/// A streaming status update from the model pull operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct PullResponse {
    /// A human-readable status message (e.g., `"pulling manifest"`,
    /// `"downloading sha256:..."`).
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn builder_minimal() {
        let request = PullRequest::builder("llama3").build();
        assert_eq!(request.model, "llama3");
        assert!(request.insecure.is_none());
        assert!(request.stream.is_none());
    }

    #[test]
    fn builder_with_options() {
        let request = PullRequest::builder("llama3")
            .stream(true)
            .insecure(false)
            .build();
        assert_eq!(request.stream, Some(true));
        assert_eq!(request.insecure, Some(false));
    }

    #[test]
    fn request_skips_none_fields() {
        let request = PullRequest::builder("llama3").build();
        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("model"));
        assert!(!obj.contains_key("insecure"));
        assert!(!obj.contains_key("stream"));
    }

    #[test]
    fn request_includes_set_fields() {
        let request = PullRequest::builder("llama3").stream(true).build();
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json, json!({"model": "llama3", "stream": true}));
    }

    #[test]
    fn response_deserialize() {
        let json = json!({"status": "pulling manifest"});
        let response: PullResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.status, "pulling manifest");
    }
}
