use crate::quiz::models::{Quiz, Question, Answer};
use crate::quiz::storage::HybridQuizStore;
use crate::models::network::{SyncQueue, SyncItem, SyncOperation, SyncStatus, ConnectionStatus};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{Duration, sleep};
use uuid::Uuid;
use serde_json::json;
use anyhow::{Result, anyhow};
use tracing::{debug, info, warn, error};
use chrono::Utc;
use std::collections::HashMap;

/// Priority levels for sync operations
pub enum SyncPriority {
    /// Critical operations that must be synced as soon as possible
    Critical = 0,
    
    /// High priority operations
    High = 10,
    
    /// Normal priority operations
    Normal = 20,
    
    /// Low priority operations
    Low = 30,
    
    /// Background operations that can be synced when convenient
    Background = 40,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictStrategy {
    /// Use the local version
    UseLocal,
    
    /// Use the remote version
    UseRemote,
    
    /// Merge the versions
    Merge,
    
    /// Ask the user to resolve the conflict
    AskUser,
}

/// Conflict resolution result
#[derive(Debug, Clone)]
pub enum ConflictResolution<T> {
    /// Use the local version
    UseLocal(T),
    
    /// Use the remote version
    UseRemote(T),
    
    /// Use a merged version
    UseMerged(T),
    
    /// Conflict could not be resolved
    Unresolved,
}

/// Sync conflict
#[derive(Debug, Clone)]
pub struct SyncConflict<T> {
    /// Local version
    pub local: T,
    
    /// Remote version
    pub remote: T,
    
    /// Entity ID
    pub entity_id: String,
    
    /// Entity type
    pub entity_type: String,
    
    /// Timestamp of local version
    pub local_timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Timestamp of remote version
    pub remote_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Sync notification
#[derive(Debug, Clone)]
pub struct SyncNotification {
    /// Notification ID
    pub id: String,
    
    /// Notification title
    pub title: String,
    
    /// Notification message
    pub message: String,
    
    /// Notification type
    pub notification_type: SyncNotificationType,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Related entity ID
    pub entity_id: Option<String>,
    
    /// Related entity type
    pub entity_type: Option<String>,
    
    /// Whether the notification has been read
    pub read: bool,
}

/// Sync notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncNotificationType {
    /// Sync completed successfully
    SyncComplete,
    
    /// Sync failed
    SyncFailed,
    
    /// Conflict detected
    Conflict,
    
    /// New data available
    NewData,
    
    /// Sync progress update
    Progress,
}

/// Sync manager configuration
#[derive(Debug, Clone)]
pub struct SyncManagerConfig {
    /// Sync interval in seconds
    pub sync_interval: u64,
    
    /// Maximum number of retry attempts
    pub max_retries: u32,
    
    /// Initial retry delay in seconds
    pub initial_retry_delay: u64,
    
    /// Maximum retry delay in seconds
    pub max_retry_delay: u64,
    
    /// Default conflict resolution strategy
    pub default_conflict_strategy: ConflictStrategy,
    
    /// Whether to sync automatically when online
    pub auto_sync: bool,
    
    /// Whether to show notifications
    pub show_notifications: bool,
    
    /// Maximum number of items to sync in a batch
    pub batch_size: usize,
}

impl Default for SyncManagerConfig {
    fn default() -> Self {
        Self {
            sync_interval: 60, // 1 minute
            max_retries: 5,
            initial_retry_delay: 5, // 5 seconds
            max_retry_delay: 3600, // 1 hour
            default_conflict_strategy: ConflictStrategy::AskUser,
            auto_sync: true,
            show_notifications: true,
            batch_size: 10,
        }
    }
}

/// Sync manager for handling offline synchronization
pub struct SyncManager {
    /// Storage for quiz data
    store: Arc<HybridQuizStore>,
    
    /// Sync queue
    queue: Arc<RwLock<SyncQueue>>,
    
    /// Configuration
    config: SyncManagerConfig,
    
    /// Current connection status
    connection_status: Arc<RwLock<ConnectionStatus>>,
    
    /// Current sync status
    sync_status: Arc<RwLock<SyncStatus>>,
    
    /// Notification sender
    notification_tx: mpsc::Sender<SyncNotification>,
    
    /// Conflict resolution callback
    conflict_resolver: Arc<dyn Fn(SyncConflict<serde_json::Value>) -> ConflictResolution<serde_json::Value> + Send + Sync>,
    
    /// Sync progress callback
    progress_callback: Arc<dyn Fn(usize, usize) -> () + Send + Sync>,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(
        store: Arc<HybridQuizStore>,
        config: SyncManagerConfig,
        notification_tx: mpsc::Sender<SyncNotification>,
    ) -> Self {
        let default_resolver = Arc::new(move |conflict: SyncConflict<serde_json::Value>| {
            match config.default_conflict_strategy {
                ConflictStrategy::UseLocal => ConflictResolution::UseLocal(conflict.local),
                ConflictStrategy::UseRemote => ConflictResolution::UseRemote(conflict.remote),
                ConflictStrategy::Merge => {
                    // Simple merge strategy: use the newer version
                    if conflict.local_timestamp > conflict.remote_timestamp {
                        ConflictResolution::UseLocal(conflict.local)
                    } else {
                        ConflictResolution::UseRemote(conflict.remote)
                    }
                },
                ConflictStrategy::AskUser => ConflictResolution::Unresolved,
            }
        });
        
        let default_progress = Arc::new(|_current: usize, _total: usize| {
            // Default progress callback does nothing
        });
        
        Self {
            store,
            queue: Arc::new(RwLock::new(SyncQueue::new())),
            config,
            connection_status: Arc::new(RwLock::new(ConnectionStatus::Online)),
            sync_status: Arc::new(RwLock::new(SyncStatus::Synced)),
            notification_tx,
            conflict_resolver: default_resolver,
            progress_callback: default_progress,
        }
    }
    
    /// Set the conflict resolution callback
    pub fn set_conflict_resolver<F>(&mut self, resolver: F)
    where
        F: Fn(SyncConflict<serde_json::Value>) -> ConflictResolution<serde_json::Value> + Send + Sync + 'static,
    {
        self.conflict_resolver = Arc::new(resolver);
    }
    
    /// Set the progress callback
    pub fn set_progress_callback<F>(&mut self, callback: F)
    where
        F: Fn(usize, usize) -> () + Send + Sync + 'static,
    {
        self.progress_callback = Arc::new(callback);
    }
    
    /// Start the sync manager
    pub async fn start(&self) -> Result<()> {
        // Start the background sync task
        let queue = self.queue.clone();
        let connection_status = self.connection_status.clone();
        let sync_status = self.sync_status.clone();
        let store = self.store.clone();
        let config = self.config.clone();
        let notification_tx = self.notification_tx.clone();
        let conflict_resolver = self.conflict_resolver.clone();
        let progress_callback = self.progress_callback.clone();
        
        tokio::spawn(async move {
            info!("Starting sync manager background task");
            
            loop {
                // Wait for the sync interval
                sleep(Duration::from_secs(config.sync_interval)).await;
                
                // Check if we're online and auto-sync is enabled
                let is_online = {
                    let status = connection_status.read().await;
                    *status == ConnectionStatus::Online
                };
                
                if is_online && config.auto_sync {
                    // Perform sync
                    let result = Self::sync_items(
                        queue.clone(),
                        store.clone(),
                        sync_status.clone(),
                        notification_tx.clone(),
                        conflict_resolver.clone(),
                        progress_callback.clone(),
                        config.batch_size,
                    ).await;
                    
                    if let Err(e) = result {
                        error!("Sync failed: {}", e);
                        
                        // Send notification
                        if config.show_notifications {
                            let _ = notification_tx.send(SyncNotification {
                                id: Uuid::new_v4().to_string(),
                                title: "Sync Failed".to_string(),
                                message: format!("Failed to sync data: {}", e),
                                notification_type: SyncNotificationType::SyncFailed,
                                timestamp: Utc::now(),
                                entity_id: None,
                                entity_type: None,
                                read: false,
                            }).await;
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Sync items from the queue
    async fn sync_items(
        queue: Arc<RwLock<SyncQueue>>,
        store: Arc<HybridQuizStore>,
        sync_status: Arc<RwLock<SyncStatus>>,
        notification_tx: mpsc::Sender<SyncNotification>,
        conflict_resolver: Arc<dyn Fn(SyncConflict<serde_json::Value>) -> ConflictResolution<serde_json::Value> + Send + Sync>,
        progress_callback: Arc<dyn Fn(usize, usize) -> () + Send + Sync>,
        batch_size: usize,
    ) -> Result<()> {
        // Update sync status
        {
            let mut status = sync_status.write().await;
            *status = SyncStatus::Syncing;
        }
        
        // Get items to sync
        let items_to_sync = {
            let queue_read = queue.read().await;
            if queue_read.is_empty() {
                // Nothing to sync
                let mut status = sync_status.write().await;
                *status = SyncStatus::Synced;
                return Ok(());
            }
            
            // Sort by priority and take up to batch_size items
            let mut items = queue_read.items.clone();
            items.sort_by_key(|item| item.priority);
            items.truncate(batch_size);
            items
        };
        
        let total_items = items_to_sync.len();
        let mut synced_items = 0;
        let mut failed_items = Vec::new();
        
        // Process each item
        for item in items_to_sync {
            // Update progress
            progress_callback(synced_items, total_items);
            
            // Process the item
            let result = Self::process_sync_item(
                &item,
                store.clone(),
                conflict_resolver.clone(),
            ).await;
            
            match result {
                Ok(_) => {
                    // Item synced successfully, remove from queue
                    let mut queue_write = queue.write().await;
                    queue_write.remove_item(&item.id);
                    synced_items += 1;
                    
                    // Send notification for important operations
                    if item.priority <= SyncPriority::High as u32 {
                        let _ = notification_tx.send(SyncNotification {
                            id: Uuid::new_v4().to_string(),
                            title: "Sync Complete".to_string(),
                            message: format!("{} {} synced successfully", item.entity_type, item.entity_id),
                            notification_type: SyncNotificationType::SyncComplete,
                            timestamp: Utc::now(),
                            entity_id: Some(item.entity_id.clone()),
                            entity_type: Some(item.entity_type.clone()),
                            read: false,
                        }).await;
                    }
                },
                Err(e) => {
                    // Sync failed, update retry count
                    let mut queue_write = queue.write().await;
                    let item_index = queue_write.items.iter().position(|i| i.id == item.id);
                    
                    if let Some(index) = item_index {
                        let item = &mut queue_write.items[index];
                        item.retry_count += 1;
                        item.last_retry = Some(Utc::now());
                        
                        failed_items.push((item.clone(), e.to_string()));
                    }
                }
            }
        }
        
        // Update sync status
        {
            let mut status = sync_status.write().await;
            if failed_items.is_empty() {
                // Check if there are more items in the queue
                let queue_read = queue.read().await;
                if queue_read.is_empty() {
                    *status = SyncStatus::Synced;
                } else {
                    *status = SyncStatus::Pending;
                }
            } else {
                *status = SyncStatus::Failed;
                
                // Send notification for failed items
                for (item, error) in failed_items {
                    let _ = notification_tx.send(SyncNotification {
                        id: Uuid::new_v4().to_string(),
                        title: "Sync Failed".to_string(),
                        message: format!("Failed to sync {} {}: {}", item.entity_type, item.entity_id, error),
                        notification_type: SyncNotificationType::SyncFailed,
                        timestamp: Utc::now(),
                        entity_id: Some(item.entity_id),
                        entity_type: Some(item.entity_type),
                        read: false,
                    }).await;
                }
            }
        }
        
        // Final progress update
        progress_callback(synced_items, total_items);
        
        Ok(())
    }
    
    /// Process a single sync item
    async fn process_sync_item(
        item: &SyncItem,
        store: Arc<HybridQuizStore>,
        conflict_resolver: Arc<dyn Fn(SyncConflict<serde_json::Value>) -> ConflictResolution<serde_json::Value> + Send + Sync>,
    ) -> Result<()> {
        match (item.entity_type.as_str(), item.operation) {
            ("quiz", SyncOperation::Create) => {
                // Create a new quiz
                let quiz: Quiz = serde_json::from_value(item.payload.clone())?;
                store.create_quiz(quiz).await?;
            },
            ("quiz", SyncOperation::Update) => {
                // Update an existing quiz
                let quiz: Quiz = serde_json::from_value(item.payload.clone())?;
                
                // Check for conflicts
                let remote_quiz = store.get_remote_quiz(Uuid::parse_str(&item.entity_id)?).await;
                
                if let Ok(remote_quiz) = remote_quiz {
                    // Check if there's a conflict
                    if remote_quiz.updated_at > quiz.updated_at {
                        // Conflict detected
                        let conflict = SyncConflict {
                            local: serde_json::to_value(&quiz)?,
                            remote: serde_json::to_value(&remote_quiz)?,
                            entity_id: item.entity_id.clone(),
                            entity_type: item.entity_type.clone(),
                            local_timestamp: quiz.updated_at,
                            remote_timestamp: remote_quiz.updated_at,
                        };
                        
                        // Resolve conflict
                        let resolution = conflict_resolver(conflict);
                        
                        match resolution {
                            ConflictResolution::UseLocal(_) => {
                                // Use local version
                                store.update_quiz(quiz).await?;
                            },
                            ConflictResolution::UseRemote(_) => {
                                // Use remote version, nothing to do
                            },
                            ConflictResolution::UseMerged(merged) => {
                                // Use merged version
                                let merged_quiz: Quiz = serde_json::from_value(merged)?;
                                store.update_quiz(merged_quiz).await?;
                            },
                            ConflictResolution::Unresolved => {
                                // Cannot resolve automatically
                                return Err(anyhow!("Unresolved conflict for quiz {}", item.entity_id));
                            },
                        }
                    } else {
                        // No conflict, update normally
                        store.update_quiz(quiz).await?;
                    }
                } else {
                    // Remote quiz not found, create it
                    store.create_quiz(quiz).await?;
                }
            },
            ("quiz", SyncOperation::Delete) => {
                // Delete a quiz
                let quiz_id = Uuid::parse_str(&item.entity_id)?;
                store.delete_quiz(quiz_id).await?;
            },
            ("question", SyncOperation::Create) => {
                // Create a new question
                let question: Question = serde_json::from_value(item.payload.clone())?;
                store.create_question(question).await?;
            },
            ("question", SyncOperation::Update) => {
                // Update an existing question
                let question: Question = serde_json::from_value(item.payload.clone())?;
                
                // Check for conflicts
                let remote_question = store.get_remote_question(Uuid::parse_str(&item.entity_id)?).await;
                
                if let Ok(remote_question) = remote_question {
                    // Check if there's a conflict
                    if remote_question.updated_at > question.updated_at {
                        // Conflict detected
                        let conflict = SyncConflict {
                            local: serde_json::to_value(&question)?,
                            remote: serde_json::to_value(&remote_question)?,
                            entity_id: item.entity_id.clone(),
                            entity_type: item.entity_type.clone(),
                            local_timestamp: question.updated_at,
                            remote_timestamp: remote_question.updated_at,
                        };
                        
                        // Resolve conflict
                        let resolution = conflict_resolver(conflict);
                        
                        match resolution {
                            ConflictResolution::UseLocal(_) => {
                                // Use local version
                                store.update_question(question).await?;
                            },
                            ConflictResolution::UseRemote(_) => {
                                // Use remote version, nothing to do
                            },
                            ConflictResolution::UseMerged(merged) => {
                                // Use merged version
                                let merged_question: Question = serde_json::from_value(merged)?;
                                store.update_question(merged_question).await?;
                            },
                            ConflictResolution::Unresolved => {
                                // Cannot resolve automatically
                                return Err(anyhow!("Unresolved conflict for question {}", item.entity_id));
                            },
                        }
                    } else {
                        // No conflict, update normally
                        store.update_question(question).await?;
                    }
                } else {
                    // Remote question not found, create it
                    store.create_question(question).await?;
                }
            },
            ("question", SyncOperation::Delete) => {
                // Delete a question
                let question_id = Uuid::parse_str(&item.entity_id)?;
                store.delete_question(question_id).await?;
            },
            // Add more entity types and operations as needed
            _ => {
                return Err(anyhow!("Unsupported entity type or operation: {} {:?}", item.entity_type, item.operation));
            }
        }
        
        Ok(())
    }
    
    /// Add an item to the sync queue
    pub async fn queue_sync_item(
        &self,
        entity_type: &str,
        entity_id: &str,
        operation: SyncOperation,
        payload: serde_json::Value,
        priority: SyncPriority,
    ) -> Result<()> {
        let item = SyncItem {
            id: Uuid::new_v4().to_string(),
            operation,
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            payload,
            created_at: Utc::now(),
            retry_count: 0,
            last_retry: None,
            priority: priority as u32,
        };
        
        let mut queue = self.queue.write().await;
        queue.add_item(item);
        
        // Update sync status
        let mut status = self.sync_status.write().await;
        *status = SyncStatus::Pending;
        
        Ok(())
    }
    
    /// Get the current sync status
    pub async fn get_sync_status(&self) -> SyncStatus {
        let status = self.sync_status.read().await;
        *status
    }
    
    /// Get the current connection status
    pub async fn get_connection_status(&self) -> ConnectionStatus {
        let status = self.connection_status.read().await;
        *status
    }
    
    /// Set the connection status
    pub async fn set_connection_status(&self, status: ConnectionStatus) {
        let mut current = self.connection_status.write().await;
        *current = status;
        
        // If we're back online and auto-sync is enabled, trigger a sync
        if status == ConnectionStatus::Online && self.config.auto_sync {
            self.sync_now().await;
        }
    }
    
    /// Trigger an immediate sync
    pub async fn sync_now(&self) {
        let queue = self.queue.clone();
        let connection_status = self.connection_status.clone();
        let sync_status = self.sync_status.clone();
        let store = self.store.clone();
        let notification_tx = self.notification_tx.clone();
        let conflict_resolver = self.conflict_resolver.clone();
        let progress_callback = self.progress_callback.clone();
        let batch_size = self.config.batch_size;
        
        tokio::spawn(async move {
            // Check if we're online
            let is_online = {
                let status = connection_status.read().await;
                *status == ConnectionStatus::Online
            };
            
            if is_online {
                // Perform sync
                let result = Self::sync_items(
                    queue,
                    store,
                    sync_status,
                    notification_tx.clone(),
                    conflict_resolver,
                    progress_callback,
                    batch_size,
                ).await;
                
                if let Err(e) = result {
                    error!("Manual sync failed: {}", e);
                    
                    // Send notification
                    let _ = notification_tx.send(SyncNotification {
                        id: Uuid::new_v4().to_string(),
                        title: "Manual Sync Failed".to_string(),
                        message: format!("Failed to sync data: {}", e),
                        notification_type: SyncNotificationType::SyncFailed,
                        timestamp: Utc::now(),
                        entity_id: None,
                        entity_type: None,
                        read: false,
                    }).await;
                }
            }
        });
    }
    
    /// Get the number of pending sync items
    pub async fn get_pending_count(&self) -> usize {
        let queue = self.queue.read().await;
        queue.len()
    }
    
    /// Get all pending sync items
    pub async fn get_pending_items(&self) -> Vec<SyncItem> {
        let queue = self.queue.read().await;
        queue.items.clone()
    }
    
    /// Clear all pending sync items
    pub async fn clear_pending_items(&self) -> Result<()> {
        let mut queue = self.queue.write().await;
        queue.items.clear();
        
        // Update sync status
        let mut status = self.sync_status.write().await;
        *status = SyncStatus::Synced;
        
        Ok(())
    }
    
    /// Merge two versions of a quiz
    pub fn merge_quizzes(local: &Quiz, remote: &Quiz) -> Quiz {
        // Create a new quiz with the newer metadata
        let base = if local.updated_at > remote.updated_at {
            local.clone()
        } else {
            remote.clone()
        };
        
        // Merge questions
        let mut questions = HashMap::new();
        
        // Add all local questions
        for question in &local.questions {
            questions.insert(question.id, question.clone());
        }
        
        // Add or update with remote questions
        for question in &remote.questions {
            if let Some(local_question) = questions.get(&question.id) {
                // Question exists in both versions, use the newer one
                if question.updated_at > local_question.updated_at {
                    questions.insert(question.id, question.clone());
                }
            } else {
                // Question only exists in remote, add it
                questions.insert(question.id, question.clone());
            }
        }
        
        // Create the merged quiz
        Quiz {
            id: base.id,
            title: base.title,
            description: base.description.clone(),
            created_at: base.created_at,
            updated_at: Utc::now(), // Use current time for the merged version
            questions: questions.into_values().collect(),
            settings: base.settings.clone(),
            author_id: base.author_id,
            visibility: base.visibility,
            tags: base.tags.clone(),
            study_mode: base.study_mode,
        }
    }
}
