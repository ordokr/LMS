use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    Offline,
    NetworkError(String),
    ServerError(u16, String),
    AuthError(String),
    Deserialization(String),
    UnexpectedError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Offline => write!(f, "You are currently offline"),
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ServerError(code, msg) => write!(f, "Server error ({}): {}", code, msg),
            ApiError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            ApiError::Deserialization(msg) => write!(f, "Failed to parse response: {}", msg),
            ApiError::UnexpectedError(msg) => write!(f, "Unexpected error: {}", msg),
        }
    }
}