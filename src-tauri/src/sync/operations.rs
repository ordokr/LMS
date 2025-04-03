use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

/// Represents a CRDT operation type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationType {
    Create,
    Update,
    Delete,
    Reference,
}

/// Represents a sync operation that can be performed offline and synchronized later
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    pub id: String,
    pub device_id: String,
    pub user_id: i64,
    pub operation_type: OperationType,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub payload: Value,
    pub timestamp: i64,
    pub vector_clock: HashMap<String, i64>,
    pub synced: bool,
    pub synced_at: Option<i64>,
}

impl SyncOperation {
    pub fn new(
        device_id: &str,
        user_id: i64,
        operation_type: OperationType,
        entity_type: &str,
        entity_id: Option<&str>,
        payload: Value,
        vector_clock: HashMap<String, i64>,
    ) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            id: Uuid::new_v4().to_string(),
            device_id: device_id.to_string(),
            user_id,
            operation_type,
            entity_type: entity_type.to_string(),
            entity_id: entity_id.map(ToString::to_string),
            payload,
            timestamp: now,
            vector_clock,
            synced: false,
            synced_at: None,
        }
    }
    
    // Factory methods for common operations
    
    pub fn create(
        device_id: &str,
        user_id: i64,
        entity_type: &str,
        payload: Value,
        vector_clock: HashMap<String, i64>,
    ) -> Self {
        Self::new(
            device_id,
            user_id,
            OperationType::Create,
            entity_type,
            None,
            payload,
            vector_clock,
        )
    }
    
    pub fn update(
        device_id: &str,
        user_id: i64,
        entity_type: &str,
        entity_id: &str,
        payload: Value,
        vector_clock: HashMap<String, i64>,
    ) -> Self {
        Self::new(
            device_id,
            user_id,
            OperationType::Update,
            entity_type,
            Some(entity_id),
            payload,
            vector_clock,
        )
    }
    
    pub fn delete(
        device_id: &str,
        user_id: i64,
        entity_type: &str,
        entity_id: &str,
        vector_clock: HashMap<String, i64>,
    ) -> Self {
        Self::new(
            device_id,
            user_id,
            OperationType::Delete,
            entity_type,
            Some(entity_id),
            Value::Null,
            vector_clock,
        )
    }
    
    pub fn reference(
        device_id: &str,
        user_id: i64,
        source_type: &str,
        source_id: &str,
        target_type: &str,
        target_id: &str,
        vector_clock: HashMap<String, i64>,
    ) -> Self {
        let payload = serde_json::json!({
            "source_type": source_type,
            "source_id": source_id,
            "target_type": target_type,
            "target_id": target_id,
        });
        
        Self::new(
            device_id,
            user_id,
            OperationType::Reference,
            "reference",
            None,
            payload,
            vector_clock,
        )
    }
}

/// Batch of sync operations sent together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBatch {
    pub device_id: String,
    pub user_id: i64,
    pub operations: Vec<SyncOperation>,
    pub timestamp: i64,
    pub vector_clock: HashMap<String, i64>,
}

impl SyncBatch {
    pub fn new(device_id: &str, user_id: i64, operations: Vec<SyncOperation>, vector_clock: HashMap<String, i64>) -> Self {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        
        Self {
            device_id: device_id.to_string(),
            user_id,
            operations,
            timestamp: now,
            vector_clock,
        }
    }
}