//! Types shared across multiple Ollama API endpoints.
//!
//! This module provides:
//!
//! - [`Options`] / [`OptionsBuilder`] -- sampling and generation parameters.
//! - [`Think`] / [`ThinkLevel`] -- controls for extended-thinking (reasoning) mode.
//! - [`Stop`] -- stop-sequence configuration.
//! - [`ModelDetails`] -- metadata returned when listing models.

use serde::{Deserialize, Serialize};

/// Detailed metadata about a model, returned by the tags and ps endpoints.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDetails {
    /// The model file format (e.g., `"gguf"`).
    pub format: String,
    /// The primary model family (e.g., `"llama"`).
    pub family: String,
    /// Additional model families, if any (e.g., `["llama", "clip"]`).
    pub families: Option<Vec<String>>,
    /// Human-readable parameter count (e.g., `"8B"`).
    pub parameter_size: String,
    /// Quantization level (e.g., `"Q4_0"`).
    pub quantization_level: String,
}

/// Controls extended-thinking (reasoning) mode for supported models.
///
/// Can be a simple boolean toggle or a named level. Serialized as an untagged
/// enum so `true`, `false`, `"high"`, `"medium"`, and `"low"` are all valid JSON
/// representations.
///
/// # Examples
///
/// ```
/// use ollama_rs::types::common::Think;
///
/// // Enable thinking
/// let think = Think::Bool(true);
///
/// // Use a specific thinking level
/// use ollama_rs::types::common::ThinkLevel;
/// let think = Think::Level(ThinkLevel::High);
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Think {
    /// Enable (`true`) or disable (`false`) thinking mode.
    Bool(bool),
    /// Use a named thinking intensity level.
    Level(ThinkLevel),
}

/// Named intensity levels for extended-thinking mode.
///
/// Serialized as lowercase strings: `"high"`, `"medium"`, `"low"`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkLevel {
    /// Maximum reasoning depth.
    High,
    /// Balanced reasoning depth.
    Medium,
    /// Minimal reasoning depth.
    Low,
}

/// Runtime options that control text generation behavior.
///
/// All fields are optional. Only fields set to `Some` are included in the
/// serialized JSON request, letting the server apply its own defaults for
/// omitted parameters.
///
/// Use [`Options::builder()`] for ergonomic construction.
///
/// # Examples
///
/// ```
/// use ollama_rs::types::common::{Options, Stop};
///
/// let options = Options::builder()
///     .temperature(0.7)
///     .top_k(40)
///     .stop(Stop::Single("END".to_string()))
///     .build();
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Options {
    /// Random seed for reproducible outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// Controls randomness in generation. Higher values (e.g., `1.5`) produce
    /// more creative output; lower values (e.g., `0.2`) produce more
    /// deterministic output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Limits the next-token selection to the *K* most likely tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    /// Cumulative probability threshold for nucleus sampling.
    /// A value of `0.9` means only the smallest set of tokens whose cumulative
    /// probability exceeds 90% are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Minimum probability threshold for token selection.
    /// Tokens with probability below this value are discarded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,

    /// One or more stop sequences that will halt generation when produced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Stop>,

    /// Context window size in tokens. Determines how many tokens the model
    /// can attend to at once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<u32>,

    /// Maximum number of tokens to generate in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<u32>,
}

impl Options {
    /// Returns an [`OptionsBuilder`] for constructing an `Options` value.
    pub fn builder() -> OptionsBuilder {
        OptionsBuilder {
            options: Options::default(),
        }
    }
}

/// A builder for constructing [`Options`] with only the desired parameters set.
///
/// Obtain a builder via [`Options::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::common::Options;
///
/// let options = Options::builder()
///     .seed(42)
///     .temperature(0.8)
///     .num_predict(256)
///     .build();
/// ```
pub struct OptionsBuilder {
    options: Options,
}

impl OptionsBuilder {
    /// Sets the random seed for reproducible outputs.
    pub fn seed(mut self, seed: u64) -> Self {
        self.options.seed = Some(seed);
        self
    }

    /// Sets the temperature for generation randomness.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.options.temperature = Some(temperature);
        self
    }

    /// Sets the top-K sampling parameter.
    pub fn top_k(mut self, top_k: u32) -> Self {
        self.options.top_k = Some(top_k);
        self
    }

    /// Sets the nucleus sampling probability threshold.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.options.top_p = Some(top_p);
        self
    }

    /// Sets the minimum probability threshold for token selection.
    pub fn min_p(mut self, min_p: f32) -> Self {
        self.options.min_p = Some(min_p);
        self
    }

    /// Sets one or more stop sequences.
    pub fn stop(mut self, stop: Stop) -> Self {
        self.options.stop = Some(stop);
        self
    }

    /// Sets the context window size in tokens.
    pub fn num_ctx(mut self, num_ctx: u32) -> Self {
        self.options.num_ctx = Some(num_ctx);
        self
    }

    /// Sets the maximum number of tokens to generate.
    pub fn num_predict(mut self, num_predict: u32) -> Self {
        self.options.num_predict = Some(num_predict);
        self
    }

    /// Consumes the builder and returns the configured [`Options`].
    pub fn build(self) -> Options {
        self.options
    }
}

/// Stop sequences that halt text generation when produced by the model.
///
/// Serialized as an untagged enum: a single string or an array of strings.
///
/// # Examples
///
/// ```
/// use ollama_rs::types::common::Stop;
///
/// let single = Stop::Single("END".to_string());
/// let multiple = Stop::Multiple(vec!["END".to_string(), "STOP".to_string()]);
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Stop {
    /// A single stop sequence.
    Single(String),
    /// Multiple stop sequences.
    Multiple(Vec<String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn think_bool_true() {
        let think: Think = serde_json::from_value(json!(true)).unwrap();
        assert!(matches!(think, Think::Bool(true)));
    }

    #[test]
    fn think_bool_false() {
        let think: Think = serde_json::from_value(json!(false)).unwrap();
        assert!(matches!(think, Think::Bool(false)));
    }

    #[test]
    fn think_level_high() {
        let think: Think = serde_json::from_value(json!("high")).unwrap();
        assert!(matches!(think, Think::Level(ThinkLevel::High)));
    }

    #[test]
    fn think_level_medium() {
        let think: Think = serde_json::from_value(json!("medium")).unwrap();
        assert!(matches!(think, Think::Level(ThinkLevel::Medium)));
    }

    #[test]
    fn think_level_low() {
        let think: Think = serde_json::from_value(json!("low")).unwrap();
        assert!(matches!(think, Think::Level(ThinkLevel::Low)));
    }

    #[test]
    fn think_bool_round_trip() {
        let think = Think::Bool(true);
        let json = serde_json::to_value(&think).unwrap();
        assert_eq!(json, json!(true));
    }

    #[test]
    fn think_level_round_trip() {
        let think = Think::Level(ThinkLevel::High);
        let json = serde_json::to_value(&think).unwrap();
        assert_eq!(json, json!("high"));
    }

    #[test]
    fn stop_single() {
        let stop: Stop = serde_json::from_value(json!("end")).unwrap();
        assert!(matches!(stop, Stop::Single(s) if s == "end"));
    }

    #[test]
    fn stop_multiple() {
        let stop: Stop = serde_json::from_value(json!(["end", "stop"])).unwrap();
        match stop {
            Stop::Multiple(v) => assert_eq!(v, vec!["end", "stop"]),
            _ => panic!("expected Stop::Multiple"),
        }
    }

    #[test]
    fn stop_single_round_trip() {
        let stop = Stop::Single("end".to_string());
        let json = serde_json::to_value(&stop).unwrap();
        assert_eq!(json, json!("end"));
    }

    #[test]
    fn stop_multiple_round_trip() {
        let stop = Stop::Multiple(vec!["end".to_string(), "stop".to_string()]);
        let json = serde_json::to_value(&stop).unwrap();
        assert_eq!(json, json!(["end", "stop"]));
    }

    #[test]
    fn options_default_serializes_empty() {
        let options = Options::default();
        let json = serde_json::to_value(&options).unwrap();
        assert_eq!(json, json!({}));
    }

    #[test]
    fn options_skips_none_fields() {
        let options = Options::builder().seed(42).temperature(0.5).build();
        let json = serde_json::to_value(&options).unwrap();
        assert_eq!(json, json!({"seed": 42, "temperature": 0.5}));
        assert!(!json.as_object().unwrap().contains_key("top_k"));
    }

    #[test]
    fn options_builder_all_fields() {
        let options = Options::builder()
            .seed(42)
            .temperature(0.7)
            .top_k(40)
            .top_p(0.9)
            .min_p(0.05)
            .stop(Stop::Single("end".to_string()))
            .num_ctx(4096)
            .num_predict(128)
            .build();

        assert_eq!(options.seed, Some(42));
        assert_eq!(options.temperature, Some(0.7));
        assert_eq!(options.top_k, Some(40));
        assert_eq!(options.top_p, Some(0.9));
        assert_eq!(options.min_p, Some(0.05));
        assert!(options.stop.is_some());
        assert_eq!(options.num_ctx, Some(4096));
        assert_eq!(options.num_predict, Some(128));
    }

    #[test]
    fn options_round_trip() {
        let json = json!({
            "seed": 42,
            "temperature": 0.5,
            "top_k": 40,
            "num_ctx": 4096
        });
        let options: Options = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(options.seed, Some(42));
        assert_eq!(options.temperature, Some(0.5));
        assert_eq!(options.top_k, Some(40));
        assert_eq!(options.num_ctx, Some(4096));
        assert_eq!(options.top_p, None);

        let reserialized = serde_json::to_value(&options).unwrap();
        assert_eq!(reserialized, json);
    }

    #[test]
    fn model_details_round_trip() {
        let json = json!({
            "format": "gguf",
            "family": "llama",
            "families": ["llama", "clip"],
            "parameter_size": "8B",
            "quantization_level": "Q4_0"
        });
        let details: ModelDetails = serde_json::from_value(json).unwrap();
        assert_eq!(details.format, "gguf");
        assert_eq!(details.family, "llama");
        assert_eq!(
            details.families,
            Some(vec!["llama".to_string(), "clip".to_string()])
        );
        assert_eq!(details.parameter_size, "8B");
        assert_eq!(details.quantization_level, "Q4_0");
    }

    #[test]
    fn model_details_without_families() {
        let json = json!({
            "format": "gguf",
            "family": "llama",
            "parameter_size": "8B",
            "quantization_level": "Q4_0"
        });
        let details: ModelDetails = serde_json::from_value(json).unwrap();
        assert_eq!(details.families, None);
    }
}
