// Unified error handling module
// This module contains a consolidated error handling system that replaces redundant error implementations

mod error;
mod api_error;
mod error_handler;
mod error_context;
mod error_mapper;

// Re-export error types
pub use error::{Error, ErrorKind, ErrorCode, Result};
pub use api_error::{ApiError, ApiErrorKind, ApiResult};
pub use error_handler::{ErrorHandler, handle_error, handle_api_error, get_global_error_handler};
pub use error_context::{ErrorContext, with_context, with_context_fn};
pub use error_mapper::{ErrorMapper, map_error, map_api_error};
