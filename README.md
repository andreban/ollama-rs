# ollama-rs

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

An async Rust client library for the [Ollama](https://ollama.com/) API. Provides a streaming-first interface for text generation, multi-turn chat, model management, and advanced features like structured output and tool calling.

## Features

- Fully async with [tokio](https://tokio.rs/) and streaming responses via `futures::Stream`
- Text generation and multi-turn chat conversations
- Structured JSON output with schema validation
- Tool calling / function calling support
- Model management (list, pull, delete, inspect running models)
- Text embeddings generation
- Builder pattern for constructing requests
- Configurable generation parameters (temperature, top-k, top-p, and more)
- Thinking / reasoning mode support

## Installation

Add `ollama-rs` to your `Cargo.toml`:

```toml
[dependencies]
ollama-rs = { git = "https://github.com/andreban/ollama-rs.git" }
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"
```

## Prerequisites

A running [Ollama](https://ollama.com/) server. By default, Ollama listens on `http://localhost:11434`.

## Quick Start

### Text Generation

```rust
use std::io::Write;
use futures_util::StreamExt;
use ollama_rs::{OllamaClient, types::generate::GenerateRequest};

#[tokio::main]
async fn main() {
    let client = OllamaClient::new("http://localhost:11434");
    let request = GenerateRequest::builder("llama3:8b")
        .prompt("Why is the sky blue?")
        .build();

    let mut stream = client.generate(request);
    while let Some(response) = stream.next().await {
        match response {
            Ok(token) => {
                print!("{}", token.response);
                std::io::stdout().flush().unwrap();
                if token.done {
                    break;
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
```

### Chat

```rust
use std::io::Write;
use futures_util::StreamExt;
use ollama_rs::{OllamaClient, types::chat::{ChatRequest, Message}};

#[tokio::main]
async fn main() {
    let client = OllamaClient::new("http://localhost:11434");
    let messages = vec![
        Message::system("You are a helpful assistant."),
        Message::user("What is the capital of France?"),
    ];
    let request = ChatRequest::builder("llama3:8b")
        .messages(messages)
        .build();

    let mut stream = client.chat(request);
    while let Some(response) = stream.next().await {
        let response = response.unwrap();
        print!("{}", response.message.content);
        std::io::stdout().flush().unwrap();
        if response.done {
            break;
        }
    }
}
```

### Structured Output

Force the model to respond with JSON matching a specific schema:

```rust
use ollama_rs::{OllamaClient, types::generate::GenerateRequest};
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "answer": { "type": "string" },
        "confidence": { "type": "number" }
    }
});

let request = GenerateRequest::builder("llama3:8b")
    .prompt("What is 2 + 2?")
    .stream(false)
    .format(schema)
    .build();
```

### Tool Calling

Define tools the model can invoke during a chat conversation:

```rust
use ollama_rs::types::chat::{ChatRequest, Function, Message, Tool, ToolType};
use serde_json::json;

let tools = vec![Tool {
    tool_type: ToolType::Function,
    function: Function {
        name: "get_weather".to_string(),
        description: "Get the current weather for a city.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "city": { "type": "string", "description": "The name of the city" }
            },
            "required": ["city"]
        }),
    },
}];

let request = ChatRequest::builder("llama3:8b")
    .messages(vec![Message::user("What is the weather in Paris?")])
    .stream(false)
    .tools(tools)
    .build();
```

When the model decides to call a tool, the response `message.tool_calls` field will contain the tool name and arguments. You can then execute the function and send the result back via `Message::tool_response(...)` which returns an `OllamaResult<Message>`.

## API Reference

### `OllamaClient`

| Method | Description |
|--------|-------------|
| `new(server_address)` | Create a new client with a 30-second connection timeout |
| `default()` | Create a client connecting to `http://localhost:11434` |
| `builder(server_address)` | Create a client with custom configuration (see below) |
| `version()` | Get the Ollama server version |
| `tags()` | List all available models |
| `ps()` | List currently running/loaded models |
| `generate(request)` | Generate text (streaming) |
| `chat(request)` | Chat conversation (streaming) |
| `pull(request)` | Pull/download a model (streaming) |
| `delete(request)` | Delete a model from the server |
| `show_model(request)` | Show information about a model |
| `embed(request)` | Generate vector embeddings |

**`OllamaClient::builder(server_address)`** -- `.connection_timeout(Duration)`, `.build()`

```rust
use std::time::Duration;
use ollama_rs::OllamaClient;

let client = OllamaClient::builder("http://localhost:11434")
    .connection_timeout(Duration::from_secs(60))
    .build();
```

### Request Builders

**`GenerateRequest::builder(model)`** -- `.prompt()`, `.system_prompt()`, `.format()`, `.options()`, `.stream()`, `.think()`, `.images()`, `.suffix()`

**`ChatRequest::builder(model)`** -- `.messages()`, `.tools()`, `.format()`, `.options()`, `.stream()`, `.think()`

**`PullRequest::builder(model)`** -- `.stream()`

**`EmbedRequest::builder(model)`** -- `.input()`, `.inputs()`, `.truncate()`, `.dimensions()`, `.keep_alive()`, `.options()`

**`ShowModelRequest::new(model)`** -- `.verbose()`

### Generation Options

Configure sampling parameters via `Options::builder()`:

| Option | Description |
|--------|-------------|
| `temperature(f32)` | Controls randomness (0.0 - 2.0) |
| `top_k(u32)` | Top-K sampling |
| `top_p(f32)` | Nucleus sampling threshold |
| `min_p(f32)` | Minimum probability filter |
| `seed(u64)` | Random seed for reproducibility |
| `num_ctx(u32)` | Context window size |
| `num_predict(u32)` | Maximum tokens to generate |
| `stop(Stop)` | Stop sequences |

## Examples

The `examples/` directory contains runnable programs:

| Example | Description |
|---------|-------------|
| `generate` | Basic text generation |
| `chat` | Interactive multi-turn chat |
| `structured_output` | JSON structured output with schema |
| `tool_call` | Function calling / tool use |
| `pull` | Download a model |
| `delete` | Delete a model |
| `show_model` | Show model information |
| `embed` | Generate text embeddings |
| `tags` | List available models |
| `ps` | List running models |
| `version` | Query server version |

Run an example:

```sh
OLLAMA_SERVER=http://localhost:11434 cargo run --example chat
```

## Configuration

| Environment Variable | Description |
|----------------------|-------------|
| `OLLAMA_SERVER` | Ollama server address (e.g., `http://localhost:11434`) |
| `RUST_LOG` | Log level filter (e.g., `ollama_rs=debug`) |

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
