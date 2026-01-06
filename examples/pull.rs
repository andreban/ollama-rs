use std::{env, error::Error, io::Write};

use futures_util::StreamExt;
use ollama_rs::{OllamaClient, types::pull::PullRequest};

const MODEL: &str = "functiongemma";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);

    let request = PullRequest::builder(MODEL).stream(true).build();
    let mut stream = ollama_client.pull(request);
    while let Some(response) = stream.next().await {
        let response = response?;
        println!("{:?}", response);
        std::io::stdout().flush()?;
    }
    Ok(())
}
