use std::fmt;
use crate::errors::error::{Error, Result};
use crate::errors::api_error::{ApiError, ApiResult};

/// Error context for adding context to errors
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Context message
    pub message: String,
    
    /// Additional context information
    pub additional_info: Option<String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            additional_info: None,
        }
    }
    
    /// Add additional information to the context
    pub fn with_info(mut self, info: impl Into<String>) -> Self {
        self.additional_info = Some(info.into());
        self
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        
        if let Some(info) = &self.additional_info {
            write!(f, " ({})", info)?;
        }
        
        Ok(())
    }
}

/// Add context to a Result
pub fn with_context<T, E, C>(result: std::result::Result<T, E>, context: C) -> Result<T>
where
    E: Into<Error>,
    C: Into<ErrorContext>,
{
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            let context = context.into();
            let mut error = err.into();
            error.context = Some(context.to_string());
            Err(error)
        }
    }
}

/// Add context to an ApiResult
pub fn with_api_context<T, E, C>(result: std::result::Result<T, E>, context: C) -> ApiResult<T>
where
    E: Into<ApiError>,
    C: Into<ErrorContext>,
{
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            let context = context.into();
            let mut error = err.into();
            error.context = Some(context.to_string());
            Err(error)
        }
    }
}

/// Add context to a Result using a function
pub fn with_context_fn<T, E, F, C>(result: std::result::Result<T, E>, context_fn: F) -> Result<T>
where
    E: Into<Error>,
    F: FnOnce() -> C,
    C: Into<ErrorContext>,
{
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            let context = context_fn().into();
            let mut error = err.into();
            error.context = Some(context.to_string());
            Err(error)
        }
    }
}

/// Add context to an ApiResult using a function
pub fn with_api_context_fn<T, E, F, C>(result: std::result::Result<T, E>, context_fn: F) -> ApiResult<T>
where
    E: Into<ApiError>,
    F: FnOnce() -> C,
    C: Into<ErrorContext>,
{
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            let context = context_fn().into();
            let mut error = err.into();
            error.context = Some(context.to_string());
            Err(error)
        }
    }
}

/// Extension trait for Result to add context
pub trait ResultExt<T, E> {
    /// Add context to a Result
    fn with_context<C>(self, context: C) -> Result<T>
    where
        E: Into<Error>,
        C: Into<ErrorContext>;
        
    /// Add context to a Result using a function
    fn with_context_fn<F, C>(self, context_fn: F) -> Result<T>
    where
        E: Into<Error>,
        F: FnOnce() -> C,
        C: Into<ErrorContext>;
}

impl<T, E> ResultExt<T, E> for std::result::Result<T, E> {
    fn with_context<C>(self, context: C) -> Result<T>
    where
        E: Into<Error>,
        C: Into<ErrorContext>,
    {
        with_context(self, context)
    }
    
    fn with_context_fn<F, C>(self, context_fn: F) -> Result<T>
    where
        E: Into<Error>,
        F: FnOnce() -> C,
        C: Into<ErrorContext>,
    {
        with_context_fn(self, context_fn)
    }
}

/// Extension trait for ApiResult to add context
pub trait ApiResultExt<T, E> {
    /// Add context to an ApiResult
    fn with_api_context<C>(self, context: C) -> ApiResult<T>
    where
        E: Into<ApiError>,
        C: Into<ErrorContext>;
        
    /// Add context to an ApiResult using a function
    fn with_api_context_fn<F, C>(self, context_fn: F) -> ApiResult<T>
    where
        E: Into<ApiError>,
        F: FnOnce() -> C,
        C: Into<ErrorContext>;
}

impl<T, E> ApiResultExt<T, E> for std::result::Result<T, E> {
    fn with_api_context<C>(self, context: C) -> ApiResult<T>
    where
        E: Into<ApiError>,
        C: Into<ErrorContext>,
    {
        with_api_context(self, context)
    }
    
    fn with_api_context_fn<F, C>(self, context_fn: F) -> ApiResult<T>
    where
        E: Into<ApiError>,
        F: FnOnce() -> C,
        C: Into<ErrorContext>,
    {
        with_api_context_fn(self, context_fn)
    }
}
