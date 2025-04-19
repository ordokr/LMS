use std::fmt;
use std::error::Error as StdError;
use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Unified error type for the application
#[derive(Error, Debug, Clone)]
pub struct Error {
    /// The kind of error
    pub kind: ErrorKind,
    
    /// Error code for categorization and client-side handling
    pub code: ErrorCode,
    
    /// Human-readable error message
    pub message: String,
    
    /// Optional context information for debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    
    /// Optional source error (not serialized)
    #[serde(skip)]
    pub source: Option<Box<dyn StdError + Send + Sync + 'static>>,
    
    /// Optional stack trace for debugging (not serialized)
    #[serde(skip)]
    pub stack_trace: Option<String>,
}

/// Error kinds for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorKind {
    /// Database-related errors
    Database,
    
    /// Validation errors (invalid input, etc.)
    Validation,
    
    /// Authentication errors (invalid credentials, etc.)
    Authentication,
    
    /// Authorization errors (insufficient permissions, etc.)
    Authorization,
    
    /// Not found errors (resource not found, etc.)
    NotFound,
    
    /// External API errors (API calls to external services)
    ExternalApi,
    
    /// Parsing errors (JSON parsing, etc.)
    Parsing,
    
    /// Internal errors (unexpected errors, etc.)
    Internal,
    
    /// Network errors (connection issues, etc.)
    Network,
    
    /// Conflict errors (resource already exists, etc.)
    Conflict,
    
    /// Timeout errors (operation timed out, etc.)
    Timeout,
    
    /// Rate limit errors (too many requests, etc.)
    RateLimit,
    
    /// Bad gateway errors (upstream service unavailable, etc.)
    BadGateway,
    
    /// Service unavailable errors (service temporarily unavailable, etc.)
    ServiceUnavailable,
}

/// Error codes for client-side handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    /// Database error
    DatabaseError,
    
    /// Resource not found
    NotFound,
    
    /// Validation error
    ValidationError,
    
    /// Authentication error
    AuthError,
    
    /// Authorization error
    AuthorizationError,
    
    /// External API error
    ExternalApiError,
    
    /// Parsing error
    ParsingError,
    
    /// Internal server error
    InternalError,
    
    /// Network error
    NetworkError,
    
    /// Conflict error
    ConflictError,
    
    /// Timeout error
    TimeoutError,
    
    /// Rate limit error
    RateLimitError,
    
    /// Bad gateway error
    BadGatewayError,
    
    /// Service unavailable error
    ServiceUnavailableError,
}

impl Error {
    /// Create a new error
    pub fn new(kind: ErrorKind, code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            kind,
            code,
            message: message.into(),
            context: None,
            source: None,
            stack_trace: None,
        }
    }
    
    /// Create a new error with context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
    
    /// Create a new error with source
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
    
    /// Create a new error with stack trace
    pub fn with_stack_trace(mut self, stack_trace: impl Into<String>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }
    
    /// Create a database error
    pub fn database(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Database, ErrorCode::DatabaseError, message)
    }
    
    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound, ErrorCode::NotFound, message)
    }
    
    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Validation, ErrorCode::ValidationError, message)
    }
    
    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Authentication, ErrorCode::AuthError, message)
    }
    
    /// Create an authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Authorization, ErrorCode::AuthorizationError, message)
    }
    
    /// Create an external API error
    pub fn external_api(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::ExternalApi, ErrorCode::ExternalApiError, message)
    }
    
    /// Create a parsing error
    pub fn parsing(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Parsing, ErrorCode::ParsingError, message)
    }
    
    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Internal, ErrorCode::InternalError, message)
    }
    
    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Network, ErrorCode::NetworkError, message)
    }
    
    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Conflict, ErrorCode::ConflictError, message)
    }
    
    /// Create a timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Timeout, ErrorCode::TimeoutError, message)
    }
    
    /// Create a rate limit error
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::RateLimit, ErrorCode::RateLimitError, message)
    }
    
    /// Create a bad gateway error
    pub fn bad_gateway(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::BadGateway, ErrorCode::BadGatewayError, message)
    }
    
    /// Create a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::ServiceUnavailable, ErrorCode::ServiceUnavailableError, message)
    }
    
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self.kind {
            ErrorKind::Authentication => 401, // Unauthorized
            ErrorKind::Authorization => 403,  // Forbidden
            ErrorKind::NotFound => 404,       // Not Found
            ErrorKind::Validation => 400,     // Bad Request
            ErrorKind::Conflict => 409,       // Conflict
            ErrorKind::Timeout => 408,        // Request Timeout
            ErrorKind::RateLimit => 429,      // Too Many Requests
            ErrorKind::BadGateway => 502,     // Bad Gateway
            ErrorKind::ServiceUnavailable => 503, // Service Unavailable
            ErrorKind::ExternalApi => 502,    // Bad Gateway
            _ => 500,                         // Internal Server Error
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        
        if let Some(context) = &self.context {
            write!(f, " ({})", context)?;
        }
        
        Ok(())
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        
        let mut state = serializer.serialize_struct("Error", 4)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("message", &self.message)?;
        
        if let Some(context) = &self.context {
            state.serialize_field("context", context)?;
        }
        
        state.end()
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ErrorHelper {
            kind: ErrorKind,
            code: ErrorCode,
            message: String,
            context: Option<String>,
        }
        
        let helper = ErrorHelper::deserialize(deserializer)?;
        
        Ok(Error {
            kind: helper.kind,
            code: helper.code,
            message: helper.message,
            context: helper.context,
            source: None,
            stack_trace: None,
        })
    }
}

/// Conversion from sqlx::Error to Error
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::not_found("Resource not found"),
            _ => Self::database(format!("Database error: {}", err))
                .with_source(err),
        }
    }
}

/// Conversion from serde_json::Error to Error
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::parsing(format!("JSON parsing error: {}", err))
            .with_source(err)
    }
}

/// Conversion from reqwest::Error to Error
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::timeout(format!("Request timed out: {}", err))
                .with_source(err)
        } else if err.is_connect() {
            Self::network(format!("Connection error: {}", err))
                .with_source(err)
        } else {
            Self::external_api(format!("Request error: {}", err))
                .with_source(err)
        }
    }
}

/// Result type using our Error
pub type Result<T> = std::result::Result<T, Error>;
