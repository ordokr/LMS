//! Synchronization Transaction model
//!
//! Manages operations and transactions for synchronizing between Canvas and Discourse

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// The type of entity being synchronized
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    Course,
    Module,
    Assignment,
    Discussion,
    Announcement,
    User,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityType::Course => write!(f, "Course"),
            EntityType::Module => write!(f, "Module"),
            EntityType::Assignment => write!(f, "Assignment"),
            EntityType::Discussion => write!(f, "Discussion"),
            EntityType::Announcement => write!(f, "Announcement"),
            EntityType::User => write!(f, "User"),
        }
    }
}

/// The direction of synchronization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncDirection {
    CanvasToDiscourse,
    DiscourseToCanvas,
    Bidirectional,
}

impl fmt::Display for SyncDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncDirection::CanvasToDiscourse => write!(f, "Canvas → Discourse"),
            SyncDirection::DiscourseToCanvas => write!(f, "Discourse → Canvas"),
            SyncDirection::Bidirectional => write!(f, "Bidirectional"),
        }
    }
}

/// The status of a synchronization transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncStatus::Pending => write!(f, "Pending"),
            SyncStatus::InProgress => write!(f, "In Progress"),
            SyncStatus::Completed => write!(f, "Completed"),
            SyncStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// A transaction record for a synchronization operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncTransaction {
    /// Unique ID for the transaction
    pub transaction_id: String,
    
    /// Type of entity being synchronized
    pub entity_type: EntityType,
    
    /// ID of the entity in the source system
    pub source_entity_id: String,
    
    /// ID of the entity in the target system (if known)
    pub target_entity_id: Option<String>,
    
    /// Direction of synchronization
    pub direction: SyncDirection,
    
    /// Current status of the synchronization
    pub status: SyncStatus,
    
    /// Timestamp when the transaction was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the transaction was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Error message if the synchronization failed
    pub error_message: Option<String>,
    
    /// Additional data about the transaction
    pub metadata: Option<serde_json::Value>,
}

impl SyncTransaction {
    /// Create a new synchronization transaction
    pub fn new(
        entity_type: EntityType,
        source_entity_id: &str,
        direction: SyncDirection,
    ) -> Self {
        let now = Utc::now();
        let transaction_id = format!("sync-{}-{}-{}", entity_type, source_entity_id, now.timestamp());
        
        Self {
            transaction_id,
            entity_type,
            source_entity_id: source_entity_id.to_string(),
            target_entity_id: None,
            direction,
            status: SyncStatus::Pending,
            created_at: now,
            updated_at: now,
            error_message: None,
            metadata: None,
        }
    }
    
    /// Update the status of the transaction
    pub fn update_status(&mut self, status: SyncStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// Mark the transaction as failed with an error message
    pub fn mark_failed(&mut self, error_message: &str) {
        self.status = SyncStatus::Failed;
        self.error_message = Some(error_message.to_string());
        self.updated_at = Utc::now();
    }
    
    /// Mark the transaction as completed with the target entity ID
    pub fn mark_completed(&mut self, target_entity_id: &str) {
        self.status = SyncStatus::Completed;
        self.target_entity_id = Some(target_entity_id.to_string());
        self.updated_at = Utc::now();
    }
    
    /// Set additional metadata for the transaction
    pub fn set_metadata(&mut self, metadata: serde_json::Value) {
        self.metadata = Some(metadata);
        self.updated_at = Utc::now();
    }
}
