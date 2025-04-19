use std::fmt;
use crate::errors::error::{Error, Result};
use crate::errors::api_error::{ApiError, ApiResult};

/// Error mapper for mapping errors between different error types
#[derive(Debug, Clone)]
pub struct ErrorMapper<F, T> {
    /// Mapping function
    pub mapper: F,
    
    /// Phantom data for target error type
    pub _phantom: std::marker::PhantomData<T>,
}

impl<F, E, T> ErrorMapper<F, T>
where
    F: Fn(E) -> T,
{
    /// Create a new error mapper
    pub fn new(mapper: F) -> Self {
        Self {
            mapper,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Map an error
    pub fn map(&self, error: E) -> T {
        (self.mapper)(error)
    }
    
    /// Map a result
    pub fn map_result<V>(&self, result: std::result::Result<V, E>) -> std::result::Result<V, T> {
        result.map_err(|e| self.map(e))
    }
}

/// Map an error to an Error
pub fn map_error<E, F>(error: E, mapper: F) -> Error
where
    F: FnOnce(E) -> Error,
{
    mapper(error)
}

/// Map an error to an ApiError
pub fn map_api_error<E, F>(error: E, mapper: F) -> ApiError
where
    F: FnOnce(E) -> ApiError,
{
    mapper(error)
}

/// Map a result to a Result<T, Error>
pub fn map_result<T, E, F>(result: std::result::Result<T, E>, mapper: F) -> Result<T>
where
    F: FnOnce(E) -> Error,
{
    result.map_err(mapper)
}

/// Map a result to an ApiResult<T>
pub fn map_api_result<T, E, F>(result: std::result::Result<T, E>, mapper: F) -> ApiResult<T>
where
    F: FnOnce(E) -> ApiError,
{
    result.map_err(mapper)
}

/// Extension trait for Result to map errors
pub trait MapErrorExt<T, E> {
    /// Map an error to an Error
    fn map_error<F>(self, mapper: F) -> Result<T>
    where
        F: FnOnce(E) -> Error;
        
    /// Map an error to an ApiError
    fn map_api_error<F>(self, mapper: F) -> ApiResult<T>
    where
        F: FnOnce(E) -> ApiError;
}

impl<T, E> MapErrorExt<T, E> for std::result::Result<T, E> {
    fn map_error<F>(self, mapper: F) -> Result<T>
    where
        F: FnOnce(E) -> Error,
    {
        self.map_err(mapper)
    }
    
    fn map_api_error<F>(self, mapper: F) -> ApiResult<T>
    where
        F: FnOnce(E) -> ApiError,
    {
        self.map_err(mapper)
    }
}

/// Common error mappers
pub mod mappers {
    use super::*;
    use crate::errors::error::{Error, ErrorKind, ErrorCode};
    use crate::errors::api_error::{ApiError, ApiErrorKind};
    
    /// Map a sqlx::Error to an Error
    pub fn sqlx_error_mapper(err: sqlx::Error) -> Error {
        match err {
            sqlx::Error::RowNotFound => Error::not_found("Resource not found"),
            _ => Error::database(format!("Database error: {}", err))
                .with_source(err),
        }
    }
    
    /// Map a reqwest::Error to an Error
    pub fn reqwest_error_mapper(err: reqwest::Error) -> Error {
        if err.is_timeout() {
            Error::timeout(format!("Request timed out: {}", err))
                .with_source(err)
        } else if err.is_connect() {
            Error::network(format!("Connection error: {}", err))
                .with_source(err)
        } else {
            Error::external_api(format!("Request error: {}", err))
                .with_source(err)
        }
    }
    
    /// Map a serde_json::Error to an Error
    pub fn serde_json_error_mapper(err: serde_json::Error) -> Error {
        Error::parsing(format!("JSON parsing error: {}", err))
            .with_source(err)
    }
    
    /// Map a reqwest::Error to an ApiError
    pub fn reqwest_api_error_mapper(err: reqwest::Error) -> ApiError {
        if err.is_timeout() {
            ApiError::timeout(format!("Request timed out: {}", err))
        } else if err.is_connect() {
            ApiError::network(format!("Connection error: {}", err))
        } else if let Some(status) = err.status() {
            ApiError::server(status.as_u16(), format!("Server error: {}", err))
        } else {
            ApiError::unexpected(format!("Unexpected error: {}", err))
        }
    }
    
    /// Map a serde_json::Error to an ApiError
    pub fn serde_json_api_error_mapper(err: serde_json::Error) -> ApiError {
        ApiError::deserialization(format!("Failed to parse response: {}", err))
    }
}
