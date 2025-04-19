use std::panic;
use log::{error, warn, debug};
use crate::errors::error::Error;
use crate::errors::api_error::ApiError;
use crate::utils::logger;

/// ErrorHandler provides centralized error handling functionality
/// for both synchronous and asynchronous operations
#[derive(Debug, Clone)]
pub struct ErrorHandler {
    /// Whether to show errors in the UI
    show_ui_errors: bool,
    
    /// Whether to log all errors
    log_all_errors: bool,
    
    /// Whether to include stack traces in error logs
    include_stack_traces: bool,
    
    /// Whether to include source errors in error logs
    include_source_errors: bool,
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self {
            show_ui_errors: true,
            log_all_errors: true,
            include_stack_traces: true,
            include_source_errors: true,
        }
    }
}

impl ErrorHandler {
    /// Create a new ErrorHandler
    pub fn new(
        show_ui_errors: bool,
        log_all_errors: bool,
        include_stack_traces: bool,
        include_source_errors: bool,
    ) -> Self {
        Self {
            show_ui_errors,
            log_all_errors,
            include_stack_traces,
            include_source_errors,
        }
    }
    
    /// Handle application errors
    pub fn handle_error(&self, err: &Error) {
        if self.log_all_errors {
            let mut log_message = format!("Error [{}]: {}", err.code, err);
            
            if let Some(context) = &err.context {
                log_message.push_str(&format!("\nContext: {}", context));
            }
            
            if self.include_source_errors {
                if let Some(source) = &err.source {
                    log_message.push_str(&format!("\nSource: {}", source));
                }
            }
            
            if self.include_stack_traces {
                if let Some(stack_trace) = &err.stack_trace {
                    log_message.push_str(&format!("\nStack trace:\n{}", stack_trace));
                } else {
                    // Capture stack trace if not already present
                    let backtrace = std::backtrace::Backtrace::capture();
                    log_message.push_str(&format!("\nStack trace:\n{}", backtrace));
                }
            }
            
            match err.kind {
                crate::errors::error::ErrorKind::Internal => error!("{}", log_message),
                crate::errors::error::ErrorKind::Database => error!("{}", log_message),
                crate::errors::error::ErrorKind::ExternalApi => error!("{}", log_message),
                crate::errors::error::ErrorKind::Authentication => warn!("{}", log_message),
                crate::errors::error::ErrorKind::Authorization => warn!("{}", log_message),
                crate::errors::error::ErrorKind::NotFound => debug!("{}", log_message),
                crate::errors::error::ErrorKind::Validation => debug!("{}", log_message),
                _ => error!("{}", log_message),
            }
        }
        
        // Here you would implement UI notification logic
        if self.show_ui_errors {
            // Example: send to frontend via a Tauri event
            #[cfg(feature = "tauri-runtime")]
            if let Some(app_handle) = tauri::AppHandle::try_from_env() {
                let _ = app_handle.emit_all("error", serde_json::to_string(err).unwrap_or_else(|_| format!("{}", err)));
            }
        }
    }
    
    /// Handle API errors
    pub fn handle_api_error(&self, err: &ApiError) {
        if self.log_all_errors {
            let mut log_message = format!("API Error [{}]: {}", err.kind, err);
            
            if let Some(status_code) = err.status_code {
                log_message.push_str(&format!("\nStatus code: {}", status_code));
            }
            
            if let Some(context) = &err.context {
                log_message.push_str(&format!("\nContext: {}", context));
            }
            
            match err.kind {
                crate::errors::api_error::ApiErrorKind::Offline => warn!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::NetworkError => warn!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::ServerError => error!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::AuthError => warn!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::AuthorizationError => warn!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::DeserializationError => error!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::RateLimitError => warn!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::TimeoutError => warn!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::BadGatewayError => error!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::ServiceUnavailableError => error!("{}", log_message),
                crate::errors::api_error::ApiErrorKind::UnexpectedError => error!("{}", log_message),
            }
        }
        
        // Special handling for offline errors
        if matches!(err.kind, crate::errors::api_error::ApiErrorKind::Offline) {
            warn!("Application is offline, some features may be unavailable");
            // Implement offline mode logic here
        }
        
        // Here you would implement UI notification logic
        if self.show_ui_errors {
            // Example: send to frontend via a Tauri event
            #[cfg(feature = "tauri-runtime")]
            if let Some(app_handle) = tauri::AppHandle::try_from_env() {
                let _ = app_handle.emit_all("api-error", serde_json::to_string(err).unwrap_or_else(|_| format!("{}", err)));
            }
        }
    }
    
    /// Setup global panic hook to capture and log unexpected panics
    pub fn setup_panic_hook() {
        let original_hook = panic::take_hook();
        
        panic::set_hook(Box::new(move |panic_info| {
            // Log the panic information
            let backtrace = std::backtrace::Backtrace::capture();
            error!("Application panic: {}\nBacktrace:\n{}", panic_info, backtrace);
            
            // Call the original panic hook
            original_hook(panic_info);
        }));
    }
    
    /// Helper function for handling results
    pub fn handle_result<T>(&self, result: crate::errors::error::Result<T>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                self.handle_error(&err);
                None
            }
        }
    }
    
    /// Helper function for handling API results
    pub fn handle_api_result<T>(&self, result: crate::errors::api_error::ApiResult<T>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                self.handle_api_error(&err);
                None
            }
        }
    }
}

/// Get a global default error handler instance
pub fn get_global_error_handler() -> &'static ErrorHandler {
    use once_cell::sync::Lazy;
    
    static ERROR_HANDLER: Lazy<ErrorHandler> = Lazy::new(|| {
        // Initialize logger if needed
        if !logger::is_logger_initialized() {
            logger::init_logger();
        }
        
        ErrorHandler::default()
    });
    
    &ERROR_HANDLER
}

/// Convenience function to handle an error with the global handler
pub fn handle_error(err: &Error) {
    get_global_error_handler().handle_error(err);
}

/// Convenience function to handle an API error with the global handler
pub fn handle_api_error(err: &ApiError) {
    get_global_error_handler().handle_api_error(err);
}
