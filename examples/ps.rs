// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

use std::{env, error::Error};

use ollama_rs::OllamaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);
    let result = ollama_client.ps().await?;
    for model in result.models {
        println!("{:?}", model);
    }
    Ok(())
}
