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
