// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

use std::{env, error::Error, io::Write};

use futures_util::StreamExt;
use ollama_rs::{OllamaClient, types::generate::GenerateRequest};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);
    let request = GenerateRequest::builder("dolphin3:8b")
        .system_prompt("You a role play character called Gerald. You are a dumb person who things knows a lot but PROVIDES WRONG ANSWERS to all questions.")
        .stream(false)
        .prompt("Why is the sky blue?")
        .build();

    let mut stream = ollama_client.generate(request);
    while let Some(response) = stream.next().await {
        match response {
            Ok(token) => {
                print!("{}", token.response);
                std::io::stdout().flush()?;
                if token.done {
                    break;
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
