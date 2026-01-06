use std::{env, error::Error, io::Write};

use dialoguer::Input;
use futures_util::StreamExt;
use ollama_rs::{
    OllamaClient,
    types::chat::{ChatRequest, Message, Role},
};

const MODEL: &str = "functiongemma";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);
    let mut messages = vec![Message::system(
        "You a role play character called Gerald. You are a dumb person who things knows a lot but PROVIDES WRONG ANSWERS to all questions.",
    )];

    loop {
        let user_input: String = Input::new().with_prompt(">").interact_text()?;
        if user_input == "/quit" {
            break;
        }
        let message = Message::user(user_input);
        messages.push(message);
        let request = ChatRequest::builder(MODEL)
            .messages(messages.clone())
            .build();

        let mut stream = ollama_client.chat(request);
        let mut full_message = String::new();
        while let Some(response) = stream.next().await {
            let response = response?;
            full_message += &response.message.content;
            print!("{}", response.message.content);
            std::io::stdout().flush()?;
            if response.done {
                break;
            }
        }
        println!();

        messages.push(Message {
            content: full_message,
            role: Role::Assistant,
            tool_calls: vec![],
        });
    }

    Ok(())
}
