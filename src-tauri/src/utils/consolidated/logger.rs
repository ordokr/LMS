use log::{debug, error, info, warn, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use serde::Serialize;
use std::fmt::Debug;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::errors::error::{Error, Result};

// Static flag to track if logger has been initialized
static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Logger for the application
/// 
/// This is a simple wrapper around the standard Rust logging facade
/// that maintains a consistent API.
#[derive(Debug, Clone)]
pub struct Logger {
    /// Module name for the logger
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

/// Initialize the logger with default configuration
/// 
/// This function initializes the logger with console and file output.
/// It should be called once at the start of the application.
pub fn init_logger() -> Result<()> {
    // Check if logger is already initialized
    if LOGGER_INITIALIZED.load(Ordering::SeqCst) {
        return Ok(());
    }
    
    // Create a console appender
    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {h({l})} {m}{n}")))
        .build();
    
    // Create a file appender
    let log_dir = std::env::current_dir()
        .map_err(|e| Error::internal(format!("Failed to get current directory: {}", e)))?
        .join("logs");
    
    // Create logs directory if it doesn't exist
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)
            .map_err(|e| Error::internal(format!("Failed to create logs directory: {}", e)))?;
    }
    
    let log_file = log_dir.join("application.log");
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} {m}{n}")))
        .build(log_file)
        .map_err(|e| Error::internal(format!("Failed to create file appender: {}", e)))?;
    
    // Build the configuration
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("file", Box::new(file)),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Debug),
        )
        .map_err(|e| Error::internal(format!("Failed to build logger config: {}", e)))?;
    
    // Initialize the logger
    log4rs::init_config(config)
        .map_err(|e| Error::internal(format!("Failed to initialize logger: {}", e)))?;
    
    // Set the initialized flag
    LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
    
    Ok(())
}

/// Initialize the logger with a custom configuration file
/// 
/// # Arguments
/// * `config_path` - Path to the log4rs YAML configuration file
pub fn init_logger_with_config(config_path: &Path) -> Result<()> {
    // Check if logger is already initialized
    if LOGGER_INITIALIZED.load(Ordering::SeqCst) {
        return Ok(());
    }
    
    // Initialize the logger with the config file
    log4rs::init_file(config_path, Default::default())
        .map_err(|e| Error::internal(format!("Failed to initialize logger with config file: {}", e)))?;
    
    // Set the initialized flag
    LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
    
    Ok(())
}

/// Check if the logger has been initialized
pub fn is_logger_initialized() -> bool {
    LOGGER_INITIALIZED.load(Ordering::SeqCst)
}

/// Log an info message
/// 
/// # Arguments
/// * `module` - Module name
/// * `message` - Message to log
/// * `context` - Optional context data, must implement Serialize
pub fn log_info<T: Serialize + Debug>(module: &str, message: &str, context: Option<T>) {
    let logger = Logger::new(module);
    logger.info(message, context);
}

/// Log an error message
/// 
/// # Arguments
/// * `module` - Module name
/// * `message` - Message to log
/// * `context` - Optional context data, must implement Serialize
pub fn log_error<T: Serialize + Debug>(module: &str, message: &str, context: Option<T>) {
    let logger = Logger::new(module);
    logger.error(message, context);
}

/// Log a warning message
/// 
/// # Arguments
/// * `module` - Module name
/// * `message` - Message to log
/// * `context` - Optional context data, must implement Serialize
pub fn log_warn<T: Serialize + Debug>(module: &str, message: &str, context: Option<T>) {
    let logger = Logger::new(module);
    logger.warn(message, context);
}

/// Log a debug message
/// 
/// # Arguments
/// * `module` - Module name
/// * `message` - Message to log
/// * `context` - Optional context data, must implement Serialize
pub fn log_debug<T: Serialize + Debug>(module: &str, message: &str, context: Option<T>) {
    let logger = Logger::new(module);
    logger.debug(message, context);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_logger() {
        let logger = create_logger("test");
        assert_eq!(logger.module_name, "test");
    }
    
    #[test]
    fn test_logger_methods() {
        let logger = create_logger("test");
        
        // These should not panic
        logger.info("Info message", None::<String>);
        logger.error("Error message", None::<String>);
        logger.warn("Warning message", None::<String>);
        logger.debug("Debug message", None::<String>);
        
        let context = serde_json::json!({
            "key": "value",
            "number": 123
        });
        
        // These should not panic
        logger.info("Info message with context", Some(context.clone()));
        logger.error("Error message with context", Some(context.clone()));
        logger.warn("Warning message with context", Some(context.clone()));
        logger.debug("Debug message with context", Some(context));
    }
}
