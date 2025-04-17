use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use log::{debug, info, warn};

use crate::core::errors::AppError;
use super::operations::{SyncOperation, SyncBatch, OperationType};
use super::conflicts::{ConflictResolver, ConflictResolution};
use super::version_vector::VersionVector;

pub struct SyncEngine {
    db: Pool<Sqlite>,
    device_id: String,
    vector_clock: Arc<Mutex<VersionVector>>,
    conflict_resolver: Arc<ConflictResolver>,
    // Configuration for large systems
    max_batch_size: usize,
    prune_threshold: i64,
    compression_enabled: bool,
}

impl SyncEngine {
    pub fn new(db: Pool<Sqlite>) -> Self {
        // Generate a unique device ID or load from storage
        let device_id = Uuid::new_v4().to_string();

        Self {
            db,
            device_id,
            vector_clock: Arc::new(Mutex::new(VersionVector::new())),
            conflict_resolver: Arc::new(ConflictResolver::new()),
            max_batch_size: 1000,
            prune_threshold: 10,
            compression_enabled: true,
        }
    }

    /// Create a new SyncEngine with custom configuration
    pub fn with_config(
        db: Pool<Sqlite>,
        device_id: Option<String>,
        max_batch_size: usize,
        prune_threshold: i64,
        compression_enabled: bool,
        conflict_cache_size: usize,
    ) -> Self {
        let device_id = device_id.unwrap_or_else(|| Uuid::new_v4().to_string());

        Self {
            db,
            device_id,
            vector_clock: Arc::new(Mutex::new(VersionVector::new())),
            conflict_resolver: Arc::new(ConflictResolver::with_config(
                conflict_cache_size,
                max_batch_size / 10, // Use smaller batches for conflict resolution
            )),
            max_batch_size,
            prune_threshold,
            compression_enabled,
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
                let counters = clock.to_hashmap();
                let mut new_counters = counters.clone();
                new_counters.insert(device_id, timestamp);
                *clock = VersionVector::from_hashmap(new_counters);
            }
        }

        // Ensure current device is in vector clock
        let device_counter = clock.get(&self.device_id);
        if device_counter == 0 {
            let counters = clock.to_hashmap();
            let mut new_counters = counters.clone();
            new_counters.insert(self.device_id.clone(), 0);
            *clock = VersionVector::from_hashmap(new_counters);
        }

        info!("Initialized vector clock: {:?}", clock);
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
        clock.increment(&self.device_id);
        debug!("Incremented vector clock for device {}: {:?}", self.device_id, clock);

        // Create the operation
        let operation = SyncOperation::new(
            &self.device_id,
            user_id,
            operation_type,
            entity_type,
            entity_id,
            payload,
            clock.to_hashmap(),
        );

        // Store in database
        self.store_operation(&operation).await?;

        info!("Queued operation: {} ({})", operation.id, operation.operation_type);
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
            clock.to_hashmap(),
        );

        info!("Created sync batch with {} operations", batch.operations.len());
        Ok(Some(batch))
    }

    // Apply operations from a received sync batch
    pub async fn apply_sync_batch(&self, batch: SyncBatch) -> Result<(), AppError> {
        let mut clock = self.vector_clock.lock().await;

        // Merge vector clocks using VersionVector
        let remote_vv = VersionVector::from_hashmap(batch.vector_clock);
        clock.merge(&remote_vv);

        // Prune inactive entries if enabled
        if self.compression_enabled {
            let pruned = clock.prune_inactive_entries(self.prune_threshold);
            if pruned > 0 {
                debug!("Pruned {} inactive entries from vector clock", pruned);
            }
        }

        info!("Merged vector clock with remote: {:?}", clock);

        // For large batches, use optimized batch processing
        if batch.operations.len() > self.max_batch_size {
            return self.apply_large_sync_batch(batch).await;
        }

        // Process operations with conflict resolution
        for remote_op in batch.operations {
            // Find any conflicting operations
            let conflicts = self.find_conflicts(&remote_op).await?;

            if conflicts.is_empty() {
                // No conflicts, just store the operation
                debug!("No conflicts found for operation {}", remote_op.id);
                self.store_operation(&remote_op).await?;
            } else {
                // Resolve conflicts
                info!("Found {} conflicts for operation {}", conflicts.len(), remote_op.id);
                self.resolve_conflicts(remote_op, conflicts).await?;
            }
        }

        Ok(())
    }

    // Apply a large sync batch using optimized batch processing
    async fn apply_large_sync_batch(&self, batch: SyncBatch) -> Result<(), AppError> {
        info!("Processing large sync batch with {} operations", batch.operations.len());

        // Split the batch into smaller chunks
        for chunk in batch.operations.chunks(self.max_batch_size) {
            // First, find all local operations that might conflict with this chunk
            let mut potential_conflicts = Vec::new();

            // Get unique entity types and IDs in this chunk
            let mut entity_types = HashSet::new();
            let mut entity_ids = HashSet::new();

            for op in chunk {
                entity_types.insert(op.entity_type.clone());
                if let Some(id) = &op.entity_id {
                    entity_ids.insert(id.clone());
                }
            }

            // Query for potential conflicts
            for entity_type in &entity_types {
                let mut query = "SELECT * FROM sync_operations WHERE entity_type = ?".to_string();
                let mut params = vec![entity_type.clone()];

                if !entity_ids.is_empty() {
                    query.push_str(" AND entity_id IN (");
                    for (i, _) in entity_ids.iter().enumerate() {
                        if i > 0 {
                            query.push_str(", ?");
                        } else {
                            query.push_str("?");
                        }
                    }
                    query.push_str(")");

                    for id in &entity_ids {
                        params.push(id.clone());
                    }
                }

                // Execute the query
                let rows = sqlx::query(&query)
                    .bind_all(params)
                    .fetch_all(&self.db)
                    .await?;

                // Convert rows to operations
                for row in rows {
                    let op = self.row_to_operation(row)?;
                    potential_conflicts.push(op);
                }
            }

            // Combine remote operations and potential conflicts
            let mut all_operations = Vec::new();
            all_operations.extend(chunk.to_vec());
            all_operations.extend(potential_conflicts);

            // Use batch conflict resolution
            let resolved_ops = self.conflict_resolver.resolve_conflicts_batch(&all_operations);

            // Store the resolved operations
            for op in resolved_ops {
                // Only store operations that aren't already in the database
                let existing = sqlx::query!("SELECT id FROM sync_operations WHERE id = ?", op.id)
                    .fetch_optional(&self.db)
                    .await?;

                if existing.is_none() {
                    self.store_operation(&op).await?;
                }
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

    // Helper method to convert a database row to a SyncOperation
    fn row_to_operation(&self, row: sqlx::sqlite::SqliteRow) -> Result<SyncOperation, AppError> {
        let id: String = row.try_get("id")?;
        let device_id: String = row.try_get("device_id")?;
        let user_id: i64 = row.try_get("user_id")?;
        let operation_type_val: i64 = row.try_get("operation_type")?;
        let entity_type: String = row.try_get("entity_type")?;
        let entity_id: Option<String> = row.try_get("entity_id")?;
        let payload_str: String = row.try_get("payload")?;
        let timestamp: i64 = row.try_get("timestamp")?;
        let vector_clock_str: String = row.try_get("vector_clock")?;
        let synced: i64 = row.try_get("synced")?;
        let synced_at: Option<i64> = row.try_get("synced_at")?;

        let op_type = match operation_type_val {
            0 => OperationType::Create,
            1 => OperationType::Update,
            2 => OperationType::Delete,
            3 => OperationType::Reference,
            _ => return Err(AppError::SyncError(format!("Unknown operation type: {}", operation_type_val))),
        };

        let vector_clock: HashMap<String, i64> = serde_json::from_str(&vector_clock_str)
            .map_err(|e| AppError::SyncError(format!("Failed to deserialize vector clock: {}", e)))?;

        let payload: serde_json::Value = serde_json::from_str(&payload_str)
            .map_err(|e| AppError::SyncError(format!("Failed to deserialize payload: {}", e)))?;

        Ok(SyncOperation {
            id,
            device_id,
            user_id,
            operation_type: op_type,
            entity_type,
            entity_id,
            payload,
            timestamp,
            vector_clock,
            synced: synced != 0,
            synced_at,
        })
    }
}