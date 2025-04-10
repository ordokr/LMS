use std::fmt;
use serde::{Serialize, Deserialize};
use crate::error::Error as AppError;

/// API-specific error types that can occur when making API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    /// The application is currently offline
    Offline,
    
    /// General network errors (connection refused, timeout, etc.)
    NetworkError(String),
    
    /// Server returned an error status code with message
    ServerError(u16, String),
    
    /// Authentication related errors (invalid credentials, expired token, etc.)
    AuthError(String),
    
    /// Failed to deserialize API response
    Deserialization(String),
    
    /// Unexpected or unhandled errors
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

impl From<ApiError> for AppError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::Offline => AppError::ExternalApi("Offline".to_string()),
            ApiError::NetworkError(msg) => AppError::ExternalApi(format!("Network error: {}", msg)),
            ApiError::ServerError(code, msg) => AppError::ExternalApi(format!("Server error ({}): {}", code, msg)),
            ApiError::AuthError(msg) => AppError::Auth(msg),
            ApiError::Deserialization(msg) => AppError::Parsing(msg),
            ApiError::UnexpectedError(msg) => AppError::Internal(msg),
        }
    }
}

/// Result type using our ApiError
pub type ApiResult<T> = Result<T, ApiError>;
