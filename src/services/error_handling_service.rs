use std::fmt;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use anyhow::{Result, anyhow, Context};
use log::{error, warn, info, debug};
use std::collections::HashMap;

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Critical errors that require immediate attention
    Critical,
    /// Serious errors that affect functionality but don't require immediate attention
    Error,
    /// Warnings that don't affect core functionality
    Warning,
    /// Informational messages about potential issues
    Info,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Critical => write!(f, "Critical"),
            ErrorSeverity::Error => write!(f, "Error"),
            ErrorSeverity::Warning => write!(f, "Warning"),
            ErrorSeverity::Info => write!(f, "Info"),
        }
    }
}

/// Error categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// API connection errors
    ApiConnection,
    /// Authentication errors
    Authentication,
    /// Authorization errors
    Authorization,
    /// Data validation errors
    Validation,
    /// Data synchronization errors
    Synchronization,
    /// Database errors
    Database,
    /// Configuration errors
    Configuration,
    /// System errors
    System,
    /// Unknown errors
    Unknown,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::ApiConnection => write!(f, "API Connection"),
            ErrorCategory::Authentication => write!(f, "Authentication"),
            ErrorCategory::Authorization => write!(f, "Authorization"),
            ErrorCategory::Validation => write!(f, "Validation"),
            ErrorCategory::Synchronization => write!(f, "Synchronization"),
            ErrorCategory::Database => write!(f, "Database"),
            ErrorCategory::Configuration => write!(f, "Configuration"),
            ErrorCategory::System => write!(f, "System"),
            ErrorCategory::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Error record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecord {
    /// Unique identifier for the error
    pub id: Uuid,
    /// Error message
    pub message: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Error category
    pub category: ErrorCategory,
    /// Source of the error (e.g., "canvas_api", "discourse_api", "sync_manager")
    pub source: String,
    /// Related entity type (e.g., "user", "course", "discussion")
    pub entity_type: Option<String>,
    /// Related entity ID
    pub entity_id: Option<String>,
    /// Error details (e.g., stack trace, request/response details)
    pub details: Option<String>,
    /// Timestamp when the error occurred
    pub timestamp: DateTime<Utc>,
    /// Whether the error has been resolved
    pub resolved: bool,
    /// Timestamp when the error was resolved
    pub resolved_at: Option<DateTime<Utc>>,
    /// Resolution details
    pub resolution: Option<String>,
    /// Number of retries attempted
    pub retry_count: u32,
    /// Maximum number of retries allowed
    pub max_retries: u32,
    /// Whether the error is retriable
    pub retriable: bool,
    /// Next retry timestamp
    pub next_retry: Option<DateTime<Utc>>,
}

impl ErrorRecord {
    /// Create a new error record
    pub fn new(
        message: String,
        severity: ErrorSeverity,
        category: ErrorCategory,
        source: String,
        entity_type: Option<String>,
        entity_id: Option<String>,
        details: Option<String>,
        retriable: bool,
        max_retries: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            message,
            severity,
            category,
            source,
            entity_type,
            entity_id,
            details,
            timestamp: Utc::now(),
            resolved: false,
            resolved_at: None,
            resolution: None,
            retry_count: 0,
            max_retries,
            retriable,
            next_retry: if retriable { Some(Utc::now()) } else { None },
        }
    }

    /// Mark the error as resolved
    pub fn resolve(&mut self, resolution: String) {
        self.resolved = true;
        self.resolved_at = Some(Utc::now());
        self.resolution = Some(resolution);
    }

    /// Increment the retry count
    pub fn increment_retry(&mut self) -> bool {
        if !self.retriable || self.retry_count >= self.max_retries {
            return false;
        }

        self.retry_count += 1;
        
        // Exponential backoff for retries: 2^retry_count seconds
        let backoff_seconds = 2u64.pow(self.retry_count);
        self.next_retry = Some(Utc::now() + chrono::Duration::seconds(backoff_seconds as i64));
        
        true
    }

    /// Check if the error is ready for retry
    pub fn ready_for_retry(&self) -> bool {
        if !self.retriable || self.resolved || self.retry_count >= self.max_retries {
            return false;
        }

        if let Some(next_retry) = self.next_retry {
            return Utc::now() >= next_retry;
        }

        false
    }
}

/// Error handling service
pub struct ErrorHandlingService {
    /// Error records
    errors: Mutex<Vec<ErrorRecord>>,
    /// Error handlers by category
    handlers: Mutex<HashMap<ErrorCategory, Vec<Box<dyn ErrorHandler + Send + Sync>>>>,
}

/// Error handler trait
#[async_trait::async_trait]
pub trait ErrorHandler: Send + Sync {
    /// Handle an error
    async fn handle(&self, error: &mut ErrorRecord) -> Result<()>;
    /// Get the error categories this handler can handle
    fn categories(&self) -> Vec<ErrorCategory>;
}

impl ErrorHandlingService {
    /// Create a new error handling service
    pub fn new() -> Self {
        Self {
            errors: Mutex::new(Vec::new()),
            handlers: Mutex::new(HashMap::new()),
        }
    }

    /// Register an error handler
    pub async fn register_handler(&self, handler: Box<dyn ErrorHandler + Send + Sync>) {
        let mut handlers = self.handlers.lock().await;
        
        for category in handler.categories() {
            handlers
                .entry(category)
                .or_insert_with(Vec::new)
                .push(handler.clone());
        }
    }

    /// Record and handle an error
    pub async fn handle_error(
        &self,
        message: String,
        severity: ErrorSeverity,
        category: ErrorCategory,
        source: String,
        entity_type: Option<String>,
        entity_id: Option<String>,
        details: Option<String>,
        retriable: bool,
        max_retries: u32,
    ) -> Result<Uuid> {
        // Create a new error record
        let mut error_record = ErrorRecord::new(
            message.clone(),
            severity,
            category,
            source.clone(),
            entity_type.clone(),
            entity_id.clone(),
            details.clone(),
            retriable,
            max_retries,
        );

        // Log the error
        match severity {
            ErrorSeverity::Critical => error!("[{}] {}: {}", severity, source, message),
            ErrorSeverity::Error => error!("[{}] {}: {}", severity, source, message),
            ErrorSeverity::Warning => warn!("[{}] {}: {}", severity, source, message),
            ErrorSeverity::Info => info!("[{}] {}: {}", severity, source, message),
        }

        // Handle the error with registered handlers
        self.process_error(&mut error_record).await?;

        // Store the error record
        let error_id = error_record.id;
        let mut errors = self.errors.lock().await;
        errors.push(error_record);

        Ok(error_id)
    }

    /// Process an error with registered handlers
    async fn process_error(&self, error: &mut ErrorRecord) -> Result<()> {
        let handlers = self.handlers.lock().await;
        
        if let Some(category_handlers) = handlers.get(&error.category) {
            for handler in category_handlers {
                if let Err(e) = handler.handle(error).await {
                    warn!("Error handler failed: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Get all error records
    pub async fn get_errors(&self) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors.clone()
    }

    /// Get error records by category
    pub async fn get_errors_by_category(&self, category: ErrorCategory) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors
            .iter()
            .filter(|e| e.category == category)
            .cloned()
            .collect()
    }

    /// Get error records by severity
    pub async fn get_errors_by_severity(&self, severity: ErrorSeverity) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors
            .iter()
            .filter(|e| e.severity == severity)
            .cloned()
            .collect()
    }

    /// Get error records by source
    pub async fn get_errors_by_source(&self, source: &str) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors
            .iter()
            .filter(|e| e.source == source)
            .cloned()
            .collect()
    }

    /// Get error records by entity
    pub async fn get_errors_by_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors
            .iter()
            .filter(|e| {
                e.entity_type.as_deref() == Some(entity_type) && e.entity_id.as_deref() == Some(entity_id)
            })
            .cloned()
            .collect()
    }

    /// Get unresolved error records
    pub async fn get_unresolved_errors(&self) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors
            .iter()
            .filter(|e| !e.resolved)
            .cloned()
            .collect()
    }

    /// Get retriable error records
    pub async fn get_retriable_errors(&self) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors
            .iter()
            .filter(|e| e.retriable && !e.resolved && e.retry_count < e.max_retries)
            .cloned()
            .collect()
    }

    /// Get error records ready for retry
    pub async fn get_errors_ready_for_retry(&self) -> Vec<ErrorRecord> {
        let errors = self.errors.lock().await;
        errors
            .iter()
            .filter(|e| e.ready_for_retry())
            .cloned()
            .collect()
    }

    /// Resolve an error
    pub async fn resolve_error(&self, error_id: Uuid, resolution: String) -> Result<()> {
        let mut errors = self.errors.lock().await;
        
        if let Some(error) = errors.iter_mut().find(|e| e.id == error_id) {
            error.resolve(resolution);
            Ok(())
        } else {
            Err(anyhow!("Error not found"))
        }
    }

    /// Retry an error
    pub async fn retry_error(&self, error_id: Uuid) -> Result<bool> {
        let mut errors = self.errors.lock().await;
        
        if let Some(error) = errors.iter_mut().find(|e| e.id == error_id) {
            if error.ready_for_retry() {
                // Process the error with registered handlers
                self.process_error(error).await?;
                
                // Increment the retry count
                Ok(error.increment_retry())
            } else {
                Ok(false)
            }
        } else {
            Err(anyhow!("Error not found"))
        }
    }

    /// Clear resolved errors
    pub async fn clear_resolved_errors(&self) -> Result<usize> {
        let mut errors = self.errors.lock().await;
        let initial_count = errors.len();
        
        errors.retain(|e| !e.resolved);
        
        Ok(initial_count - errors.len())
    }

    /// Clear all errors
    pub async fn clear_all_errors(&self) -> Result<usize> {
        let mut errors = self.errors.lock().await;
        let count = errors.len();
        
        errors.clear();
        
        Ok(count)
    }
}

/// API connection error handler
pub struct ApiConnectionErrorHandler {
    /// Maximum number of retries
    max_retries: u32,
}

impl ApiConnectionErrorHandler {
    /// Create a new API connection error handler
    pub fn new(max_retries: u32) -> Self {
        Self { max_retries }
    }
}

#[async_trait::async_trait]
impl ErrorHandler for ApiConnectionErrorHandler {
    async fn handle(&self, error: &mut ErrorRecord) -> Result<()> {
        // For API connection errors, we'll retry with exponential backoff
        if error.retriable && error.retry_count < self.max_retries {
            debug!("Scheduling retry for API connection error: {}", error.id);
            error.increment_retry();
        }
        
        Ok(())
    }

    fn categories(&self) -> Vec<ErrorCategory> {
        vec![ErrorCategory::ApiConnection]
    }
}

/// Synchronization error handler
pub struct SynchronizationErrorHandler {
    /// Maximum number of retries
    max_retries: u32,
}

impl SynchronizationErrorHandler {
    /// Create a new synchronization error handler
    pub fn new(max_retries: u32) -> Self {
        Self { max_retries }
    }
}

#[async_trait::async_trait]
impl ErrorHandler for SynchronizationErrorHandler {
    async fn handle(&self, error: &mut ErrorRecord) -> Result<()> {
        // For synchronization errors, we'll retry with exponential backoff
        if error.retriable && error.retry_count < self.max_retries {
            debug!("Scheduling retry for synchronization error: {}", error.id);
            error.increment_retry();
        }
        
        Ok(())
    }

    fn categories(&self) -> Vec<ErrorCategory> {
        vec![ErrorCategory::Synchronization]
    }
}

/// Database error handler
pub struct DatabaseErrorHandler;

#[async_trait::async_trait]
impl ErrorHandler for DatabaseErrorHandler {
    async fn handle(&self, error: &mut ErrorRecord) -> Result<()> {
        // For database errors, we'll log them but not retry automatically
        error!("Database error: {}", error.message);
        
        Ok(())
    }

    fn categories(&self) -> Vec<ErrorCategory> {
        vec![ErrorCategory::Database]
    }
}

/// Configuration error handler
pub struct ConfigurationErrorHandler;

#[async_trait::async_trait]
impl ErrorHandler for ConfigurationErrorHandler {
    async fn handle(&self, error: &mut ErrorRecord) -> Result<()> {
        // For configuration errors, we'll log them but not retry automatically
        error!("Configuration error: {}", error.message);
        
        Ok(())
    }

    fn categories(&self) -> Vec<ErrorCategory> {
        vec![ErrorCategory::Configuration]
    }
}

/// System error handler
pub struct SystemErrorHandler;

#[async_trait::async_trait]
impl ErrorHandler for SystemErrorHandler {
    async fn handle(&self, error: &mut ErrorRecord) -> Result<()> {
        // For system errors, we'll log them but not retry automatically
        error!("System error: {}", error.message);
        
        Ok(())
    }

    fn categories(&self) -> Vec<ErrorCategory> {
        vec![ErrorCategory::System]
    }
}

/// Error handling extension trait for Result
pub trait ErrorHandlingExt<T> {
    /// Handle an error with the error handling service
    fn handle_error(
        self,
        error_service: &ErrorHandlingService,
        message: &str,
        severity: ErrorSeverity,
        category: ErrorCategory,
        source: &str,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        details: Option<&str>,
        retriable: bool,
        max_retries: u32,
    ) -> Result<T>;
}

impl<T, E> ErrorHandlingExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn handle_error(
        self,
        error_service: &ErrorHandlingService,
        message: &str,
        severity: ErrorSeverity,
        category: ErrorCategory,
        source: &str,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        details: Option<&str>,
        retriable: bool,
        max_retries: u32,
    ) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                let error_message = format!("{}: {}", message, err);
                let error_details = format!("{:?}", err);
                
                let _ = error_service.handle_error(
                    error_message.clone(),
                    severity,
                    category,
                    source.to_string(),
                    entity_type.map(|s| s.to_string()),
                    entity_id.map(|s| s.to_string()),
                    details.map(|s| s.to_string()).or(Some(error_details)),
                    retriable,
                    max_retries,
                );
                
                Err(anyhow!(error_message))
            }
        }
    }
}
