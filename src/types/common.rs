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
