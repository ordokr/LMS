use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}