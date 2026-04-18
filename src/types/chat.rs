// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

//! Types for the chat conversation endpoint (`POST /api/chat`).
//!
//! Use [`ChatRequest::builder()`] to construct a request and pass it to
//! [`OllamaClient::chat()`](crate::OllamaClient::chat). The response is
//! streamed as a sequence of [`ChatResponse`] chunks.
//!
//! # Examples
//!
//! ```no_run
//! # use ollama_rs::OllamaClient;
//! # use ollama_rs::types::chat::{ChatRequest, Message};
//! # use futures_util::StreamExt;
//! # async fn run() -> ollama_rs::error::OllamaResult<()> {
//! let client = OllamaClient::default();
//!
//! let request = ChatRequest::builder("llama3")
//!     .messages(vec![
//!         Message::system("You are a helpful assistant."),
//!         Message::user("What is Rust?"),
//!     ])
//!     .build();
//!
//! let mut stream = client.chat(request);
//! while let Some(chunk) = stream.next().await {
//!     print!("{}", chunk?.message.content);
//! }
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::OllamaResult;
use crate::types::common::{Options, Think};

/// The role of a participant in a chat conversation.
///
/// Serialized as a lowercase string: `"user"`, `"system"`, `"assistant"`, `"tool"`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// A human user.
    User,
    /// A system prompt that sets the assistant's behavior.
    System,
    /// The AI assistant.
    Assistant,
    /// A tool response providing data back to the model.
    Tool,
}

/// A single message in a chat conversation.
///
/// Use the convenience constructors [`Message::system()`], [`Message::user()`],
/// and [`Message::tool_response()`] to create messages for the common roles.
/// Assistant messages are typically received from the model via [`ChatResponse`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// The text content of the message.
    pub content: String,
    /// The role of the message sender.
    pub role: Role,
    /// Tool calls requested by the assistant, if any.
    ///
    /// Empty for non-assistant messages. Omitted from serialization when empty.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

impl Message {
    /// Creates a system message that sets the assistant's behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama_rs::types::chat::Message;
    ///
    /// let msg = Message::system("You are a helpful assistant.");
    /// ```
    pub fn system<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            role: Role::System,
            tool_calls: vec![],
        }
    }

    /// Creates a user message.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama_rs::types::chat::Message;
    ///
    /// let msg = Message::user("Hello, how are you?");
    /// ```
    pub fn user<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            role: Role::User,
            tool_calls: vec![],
        }
    }

    /// Creates a tool response message from a JSON value.
    ///
    /// The value is serialized to a JSON string and placed in the message content.
    ///
    /// # Errors
    ///
    /// Returns [`OllamaError::ResponseParseError`](crate::error::OllamaError::ResponseParseError)
    /// if the value cannot be serialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollama_rs::types::chat::Message;
    /// use serde_json::json;
    ///
    /// let msg = Message::tool_response(&json!({"temperature": 22.0})).unwrap();
    /// ```
    pub fn tool_response(content: &Value) -> OllamaResult<Self> {
        Ok(Message {
            content: serde_json::to_string(content)?,
            role: Role::Tool,
            tool_calls: vec![],
        })
    }
}

/// A request to the chat endpoint (`POST /api/chat`).
///
/// Construct via [`ChatRequest::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::chat::{ChatRequest, Message};
///
/// let request = ChatRequest::builder("llama3")
///     .messages(vec![Message::user("Hello!")])
///     .stream(true)
///     .build();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    /// The model name to use for generation (e.g., `"llama3"`).
    pub model: String,

    /// The conversation history as a list of messages.
    pub messages: Vec<Message>,

    /// Whether to stream the response. When `None`, the server default applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Runtime options that control text generation behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,

    /// Tool definitions available for the model to call.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// A JSON schema to constrain the response format for structured output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Value>,

    /// Controls extended-thinking (reasoning) mode. Can be a boolean
    /// (`true`/`false`) or a level (`"high"`, `"medium"`, `"low"`) for
    /// supported models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<Think>,
}

impl ChatRequest {
    /// Returns a [`ChatRequestBuilder`] for the given model.
    pub fn builder<M: Into<String>>(model: M) -> ChatRequestBuilder {
        ChatRequestBuilder::new(model)
    }
}

/// A single chunk of a streaming chat response.
///
/// When streaming, each chunk contains a partial [`Message`]. The final chunk
/// has [`done`](ChatResponse::done) set to `true`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    /// The model that generated this response.
    pub model: String,
    /// ISO 8601 timestamp of when this chunk was created.
    pub created_at: String,
    /// The (partial) assistant message for this chunk.
    pub message: Message,
    /// `true` when this is the final chunk of the response.
    pub done: bool,
}

/// A builder for constructing a [`ChatRequest`].
///
/// Obtain a builder via [`ChatRequest::builder()`].
///
/// # Examples
///
/// ```
/// use ollama_rs::types::chat::{ChatRequest, Message};
/// use ollama_rs::types::common::Options;
///
/// let request = ChatRequest::builder("llama3")
///     .messages(vec![Message::user("Hi")])
///     .options(Options::builder().temperature(0.5).build())
///     .build();
/// ```
pub struct ChatRequestBuilder {
    chat_request: ChatRequest,
}

impl ChatRequestBuilder {
    fn new<M: Into<String>>(model: M) -> Self {
        Self {
            chat_request: ChatRequest {
                model: model.into(),
                messages: vec![],
                stream: None,
                options: None,
                tools: vec![],
                format: None,
                think: None,
            },
        }
    }

    /// Sets the conversation history.
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.chat_request.messages = messages;
        self
    }

    /// Sets runtime generation options.
    pub fn options(mut self, options: Options) -> Self {
        self.chat_request.options = Some(options);
        self
    }

    /// Sets the tools available for the model to call.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.chat_request.tools = tools;
        self
    }

    /// Sets whether to stream the response.
    pub fn stream(mut self, stream: bool) -> Self {
        self.chat_request.stream = Some(stream);
        self
    }

    /// Sets a JSON schema to constrain the response format.
    pub fn format(mut self, json_schema: Value) -> Self {
        self.chat_request.format = Some(json_schema);
        self
    }

    /// Enables or configures extended-thinking mode.
    pub fn think(mut self, think: Think) -> Self {
        self.chat_request.think = Some(think);
        self
    }

    /// Consumes the builder and returns the configured [`ChatRequest`].
    pub fn build(self) -> ChatRequest {
        self.chat_request
    }
}

/// A tool definition that the model can invoke during a chat.
///
/// # Examples
///
/// ```
/// use ollama_rs::types::chat::{Tool, ToolType, Function};
/// use serde_json::json;
///
/// let tool = Tool {
///     tool_type: ToolType::Function,
///     function: Function {
///         name: "get_weather".to_string(),
///         description: "Get current weather for a city".to_string(),
///         parameters: json!({
///             "type": "object",
///             "properties": {
///                 "city": { "type": "string" }
///             },
///             "required": ["city"]
///         }),
///     },
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    /// The type of tool (currently only `Function`).
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    /// The function definition.
    pub function: Function,
}

/// The kind of tool. Currently only [`Function`](ToolType::Function) is supported.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    /// A callable function.
    Function,
}

/// A function definition for tool calling.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
    /// The function name the model will use to invoke this tool.
    pub name: String,
    /// A JSON Schema describing the function's parameters.
    pub parameters: Value,
    /// A description of what the function does, to guide the model.
    pub description: String,
}

/// A tool call requested by the model in an assistant message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    /// The function the model wants to invoke.
    pub function: ToolCallFunction,
}

/// Details of a specific function call requested by the model.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallFunction {
    /// The name of the function to call.
    pub name: String,
    /// The arguments to pass, as a JSON value.
    pub arguments: Value,
    /// The index of this tool call within the message (for parallel calls).
    pub index: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn role_serializes_lowercase() {
        assert_eq!(serde_json::to_value(&Role::User).unwrap(), json!("user"));
        assert_eq!(
            serde_json::to_value(&Role::System).unwrap(),
            json!("system")
        );
        assert_eq!(
            serde_json::to_value(&Role::Assistant).unwrap(),
            json!("assistant")
        );
        assert_eq!(serde_json::to_value(&Role::Tool).unwrap(), json!("tool"));
    }

    #[test]
    fn role_deserializes_lowercase() {
        let role: Role = serde_json::from_value(json!("user")).unwrap();
        assert!(matches!(role, Role::User));
    }

    #[test]
    fn message_system_constructor() {
        let msg = Message::system("you are helpful");
        assert_eq!(msg.content, "you are helpful");
        assert!(matches!(msg.role, Role::System));
        assert!(msg.tool_calls.is_empty());
    }

    #[test]
    fn message_user_constructor() {
        let msg = Message::user("hello");
        assert_eq!(msg.content, "hello");
        assert!(matches!(msg.role, Role::User));
    }

    #[test]
    fn message_tool_response_constructor() {
        let value = json!({"temperature": 22.0});
        let msg = Message::tool_response(&value).unwrap();
        assert!(matches!(msg.role, Role::Tool));
        assert_eq!(msg.content, serde_json::to_string(&value).unwrap());
    }

    #[test]
    fn message_skips_empty_tool_calls() {
        let msg = Message::user("hello");
        let json = serde_json::to_value(&msg).unwrap();
        assert!(!json.as_object().unwrap().contains_key("tool_calls"));
    }

    #[test]
    fn message_deserializes_without_tool_calls() {
        let json = json!({"content": "hi", "role": "user"});
        let msg: Message = serde_json::from_value(json).unwrap();
        assert_eq!(msg.content, "hi");
        assert!(msg.tool_calls.is_empty());
    }

    #[test]
    fn chat_request_always_serializes_messages() {
        let request = ChatRequest::builder("llama3").build();
        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("messages"));
        assert_eq!(obj["messages"], json!([]));
    }

    #[test]
    fn chat_request_skips_optional_fields() {
        let request = ChatRequest::builder("llama3").build();
        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("stream"));
        assert!(!obj.contains_key("options"));
        assert!(!obj.contains_key("tools"));
        assert!(!obj.contains_key("format"));
        assert!(!obj.contains_key("think"));
    }

    #[test]
    fn chat_request_builder_with_messages() {
        let messages = vec![Message::system("be helpful"), Message::user("hello")];
        let request = ChatRequest::builder("llama3")
            .messages(messages)
            .stream(false)
            .build();

        assert_eq!(request.model, "llama3");
        assert_eq!(request.messages.len(), 2);
        assert_eq!(request.stream, Some(false));
    }

    #[test]
    fn tool_type_serializes_as_type_field() {
        let tool = Tool {
            tool_type: ToolType::Function,
            function: Function {
                name: "get_weather".to_string(),
                description: "Get weather".to_string(),
                parameters: json!({"type": "object"}),
            },
        };
        let json = serde_json::to_value(&tool).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("type"));
        assert!(!obj.contains_key("tool_type"));
        assert_eq!(obj["type"], json!("function"));
    }

    #[test]
    fn tool_type_deserializes_from_type_field() {
        let json = json!({
            "type": "function",
            "function": {
                "name": "get_weather",
                "description": "Get weather",
                "parameters": {"type": "object"}
            }
        });
        let tool: Tool = serde_json::from_value(json).unwrap();
        assert!(matches!(tool.tool_type, ToolType::Function));
        assert_eq!(tool.function.name, "get_weather");
    }

    #[test]
    fn chat_response_deserialize() {
        let json = json!({
            "model": "llama3",
            "created_at": "2024-01-01T00:00:00Z",
            "message": {"content": "Hello!", "role": "assistant"},
            "done": false
        });
        let response: ChatResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.model, "llama3");
        assert_eq!(response.message.content, "Hello!");
        assert!(matches!(response.message.role, Role::Assistant));
        assert!(!response.done);
    }

    #[test]
    fn chat_response_with_tool_calls() {
        let json = json!({
            "model": "llama3",
            "created_at": "2024-01-01T00:00:00Z",
            "message": {
                "content": "",
                "role": "assistant",
                "tool_calls": [{
                    "function": {
                        "name": "get_weather",
                        "arguments": {"city": "Paris"},
                        "index": 0
                    }
                }]
            },
            "done": true
        });
        let response: ChatResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.message.tool_calls.len(), 1);
        assert_eq!(response.message.tool_calls[0].function.name, "get_weather");
        assert_eq!(response.message.tool_calls[0].function.index, 0);
    }
}
