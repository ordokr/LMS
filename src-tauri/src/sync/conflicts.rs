use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::operations::{SyncOperation, OperationType};

/// Type of conflict between operations
pub enum ConflictType {
    CreateCreate,    // Two creates for the same entity
    CreateUpdate,    // Create and update for the same entity
    CreateDelete,    // Create and delete for the same entity
    UpdateUpdate,    // Two updates for the same entity
    UpdateDelete,    // Update and delete for the same entity
    DeleteDelete,    // Two deletes for the same entity
}

/// Result of conflict resolution
pub enum ConflictResolution {
    KeepFirst,       // Keep the first operation
    KeepSecond,      // Keep the second operation
    Merge,           // Merge the operations
    KeepBoth,        // Keep both operations
}

/// Handles conflict resolution between sync operations
pub struct ConflictResolver;

impl ConflictResolver {
    /// Detect if two operations are in conflict
    pub fn detect_conflict(op1: &SyncOperation, op2: &SyncOperation) -> Option<ConflictType> {
        // If operations affect different entity types or different entities, no conflict
        if op1.entity_type != op2.entity_type {
            return None;
        }
        
        if let (Some(id1), Some(id2)) = (&op1.entity_id, &op2.entity_id) {
            if id1 != id2 {
                return None;
            }
        }
        
        // Determine conflict type based on operation types
        match (&op1.operation_type, &op2.operation_type) {
            (OperationType::Create, OperationType::Create) => Some(ConflictType::CreateCreate),
            (OperationType::Create, OperationType::Update) => Some(ConflictType::CreateUpdate),
            (OperationType::Create, OperationType::Delete) => Some(ConflictType::CreateDelete),
            (OperationType::Update, OperationType::Create) => Some(ConflictType::CreateUpdate),
            (OperationType::Update, OperationType::Update) => Some(ConflictType::UpdateUpdate),
            (OperationType::Update, OperationType::Delete) => Some(ConflictType::UpdateDelete),
            (OperationType::Delete, OperationType::Create) => Some(ConflictType::CreateDelete),
            (OperationType::Delete, OperationType::Update) => Some(ConflictType::UpdateDelete),
            (OperationType::Delete, OperationType::Delete) => Some(ConflictType::DeleteDelete),
            _ => None, // Reference operations or combinations with Reference are handled separately
        }
    }
    
    /// Resolve conflict between two operations
    pub fn resolve_conflict(op1: &SyncOperation, op2: &SyncOperation) -> ConflictResolution {
        let conflict_type = Self::detect_conflict(op1, op2);
        
        if conflict_type.is_none() {
            return ConflictResolution::KeepBoth;
        }
        
        match conflict_type.unwrap() {
            ConflictType::CreateCreate => Self::resolve_create_create(op1, op2),
            ConflictType::CreateUpdate => Self::resolve_create_update(op1, op2),
            ConflictType::CreateDelete => Self::resolve_create_delete(op1, op2),
            ConflictType::UpdateUpdate => Self::resolve_update_update(op1, op2),
            ConflictType::UpdateDelete => Self::resolve_update_delete(op1, op2),
            ConflictType::DeleteDelete => ConflictResolution::KeepFirst, // Only need one delete
        }
    }
    
    /// Merge two update operations
    pub fn merge_updates(op1: &SyncOperation, op2: &SyncOperation) -> SyncOperation {
        let mut merged_payload = match &op1.payload {
            Value::Object(map) => map.clone(),
            _ => serde_json::Map::new(),
        };
        
        if let Value::Object(map2) = &op2.payload {
            for (key, value) in map2 {
                merged_payload.insert(key.clone(), value.clone());
            }
        }
        
        // Create a new operation with merged data
        let mut merged_op = op1.clone();
        merged_op.payload = Value::Object(merged_payload);
        
        // Merge vector clocks
        let mut merged_vector_clock = op1.vector_clock.clone();
        for (device, clock) in &op2.vector_clock {
            merged_vector_clock.insert(
                device.clone(),
                std::cmp::max(
                    *merged_vector_clock.get(device).unwrap_or(&0),
                    *clock
                ),
            );
        }
        merged_op.vector_clock = merged_vector_clock;
        
        // Use the later timestamp
        if op2.timestamp > op1.timestamp {
            merged_op.timestamp = op2.timestamp;
        }
        
        merged_op
    }
    
    // Specific conflict resolution strategies
    
    fn resolve_create_create(op1: &SyncOperation, op2: &SyncOperation) -> ConflictResolution {
        // For create conflicts, keep the one with more fields or the later one
        match compare_object_fields_count(&op1.payload, &op2.payload) {
            Ordering::Greater => ConflictResolution::KeepFirst,
            Ordering::Less => ConflictResolution::KeepSecond,
            Ordering::Equal => {
                // If equal number of fields, choose the later one
                if op1.timestamp >= op2.timestamp {
                    ConflictResolution::KeepFirst
                } else {
                    ConflictResolution::KeepSecond
                }
            }
        }
    }
    
    fn resolve_create_update(op1: &SyncOperation, op2: &SyncOperation) -> ConflictResolution {
        // Always apply the update to the create operation
        match (op1.operation_type, op2.operation_type) {
            (OperationType::Create, OperationType::Update) => ConflictResolution::Merge,
            (OperationType::Update, OperationType::Create) => ConflictResolution::Merge,
            _ => unreachable!(), // This function is only called for Create-Update conflicts
        }
    }
    
    fn resolve_create_delete(op1: &SyncOperation, op2: &SyncOperation) -> ConflictResolution {
        // Create followed by delete means the entity is deleted
        // Delete followed by create means the entity is recreated
        match (op1.operation_type, op2.operation_type) {
            (OperationType::Create, OperationType::Delete) => {
                if op1.timestamp < op2.timestamp {
                    ConflictResolution::KeepSecond
                } else {
                    ConflictResolution::KeepFirst
                }
            },
            (OperationType::Delete, OperationType::Create) => {
                if op1.timestamp < op2.timestamp {
                    ConflictResolution::KeepSecond
                } else {
                    ConflictResolution::KeepFirst
                }
            },
            _ => unreachable!(), // This function is only called for Create-Delete conflicts
        }
    }
    
    fn resolve_update_update(op1: &SyncOperation, op2: &SyncOperation) -> ConflictResolution {
        // Merge the updates, prioritizing specific fields based on rules
        ConflictResolution::Merge
    }
    
    fn resolve_update_delete(op1: &SyncOperation, op2: &SyncOperation) -> ConflictResolution {
        // Update followed by delete means the entity is deleted
        // Delete followed by update is invalid, but we handle it as a delete
        match (op1.operation_type, op2.operation_type) {
            (OperationType::Update, OperationType::Delete) => {
                if op1.timestamp < op2.timestamp {
                    ConflictResolution::KeepSecond
                } else {
                    ConflictResolution::KeepFirst
                }
            },
            (OperationType::Delete, OperationType::Update) => {
                if op1.timestamp < op2.timestamp {
                    ConflictResolution::KeepSecond
                } else {
                    ConflictResolution::KeepFirst
                }
            },
            _ => unreachable!(), // This function is only called for Update-Delete conflicts
        }
    }
}

// Helper function to compare the number of fields in two JSON objects
fn compare_object_fields_count(val1: &Value, val2: &Value) -> Ordering {
    match (val1, val2) {
        (Value::Object(obj1), Value::Object(obj2)) => obj1.len().cmp(&obj2.len()),
        (Value::Object(_), _) => Ordering::Greater,
        (_, Value::Object(_)) => Ordering::Less,
        _ => Ordering::Equal,
    }
}