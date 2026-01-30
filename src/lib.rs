use std::time::Duration;

use async_stream::stream;
use futures_util::{Stream, StreamExt};
use serde::{Serialize, de::DeserializeOwned};
use tokio_util::{
    codec::{FramedRead, LinesCodec},
    io::StreamReader,
};
use tracing::{debug, info};

use crate::{
    error::{OllamaError, OllamaResult},
    types::{
        chat::{ChatRequest, ChatResponse},
        generate::{GenerateRequest, GenerateResponse},
        ps::PsResponse,
        pull::{PullRequest, PullResponse},
        tags::TagsResponse,
        version::VersionResponse,
    },
};

pub mod error;
pub mod types;

const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub struct OllamaClient {
    server_address: String,
    client: reqwest::Client,
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self::new("http://localhost:11434")
    }
}

impl OllamaClient {
    pub fn new<S: AsRef<str>>(server_address: S) -> Self {
        Self {
            server_address: server_address.as_ref().to_string(),
            client: reqwest::Client::builder()
                .connect_timeout(DEFAULT_CONNECTION_TIMEOUT)
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    pub fn builder<S: AsRef<str>>(server_address: S) -> OllamaClientBuilder {
        OllamaClientBuilder {
            server_address: server_address.as_ref().to_string(),
            connection_timeout: DEFAULT_CONNECTION_TIMEOUT,
        }
    }

    /// Retrieve the version of the Ollama
    pub async fn version(&self) -> OllamaResult<VersionResponse> {
        let request_address = format!("{}/api/version", self.server_address);
        Ok(self
            .client
            .get(request_address)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Fetch a list of models and their details
    pub async fn tags(&self) -> OllamaResult<TagsResponse> {
        let request_address = format!("{}/api/tags", self.server_address);
        info!("List models: {}", request_address);
        Ok(self
            .client
            .get(request_address)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Retrieve a list of models that are currently running
    pub async fn ps(&self) -> OllamaResult<PsResponse> {
        let request_address = format!("{}/api/ps", self.server_address);
        info!("List models: {}", request_address);
        Ok(self
            .client
            .get(request_address)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    fn stream_response<R: Serialize, T: DeserializeOwned>(
        &self,
        endpoint: String,
        request: R,
    ) -> impl Stream<Item = OllamaResult<T>> {
        let client = self.client.clone();
        Box::pin(stream! {
            let response = client
                .post(endpoint)
                .json(&request)
                .send()
                .await?
                .error_for_status()?;

            debug!("{:?}", response);
            let bytes_stream = response.bytes_stream();

            let body_reader = StreamReader::new(
                bytes_stream.map(|res| res.map_err(std::io::Error::other)),
            );

            let mut lines_stream = FramedRead::new(body_reader, LinesCodec::new());

            while let Some(line_result) = lines_stream.next().await {
                match line_result {
                    Ok(line_content) => {
                        debug!(chunk = line_content, "ollama response chunk");
                        if !line_content.is_empty() {
                            yield serde_json::from_str::<T>(&line_content)
                                .map_err(OllamaError::from);
                        }
                    }
                    Err(e) => yield Err(OllamaError::from(e)),
                }
            }
        })
    }

    /// Generates a response for the provided prompt
    pub fn generate(
        &self,
        request: GenerateRequest,
    ) -> impl Stream<Item = OllamaResult<GenerateResponse>> {
        let request_address = format!("{}/api/generate", self.server_address);
        self.stream_response(request_address, request)
    }

    /// Generate the next chat message in a conversation between a user and an assistant.
    pub fn chat(&self, request: ChatRequest) -> impl Stream<Item = OllamaResult<ChatResponse>> {
        let request_address = format!("{}/api/chat", self.server_address);
        self.stream_response(request_address, request)
    }

    /// Pull a model
    pub fn pull(&self, request: PullRequest) -> impl Stream<Item = OllamaResult<PullResponse>> {
        let request_address = format!("{}/api/pull", self.server_address);
        self.stream_response(request_address, request)
    }
}

pub struct OllamaClientBuilder {
    server_address: String,
    connection_timeout: Duration,
}

impl OllamaClientBuilder {
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    pub fn build(self) -> OllamaClient {
        OllamaClient {
            server_address: self.server_address,
            client: reqwest::Client::builder()
                .connect_timeout(self.connection_timeout)
                .build()
                .expect("failed to build reqwest client"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_client() {
        let client = OllamaClient::new("http://localhost:11434");
        assert_eq!(client.server_address, "http://localhost:11434");
    }

    #[test]
    fn default_creates_localhost_client() {
        let client = OllamaClient::default();
        assert_eq!(client.server_address, "http://localhost:11434");
    }

    #[test]
    fn builder_creates_client() {
        let client = OllamaClient::builder("http://myserver:11434").build();
        assert_eq!(client.server_address, "http://myserver:11434");
    }

    #[test]
    fn builder_with_custom_timeout() {
        let client = OllamaClient::builder("http://localhost:11434")
            .connection_timeout(Duration::from_secs(60))
            .build();
        assert_eq!(client.server_address, "http://localhost:11434");
    }
}
