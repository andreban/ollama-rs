use std::{error::Error, io::Write};

use clap::Parser;
use futures_util::StreamExt;
use ollama_rs::{OllamaClient, types::pull::PullRequest};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The model to pull
    #[arg(short, long)]
    model: String,

    /// The Ollama server address
    #[arg(
        short,
        long,
        env = "OLLAMA_SERVER",
        default_value = "http://localhost:11434"
    )]
    server_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();
    let ollama_client = OllamaClient::new(args.server_address);

    let request = PullRequest::builder(&args.model).stream(true).build();
    let mut stream = ollama_client.pull(request);
    while let Some(response) = stream.next().await {
        let response = response?;
        println!("{}", response.status);
        std::io::stdout().flush()?;
    }
    Ok(())
}
