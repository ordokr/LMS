use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::core::errors::AppError;
use super::operations::{SyncOperation, SyncBatch, OperationType};
use super::conflicts::{ConflictResolver, ConflictResolution};

pub struct SyncEngine {
    db: Pool<Sqlite>,
    device_id: String,
    vector_clock: Arc<Mutex<HashMap<String, i64>>>,
}

impl SyncEngine {
    pub fn new(db: Pool<Sqlite>) -> Self {
        // Generate a unique device ID or load from storage
        let device_id = Uuid::new_v4().to_string();
        
        Self {
            db,
            device_id,
            vector_clock: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    // Initialize vector clock from database
    pub async fn initialize(&self) -> Result<(), AppError> {
        let mut clock = self.vector_clock.lock().await;
        
        // Load vector clock from database
        let rows = sqlx::query!(
            r#"
            SELECT device_id, MAX(timestamp) as last_timestamp
            FROM sync_operations
            GROUP BY device_id
            "#,
        )
        .fetch_all(&self.db)
        .await?;
        
        for row in rows {
            if let (Some(device_id), Some(timestamp)) = (row.device_id, row.last_timestamp) {
                clock.insert(device_id, timestamp);
            }
        }
        
        // Ensure current device is in vector clock
        if !clock.contains_key(&self.device_id) {
            clock.insert(self.device_id.clone(), 0);
        }
        
        Ok(())
    }
    
    // Queue a new operation for later sync
    pub async fn queue_operation(
        &self, 
        user_id: i64,
        operation_type: OperationType,
        entity_type: &str,
        entity_id: Option<&str>,
        payload: serde_json::Value,
    ) -> Result<SyncOperation, AppError> {
        let mut clock = self.vector_clock.lock().await;
        
        // Increment vector clock for this device
        let current_time = *clock.get(&self.device_id).unwrap_or(&0);
        let new_time = current_time + 1;
        clock.insert(self.device_id.clone(), new_time);
        
        // Create the operation
        let operation = SyncOperation::new(
            &self.device_id,
            user_id,
            operation_type,
            entity_type,
            entity_id,
            payload,
            clock.clone(),
        );
        
        // Store in database
        self.store_operation(&operation).await?;
        
        Ok(operation)
    }
    
    // Store operation in database
    async fn store_operation(&self, operation: &SyncOperation) -> Result<(), AppError> {
        let op_json = serde_json::to_string(&operation)
            .map_err(|e| AppError::SyncError(format!("Failed to serialize operation: {}", e)))?;
            
        let vector_clock_json = serde_json::to_string(&operation.vector_clock)
            .map_err(|e| AppError::SyncError(format!("Failed to serialize vector clock: {}", e)))?;
        
        sqlx::query!(
            r#"
            INSERT INTO sync_operations 
            (id, device_id, user_id, operation_type, entity_type, entity_id, payload, 
             timestamp, vector_clock, synced, synced_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            operation.id,
            operation.device_id,
            operation.user_id,
            operation.operation_type as i32, // This requires a custom FROM/TO implementation
            operation.entity_type,
            operation.entity_id,
            op_json,
            operation.timestamp,
            vector_clock_json,
            operation.synced,
            operation.synced_at,
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    // Get pending operations to sync
    pub async fn get_pending_operations(&self, limit: i64) -> Result<Vec<SyncOperation>, AppError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, device_id, user_id, operation_type, entity_type, entity_id, 
                   payload, timestamp, vector_clock, synced, synced_at
            FROM sync_operations
            WHERE synced = 0
            ORDER BY timestamp
            LIMIT ?
            "#,
            limit
        )
        .fetch_all(&self.db)
        .await?;
        
        let mut operations = Vec::new();
        
        for row in rows {
            let op_type = match row.operation_type {
                0 => OperationType::Create,
                1 => OperationType::Update,
                2 => OperationType::Delete,
                3 => OperationType::Reference,
                _ => continue, // Skip unknown operation types
            };
            
            let vector_clock: HashMap<String, i64> = serde_json::from_str(&row.vector_clock)
                .map_err(|e| AppError::SyncError(format!("Failed to deserialize vector clock: {}", e)))?;
            
            let payload: serde_json::Value = serde_json::from_str(&row.payload)
                .map_err(|e| AppError::SyncError(format!("Failed to deserialize payload: {}", e)))?;
            
            let operation = SyncOperation {
                id: row.id,
                device_id: row.device_id,
                user_id: row.user_id,
                operation_type: op_type,
                entity_type: row.entity_type,
                entity_id: row.entity_id,
                payload,
                timestamp: row.timestamp,
                vector_clock,
                synced: row.synced != 0,
                synced_at: row.synced_at,
            };
            
            operations.push(operation);
        }
        
        Ok(operations)
    }
    
    // Mark operations as synced
    pub async fn mark_as_synced(&self, operation_ids: &[String]) -> Result<(), AppError> {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        
        for id in operation_ids {
            sqlx::query!(
                r#"
                UPDATE sync_operations
                SET synced = 1, synced_at = ?
                WHERE id = ?
                "#,
                now,
                id
            )
            .execute(&self.db)
            .await?;
        }
        
        Ok(())
    }
    
    // Create a sync batch to send to server
    pub async fn create_sync_batch(&self, user_id: i64, limit: i64) -> Result<Option<SyncBatch>, AppError> {
        let operations = self.get_pending_operations(limit).await?;
        
        if operations.is_empty() {
            return Ok(None);
        }
        
        let clock = self.vector_clock.lock().await;
        
        let batch = SyncBatch::new(
            &self.device_id,
            user_id,
            operations,
            clock.clone(),
        );
        
        Ok(Some(batch))
    }
    
    // Apply operations from a received sync batch
    pub async fn apply_sync_batch(&self, batch: SyncBatch) -> Result<(), AppError> {
        let mut clock = self.vector_clock.lock().await;
        
        // Merge vector clocks
        for (device, remote_clock) in batch.vector_clock {
            let local_clock = clock.entry(device).or_insert(0);
            *local_clock = std::cmp::max(*local_clock, remote_clock);
        }
        
        // Process operations with conflict resolution
        for remote_op in batch.operations {
            // Find any conflicting operations
            let conflicts = self.find_conflicts(&remote_op).await?;
            
            if conflicts.is_empty() {
                // No conflicts, just store the operation
                self.store_operation(&remote_op).await?;
            } else {
                // Resolve conflicts
                self.resolve_conflicts(remote_op, conflicts).await?;
            }
        }
        
        Ok(())
    }
    
    // Find operations that might conflict with a given operation
    async fn find_conflicts(&self, operation: &SyncOperation) -> Result<Vec<SyncOperation>, AppError> {
        // This is a simplified conflict detection
        // In a real implementation, this would be more sophisticated
        
        match &operation.entity_id {
            Some(entity_id) => {
                // Find operations for the same entity
                let rows = sqlx::query!(
                    r#"
                    SELECT id, device_id, user_id, operation_type, entity_type, entity_id, 
                           payload, timestamp, vector_clock, synced, synced_at
                    FROM sync_operations
                    WHERE entity_type = ? AND entity_id = ?
                    "#,
                    operation.entity_type,
                    entity_id
                )
                .fetch_all(&self.db)
                .await?;
                
                let mut conflicts = Vec::new();
                
                for row in rows {
                    // Parse the row into a SyncOperation
                    // (Similar code as in get_pending_operations)
                    // Add to conflicts list if it conflicts with the operation
                    // This is just an outline - you'd need to implement the actual parsing
                    
                    let op_type = match row.operation_type {
                        0 => OperationType::Create,
                        1 => OperationType::Update,
                        2 => OperationType::Delete,
                        3 => OperationType::Reference,
                        _ => continue, // Skip unknown operation types
                    };
                    
                    let vector_clock: HashMap<String, i64> = serde_json::from_str(&row.vector_clock)
                        .map_err(|e| AppError::SyncError(format!("Failed to deserialize vector clock: {}", e)))?;
                    
                    let payload: serde_json::Value = serde_json::from_str(&row.payload)
                        .map_err(|e| AppError::SyncError(format!("Failed to deserialize payload: {}", e)))?;
                    
                    let local_op = SyncOperation {
                        id: row.id,
                        device_id: row.device_id,
                        user_id: row.user_id,
                        operation_type: op_type,
                        entity_type: row.entity_type,
                        entity_id: row.entity_id,
                        payload,
                        timestamp: row.timestamp,
                        vector_clock,
                        synced: row.synced != 0,
                        synced_at: row.synced_at,
                    };
                    
                    // Check if there's a conflict
                    if ConflictResolver::detect_conflict(&local_op, operation).is_some() {
                        conflicts.push(local_op);
                    }
                }
                
                Ok(conflicts)
            },
            None => Ok(Vec::new()),
        }
    }
    
    // Resolve conflicts between operations
    async fn resolve_conflicts(&self, remote_op: SyncOperation, conflicts: Vec<SyncOperation>) -> Result<(), AppError> {
        for local_op in conflicts {
            match ConflictResolver::resolve_conflict(&local_op, &remote_op) {
                ConflictResolution::KeepFirst => {
                    // Keep the local operation, discard remote
                    // Nothing to do, as local operation is already stored
                },
                ConflictResolution::KeepSecond => {
                    // Replace local with remote
                    self.delete_operation(&local_op.id).await?;
                    self.store_operation(&remote_op).await?;
                },
                ConflictResolution::Merge => {
                    // Merge operations
                    let merged_op = ConflictResolver::merge_updates(&local_op, &remote_op);
                    self.delete_operation(&local_op.id).await?;
                    self.store_operation(&merged_op).await?;
                },
                ConflictResolution::KeepBoth => {
                    // Store both operations
                    self.store_operation(&remote_op).await?;
                },
            }
        }
        
        Ok(())
    }
    
    // Delete an operation by ID
    async fn delete_operation(&self, operation_id: &str) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM sync_operations
            WHERE id = ?
            "#,
            operation_id
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
}