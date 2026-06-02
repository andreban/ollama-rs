// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

//! Streams a chat response from a thinking-capable model and prints the
//! reasoning ("thinking") and the visible answer separately.
//!
//! Requires `OLLAMA_SERVER` (e.g. `http://localhost:11434`) and a model that
//! supports thinking mode, such as `gemma4:e2b`, `gpt-oss`, or `deepseek-r1`.

use std::{env, error::Error, io::Write};

use futures_util::StreamExt;
use ollama_rs::{
    OllamaClient,
    types::{
        chat::{ChatRequest, Message},
        common::Think,
    },
};

const MODEL: &str = "gemma4:e2b";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);

    let request = ChatRequest::builder(MODEL)
        .messages(vec![Message::user(
            "A bat and a ball cost $1.10 total. The bat costs $1.00 more than the ball. \
             How much does the ball cost?",
        )])
        .think(Think::Bool(true))
        .build();

    let mut stream = ollama_client.chat(request);
    let mut in_thinking_section = false;
    let mut in_answer_section = false;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        if let Some(thought) = chunk.message.thinking.as_deref()
            && !thought.is_empty()
        {
            if !in_thinking_section {
                println!("--- thinking ---");
                in_thinking_section = true;
            }
            print!("{}", thought);
            std::io::stdout().flush()?;
        }

        if !chunk.message.content.is_empty() {
            if !in_answer_section {
                if in_thinking_section {
                    println!("\n--- answer ---");
                } else {
                    println!("--- answer ---");
                }
                in_answer_section = true;
            }
            print!("{}", chunk.message.content);
            std::io::stdout().flush()?;
        }

        if chunk.done {
            println!();
            break;
        }
    }

    Ok(())
}
