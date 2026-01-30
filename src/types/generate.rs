//! Types for the text generation endpoint (`POST /api/generate`).
//!
//! Use [`GenerateRequest::builder()`] to construct a request and pass it to
//! [`OllamaClient::generate()`](crate::OllamaClient::generate). The response
//! is streamed as a sequence of [`GenerateResponse`] chunks.
//!
//! # Examples
//!
//! ```no_run
//! # use ollama_rs::OllamaClient;
//! # use ollama_rs::types::generate::GenerateRequest;
//! # use futures_util::StreamExt;
//! # async fn run() -> ollama_rs::error::OllamaResult<()> {
//! let client = OllamaClient::default();
//!
//! let request = GenerateRequest::builder("llama3")
//!     .prompt("Why is the sky blue?")
//!     .system_prompt("Answer in one sentence.")
//!     .build();
//!
//! let mut stream = client.generate(request);
//! while let Some(chunk) = stream.next().await {
//!     print!("{}", chunk?.response);
//! }
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::common::{Options, Think};

/// A request to the text generation endpoint (`POST /api/generate`).
///
/// Construct via [`GenerateRequest::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::generate::GenerateRequest;
///
/// let request = GenerateRequest::builder("llama3")
///     .prompt("Hello, world!")
///     .build();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// The model name to use for generation (e.g., `"llama3"`).
    pub model: String,

    /// The prompt text for the model to generate a response from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Text that appears after the user prompt and before the model response.
    /// Used for fill-in-the-middle models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// A system prompt that sets the model's behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Whether to stream the response. When `None`, the server default applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Base64-encoded images for multimodal models that support image input.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,

    /// A structured output format constraint. Accepts the string `"json"` for
    /// free-form JSON or a JSON Schema object for strict validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Value>,

    /// Controls extended-thinking (reasoning) mode. Can be a boolean
    /// (`true`/`false`) or a level (`"high"`, `"medium"`, `"low"`) for
    /// supported models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<Think>,

    /// Runtime options that control text generation behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
}

impl GenerateRequest {
    /// Returns a [`GenerateRequestBuilder`] for the given model.
    pub fn builder<M: Into<String>>(model: M) -> GenerateRequestBuilder {
        GenerateRequestBuilder::new(model)
    }
}

/// A builder for constructing a [`GenerateRequest`].
///
/// Obtain a builder via [`GenerateRequest::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::generate::GenerateRequest;
/// use ollama_rs::types::common::Options;
///
/// let request = GenerateRequest::builder("llama3")
///     .prompt("Tell me a joke")
///     .options(Options::builder().temperature(1.0).build())
///     .build();
/// ```
pub struct GenerateRequestBuilder {
    generate_request: GenerateRequest,
}

impl GenerateRequestBuilder {
    fn new<M: Into<String>>(model: M) -> Self {
        Self {
            generate_request: GenerateRequest {
                model: model.into(),
                prompt: None,
                suffix: None,
                system: None,
                stream: None,
                images: vec![],
                format: None,
                think: None,
                options: None,
            },
        }
    }

    /// Sets the system prompt that guides the model's behavior.
    pub fn system_prompt<P: Into<String>>(mut self, system_prompt: P) -> Self {
        self.generate_request.system = Some(system_prompt.into());
        self
    }

    /// Sets the prompt text to generate a response from.
    pub fn prompt<P: Into<String>>(mut self, prompt: P) -> Self {
        self.generate_request.prompt = Some(prompt.into());
        self
    }

    /// Sets whether to stream the response.
    pub fn stream(mut self, stream: bool) -> Self {
        self.generate_request.stream = Some(stream);
        self
    }

    /// Sets the suffix for fill-in-the-middle generation.
    pub fn suffix(mut self, suffix: String) -> Self {
        self.generate_request.suffix = Some(suffix);
        self
    }

    /// Sets base64-encoded images for multimodal generation.
    pub fn images(mut self, images: Vec<String>) -> Self {
        self.generate_request.images = images;
        self
    }

    /// Sets a structured output format constraint (JSON or a JSON Schema).
    pub fn format<T: Into<Value>>(mut self, value: T) -> Self {
        self.generate_request.format = Some(value.into());
        self
    }

    /// Enables or configures extended-thinking mode.
    pub fn think(mut self, think: Think) -> Self {
        self.generate_request.think = Some(think);
        self
    }

    /// Sets runtime generation options.
    pub fn options(mut self, options: Options) -> Self {
        self.generate_request.options = Some(options);
        self
    }

    /// Consumes the builder and returns the configured [`GenerateRequest`].
    pub fn build(self) -> GenerateRequest {
        self.generate_request
    }
}

/// A single chunk of a streaming text generation response.
///
/// When streaming, each chunk contains a fragment of the generated text in the
/// [`response`](GenerateResponse::response) field. The final chunk has
/// [`done`](GenerateResponse::done) set to `true` and includes performance
/// statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    /// The model that generated this response.
    pub model: String,

    /// ISO 8601 timestamp of when this chunk was created.
    pub created_at: String,

    /// The generated text fragment for this chunk.
    pub response: String,

    /// The model's thinking/reasoning output, if thinking mode was enabled.
    pub thinking: Option<String>,

    /// `true` when this is the final chunk of the response.
    pub done: bool,

    /// The reason generation stopped (e.g., `"stop"`). Only present in the
    /// final chunk.
    pub done_reason: Option<String>,

    /// Total time spent generating the response, in nanoseconds. Only present
    /// in the final chunk.
    pub total_duration: Option<u64>,

    /// Time spent loading the model, in nanoseconds. Only present in the final
    /// chunk.
    pub load_duration: Option<u64>,

    /// Number of tokens in the evaluated prompt. Only present in the final
    /// chunk.
    pub prompt_eval_count: Option<u64>,

    /// Time spent evaluating the prompt, in nanoseconds. Only present in the
    /// final chunk.
    pub prompt_eval_duration: Option<u64>,

    /// Number of tokens generated in the response. Only present in the final
    /// chunk.
    pub eval_count: Option<u64>,

    /// Time spent generating output tokens, in nanoseconds. Only present in
    /// the final chunk.
    pub eval_duration: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn builder_minimal() {
        let request = GenerateRequest::builder("llama3").build();
        assert_eq!(request.model, "llama3");
        assert!(request.prompt.is_none());
        assert!(request.system.is_none());
        assert!(request.images.is_empty());
    }

    #[test]
    fn builder_with_all_fields() {
        let request = GenerateRequest::builder("llama3")
            .prompt("hello")
            .system_prompt("you are helpful")
            .suffix("end".to_string())
            .stream(false)
            .images(vec!["base64data".to_string()])
            .format(json!("json"))
            .think(Think::Bool(true))
            .options(Options::builder().seed(42).build())
            .build();

        assert_eq!(request.model, "llama3");
        assert_eq!(request.prompt, Some("hello".to_string()));
        assert_eq!(request.system, Some("you are helpful".to_string()));
        assert_eq!(request.suffix, Some("end".to_string()));
        assert_eq!(request.stream, Some(false));
        assert_eq!(request.images, vec!["base64data".to_string()]);
        assert!(request.format.is_some());
        assert!(request.think.is_some());
        assert!(request.options.is_some());
    }

    #[test]
    fn request_skips_none_fields() {
        let request = GenerateRequest::builder("llama3").prompt("hello").build();
        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        assert!(obj.contains_key("model"));
        assert!(obj.contains_key("prompt"));
        assert!(!obj.contains_key("suffix"));
        assert!(!obj.contains_key("system"));
        assert!(!obj.contains_key("stream"));
        assert!(!obj.contains_key("images"));
        assert!(!obj.contains_key("format"));
        assert!(!obj.contains_key("think"));
        assert!(!obj.contains_key("options"));
    }

    #[test]
    fn request_includes_images_when_nonempty() {
        let request = GenerateRequest::builder("llama3")
            .images(vec!["abc".to_string()])
            .build();
        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("images"));
    }

    #[test]
    fn response_deserialize_streaming_chunk() {
        let json = json!({
            "model": "llama3",
            "created_at": "2024-01-01T00:00:00Z",
            "response": "Hello",
            "done": false
        });
        let response: GenerateResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.model, "llama3");
        assert_eq!(response.response, "Hello");
        assert!(!response.done);
        assert!(response.done_reason.is_none());
        assert!(response.total_duration.is_none());
    }

    #[test]
    fn response_deserialize_final_chunk() {
        let json = json!({
            "model": "llama3",
            "created_at": "2024-01-01T00:00:00Z",
            "response": "",
            "done": true,
            "done_reason": "stop",
            "total_duration": 5000000000u64,
            "load_duration": 1000000000u64,
            "prompt_eval_count": 10,
            "prompt_eval_duration": 500000000u64,
            "eval_count": 50,
            "eval_duration": 3500000000u64
        });
        let response: GenerateResponse = serde_json::from_value(json).unwrap();
        assert!(response.done);
        assert_eq!(response.done_reason, Some("stop".to_string()));
        assert_eq!(response.total_duration, Some(5_000_000_000));
        assert_eq!(response.eval_count, Some(50));
    }

    #[test]
    fn response_deserialize_with_thinking() {
        let json = json!({
            "model": "llama3",
            "created_at": "2024-01-01T00:00:00Z",
            "response": "The answer is 42.",
            "thinking": "Let me think about this...",
            "done": true
        });
        let response: GenerateResponse = serde_json::from_value(json).unwrap();
        assert_eq!(
            response.thinking,
            Some("Let me think about this...".to_string())
        );
    }
}
