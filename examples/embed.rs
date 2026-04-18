// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

use std::{env, error::Error};

use ollama_rs::OllamaClient;
use ollama_rs::types::embed::EmbedRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);
    let request = EmbedRequest::builder("embeddinggemma")
        .input("Generate embeddings for this text")
        .build();

    let response = ollama_client.embed(request).await?;
    for (i, embedding) in response.embeddings.iter().enumerate() {
        println!("Embedding {}: {} dimensions", i, embedding.len());
        if embedding.len() >= 3 {
            println!("  First 3 values: {:?}", &embedding[..3]);
        }
    }
    Ok(())
}
