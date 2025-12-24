use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// Model name
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]

    /// Text for the model to generate a response from
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]

    /// Used for fill-in-the-middle models, text that appears after the user prompt and before the
    /// model response
    pub suffix: Option<String>,

    /// System prompt for the model to generate a response from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
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
    pub total_duration: Option<usize>,

    /// Time spent loading the model in nanoseconds
    pub load_duration: Option<usize>,

    /// Number of input tokens in the prompt
    pub prompt_eval_count: Option<usize>,

    /// Time spent evaluating the prompt in nanoseconds
    pub prompt_eval_duration: Option<usize>,

    /// Number of output tokens generated in the response
    pub eval_count: Option<usize>,

    /// Time spent generating tokens in nanoseconds
    pub eval_duration: Option<usize>,
}
