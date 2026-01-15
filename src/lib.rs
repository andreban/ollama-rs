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

#[derive(Clone)]
pub struct OllamaClient {
    server_address: String,
}

impl OllamaClient {
    pub fn new<S: AsRef<str>>(server_address: S) -> Self {
        Self {
            server_address: server_address.as_ref().to_string(),
        }
    }

    /// Retrieve the version of the Ollama
    pub async fn version(&self) -> OllamaResult<VersionResponse> {
        let request_address = format!("{}/api/version", self.server_address);
        Ok(reqwest::get(request_address)
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Fetch a list of models and their details
    pub async fn tags(&self) -> OllamaResult<TagsResponse> {
        let request_address = format!("{}/api/tags", self.server_address);
        info!("List models: {}", request_address);
        Ok(reqwest::get(request_address)
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Retrieve a list of models that are currently running
    pub async fn ps(&self) -> OllamaResult<PsResponse> {
        let request_address = format!("{}/api/ps", self.server_address);
        info!("List models: {}", request_address);
        Ok(reqwest::get(request_address)
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
        let client = reqwest::Client::new();
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
                        if let Ok(parsed) = serde_json::from_str::<T>(&line_content) {
                            yield Ok(parsed);
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
