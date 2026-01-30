use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::OllamaResult;
use crate::types::common::Options;

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
