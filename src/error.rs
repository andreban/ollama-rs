use std::{error::Error, fmt::Display};

use tokio_util::codec::LinesCodecError;

pub type OllamaResult<T> = Result<T, OllamaError>;

#[derive(Debug)]
pub enum OllamaError {
    NetworkError(reqwest::Error),
    ResponseParseError(serde_json::Error),
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
