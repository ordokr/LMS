use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use anyhow::{Result, anyhow};
use log::{info, warn, error, debug};
use crate::services::error_handling_service::{ErrorHandlingService, ErrorRecord, ErrorCategory, ErrorSeverity};
use crate::services::sync_manager::SyncManager;
use crate::services::api_config_service::ApiConfigService;
use uuid::Uuid;

/// Recovery strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Retry the operation
    Retry,
    /// Skip the entity and continue
    Skip,
    /// Rollback changes
    Rollback,
    /// Manual intervention required
    Manual,
}

/// Recovery service
pub struct RecoveryService {
    /// Error handling service
    error_service: Arc<ErrorHandlingService>,
    /// Sync manager
    sync_manager: Arc<SyncManager>,
    /// API config service
    api_config: Arc<Mutex<ApiConfigService>>,
    /// Whether the recovery service is running
    running: Mutex<bool>,
}

impl RecoveryService {
    /// Create a new recovery service
    pub fn new(
        error_service: Arc<ErrorHandlingService>,
        sync_manager: Arc<SyncManager>,
        api_config: Arc<Mutex<ApiConfigService>>,
    ) -> Self {
        Self {
            error_service,
            sync_manager,
            api_config,
            running: Mutex::new(false),
        }
    }

    /// Start the recovery service
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        
        if *running {
            return Err(anyhow!("Recovery service is already running"));
        }
        
        *running = true;
        
        // Clone the Arc references for the background task
        let error_service = self.error_service.clone();
        let sync_manager = self.sync_manager.clone();
        let api_config = self.api_config.clone();
        let running_mutex = self.running.clone();
        
        // Spawn a background task to process retriable errors
        tokio::spawn(async move {
            info!("Recovery service started");
            
            while *running_mutex.lock().await {
                // Process errors ready for retry
                if let Err(e) = Self::process_retriable_errors(
                    &error_service,
                    &sync_manager,
                    &api_config,
                ).await {
                    error!("Error processing retriable errors: {}", e);
                }
                
                // Sleep for a while before checking again
                sleep(Duration::from_secs(30)).await;
            }
            
            info!("Recovery service stopped");
        });
        
        Ok(())
    }

    /// Stop the recovery service
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        
        if !*running {
            return Err(anyhow!("Recovery service is not running"));
        }
        
        *running = false;
        
        Ok(())
    }

    /// Process errors ready for retry
    async fn process_retriable_errors(
        error_service: &ErrorHandlingService,
        sync_manager: &SyncManager,
        api_config: &Arc<Mutex<ApiConfigService>>,
    ) -> Result<()> {
        // Get errors ready for retry
        let errors = error_service.get_errors_ready_for_retry().await;
        
        if errors.is_empty() {
            return Ok(());
        }
        
        debug!("Processing {} retriable errors", errors.len());
        
        for error in errors {
            // Determine the recovery strategy
            let strategy = Self::determine_recovery_strategy(&error);
            
            match strategy {
                RecoveryStrategy::Retry => {
                    debug!("Retrying error: {}", error.id);
                    
                    // Attempt to retry the error
                    if let Err(e) = Self::retry_error(
                        error_service,
                        sync_manager,
                        api_config,
                        &error,
                    ).await {
                        warn!("Failed to retry error {}: {}", error.id, e);
                    }
                },
                RecoveryStrategy::Skip => {
                    debug!("Skipping error: {}", error.id);
                    
                    // Mark the error as resolved
                    if let Err(e) = error_service.resolve_error(
                        error.id,
                        "Skipped during recovery".to_string(),
                    ).await {
                        warn!("Failed to mark error {} as resolved: {}", error.id, e);
                    }
                },
                RecoveryStrategy::Rollback => {
                    debug!("Rolling back error: {}", error.id);
                    
                    // Attempt to rollback changes
                    if let Err(e) = Self::rollback_error(
                        error_service,
                        sync_manager,
                        api_config,
                        &error,
                    ).await {
                        warn!("Failed to rollback error {}: {}", error.id, e);
                    }
                },
                RecoveryStrategy::Manual => {
                    debug!("Manual intervention required for error: {}", error.id);
                    
                    // Log that manual intervention is required
                    warn!("Manual intervention required for error {}: {}", error.id, error.message);
                },
            }
        }
        
        Ok(())
    }

    /// Determine the recovery strategy for an error
    fn determine_recovery_strategy(error: &ErrorRecord) -> RecoveryStrategy {
        // Determine the recovery strategy based on the error category and severity
        match (error.category, error.severity) {
            // For API connection errors, retry if retriable
            (ErrorCategory::ApiConnection, _) if error.retriable => RecoveryStrategy::Retry,
            
            // For synchronization errors, retry if retriable
            (ErrorCategory::Synchronization, _) if error.retriable => RecoveryStrategy::Retry,
            
            // For validation errors, skip
            (ErrorCategory::Validation, _) => RecoveryStrategy::Skip,
            
            // For critical errors, require manual intervention
            (_, ErrorSeverity::Critical) => RecoveryStrategy::Manual,
            
            // For database errors, require manual intervention
            (ErrorCategory::Database, _) => RecoveryStrategy::Manual,
            
            // For configuration errors, require manual intervention
            (ErrorCategory::Configuration, _) => RecoveryStrategy::Manual,
            
            // For system errors, require manual intervention
            (ErrorCategory::System, _) => RecoveryStrategy::Manual,
            
            // For authentication/authorization errors, require manual intervention
            (ErrorCategory::Authentication, _) | (ErrorCategory::Authorization, _) => RecoveryStrategy::Manual,
            
            // For unknown errors, require manual intervention
            (ErrorCategory::Unknown, _) => RecoveryStrategy::Manual,
            
            // For other errors, skip
            _ => RecoveryStrategy::Skip,
        }
    }

    /// Retry an error
    async fn retry_error(
        error_service: &ErrorHandlingService,
        sync_manager: &SyncManager,
        api_config: &Arc<Mutex<ApiConfigService>>,
        error: &ErrorRecord,
    ) -> Result<()> {
        // Increment the retry count
        if !error_service.retry_error(error.id).await? {
            return Err(anyhow!("Error is not ready for retry"));
        }
        
        // Handle different error categories
        match error.category {
            ErrorCategory::ApiConnection => {
                // For API connection errors, test the connections
                let api_config_guard = api_config.lock().await;
                
                if error.source == "canvas_api" {
                    debug!("Testing Canvas API connection");
                    api_config_guard.test_canvas_connection().await?;
                } else if error.source == "discourse_api" {
                    debug!("Testing Discourse API connection");
                    api_config_guard.test_discourse_connection().await?;
                }
            },
            ErrorCategory::Synchronization => {
                // For synchronization errors, retry the sync if entity information is available
                if let (Some(entity_type), Some(entity_id)) = (&error.entity_type, &error.entity_id) {
                    debug!("Retrying synchronization for {}: {}", entity_type, entity_id);
                    
                    // Determine the sync direction from the error details
                    let direction = if let Some(details) = &error.details {
                        if details.contains("CanvasToDiscourse") {
                            crate::services::sync_service::SyncDirection::CanvasToDiscourse
                        } else if details.contains("DiscourseToCanvas") {
                            crate::services::sync_service::SyncDirection::DiscourseToCanvas
                        } else {
                            crate::services::sync_service::SyncDirection::Bidirectional
                        }
                    } else {
                        crate::services::sync_service::SyncDirection::Bidirectional
                    };
                    
                    // Retry the sync
                    sync_manager.sync_entity(entity_type, entity_id, direction).await?;
                }
            },
            _ => {
                // For other error categories, we don't have a specific retry strategy
                warn!("No specific retry strategy for error category: {:?}", error.category);
            },
        }
        
        Ok(())
    }

    /// Rollback changes for an error
    async fn rollback_error(
        error_service: &ErrorHandlingService,
        sync_manager: &SyncManager,
        api_config: &Arc<Mutex<ApiConfigService>>,
        error: &ErrorRecord,
    ) -> Result<()> {
        // Handle different error categories
        match error.category {
            ErrorCategory::Synchronization => {
                // For synchronization errors, rollback the sync if entity information is available
                if let (Some(entity_type), Some(entity_id)) = (&error.entity_type, &error.entity_id) {
                    debug!("Rolling back synchronization for {}: {}", entity_type, entity_id);
                    
                    // Determine the opposite sync direction from the error details
                    let direction = if let Some(details) = &error.details {
                        if details.contains("CanvasToDiscourse") {
                            crate::services::sync_service::SyncDirection::DiscourseToCanvas
                        } else if details.contains("DiscourseToCanvas") {
                            crate::services::sync_service::SyncDirection::CanvasToDiscourse
                        } else {
                            crate::services::sync_service::SyncDirection::Bidirectional
                        }
                    } else {
                        crate::services::sync_service::SyncDirection::Bidirectional
                    };
                    
                    // Rollback the sync by syncing in the opposite direction
                    sync_manager.sync_entity(entity_type, entity_id, direction).await?;
                }
            },
            _ => {
                // For other error categories, we don't have a specific rollback strategy
                warn!("No specific rollback strategy for error category: {:?}", error.category);
            },
        }
        
        Ok(())
    }

    /// Manually resolve an error
    pub async fn manually_resolve_error(
        &self,
        error_id: Uuid,
        resolution: String,
    ) -> Result<()> {
        self.error_service.resolve_error(error_id, resolution).await
    }

    /// Get all errors
    pub async fn get_all_errors(&self) -> Vec<ErrorRecord> {
        self.error_service.get_errors().await
    }

    /// Get unresolved errors
    pub async fn get_unresolved_errors(&self) -> Vec<ErrorRecord> {
        self.error_service.get_unresolved_errors().await
    }

    /// Get errors by category
    pub async fn get_errors_by_category(&self, category: ErrorCategory) -> Vec<ErrorRecord> {
        self.error_service.get_errors_by_category(category).await
    }

    /// Get errors by severity
    pub async fn get_errors_by_severity(&self, severity: ErrorSeverity) -> Vec<ErrorRecord> {
        self.error_service.get_errors_by_severity(severity).await
    }

    /// Get errors by source
    pub async fn get_errors_by_source(&self, source: &str) -> Vec<ErrorRecord> {
        self.error_service.get_errors_by_source(source).await
    }

    /// Get errors by entity
    pub async fn get_errors_by_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Vec<ErrorRecord> {
        self.error_service.get_errors_by_entity(entity_type, entity_id).await
    }

    /// Clear resolved errors
    pub async fn clear_resolved_errors(&self) -> Result<usize> {
        self.error_service.clear_resolved_errors().await
    }
}
