use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::OllamaResult;
use crate::types::common::{Options, Think};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
    Tool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: Role,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

impl Message {
    pub fn system<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            role: Role::System,
            tool_calls: vec![],
        }
    }

    pub fn user<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
            role: Role::User,
            tool_calls: vec![],
        }
    }

    pub fn tool_response(content: &Value) -> OllamaResult<Self> {
        Ok(Message {
            content: serde_json::to_string(content)?,
            role: Role::Tool,
            tool_calls: vec![],
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,

    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Runtime options that control text generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Value>,

    /// When set, returns separate thinking output in addition to content. Can be a boolean
    /// (true/false) or a string ("high", "medium", "low") for supported models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<Think>,
}

impl ChatRequest {
    pub fn builder<M: Into<String>>(model: M) -> ChatRequestBuilder {
        ChatRequestBuilder::new(model)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: Message,
    pub done: bool,
}

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

    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.chat_request.messages = messages;
        self
    }

    pub fn options(mut self, options: Options) -> Self {
        self.chat_request.options = Some(options);
        self
    }

    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.chat_request.tools = tools;
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.chat_request.stream = Some(stream);
        self
    }

    pub fn format(mut self, json_schema: Value) -> Self {
        self.chat_request.format = Some(json_schema);
        self
    }

    pub fn think(mut self, think: Think) -> Self {
        self.chat_request.think = Some(think);
        self
    }

    pub fn build(self) -> ChatRequest {
        self.chat_request
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub function: Function,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    Function,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Value,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub function: ToolCallFunction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: Value,
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
