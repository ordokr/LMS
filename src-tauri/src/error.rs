use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("External API error: {0}")]
    ExternalApi(String),
    
    #[error("Parsing error: {0}")]
    Parsing(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}