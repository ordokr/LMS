use std::fmt;
use serde::{Serialize, Deserialize};
use crate::errors::error::{Error, ErrorKind, ErrorCode};

/// API-specific error types that can occur when making API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// The kind of API error
    pub kind: ApiErrorKind,
    
    /// Human-readable error message
    pub message: String,
    
    /// Optional status code for HTTP errors
    pub status_code: Option<u16>,
    
    /// Optional context information for debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

/// API error kinds for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiErrorKind {
    /// The application is currently offline
    Offline,
    
    /// General network errors (connection refused, timeout, etc.)
    NetworkError,
    
    /// Server returned an error status code with message
    ServerError,
    
    /// Authentication related errors (invalid credentials, expired token, etc.)
    AuthError,
    
    /// Authorization related errors (insufficient permissions, etc.)
    AuthorizationError,
    
    /// Failed to deserialize API response
    DeserializationError,
    
    /// Rate limit exceeded
    RateLimitError,
    
    /// Request timed out
    TimeoutError,
    
    /// Bad gateway (upstream service unavailable)
    BadGatewayError,
    
    /// Service unavailable
    ServiceUnavailableError,
    
    /// Unexpected or unhandled errors
    UnexpectedError,
}

impl ApiError {
    /// Create a new API error
    pub fn new(kind: ApiErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            status_code: None,
            context: None,
        }
    }
    
    /// Create a new API error with status code
    pub fn with_status_code(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }
    
    /// Create a new API error with context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
    
    /// Create an offline error
    pub fn offline() -> Self {
        Self::new(ApiErrorKind::Offline, "You are currently offline")
    }
    
    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::NetworkError, message)
    }
    
    /// Create a server error
    pub fn server(status_code: u16, message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::ServerError, message)
            .with_status_code(status_code)
    }
    
    /// Create an authentication error
    pub fn auth(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::AuthError, message)
            .with_status_code(401)
    }
    
    /// Create an authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::AuthorizationError, message)
            .with_status_code(403)
    }
    
    /// Create a deserialization error
    pub fn deserialization(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::DeserializationError, message)
    }
    
    /// Create a rate limit error
    pub fn rate_limit(message: impl Into<String>, retry_after: Option<u64>) -> Self {
        let mut error = Self::new(ApiErrorKind::RateLimitError, message)
            .with_status_code(429);
            
        if let Some(retry_after) = retry_after {
            error = error.with_context(format!("Retry after {} seconds", retry_after));
        }
        
        error
    }
    
    /// Create a timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::TimeoutError, message)
            .with_status_code(408)
    }
    
    /// Create a bad gateway error
    pub fn bad_gateway(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::BadGatewayError, message)
            .with_status_code(502)
    }
    
    /// Create a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::ServiceUnavailableError, message)
            .with_status_code(503)
    }
    
    /// Create an unexpected error
    pub fn unexpected(message: impl Into<String>) -> Self {
        Self::new(ApiErrorKind::UnexpectedError, message)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        
        if let Some(status_code) = self.status_code {
            write!(f, " (Status: {})", status_code)?;
        }
        
        if let Some(context) = &self.context {
            write!(f, " ({})", context)?;
        }
        
        Ok(())
    }
}

impl From<ApiError> for Error {
    fn from(error: ApiError) -> Self {
        match error.kind {
            ApiErrorKind::Offline => Error::network("Offline")
                .with_context(error.message),
                
            ApiErrorKind::NetworkError => Error::network(error.message),
            
            ApiErrorKind::ServerError => {
                if let Some(status_code) = error.status_code {
                    match status_code {
                        401 => Error::authentication(error.message),
                        403 => Error::authorization(error.message),
                        404 => Error::not_found(error.message),
                        409 => Error::conflict(error.message),
                        429 => Error::rate_limit(error.message),
                        502 => Error::bad_gateway(error.message),
                        503 => Error::service_unavailable(error.message),
                        _ => Error::external_api(error.message)
                            .with_context(format!("Status code: {}", status_code)),
                    }
                } else {
                    Error::external_api(error.message)
                }
            },
            
            ApiErrorKind::AuthError => Error::authentication(error.message),
            
            ApiErrorKind::AuthorizationError => Error::authorization(error.message),
            
            ApiErrorKind::DeserializationError => Error::parsing(error.message),
            
            ApiErrorKind::RateLimitError => Error::rate_limit(error.message),
            
            ApiErrorKind::TimeoutError => Error::timeout(error.message),
            
            ApiErrorKind::BadGatewayError => Error::bad_gateway(error.message),
            
            ApiErrorKind::ServiceUnavailableError => Error::service_unavailable(error.message),
            
            ApiErrorKind::UnexpectedError => Error::internal(error.message),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::timeout(format!("Request timed out: {}", err))
        } else if err.is_connect() {
            Self::network(format!("Connection error: {}", err))
        } else if let Some(status) = err.status() {
            Self::server(status.as_u16(), format!("Server error: {}", err))
        } else {
            Self::unexpected(format!("Unexpected error: {}", err))
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        Self::deserialization(format!("Failed to parse response: {}", err))
    }
}

/// Result type using our ApiError
pub type ApiResult<T> = std::result::Result<T, ApiError>;
