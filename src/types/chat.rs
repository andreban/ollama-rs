use serde::{Deserialize, Serialize};

use crate::types::common::Options;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Runtime options that control text generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
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

    pub fn build(self) -> ChatRequest {
        self.chat_request
    }
}
