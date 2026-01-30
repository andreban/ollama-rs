use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::common::{Options, Think};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// Model name
    pub model: String,

    /// Text for the model to generate a response from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// Used for fill-in-the-middle models, text that appears after the user prompt and before the
    /// model response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// System prompt for the model to generate a response from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// When true, returns a stream of partial responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Base64-encoded images for models that support image input
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,

    /// Structured output format for the model to generate a response from.
    /// Supports either the string "json" or a JSON schema object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Value>,

    /// When true, returns separate thinking output in addition to content. Can be a boolean
    /// (true/false) or a string ("high", "medium", "low") for supported models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<Think>,

    /// Runtime options that control text generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
}

impl GenerateRequest {
    pub fn builder<M: Into<String>>(model: M) -> GenerateRequestBuilder {
        GenerateRequestBuilder::new(model)
    }
}

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

    pub fn system_prompt<P: Into<String>>(mut self, system_prompt: P) -> Self {
        self.generate_request.system = Some(system_prompt.into());
        self
    }

    pub fn prompt<P: Into<String>>(mut self, prompt: P) -> Self {
        self.generate_request.prompt = Some(prompt.into());
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.generate_request.stream = Some(stream);
        self
    }

    pub fn suffix(mut self, suffix: String) -> Self {
        self.generate_request.suffix = Some(suffix);
        self
    }

    pub fn images(mut self, images: Vec<String>) -> Self {
        self.generate_request.images = images;
        self
    }

    pub fn format<T: Into<Value>>(mut self, value: T) -> Self {
        self.generate_request.format = Some(value.into());
        self
    }

    pub fn think(mut self, think: Think) -> Self {
        self.generate_request.think = Some(think);
        self
    }

    pub fn options(mut self, options: Options) -> Self {
        self.generate_request.options = Some(options);
        self
    }

    pub fn build(self) -> GenerateRequest {
        self.generate_request
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    /// Model name
    pub model: String,

    /// ISO 8601 timestamp of response creation
    pub created_at: String,

    /// The model's generated text response
    pub response: String,

    /// The model's generated thinking output
    pub thinking: Option<String>,

    /// Indicates whether generation has finished
    pub done: bool,

    /// Reason the generation stopped
    pub done_reason: Option<String>,

    /// Time spent generating the response in nanoseconds
    pub total_duration: Option<u64>,

    /// Time spent loading the model in nanoseconds
    pub load_duration: Option<u64>,

    /// Number of input tokens in the prompt
    pub prompt_eval_count: Option<u64>,

    /// Time spent evaluating the prompt in nanoseconds
    pub prompt_eval_duration: Option<u64>,

    /// Number of output tokens generated in the response
    pub eval_count: Option<u64>,

    /// Time spent generating tokens in nanoseconds
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
