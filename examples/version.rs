use std::{env, error::Error};

use ollama_rs::OllamaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);
    let version_response = ollama_client.version().await?;
    println!("{}", version_response.version);
    Ok(())
}
