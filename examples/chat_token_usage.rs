// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

//! Sends a single chat request and prints the token usage and timing stats
//! reported on the final response chunk.
//!
//! Requires `OLLAMA_SERVER` (e.g. `http://localhost:11434`).

use std::{env, error::Error, io::Write};

use futures_util::StreamExt;
use ollama_rs::{
    OllamaClient,
    types::chat::{ChatRequest, Message},
};

const MODEL: &str = "gemma4:e2b";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);

    let request = ChatRequest::builder(MODEL)
        .messages(vec![Message::user(
            "In one sentence, why is the sky blue?",
        )])
        .build();

    let mut stream = ollama_client.chat(request);
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        print!("{}", chunk.message.content);
        std::io::stdout().flush()?;

        if chunk.done {
            println!("\n\n--- usage ---");
            println!(
                "prompt tokens:   {}",
                fmt_count(chunk.prompt_eval_count)
            );
            println!("response tokens: {}", fmt_count(chunk.eval_count));
            println!(
                "total tokens:    {}",
                fmt_count(sum_opt(chunk.prompt_eval_count, chunk.eval_count))
            );
            println!();
            println!("--- timing ---");
            println!("total:        {}", fmt_duration(chunk.total_duration));
            println!("load:         {}", fmt_duration(chunk.load_duration));
            println!(
                "prompt eval:  {}",
                fmt_duration(chunk.prompt_eval_duration)
            );
            println!("response gen: {}", fmt_duration(chunk.eval_duration));

            if let (Some(eval_count), Some(eval_duration)) =
                (chunk.eval_count, chunk.eval_duration)
                && eval_duration > 0
            {
                let tokens_per_sec =
                    eval_count as f64 / (eval_duration as f64 / 1_000_000_000.0);
                println!("\nthroughput:   {:.1} tok/s", tokens_per_sec);
            }

            if let Some(reason) = chunk.done_reason {
                println!("\ndone_reason:  {}", reason);
            }
            break;
        }
    }

    Ok(())
}

fn fmt_count(value: Option<u64>) -> String {
    value.map_or_else(|| "n/a".to_string(), |v| v.to_string())
}

fn fmt_duration(nanos: Option<u64>) -> String {
    nanos.map_or_else(
        || "n/a".to_string(),
        |n| format!("{:.3} s", n as f64 / 1_000_000_000.0),
    )
}

fn sum_opt(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    Some(a? + b?)
}
