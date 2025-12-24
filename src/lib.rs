use async_stream::stream;
use futures_util::{Stream, StreamExt};
use serde_json::Value;
use tokio_util::{
    codec::{FramedRead, LinesCodec},
    io::StreamReader,
};
use tracing::info;

use crate::{
    error::{OllamaError, OllamaResult},
    types::{
        generate::{GenerateRequest, GenerateResponse},
        ps::RunningModel,
        tags::Model,
    },
};

pub mod error;
pub mod types;

pub struct OllamaClient {
    server_address: String,
}

impl OllamaClient {
    pub fn new<S: AsRef<str>>(server_address: S) -> Self {
        Self {
            server_address: server_address.as_ref().to_string(),
        }
    }

    /// Fetch a list of models and their details
    pub async fn tags(&self) -> OllamaResult<Vec<Model>> {
        let request_address = format!("{}/api/tags", self.server_address);
        info!("List models: {}", request_address);
        let mut response: Value = reqwest::get(request_address)
            .await?
            .error_for_status()?
            .json()
            .await?;

        let Some(response) = response.as_object_mut() else {
            return Ok(vec![]);
        };

        let Some(models) = response.remove("models") else {
            return Ok(vec![]);
        };

        let models = serde_json::from_value(models)?;
        Ok(models)
    }

    /// Retrieve a list of models that are currently running
    pub async fn list_runnning_models(&self) -> OllamaResult<Vec<RunningModel>> {
        let request_address = format!("{}/api/ps", self.server_address);
        info!("List models: {}", request_address);
        let mut response: Value = reqwest::get(request_address)
            .await?
            .error_for_status()?
            .json()
            .await?;

        let Some(response) = response.as_object_mut() else {
            return Ok(vec![]);
        };

        let Some(models) = response.remove("models") else {
            return Ok(vec![]);
        };

        let models = serde_json::from_value(models)?;
        Ok(models)
    }

    /// Generates a response for the provided prompt
    pub async fn generate(
        &self,
        request: GenerateRequest,
    ) -> impl Stream<Item = OllamaResult<GenerateResponse>> {
        let request_address = format!("{}/api/generate", self.server_address);
        let client = reqwest::Client::new();

        // The stream macro creates an asynchronous generator
        Box::pin(stream! {
            let response = client
                .post(request_address)
                .json(&request)
                .send()
                .await
                .map_err(|e| OllamaError::from(e))?; // Adjust based on your error type

            let bytes_stream = response.bytes_stream();

            let body_reader = StreamReader::new(
                bytes_stream.map(|res| res.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))),
            );

            let mut lines_stream = FramedRead::new(body_reader, LinesCodec::new());

            while let Some(line_result) = lines_stream.next().await {
                match line_result {
                    Ok(line_content) => {
                        if let Ok(parsed) = serde_json::from_str::<GenerateResponse>(&line_content) {
                            let done = parsed.done;
                            yield Ok(parsed);
                            if done { break; }
                        }
                    }
                    Err(e) => yield Err(OllamaError::from(e)),
                }
            }
        })
    }
}
