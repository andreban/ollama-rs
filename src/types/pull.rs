use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl PullRequest {
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

pub struct PullRequestBuilder {
    pull_request: PullRequest,
}

impl PullRequestBuilder {
    pub fn stream(mut self, stream: bool) -> Self {
        self.pull_request.stream = Some(stream);
        self
    }

    pub fn insecure(mut self, insecure: bool) -> Self {
        self.pull_request.insecure = Some(insecure);
        self
    }

    pub fn build(self) -> PullRequest {
        self.pull_request
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullResponse {
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
