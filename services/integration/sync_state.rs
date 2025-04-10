// services/integration/sync_state.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::shared::logger::Logger;
use crate::shared::db::Database;

/// Sync State Manager
///
/// Tracks the synchronization state between Canvas and Discourse systems.
/// Maintains records of which entities have been synced, when, and their current status.
pub struct SyncState {
    db: Arc<Mutex<Database>>,
    logger: Logger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    pub entity_type: String,
    pub source_id: String,
    pub source_system: String,
    pub target_id: Option<String>,
    pub last_synced: Option<DateTime<Utc>>,
    pub status: String,
    pub error: Option<String>,
    pub version: i32,
}

impl SyncState {
    /// Create a new sync state manager
    pub fn new() -> Self {
        SyncState {
            db: Arc::new(Mutex::new(Database::new())),
            logger: Logger::new("SyncState"),
        }
    }
    
    /// Get the sync status for an entity
    ///
    /// @param entity_type - Type of entity (user, course, etc.)
    /// @param source_id - ID in the source system
    /// @param source_system - Source system ('canvas' or 'discourse')
    /// @returns Sync status information
    pub async fn get_sync_status(
        &self,
        entity_type: &str,
        source_id: &str,
        source_system: &str
    ) -> Result<Option<SyncRecord>, Box<dyn Error>> {
        try {
            let db = self.db.lock().await;
            
            let status = db.sync_state.find_one(
                entity_type.to_string(),
                source_id.to_string(),
                source_system.to_string()
            ).await?;
            
            Ok(status)
        } catch e {
            self.logger.error(&format!("Error retrieving sync status: {}", e), None);
            Err(e.into())
        }
    }
    
    /// Update sync status for an entity
    ///
    /// @param entity_type - Type of entity (user, course, etc.)
    /// @param source_id - ID in the source system
    /// @param source_system - Source system ('canvas' or 'discourse')
    /// @param target_id - ID in the target system
    /// @param status - Sync status
    /// @param error - Error message if any
    /// @returns Updated sync record
    pub async fn update_sync_status(
        &self,
        entity_type: &str,
        source_id: &str,
        source_system: &str,
        target_id: Option<String>,
        status: &str,
        error: Option<String>
    ) -> Result<SyncRecord, Box<dyn Error>> {
        try {
            let mut db = self.db.lock().await;
            
            // Find existing record or create new one
            let existing = db.sync_state.find_one(
                entity_type.to_string(),
                source_id.to_string(),
                source_system.to_string()
            ).await?;
            
            let version = match &existing {
                Some(record) => record.version + 1,
                None => 1
            };
            
            let record = SyncRecord {
                entity_type: entity_type.to_string(),
                source_id: source_id.to_string(),
                source_system: source_system.to_string(),
                target_id,
                last_synced: Some(Utc::now()),
                status: status.to_string(),
                error,
                version,
            };
            
            db.sync_state.upsert(record.clone()).await?;
            
            self.logger.info(
                &format!("Updated sync status for {}/{}/{} to {}",
                    entity_type, source_id, source_system, status
                ),
                None
            );
            
            Ok(record)
        } catch e {
            self.logger.error(&format!("Error updating sync status: {}", e), None);
            Err(e.into())
        }
    }
    
    /// Get all entities of a specific type that need synchronization
    ///
    /// @param entity_type - Type of entity (user, course, etc.)
    /// @param status - (Optional) Filter by status
    /// @returns List of entities that need synchronization
    pub async fn get_pending_sync_entities(
        &self,
        entity_type: &str,
        status: Option<&str>
    ) -> Result<Vec<SyncRecord>, Box<dyn Error>> {
        try {
            let db = self.db.lock().await;
            
            let filters = match status {
                Some(status_val) => vec![
                    ("entity_type".to_string(), entity_type.to_string()),
                    ("status".to_string(), status_val.to_string())
                ],
                None => vec![
                    ("entity_type".to_string(), entity_type.to_string()),
                ]
            };
            
            let records = db.sync_state.find_many(filters).await?;
            
            Ok(records)
        } catch e {
            self.logger.error(&format!("Error retrieving pending sync entities: {}", e), None);
            Err(e.into())
        }
    }
}
