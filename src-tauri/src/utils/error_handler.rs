use std::panic;
use log::{error, warn};
use crate::error::Error;
use crate::utils::errors::ApiError;
use crate::utils::logger;

/// ErrorHandler provides centralized error handling functionality
/// for both synchronous and asynchronous operations
pub struct ErrorHandler {
    /// Whether to show errors in the UI
    show_ui_errors: bool,
    /// Whether to log all errors
    log_all_errors: bool,
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self {
            show_ui_errors: true,
            log_all_errors: true,
        }
    }
}

impl ErrorHandler {
    /// Create a new ErrorHandler
    pub fn new(show_ui_errors: bool, log_all_errors: bool) -> Self {
        Self {
            show_ui_errors,
            log_all_errors,
        }
    }

    /// Handle application errors
    pub fn handle_error(&self, err: &Error) {
        if self.log_all_errors {
            error!("Application error: {}", err);
        }

        // Here you would implement UI notification logic
        if self.show_ui_errors {
            // Example: send to frontend via a Tauri event
            #[cfg(feature = "tauri-runtime")]
            if let Some(app_handle) = tauri::AppHandle::try_from_env() {
                let _ = app_handle.emit_all("error", format!("{}", err));
            }
        }
    }

    /// Handle API errors
    pub fn handle_api_error(&self, err: &ApiError) {
        if self.log_all_errors {
            error!("API error: {}", err);
        }

        // Special handling for offline errors
        if matches!(err, ApiError::Offline) {
            warn!("Application is offline, some features may be unavailable");
            // Implement offline mode logic here
        }

        // Here you would implement UI notification logic
        if self.show_ui_errors {
            // Example: send to frontend via a Tauri event
            #[cfg(feature = "tauri-runtime")]
            if let Some(app_handle) = tauri::AppHandle::try_from_env() {
                let _ = app_handle.emit_all("api-error", format!("{}", err));
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
    pub fn handle_result<T>(&self, result: Result<T, Error>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                self.handle_error(&err);
                None
            }
        }
    }

    /// Helper function for handling API results
    pub fn handle_api_result<T>(&self, result: Result<T, ApiError>) -> Option<T> {
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
        if logger::is_logger_initialized() == false {
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
