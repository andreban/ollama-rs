//! # ollama-rs
//!
//! An async Rust client library for the [Ollama](https://ollama.com/) API.
//!
//! This crate provides a streaming-first interface for interacting with Ollama,
//! supporting text generation, multi-turn chat conversations, model management,
//! structured JSON output, and tool calling.
//!
//! ## Quick Start
//!
//! ```no_run
//! use ollama_rs::OllamaClient;
//! use ollama_rs::types::generate::GenerateRequest;
//! use futures_util::StreamExt;
//!
//! # async fn run() -> ollama_rs::error::OllamaResult<()> {
//! let client = OllamaClient::default();
//!
//! let request = GenerateRequest::builder("llama3")
//!     .prompt("Why is the sky blue?")
//!     .build();
//!
//! let mut stream = client.generate(request);
//! while let Some(chunk) = stream.next().await {
//!     let response = chunk?;
//!     print!("{}", response.response);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Chat Conversations
//!
//! ```no_run
//! use ollama_rs::OllamaClient;
//! use ollama_rs::types::chat::{ChatRequest, Message};
//! use futures_util::StreamExt;
//!
//! # async fn run() -> ollama_rs::error::OllamaResult<()> {
//! let client = OllamaClient::default();
//!
//! let request = ChatRequest::builder("llama3")
//!     .messages(vec![
//!         Message::system("You are a helpful assistant."),
//!         Message::user("Hello!"),
//!     ])
//!     .build();
//!
//! let mut stream = client.chat(request);
//! while let Some(chunk) = stream.next().await {
//!     let response = chunk?;
//!     print!("{}", response.message.content);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Custom Client Configuration
//!
//! ```
//! use ollama_rs::OllamaClient;
//! use std::time::Duration;
//!
//! // Connect to a remote server with a custom timeout
//! let client = OllamaClient::builder("http://my-server:11434")
//!     .connection_timeout(Duration::from_secs(60))
//!     .build();
//! ```
//!
//! ## Features
//!
//! - **Streaming responses** -- all generation endpoints return [`futures_util::Stream`]s,
//!   allowing token-by-token processing.
//! - **Builder pattern** -- all request types use builders for ergonomic construction.
//! - **Multi-turn chat** -- maintain conversation context with [`types::chat::ChatRequest`].
//! - **Structured output** -- request JSON responses conforming to a schema via the `format` field.
//! - **Tool calling** -- let the model invoke functions with [`types::chat::Tool`] definitions.
//! - **Thinking mode** -- enable extended reasoning with [`types::common::Think`].
//! - **Model management** -- list, pull, and inspect models.

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
        delete::DeleteRequest,
        generate::{GenerateRequest, GenerateResponse},
        ps::PsResponse,
        pull::{PullRequest, PullResponse},
        tags::TagsResponse,
        version::VersionResponse,
    },
};

pub mod error;
pub mod types;

/// Default connection timeout applied when no custom timeout is specified (30 seconds).
const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// An async client for communicating with an Ollama server.
///
/// `OllamaClient` is the primary entry point for this crate. It wraps an HTTP client
/// and provides methods for every Ollama API endpoint.
///
/// # Construction
///
/// There are three ways to create a client:
///
/// - [`OllamaClient::default()`] -- connects to `http://localhost:11434` with a 30-second timeout.
/// - [`OllamaClient::new()`] -- connects to a custom address with a 30-second timeout.
/// - [`OllamaClient::builder()`] -- full control over address and connection timeout.
///
/// # Examples
///
/// ```
/// use ollama_rs::OllamaClient;
///
/// // Default local connection
/// let client = OllamaClient::default();
///
/// // Custom server address
/// let client = OllamaClient::new("http://my-server:11434");
/// ```
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
    /// Creates a new client connected to the given server address.
    ///
    /// Uses the default connection timeout of 30 seconds. For custom timeouts,
    /// use [`OllamaClient::builder()`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama_rs::OllamaClient;
    ///
    /// let client = OllamaClient::new("http://localhost:11434");
    /// ```
    pub fn new<S: AsRef<str>>(server_address: S) -> Self {
        Self {
            server_address: server_address.as_ref().to_string(),
            client: reqwest::Client::builder()
                .connect_timeout(DEFAULT_CONNECTION_TIMEOUT)
                .build()
                .expect("failed to build reqwest client"),
        }
    }

    /// Returns an [`OllamaClientBuilder`] for constructing a client with custom settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama_rs::OllamaClient;
    /// use std::time::Duration;
    ///
    /// let client = OllamaClient::builder("http://localhost:11434")
    ///     .connection_timeout(Duration::from_secs(60))
    ///     .build();
    /// ```
    pub fn builder<S: AsRef<str>>(server_address: S) -> OllamaClientBuilder {
        OllamaClientBuilder {
            server_address: server_address.as_ref().to_string(),
            connection_timeout: DEFAULT_CONNECTION_TIMEOUT,
        }
    }

    /// Retrieves the version of the connected Ollama server.
    ///
    /// Calls `GET /api/version`.
    ///
    /// # Errors
    ///
    /// Returns [`OllamaError::NetworkError`] if the server is unreachable or returns
    /// a non-success status code.
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

    /// Lists all models available on the Ollama server along with their details.
    ///
    /// Calls `GET /api/tags`.
    ///
    /// # Errors
    ///
    /// Returns [`OllamaError::NetworkError`] if the server is unreachable or returns
    /// a non-success status code.
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

    /// Lists models that are currently loaded and running on the Ollama server.
    ///
    /// Calls `GET /api/ps`.
    ///
    /// # Errors
    ///
    /// Returns [`OllamaError::NetworkError`] if the server is unreachable or returns
    /// a non-success status code.
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

    /// Deletes a model from the Ollama server.
    ///
    /// Calls `DELETE /api/delete`.
    ///
    /// # Errors
    ///
    /// Returns [`OllamaError::NetworkError`] if the server is unreachable, the model
    /// does not exist, or the server returns a non-success status code.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ollama_rs::OllamaClient;
    /// # use ollama_rs::types::delete::DeleteRequest;
    /// # async fn run() -> ollama_rs::error::OllamaResult<()> {
    /// let client = OllamaClient::default();
    /// let request = DeleteRequest::new("gemma3");
    /// client.delete(request).await?;
    /// println!("Model deleted successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, request: DeleteRequest) -> OllamaResult<()> {
        let request_address = format!("{}/api/delete", self.server_address);
        info!("Delete model: {}", request.model);
        self.client
            .delete(request_address)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
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

    /// Generates a text completion for the given prompt.
    ///
    /// Returns a stream of [`GenerateResponse`] chunks. Each chunk contains a
    /// fragment of the generated text. The final chunk has `done` set to `true`
    /// and includes timing and token count statistics.
    ///
    /// Calls `POST /api/generate`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ollama_rs::OllamaClient;
    /// # use ollama_rs::types::generate::GenerateRequest;
    /// # use futures_util::StreamExt;
    /// # async fn run() -> ollama_rs::error::OllamaResult<()> {
    /// let client = OllamaClient::default();
    /// let request = GenerateRequest::builder("llama3")
    ///     .prompt("Explain quantum computing in one sentence.")
    ///     .build();
    ///
    /// let mut stream = client.generate(request);
    /// while let Some(chunk) = stream.next().await {
    ///     print!("{}", chunk?.response);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate(
        &self,
        request: GenerateRequest,
    ) -> impl Stream<Item = OllamaResult<GenerateResponse>> {
        let request_address = format!("{}/api/generate", self.server_address);
        self.stream_response(request_address, request)
    }

    /// Generates the next message in a multi-turn chat conversation.
    ///
    /// Returns a stream of [`ChatResponse`] chunks. Each chunk contains a partial
    /// [`Message`](types::chat::Message) from the assistant. The final chunk has
    /// `done` set to `true`.
    ///
    /// Calls `POST /api/chat`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ollama_rs::OllamaClient;
    /// # use ollama_rs::types::chat::{ChatRequest, Message};
    /// # use futures_util::StreamExt;
    /// # async fn run() -> ollama_rs::error::OllamaResult<()> {
    /// let client = OllamaClient::default();
    /// let request = ChatRequest::builder("llama3")
    ///     .messages(vec![Message::user("What is 2 + 2?")])
    ///     .build();
    ///
    /// let mut stream = client.chat(request);
    /// while let Some(chunk) = stream.next().await {
    ///     print!("{}", chunk?.message.content);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn chat(&self, request: ChatRequest) -> impl Stream<Item = OllamaResult<ChatResponse>> {
        let request_address = format!("{}/api/chat", self.server_address);
        self.stream_response(request_address, request)
    }

    /// Downloads (pulls) a model from the Ollama registry.
    ///
    /// Returns a stream of [`PullResponse`] chunks reporting the download status.
    ///
    /// Calls `POST /api/pull`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ollama_rs::OllamaClient;
    /// # use ollama_rs::types::pull::PullRequest;
    /// # use futures_util::StreamExt;
    /// # async fn run() -> ollama_rs::error::OllamaResult<()> {
    /// let client = OllamaClient::default();
    /// let request = PullRequest::builder("llama3").build();
    ///
    /// let mut stream = client.pull(request);
    /// while let Some(chunk) = stream.next().await {
    ///     println!("{}", chunk?.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn pull(&self, request: PullRequest) -> impl Stream<Item = OllamaResult<PullResponse>> {
        let request_address = format!("{}/api/pull", self.server_address);
        self.stream_response(request_address, request)
    }
}

/// A builder for constructing an [`OllamaClient`] with custom configuration.
///
/// Obtain a builder via [`OllamaClient::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::OllamaClient;
/// use std::time::Duration;
///
/// let client = OllamaClient::builder("http://localhost:11434")
///     .connection_timeout(Duration::from_secs(10))
///     .build();
/// ```
pub struct OllamaClientBuilder {
    server_address: String,
    connection_timeout: Duration,
}

impl OllamaClientBuilder {
    /// Sets the TCP connection timeout for the underlying HTTP client.
    ///
    /// Defaults to 30 seconds if not specified.
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// Consumes the builder and returns a configured [`OllamaClient`].
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
