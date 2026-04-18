// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

//! Types for the embedding endpoint (`POST /api/embed`).
//!
//! Use [`EmbedRequest::builder()`] to construct a request and pass it to
//! [`OllamaClient::embed()`](crate::OllamaClient::embed).
//!
//! # Examples
//!
//! ```no_run
//! # use ollama_rs::OllamaClient;
//! # use ollama_rs::types::embed::EmbedRequest;
//! # async fn run() -> ollama_rs::error::OllamaResult<()> {
//! let client = OllamaClient::default();
//! let request = EmbedRequest::builder("embeddinggemma")
//!     .input("Generate embeddings for this text")
//!     .build();
//!
//! let response = client.embed(request).await?;
//! println!("Embeddings: {:?}", response.embeddings);
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

use crate::types::common::Options;

/// The input text(s) to generate embeddings for.
///
/// Accepts either a single string or an array of strings. Serialized as an
/// untagged enum so both `"hello"` and `["hello", "world"]` are valid JSON
/// representations.
///
/// # Examples
///
/// ```
/// use ollama_rs::types::embed::EmbedInput;
///
/// let single = EmbedInput::Single("hello".to_string());
/// let multiple = EmbedInput::Multiple(vec!["hello".to_string(), "world".to_string()]);
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbedInput {
    /// A single text string.
    Single(String),
    /// Multiple text strings.
    Multiple(Vec<String>),
}

/// A request to generate embeddings (`POST /api/embed`).
///
/// Construct via [`EmbedRequest::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::embed::EmbedRequest;
///
/// let request = EmbedRequest::builder("embeddinggemma")
///     .input("Generate embeddings for this text")
///     .build();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedRequest {
    /// The model name to use for generating embeddings.
    pub model: String,

    /// The text or array of texts to generate embeddings for.
    pub input: EmbedInput,

    /// If `true`, truncate inputs that exceed the context window. If `false`,
    /// returns an error. Defaults to `true` on the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,

    /// Number of dimensions to generate embeddings for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,

    /// How long the model stays loaded in memory (e.g., `"5m"`, `"1h"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,

    /// Runtime options for the embedding model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
}

impl EmbedRequest {
    /// Returns an [`EmbedRequestBuilder`] for the given model name.
    pub fn builder<M: Into<String>>(model: M) -> EmbedRequestBuilder {
        EmbedRequestBuilder {
            embed_request: EmbedRequest {
                model: model.into(),
                input: EmbedInput::Single(String::new()),
                truncate: None,
                dimensions: None,
                keep_alive: None,
                options: None,
            },
        }
    }
}

/// A builder for constructing an [`EmbedRequest`].
///
/// Obtain a builder via [`EmbedRequest::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::embed::EmbedRequest;
///
/// let request = EmbedRequest::builder("embeddinggemma")
///     .input("hello world")
///     .truncate(true)
///     .build();
/// ```
pub struct EmbedRequestBuilder {
    embed_request: EmbedRequest,
}

impl EmbedRequestBuilder {
    /// Sets a single text string as the input.
    pub fn input<S: Into<String>>(mut self, input: S) -> Self {
        self.embed_request.input = EmbedInput::Single(input.into());
        self
    }

    /// Sets multiple text strings as the input.
    pub fn inputs<S: Into<String>>(mut self, inputs: Vec<S>) -> Self {
        self.embed_request.input =
            EmbedInput::Multiple(inputs.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Sets whether to truncate inputs that exceed the context window.
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.embed_request.truncate = Some(truncate);
        self
    }

    /// Sets the number of dimensions for the embeddings.
    pub fn dimensions(mut self, dimensions: u32) -> Self {
        self.embed_request.dimensions = Some(dimensions);
        self
    }

    /// Sets how long the model stays loaded in memory (e.g., `"5m"`).
    pub fn keep_alive<S: Into<String>>(mut self, keep_alive: S) -> Self {
        self.embed_request.keep_alive = Some(keep_alive.into());
        self
    }

    /// Sets runtime options for the embedding model.
    pub fn options(mut self, options: Options) -> Self {
        self.embed_request.options = Some(options);
        self
    }

    /// Consumes the builder and returns the configured [`EmbedRequest`].
    pub fn build(self) -> EmbedRequest {
        self.embed_request
    }
}

/// The response from the embedding endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedResponse {
    /// The model that produced the embeddings.
    pub model: String,

    /// The generated vector embeddings. Each inner `Vec<f64>` corresponds to
    /// one input text, in the same order as the request.
    pub embeddings: Vec<Vec<f64>>,

    /// Total time spent generating embeddings, in nanoseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<u64>,

    /// Time spent loading the model, in nanoseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<u64>,

    /// Number of input tokens processed to generate the embeddings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn builder_minimal() {
        let request = EmbedRequest::builder("embeddinggemma")
            .input("hello")
            .build();
        assert_eq!(request.model, "embeddinggemma");
        assert!(matches!(request.input, EmbedInput::Single(ref s) if s == "hello"));
        assert!(request.truncate.is_none());
        assert!(request.dimensions.is_none());
        assert!(request.keep_alive.is_none());
        assert!(request.options.is_none());
    }

    #[test]
    fn builder_with_all_fields() {
        let request = EmbedRequest::builder("embeddinggemma")
            .inputs(vec!["hello", "world"])
            .truncate(false)
            .dimensions(256)
            .keep_alive("10m")
            .options(Options::builder().seed(42).build())
            .build();

        assert!(matches!(request.input, EmbedInput::Multiple(ref v) if v.len() == 2));
        assert_eq!(request.truncate, Some(false));
        assert_eq!(request.dimensions, Some(256));
        assert_eq!(request.keep_alive, Some("10m".to_string()));
        assert!(request.options.is_some());
    }

    #[test]
    fn request_skips_none_fields() {
        let request = EmbedRequest::builder("embeddinggemma")
            .input("hello")
            .build();
        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("model"));
        assert!(obj.contains_key("input"));
        assert!(!obj.contains_key("truncate"));
        assert!(!obj.contains_key("dimensions"));
        assert!(!obj.contains_key("keep_alive"));
        assert!(!obj.contains_key("options"));
    }

    #[test]
    fn request_serializes_single_input() {
        let request = EmbedRequest::builder("embeddinggemma")
            .input("hello")
            .build();
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["input"], json!("hello"));
    }

    #[test]
    fn request_serializes_multiple_inputs() {
        let request = EmbedRequest::builder("embeddinggemma")
            .inputs(vec!["hello", "world"])
            .build();
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["input"], json!(["hello", "world"]));
    }

    #[test]
    fn embed_input_single_round_trip() {
        let input = EmbedInput::Single("test".to_string());
        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(json, json!("test"));
        let deserialized: EmbedInput = serde_json::from_value(json).unwrap();
        assert!(matches!(deserialized, EmbedInput::Single(s) if s == "test"));
    }

    #[test]
    fn embed_input_multiple_round_trip() {
        let input = EmbedInput::Multiple(vec!["a".to_string(), "b".to_string()]);
        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(json, json!(["a", "b"]));
        let deserialized: EmbedInput = serde_json::from_value(json).unwrap();
        assert!(matches!(deserialized, EmbedInput::Multiple(v) if v == vec!["a", "b"]));
    }

    #[test]
    fn response_deserialize() {
        let json = json!({
            "model": "embeddinggemma",
            "embeddings": [[0.010071029, -0.0017594862, 0.05007221]],
            "total_duration": 14143917,
            "load_duration": 1019500,
            "prompt_eval_count": 8
        });
        let response: EmbedResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.model, "embeddinggemma");
        assert_eq!(response.embeddings.len(), 1);
        assert_eq!(response.embeddings[0].len(), 3);
        assert_eq!(response.total_duration, Some(14143917));
        assert_eq!(response.load_duration, Some(1019500));
        assert_eq!(response.prompt_eval_count, Some(8));
    }

    #[test]
    fn response_deserialize_minimal() {
        let json = json!({
            "model": "embeddinggemma",
            "embeddings": [[1.0, 2.0], [3.0, 4.0]]
        });
        let response: EmbedResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.embeddings.len(), 2);
        assert!(response.total_duration.is_none());
        assert!(response.load_duration.is_none());
        assert!(response.prompt_eval_count.is_none());
    }
}
