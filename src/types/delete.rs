//! Types for the model delete endpoint (`DELETE /api/delete`).
//!
//! Use [`DeleteRequest::new()`] to construct a request and pass it to
//! [`OllamaClient::delete()`](crate::OllamaClient::delete).
//!
//! # Examples
//!
//! ```no_run
//! # use ollama_rs::OllamaClient;
//! # use ollama_rs::types::delete::DeleteRequest;
//! # async fn run() -> ollama_rs::error::OllamaResult<()> {
//! let client = OllamaClient::default();
//! let request = DeleteRequest::new("gemma3");
//! client.delete(request).await?;
//! println!("Model deleted successfully");
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

/// A request to delete a model from the Ollama server (`DELETE /api/delete`).
///
/// # Examples
///
/// ```
/// use ollama_rs::types::delete::DeleteRequest;
///
/// let request = DeleteRequest::new("gemma3");
/// assert_eq!(request.model, "gemma3");
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRequest {
    /// The name of the model to delete (e.g., `"gemma3"`, `"llama3:latest"`).
    pub model: String,
}

impl DeleteRequest {
    /// Creates a new [`DeleteRequest`] for the given model name.
    pub fn new<M: Into<String>>(model: M) -> Self {
        Self {
            model: model.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn new_creates_request() {
        let request = DeleteRequest::new("gemma3");
        assert_eq!(request.model, "gemma3");
    }

    #[test]
    fn new_accepts_string() {
        let request = DeleteRequest::new(String::from("llama3:latest"));
        assert_eq!(request.model, "llama3:latest");
    }

    #[test]
    fn request_serializes_correctly() {
        let request = DeleteRequest::new("gemma3");
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json, json!({"model": "gemma3"}));
    }

    #[test]
    fn request_deserializes_correctly() {
        let json = json!({"model": "gemma3"});
        let request: DeleteRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.model, "gemma3");
    }
}
