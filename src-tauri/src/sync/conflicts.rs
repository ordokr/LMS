use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use log::{debug, info, warn};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

use super::operations::{SyncOperation, OperationType};
use super::version_vector::{VersionVector, CausalRelation};

// Cache for conflict resolution to improve performance with large datasets
struct ConflictCache {
    // Maps operation IDs to their cached conflict status
    cache: HashMap<(String, String), bool>,
    // Maximum size of the cache
    max_size: usize,
    // Last access times for LRU eviction
    last_accessed: HashMap<(String, String), Instant>,
}

impl ConflictCache {
    fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
            last_accessed: HashMap::with_capacity(max_size),
        }
    }

    fn get(&mut self, op1_id: &str, op2_id: &str) -> Option<bool> {
        let key = if op1_id < op2_id {
            (op1_id.to_string(), op2_id.to_string())
        } else {
            (op2_id.to_string(), op1_id.to_string())
        };

        if let Some(&result) = self.cache.get(&key) {
            // Update access time
            self.last_accessed.insert(key, Instant::now());
            Some(result)
        } else {
            None
        }
    }

    fn set(&mut self, op1_id: &str, op2_id: &str, has_conflict: bool) {
        // Ensure we don't exceed max size
        if self.cache.len() >= self.max_size {
            self.evict_lru();
        }

        let key = if op1_id < op2_id {
            (op1_id.to_string(), op2_id.to_string())
        } else {
            (op2_id.to_string(), op1_id.to_string())
        };

        self.cache.insert(key.clone(), has_conflict);
        self.last_accessed.insert(key, Instant::now());
    }

    fn evict_lru(&mut self) {
        if let Some((oldest_key, _)) = self.last_accessed.iter()
            .min_by_key(|(_, &time)| time) {
            let oldest_key = oldest_key.clone();
            self.cache.remove(&oldest_key);
            self.last_accessed.remove(&oldest_key);
        }
    }

    fn clear(&mut self) {
        self.cache.clear();
        self.last_accessed.clear();
    }
}

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
pub struct ConflictResolver {
    // Cache for conflict detection results
    cache: Arc<Mutex<ConflictCache>>,
    // Batch size for processing large datasets
    batch_size: usize,
}

impl ConflictResolver {
    /// Create a new conflict resolver with default settings
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(ConflictCache::new(1000))),
            batch_size: 100,
        }
    }

    /// Create a new conflict resolver with custom settings
    pub fn with_config(cache_size: usize, batch_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(ConflictCache::new(cache_size))),
            batch_size,
        }
    }

    /// Clear the conflict detection cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Detect conflicts in a batch of operations efficiently
    pub fn detect_conflicts_batch(&self, operations: &[SyncOperation]) -> Vec<(usize, usize, ConflictType)> {
        let mut conflicts = Vec::new();
        let mut processed = HashSet::new();

        // Process operations in batches for better performance with large datasets
        for i in 0..operations.len() {
            if processed.contains(&i) {
                continue;
            }

            let op1 = &operations[i];

            // Create a batch of operations to compare against
            let batch_start = (i / self.batch_size) * self.batch_size;
            let batch_end = std::cmp::min(batch_start + self.batch_size, operations.len());

            for j in batch_start..batch_end {
                if i == j || processed.contains(&j) {
                    continue;
                }

                let op2 = &operations[j];

                // Check cache first
                let has_conflict = if let Ok(mut cache) = self.cache.lock() {
                    if let Some(result) = cache.get(&op1.id, &op2.id) {
                        result
                    } else {
                        let conflict = Self::detect_conflict(op1, op2).is_some();
                        cache.set(&op1.id, &op2.id, conflict);
                        conflict
                    }
                } else {
                    Self::detect_conflict(op1, op2).is_some()
                };

                if has_conflict {
                    if let Some(conflict_type) = Self::detect_conflict(op1, op2) {
                        conflicts.push((i, j, conflict_type));

                        // For certain conflict types, we can skip further processing
                        match conflict_type {
                            ConflictType::DeleteDelete => {
                                processed.insert(j);
                            },
                            _ => {}
                        }
                    }
                }
            }
        }

        conflicts
    }

    /// Resolve conflicts in a batch of operations efficiently
    pub fn resolve_conflicts_batch(&self, operations: &[SyncOperation]) -> Vec<SyncOperation> {
        let mut result = Vec::new();
        let conflicts = self.detect_conflicts_batch(operations);
        let mut processed = HashSet::new();

        // Process conflicts
        for (i, j, _) in &conflicts {
            if processed.contains(i) || processed.contains(j) {
                continue;
            }

            let op1 = &operations[*i];
            let op2 = &operations[*j];

            match Self::resolve_conflict(op1, op2) {
                ConflictResolution::KeepFirst => {
                    result.push(op1.clone());
                    processed.insert(*i);
                    processed.insert(*j);
                },
                ConflictResolution::KeepSecond => {
                    result.push(op2.clone());
                    processed.insert(*i);
                    processed.insert(*j);
                },
                ConflictResolution::Merge => {
                    result.push(Self::merge_updates(op1, op2));
                    processed.insert(*i);
                    processed.insert(*j);
                },
                ConflictResolution::KeepBoth => {
                    result.push(op1.clone());
                    result.push(op2.clone());
                    processed.insert(*i);
                    processed.insert(*j);
                },
            }
        }

        // Add remaining operations
        for (i, op) in operations.iter().enumerate() {
            if !processed.contains(&i) {
                result.push(op.clone());
            }
        }

        result
    }

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

        // Check causal relationship using version vectors
        let vv1 = VersionVector::from_hashmap(op1.vector_clock.clone());
        let vv2 = VersionVector::from_hashmap(op2.vector_clock.clone());
        let relation = vv1.causal_relation(&vv2);

        debug!("Causal relation between operations: {:?}", relation);

        // If one operation happens before the other, they're not in conflict
        // Only concurrent operations can conflict
        if relation == CausalRelation::HappensBefore || relation == CausalRelation::HappensAfter {
            debug!("Operations have causal relationship, no conflict");
            return None;
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

        // Merge vector clocks using VersionVector
        let vv1 = VersionVector::from_hashmap(op1.vector_clock.clone());
        let vv2 = VersionVector::from_hashmap(op2.vector_clock.clone());
        let merged_vv = vv1.merged_with(&vv2);
        merged_op.vector_clock = merged_vv.to_hashmap();

        // Use the later timestamp
        if op2.timestamp > op1.timestamp {
            merged_op.timestamp = op2.timestamp;
        }

        info!("Merged operations: {} and {}", op1.id, op2.id);
        debug!("Merged vector clock: {:?}", merged_op.vector_clock);

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
        // Check causal relationship using version vectors
        let vv1 = VersionVector::from_hashmap(op1.vector_clock.clone());
        let vv2 = VersionVector::from_hashmap(op2.vector_clock.clone());
        let relation = vv1.causal_relation(&vv2);

        match relation {
            CausalRelation::HappensBefore => {
                // op1 happens before op2, so op2 has the latest changes
                debug!("Update-Update: op1 happens before op2, keeping op2");
                ConflictResolution::KeepSecond
            },
            CausalRelation::HappensAfter => {
                // op1 happens after op2, so op1 has the latest changes
                debug!("Update-Update: op1 happens after op2, keeping op1");
                ConflictResolution::KeepFirst
            },
            CausalRelation::Concurrent => {
                // Concurrent updates, merge them
                debug!("Update-Update: concurrent updates, merging");
                ConflictResolution::Merge
            },
            CausalRelation::Identical => {
                // Identical version vectors, keep the one with the later timestamp
                debug!("Update-Update: identical version vectors, using timestamp");
                if op1.timestamp >= op2.timestamp {
                    ConflictResolution::KeepFirst
                } else {
                    ConflictResolution::KeepSecond
                }
            },
        }
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