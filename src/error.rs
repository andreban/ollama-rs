use std::{error::Error, fmt::Display};

use tokio_util::codec::LinesCodecError;

pub type OllamaResult<T> = Result<T, OllamaError>;

#[derive(Debug)]
pub enum OllamaError {
    NetworkError(reqwest::Error),
    ResponseParseError(serde_json::Error),
    LinesCoderError(LinesCodecError),
}

impl Error for OllamaError {}

impl Display for OllamaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OllamaError::NetworkError(e) => writeln!(f, "Network error: {}", e),
            OllamaError::ResponseParseError(e) => writeln!(f, "ResponseParseError error: {}", e),
            OllamaError::LinesCoderError(e) => writeln!(f, "LinesCoderError error: {}", e),
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
        Self::LinesCoderError(value)
    }
}
