use futures_util::{StreamExt};
use serde_json::Value;
use tokio_util::io::StreamReader;
use tracing::info;

use crate::{
    error::OllamaResult,
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
    pub async fn generate(&self, request: GenerateRequest) -> OllamaResult<()> {
        let request_address = format!("{}/api/generate", self.server_address);
        let client = reqwest::Client::new();
        let response = client
            .post(request_address)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;
        let stream = response.bytes_stream().;
        // let reader = BufReader::new(stream);
        let reader = StreamReader(stream);
        while reader
        while let Some(item) = stream.next().await {
            let item = item?;
            println!("Chunk: {:?}", item?);
        }

        // let stream_reader = tokio_util::io::StreamReader::new(stream);
        // let reder = BufReader::new(stream);
        // let full_response = response.text().await?;
        // let parts = full_response
        //     .lines()
        //     .map(|line| serde_json::from_str::<GenerateResponse>(line).unwrap())
        //     .collect::<Vec<_>>();
        // println!("{:#?}", parts);
        Ok(())
    }
}
