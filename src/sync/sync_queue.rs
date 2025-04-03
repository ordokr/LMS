use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncQueue {
    operations: VecDeque<SyncOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperation {
    Create {
        entity_type: String,
        local_id: i64,
        data: serde_json::Value,
    },
    Update {
        entity_type: String,
        id: i64,
        data: serde_json::Value,
    },
    Delete {
        entity_type: String,
        id: i64,
    },
}

impl SyncQueue {
    pub fn new() -> Self {
        Self {
            operations: VecDeque::new(),
        }
    }
    
    /// Add a create operation to the queue
    pub fn queue_create<T: Serialize>(&mut self, entity_type: &str, local_id: i64, data: &T) -> Result<(), String> {
        match serde_json::to_value(data) {
            Ok(json_data) => {
                self.operations.push_back(SyncOperation::Create {
                    entity_type: entity_type.to_string(),
                    local_id,
                    data: json_data,
                });
                Ok(())
            },
            Err(e) => Err(format!("Failed to serialize data: {}", e)),
        }
    }
    
    /// Add an update operation to the queue
    pub fn queue_update<T: Serialize>(&mut self, entity_type: &str, id: i64, data: &T) -> Result<(), String> {
        match serde_json::to_value(data) {
            Ok(json_data) => {
                self.operations.push_back(SyncOperation::Update {
                    entity_type: entity_type.to_string(),
                    id,
                    data: json_data,
                });
                Ok(())
            },
            Err(e) => Err(format!("Failed to serialize data: {}", e)),
        }
    }
    
    /// Add a delete operation to the queue
    pub fn queue_delete(&mut self, entity_type: &str, id: i64) {
        self.operations.push_back(SyncOperation::Delete {
            entity_type: entity_type.to_string(),
            id,
        });
    }
    
    /// Get the next operation from the queue
    pub fn next_operation(&mut self) -> Option<SyncOperation> {
        self.operations.pop_front()
    }
    
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
    
    /// Get the number of operations in the queue
    pub fn len(&self) -> usize {
        self.operations.len()
    }
    
    /// Clear all operations from the queue
    pub fn clear(&mut self) {
        self.operations.clear();
    }
}