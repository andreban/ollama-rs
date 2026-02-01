use std::{env, error::Error};

use ollama_rs::OllamaClient;
use ollama_rs::types::delete::DeleteRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let model = env::args().nth(1).expect("usage: delete <model>");
    let ollama_client = OllamaClient::new(server_address);
    ollama_client.delete(DeleteRequest::new(&model)).await?;
    println!("Model '{}' deleted successfully", model);
    Ok(())
}
