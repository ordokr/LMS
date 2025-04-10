use log::{debug, error, info, warn};
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;

/// Logger for the Canvas-Discourse integration
/// 
/// This is a simple wrapper around the standard Rust logging facade
/// that maintains an API similar to the JavaScript version.
pub struct Logger {
    module_name: String,
}

impl Logger {
    /// Create a new logger with the given module name
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
        }
    }

    /// Log an info message
    /// 
    /// # Arguments
    /// * `message` - Message to log
    /// * `context` - Optional context data, must implement Serialize
    pub fn info<T: Serialize + Debug>(&self, message: &str, context: Option<T>) {
        match context {
            Some(ctx) => info!("[{}] {} {:?}", self.module_name, message, ctx),
            None => info!("[{}] {}", self.module_name, message),
        }
    }

    /// Log an error message
    /// 
    /// # Arguments
    /// * `message` - Message to log
    /// * `context` - Optional context data, must implement Serialize
    pub fn error<T: Serialize + Debug>(&self, message: &str, context: Option<T>) {
        match context {
            Some(ctx) => error!("[{}] {} {:?}", self.module_name, message, ctx),
            None => error!("[{}] {}", self.module_name, message),
        }
    }

    /// Log a warning message
    /// 
    /// # Arguments
    /// * `message` - Message to log
    /// * `context` - Optional context data, must implement Serialize
    pub fn warn<T: Serialize + Debug>(&self, message: &str, context: Option<T>) {
        match context {
            Some(ctx) => warn!("[{}] {} {:?}", self.module_name, message, ctx),
            None => warn!("[{}] {}", self.module_name, message),
        }
    }

    /// Log a debug message
    /// 
    /// # Arguments
    /// * `message` - Message to log
    /// * `context` - Optional context data, must implement Serialize
    pub fn debug<T: Serialize + Debug>(&self, message: &str, context: Option<T>) {
        match context {
            Some(ctx) => debug!("[{}] {} {:?}", self.module_name, message, ctx),
            None => debug!("[{}] {}", self.module_name, message),
        }
    }
}

/// Create a logger with the given module name
/// 
/// This function mirrors the JavaScript API for easy migration
pub fn create_logger(module_name: &str) -> Logger {
    Logger::new(module_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_logger() {
        let logger = create_logger("test");
        assert_eq!(logger.module_name, "test");
    }
}
