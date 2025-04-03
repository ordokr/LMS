use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid input: {0}")]
    ValidationError(String),
    
    #[error("Sync error: {0}")]
    SyncError(String),
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_code: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SyncError(_) => StatusCode::CONFLICT,
            AppError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
        };
        
        let error_code = match self {
            AppError::AuthError(_) => "AUTH_ERROR",
            AppError::AuthorizationError(_) => "AUTHORIZATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::SyncError(_) => "SYNC_ERROR",
            AppError::ServerError(_) => "SERVER_ERROR",
            AppError::ExternalServiceError(_) => "EXTERNAL_SERVICE_ERROR",
        };
        
        let body = Json(ErrorResponse {
            error: self.to_string(),
            error_code: error_code.to_string(),
        });
        
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".into()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}