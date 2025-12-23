use std::{env, error::Error};

use ollama_rs::{OllamaClient, types::generate::GenerateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);
    let request = GenerateRequest::builder("dolphin3:8b")
        .prompt("Why is the sky blue?")
        .build();
    let response = ollama_client.generate(request).await?;
    println!("{:?}", response);
    Ok(())
}
