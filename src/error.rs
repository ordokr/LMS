use std::fmt;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Integration error: {0}")]
    Integration(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("External API error: {0}")]
    ExternalApi(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Authorization(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Integration(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ExternalApi(msg) => (StatusCode::BAD_GATEWAY, msg),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

// Helper function to simplify error handling in API handlers
pub fn map_error<E: std::error::Error>(err: E) -> AppError {
    if let Some(sqlx_err) = err.downcast_ref::<sqlx::Error>() {
        match sqlx_err {
            sqlx::Error::RowNotFound => {
                return AppError::NotFound(err.to_string());
            }
            _ => return AppError::Database(sqlx_err.clone()),
        }
    }
    
    if err.to_string().contains("not found") {
        return AppError::NotFound(err.to_string());
    }
    
    if err.to_string().contains("permission") {
        return AppError::Authorization(err.to_string());
    }
    
    // Add more specific error mappings as needed
    
    AppError::Internal(err.to_string())
}

// Result type alias for API handlers
pub type AppResult<T> = Result<T, AppError>;