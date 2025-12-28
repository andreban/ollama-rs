use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Think {
    Bool(bool),
    Level(ThinkLevel),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Options {
    /// Random seed used for reproducible outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// Controls randomness in generation (higher = more random)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Limits next token selection to the K most likely
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    /// Cumulative probability threshold for nucleus sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Minimum probability threshold for token selection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,

    /// Stop sequences that will halt generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,

    /// Context length size (number of tokens)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<u32>,

    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<u32>,
}

impl Options {
    pub fn builder() -> OptionsBuilder {
        OptionsBuilder {
            options: Options::default(),
        }
    }
}

pub struct OptionsBuilder {
    options: Options,
}

impl OptionsBuilder {
    pub fn seed(mut self, seed: u64) -> Self {
        self.options.seed = Some(seed);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.options.temperature = Some(temperature);
        self
    }

    pub fn top_k(mut self, top_k: u32) -> Self {
        self.options.top_k = Some(top_k);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.options.top_p = Some(top_p);
        self
    }

    pub fn min_p(mut self, min_p: f32) -> Self {
        self.options.min_p = Some(min_p);
        self
    }

    pub fn stop(mut self, stop: Stop) -> Self {
        self.options.stop = Some(stop);
        self
    }

    pub fn num_ctx(mut self, num_ctx: u32) -> Self {
        self.options.num_ctx = Some(num_ctx);
        self
    }

    pub fn num_predict(mut self, num_predict: u32) -> Self {
        self.options.num_predict = Some(num_predict);
        self
    }

    pub fn build(self) -> Options {
        self.options
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Stop {
    Single(String),
    Multiple(Vec<String>),
}
