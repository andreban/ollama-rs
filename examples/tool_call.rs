use std::{env, error::Error, io::Write};

use futures_util::StreamExt;
use ollama_rs::{
    OllamaClient,
    types::chat::{ChatRequest, Function, Message, Tool, ToolType},
};
use serde::Deserialize;
use serde_json::{Value, json};

const MODEL: &str = "functiongemma";

fn get_weather(city: &str) -> Value {
    json!({
        "city": city,
        "temperature": 22.0,
        "unit": "celsius",
        "condition": "sunny",
    })
}

#[derive(Deserialize)]
struct GetWeatherArgs {
    city: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let server_address = env::var("OLLAMA_SERVER")?;
    let ollama_client = OllamaClient::new(server_address);
    let tools = vec![Tool {
        tool_type: ToolType::Function,
        function: Function {
            name: "get_weather".to_string(),
            description: "Get the current weather for a city.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "city": { "type": "string", "description": "The name of the city" },
                },
                "required": ["city"],
            }),
        },
    }];

    let mut messages = vec![Message::user("What is the weather in Paris?")];

    loop {
        let request = ChatRequest::builder(MODEL)
            .messages(messages.clone())
            .stream(false)
            .tools(tools.clone())
            .build();

        let mut stream = ollama_client.chat(request);
        let mut full_message = String::new();
        let Some(response) = stream.next().await else {
            println!("No response from stream.");
            return Ok(());
        };

        let response = response?;

        if response.message.tool_calls.is_empty() {
            full_message += &response.message.content;
            print!("{}", response.message.content);
            std::io::stdout().flush()?;
            break;
        }

        messages.push(response.message.clone());

        let tool_call = &response.message.tool_calls[0];
        let arg: GetWeatherArgs = serde_json::from_value(tool_call.function.arguments.clone())?;
        let result = get_weather(&arg.city);
        messages.push(Message::tool_response(&result));
    }
    Ok(())
}
