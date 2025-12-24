use serde::{Deserialize, Serialize};

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
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
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
            },
        }
    }

    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.chat_request.messages = messages;
        self
    }

    pub fn build(self) -> ChatRequest {
        self.chat_request
    }
}
