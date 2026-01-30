//! Error types for the Ollama client.
//!
//! This module defines [`OllamaError`], the unified error type returned by all
//! client operations, and the [`OllamaResult<T>`] type alias for convenience.

use std::{error::Error, fmt::Display};

use tokio_util::codec::LinesCodecError;

/// A type alias for `Result<T, OllamaError>`.
///
/// Used throughout the crate as the standard return type for fallible operations.
pub type OllamaResult<T> = Result<T, OllamaError>;

/// Errors that can occur when communicating with the Ollama server.
///
/// This enum covers three failure categories:
///
/// - **Network** -- connection failures, timeouts, or HTTP error status codes.
/// - **Parsing** -- the server returned a response that could not be deserialized as JSON.
/// - **Streaming** -- an error occurred while reading a streaming response line-by-line.
///
/// All variants wrap their underlying error and implement [`std::error::Error`],
/// [`Display`], and the relevant [`From`] conversions so they work seamlessly with `?`.
#[derive(Debug)]
pub enum OllamaError {
    /// An HTTP or connection-level error from [`reqwest`].
    NetworkError(reqwest::Error),
    /// A JSON deserialization error from [`serde_json`].
    ResponseParseError(serde_json::Error),
    /// An error from the line-delimited streaming codec.
    LinesCodecError(LinesCodecError),
}

impl Error for OllamaError {}

impl Display for OllamaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OllamaError::NetworkError(e) => write!(f, "Network error: {}", e),
            OllamaError::ResponseParseError(e) => write!(f, "Response parse error: {}", e),
            OllamaError::LinesCodecError(e) => write!(f, "Lines codec error: {}", e),
        }
    }
}

impl From<reqwest::Error> for OllamaError {
    fn from(error: reqwest::Error) -> Self {
        Self::NetworkError(error)
    }
}

impl From<serde_json::Error> for OllamaError {
    fn from(error: serde_json::Error) -> Self {
        Self::ResponseParseError(error)
    }
}

impl From<LinesCodecError> for OllamaError {
    fn from(value: LinesCodecError) -> Self {
        Self::LinesCodecError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_response_parse_error() {
        let err: Result<i32, _> = serde_json::from_str("not json");
        let ollama_err = OllamaError::from(err.unwrap_err());
        let display = format!("{}", ollama_err);
        assert!(display.starts_with("Response parse error:"));
        assert!(!display.ends_with('\n'));
    }

    #[test]
    fn display_lines_codec_error() {
        let err = LinesCodecError::MaxLineLengthExceeded;
        let ollama_err = OllamaError::from(err);
        let display = format!("{}", ollama_err);
        assert!(display.starts_with("Lines codec error:"));
        assert!(!display.ends_with('\n'));
    }

    #[test]
    fn from_serde_json_error() {
        let err: Result<i32, _> = serde_json::from_str("not json");
        let ollama_err = OllamaError::from(err.unwrap_err());
        assert!(matches!(ollama_err, OllamaError::ResponseParseError(_)));
    }

    #[test]
    fn from_lines_codec_error() {
        let err = LinesCodecError::MaxLineLengthExceeded;
        let ollama_err = OllamaError::from(err);
        assert!(matches!(ollama_err, OllamaError::LinesCodecError(_)));
    }

    #[test]
    fn error_trait_is_implemented() {
        let err: Result<i32, _> = serde_json::from_str("not json");
        let ollama_err = OllamaError::from(err.unwrap_err());
        // Verify it implements std::error::Error by using it as a trait object
        let _: &dyn Error = &ollama_err;
    }
}
