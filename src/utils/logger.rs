// src/utils/logger.rs
use log::{info, error, warn};
use serde_json::Value;
use std::fmt;

/// Simple logger implementation for the Canvas-Discourse integration
pub struct Logger {
    module_name: String,
}

impl Logger {
    /// Create a new logger instance for a specific module
    pub fn new(module_name: &str) -> Self {
        Logger {
            module_name: module_name.to_string(),
        }
    }

    /// Log an info message
    pub fn info<T: fmt::Display>(&self, message: T, context: Option<&Value>) {
        match context {
            Some(ctx) => info!("[{}] {} {:?}", self.module_name, message, ctx),
            None => info!("[{}] {}", self.module_name, message),
        }
    }

    /// Log an error message
    pub fn error<T: fmt::Display>(&self, message: T, context: Option<&Value>) {
        match context {
            Some(ctx) => error!("[{}] {} {:?}", self.module_name, message, ctx),
            None => error!("[{}] {}", self.module_name, message),
        }
    }

    /// Log a warning message
    pub fn warn<T: fmt::Display>(&self, message: T, context: Option<&Value>) {
        match context {
            Some(ctx) => warn!("[{}] {} {:?}", self.module_name, message, ctx),
            None => warn!("[{}] {}", self.module_name, message),
        }
    }
}

/// Create a logger instance for a specific module
pub fn create_logger(module_name: &str) -> Logger {
    Logger::new(module_name)
}
