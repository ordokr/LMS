use crate::sync::version_vector::{VersionVector, CausalRelation};
use crate::services::sync::transaction_handler::{SyncTransactionHandler, SyncEvent, TransactionStatus};
use crate::db::redb_transaction::{RedbTransactionManager, TransactionOptions};
use crate::error::Error;
use chrono::Utc;
use log::{info, warn, error};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Entity type for sync operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    Course,
    Module,
    Topic,
    Post,
    Assignment,
    User,
}

/// Represents the sync status of an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    PendingSync,
    Synced,
    Conflict,
    Error,
}

/// Integration with version vector for robust sync operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionVectorSync {
    /// Entity ID
    pub entity_id: String,
    /// Entity type
    pub entity_type: EntityType,
    /// Canvas version vector
    pub canvas_vector: VersionVector,
    /// Discourse version vector
    pub discourse_vector: VersionVector,
    /// Local version vector
    pub local_vector: VersionVector,
    /// Last sync time
    pub last_sync: chrono::DateTime<Utc>,
    /// Sync status
    pub status: SyncStatus,
}

/// Manages sync transactions with version vector support
pub struct VersionVectorSyncManager {
    /// Database connection pool
    db_pool: Pool<Sqlite>,
    /// Transaction handler for sync operations
    transaction_handler: Arc<Mutex<SyncTransactionHandler>>,
    /// Redb transaction manager for robust transactional support
    redb_manager: Option<Arc<RedbTransactionManager>>,
    /// Cache of version vectors for entities
    version_vectors: Arc<Mutex<HashMap<String, VersionVectorSync>>>,
    /// Device ID for this instance
    device_id: String,
}

impl VersionVectorSyncManager {
    /// Create a new version vector sync manager
    pub fn new(db_pool: Pool<Sqlite>) -> Self {
        let transaction_handler = Arc::new(Mutex::new(SyncTransactionHandler::new(
            SyncEvent {
                transaction_id: None,
                entity_type: "system".to_string(),
                entity_id: "init".to_string(),
                operation: "init".to_string(),
                source_system: "local".to_string(),
                target_system: "none".to_string(),
                timestamp: Utc::now(),
                data: serde_json::json!({}),
            },
            db_pool.clone(),
        )));
        
        Self {
            db_pool,
            transaction_handler,
            redb_manager: None,
            version_vectors: Arc::new(Mutex::new(HashMap::new())),
            device_id: format!("device_{}", Uuid::new_v4().as_simple()),
        }
    }
    
    /// Create a new version vector sync manager with Redb support
    pub fn with_redb(db_pool: Pool<Sqlite>, redb_manager: Arc<RedbTransactionManager>) -> Self {
        let transaction_handler = Arc::new(Mutex::new(SyncTransactionHandler::new(
            SyncEvent {
                transaction_id: None,
                entity_type: "system".to_string(),
                entity_id: "init".to_string(),
                operation: "init".to_string(),
                source_system: "local".to_string(),
                target_system: "none".to_string(),
                timestamp: Utc::now(),
                data: serde_json::json!({}),
            },
            db_pool.clone(),
        )));
        
        Self {
            db_pool,
            transaction_handler,
            redb_manager: Some(redb_manager),
            version_vectors: Arc::new(Mutex::new(HashMap::new())),
            device_id: format!("device_{}", Uuid::new_v4().as_simple()),
        }
    }
    
    /// Set device ID
    pub fn set_device_id(&mut self, device_id: String) {
        self.device_id = device_id;
    }
    
    /// Get device ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }
    
    /// Execute a sync operation using a transaction
    pub async fn execute_sync_operation<F, T>(&self, 
        entity_type: EntityType,
        entity_id: &str,
        operation: &str,
        source_system: &str,
        target_system: &str,
        f: F
    ) -> Result<T, Error>
    where
        F: FnOnce(&mut SyncTransactionHandler) -> Result<T, Error> + Send + 'static,
        T: Send + 'static,
    {
        // Create a new transaction
        let event = SyncEvent {
            transaction_id: None,
            entity_type: format!("{:?}", entity_type),
            entity_id: entity_id.to_string(),
            operation: operation.to_string(),
            source_system: source_system.to_string(),
            target_system: target_system.to_string(),
            timestamp: Utc::now(),
            data: serde_json::json!({}),
        };
        
        // Initialize transaction handler
        let mut tx_handler = SyncTransactionHandler::new(event, self.db_pool.clone());
        
        // Begin the transaction 
        tx_handler.begin().await?;
        
        // Execute the operation
        let result = match f(&mut tx_handler).await {
            Ok(result) => {
                // Commit the transaction
                tx_handler.commit().await?;
                Ok(result)
            },
            Err(e) => {
                // Rollback the transaction
                tx_handler.rollback(&e.to_string()).await?;
                Err(e)
            }
        };
        
        // Update version vector
        self.update_version_vector(&entity_type, entity_id, source_system).await?;
        
        result
    }
    
    /// Update the version vector for an entity after a sync operation
    async fn update_version_vector(
        &self, 
        entity_type: &EntityType, 
        entity_id: &str,
        system: &str
    ) -> Result<(), Error> {
        let key = format!("{}:{}", entity_type.to_string(), entity_id);
        let mut vectors = self.version_vectors.lock().unwrap();
        
        let vector_sync = vectors.entry(key.clone()).or_insert_with(|| {
            VersionVectorSync {
                entity_id: entity_id.to_string(),
                entity_type: entity_type.clone(),
                canvas_vector: VersionVector::new(),
                discourse_vector: VersionVector::new(),
                local_vector: VersionVector::new(),
                last_sync: Utc::now(),
                status: SyncStatus::PendingSync,
            }
        });
        
        // Increment the appropriate version vector
        match system {
            "canvas" => {
                vector_sync.canvas_vector.increment(&self.device_id);
            },
            "discourse" => {
                vector_sync.discourse_vector.increment(&self.device_id);
            },
            "local" => {
                vector_sync.local_vector.increment(&self.device_id);
            },
            _ => {}
        }
        
        // Update the sync status based on causal relations
        vector_sync.status = self.determine_sync_status(
            &vector_sync.canvas_vector,
            &vector_sync.discourse_vector,
            &vector_sync.local_vector
        );
        
        // Update last sync time
        vector_sync.last_sync = Utc::now();
        
        // Persist to database
        self.save_version_vector(vector_sync).await?;
        
        Ok(())
    }
    
    /// Determine sync status based on causal relations between version vectors
    fn determine_sync_status(
        &self,
        canvas_vector: &VersionVector,
        discourse_vector: &VersionVector,
        local_vector: &VersionVector
    ) -> SyncStatus {
        // Check relations between Canvas and Discourse
        let canvas_discourse_relation = canvas_vector.causal_relation(discourse_vector);
        
        match canvas_discourse_relation {
            CausalRelation::Concurrent => {
                // Concurrent modifications - conflict
                SyncStatus::Conflict
            },
            _ => {
                // Check relations with local vector
                let canvas_local_relation = canvas_vector.causal_relation(local_vector);
                let discourse_local_relation = discourse_vector.causal_relation(local_vector);
                
                if canvas_local_relation == CausalRelation::Identical && 
                   discourse_local_relation == CausalRelation::Identical {
                    // All vectors are identical - fully synced
                    SyncStatus::Synced
                } else if canvas_local_relation == CausalRelation::Concurrent || 
                          discourse_local_relation == CausalRelation::Concurrent {
                    // Concurrent modifications with local - conflict
                    SyncStatus::Conflict
                } else {
                    // Pending sync
                    SyncStatus::PendingSync
                }
            }
        }
    }
    
    /// Save version vector to database
    async fn save_version_vector(&self, vector_sync: &VersionVectorSync) -> Result<(), Error> {
        // Serialize version vectors
        let canvas_vector_bytes = vector_sync.canvas_vector.to_bytes()?;
        let discourse_vector_bytes = vector_sync.discourse_vector.to_bytes()?;
        let local_vector_bytes = vector_sync.local_vector.to_bytes()?;
        
        // Encode to base64 for storage
        let canvas_vector_base64 = base64::encode(&canvas_vector_bytes);
        let discourse_vector_base64 = base64::encode(&discourse_vector_bytes);
        let local_vector_base64 = base64::encode(&local_vector_bytes);
        
        // Insert or update in database
        sqlx::query!(
            r#"
            INSERT INTO entity_version_vectors (
                entity_type, entity_id, canvas_vector, discourse_vector, local_vector, 
                last_sync, status
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (entity_type, entity_id) DO UPDATE SET
                canvas_vector = excluded.canvas_vector,
                discourse_vector = excluded.discourse_vector,
                local_vector = excluded.local_vector,
                last_sync = excluded.last_sync,
                status = excluded.status
            "#,
            vector_sync.entity_type.to_string(),
            vector_sync.entity_id,
            canvas_vector_base64,
            discourse_vector_base64,
            local_vector_base64,
            vector_sync.last_sync,
            vector_sync.status.to_string()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Load version vector from database
    pub async fn load_version_vector(
        &self,
        entity_type: &EntityType,
        entity_id: &str
    ) -> Result<Option<VersionVectorSync>, Error> {
        let result = sqlx::query!(
            r#"
            SELECT * FROM entity_version_vectors
            WHERE entity_type = ? AND entity_id = ?
            "#,
            entity_type.to_string(),
            entity_id
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(record) = result {
            // Decode from base64
            let canvas_vector_bytes = base64::decode(&record.canvas_vector)?;
            let discourse_vector_bytes = base64::decode(&record.discourse_vector)?;
            let local_vector_bytes = base64::decode(&record.local_vector)?;
            
            // Deserialize version vectors
            let canvas_vector = VersionVector::from_bytes(&canvas_vector_bytes)?;
            let discourse_vector = VersionVector::from_bytes(&discourse_vector_bytes)?;
            let local_vector = VersionVector::from_bytes(&local_vector_bytes)?;
            
            // Parse status
            let status = match record.status.as_str() {
                "PendingSync" => SyncStatus::PendingSync,
                "Synced" => SyncStatus::Synced,
                "Conflict" => SyncStatus::Conflict,
                "Error" => SyncStatus::Error,
                _ => SyncStatus::PendingSync,
            };
            
            let vector_sync = VersionVectorSync {
                entity_id: record.entity_id,
                entity_type: entity_type.clone(),
                canvas_vector,
                discourse_vector,
                local_vector,
                last_sync: record.last_sync.parse().unwrap_or_else(|_| Utc::now()),
                status,
            };
            
            // Cache it
            let mut vectors = self.version_vectors.lock().unwrap();
            let key = format!("{}:{}", entity_type.to_string(), entity_id);
            vectors.insert(key, vector_sync.clone());
            
            Ok(Some(vector_sync))
        } else {
            Ok(None)
        }
    }
    
    /// Check if an entity needs synchronization
    pub async fn needs_sync(
        &self,
        entity_type: &EntityType,
        entity_id: &str
    ) -> Result<bool, Error> {
        let key = format!("{}:{}", entity_type.to_string(), entity_id);
        
        // Check cache first
        {
            let vectors = self.version_vectors.lock().unwrap();
            if let Some(vector_sync) = vectors.get(&key) {
                return Ok(vector_sync.status == SyncStatus::PendingSync);
            }
        }
        
        // Load from database
        let vector_sync = self.load_version_vector(entity_type, entity_id).await?;
        
        if let Some(vector_sync) = vector_sync {
            Ok(vector_sync.status == SyncStatus::PendingSync)
        } else {
            // If no vector sync exists, assume it needs sync
            Ok(true)
        }
    }
    
    /// Detect conflicts for an entity
    pub async fn detect_conflicts(
        &self,
        entity_type: &EntityType,
        entity_id: &str,
        canvas_data: Option<serde_json::Value>,
        discourse_data: Option<serde_json::Value>
    ) -> Result<bool, Error> {
        let vector_sync = match self.load_version_vector(entity_type, entity_id).await? {
            Some(vs) => vs,
            None => {
                // Create new version vectors
                let mut vs = VersionVectorSync {
                    entity_id: entity_id.to_string(),
                    entity_type: entity_type.clone(),
                    canvas_vector: VersionVector::new(),
                    discourse_vector: VersionVector::new(),
                    local_vector: VersionVector::new(),
                    last_sync: Utc::now(),
                    status: SyncStatus::PendingSync,
                };
                
                // If we have data, increment the corresponding vectors
                if canvas_data.is_some() {
                    vs.canvas_vector.increment(&self.device_id);
                }
                
                if discourse_data.is_some() {
                    vs.discourse_vector.increment(&self.device_id);
                }
                
                self.save_version_vector(&vs).await?;
                vs
            }
        };
        
        // Check for concurrent modifications
        let relation = vector_sync.canvas_vector.causal_relation(&vector_sync.discourse_vector);
        let has_conflict = relation == CausalRelation::Concurrent;
        
        if has_conflict {
            // Update status to conflict
            let mut updated_vs = vector_sync.clone();
            updated_vs.status = SyncStatus::Conflict;
            self.save_version_vector(&updated_vs).await?;
        }
        
        Ok(has_conflict)
    }
    
    /// Resolve conflicts for an entity
    pub async fn resolve_conflicts(
        &self,
        entity_type: &EntityType,
        entity_id: &str,
        resolution_strategy: &str
    ) -> Result<(), Error> {
        let mut vector_sync = match self.load_version_vector(entity_type, entity_id).await? {
            Some(vs) => vs,
            None => {
                return Err(Error::NotFound(format!(
                    "No version vector found for entity {:?}:{}", entity_type, entity_id
                )));
            }
        };
        
        if vector_sync.status != SyncStatus::Conflict {
            return Err(Error::InvalidState(format!(
                "Entity {:?}:{} is not in conflict state", entity_type, entity_id
            )));
        }
        
        // Apply resolution strategy
        match resolution_strategy {
            "prefer_canvas" => {
                // Make discourse vector match canvas vector
                vector_sync.discourse_vector = vector_sync.canvas_vector.clone();
                // Update local vector to match
                vector_sync.local_vector = vector_sync.canvas_vector.clone();
            },
            "prefer_discourse" => {
                // Make canvas vector match discourse vector
                vector_sync.canvas_vector = vector_sync.discourse_vector.clone();
                // Update local vector to match
                vector_sync.local_vector = vector_sync.discourse_vector.clone();
            },
            "merge" => {
                // Merge both vectors
                let merged = vector_sync.canvas_vector.merged_with(&vector_sync.discourse_vector);
                vector_sync.canvas_vector = merged.clone();
                vector_sync.discourse_vector = merged.clone();
                vector_sync.local_vector = merged;
            },
            _ => {
                return Err(Error::InvalidArgument(format!(
                    "Invalid conflict resolution strategy: {}", resolution_strategy
                )));
            }
        }
        
        // Update status to synced
        vector_sync.status = SyncStatus::Synced;
        vector_sync.last_sync = Utc::now();
        
        // Save updated version vector
        self.save_version_vector(&vector_sync).await?;
        
        Ok(())
    }
    
    /// Create database tables for version vectors
    pub async fn create_tables(&self) -> Result<(), Error> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS entity_version_vectors (
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                canvas_vector TEXT NOT NULL,
                discourse_vector TEXT NOT NULL,
                local_vector TEXT NOT NULL,
                last_sync TEXT NOT NULL,
                status TEXT NOT NULL,
                PRIMARY KEY (entity_type, entity_id)
            )
            "#
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    
    async fn setup_test_db() -> Pool<Sqlite> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to connect to in-memory SQLite");
            
        // Create test tables
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS entity_version_vectors (
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                canvas_vector TEXT NOT NULL,
                discourse_vector TEXT NOT NULL,
                local_vector TEXT NOT NULL,
                last_sync TEXT NOT NULL,
                status TEXT NOT NULL,
                PRIMARY KEY (entity_type, entity_id)
            )
            "#
        )
        .execute(&pool)
        .await
        .expect("Failed to create test tables");
        
        pool
    }
    
    #[tokio::test]
    async fn test_version_vector_sync_manager() {
        let pool = setup_test_db().await;
        let manager = VersionVectorSyncManager::new(pool);
        
        // Test updating version vectors
        let entity_type = EntityType::Topic;
        let entity_id = "test-topic-1";
        
        // Update Canvas vector
        manager.update_version_vector(&entity_type, entity_id, "canvas").await.unwrap();
        
        // Load version vector
        let vector_sync = manager.load_version_vector(&entity_type, entity_id).await.unwrap();
        assert!(vector_sync.is_some());
        
        let vector_sync = vector_sync.unwrap();
        assert_eq!(vector_sync.entity_id, entity_id);
        assert_eq!(vector_sync.status, SyncStatus::PendingSync);
        assert_eq!(vector_sync.canvas_vector.size(), 1);
        assert_eq!(vector_sync.discourse_vector.size(), 0);
        
        // Update Discourse vector
        manager.update_version_vector(&entity_type, entity_id, "discourse").await.unwrap();
        
        // Load updated version vector
        let vector_sync = manager.load_version_vector(&entity_type, entity_id).await.unwrap().unwrap();
        assert_eq!(vector_sync.canvas_vector.size(), 1);
        assert_eq!(vector_sync.discourse_vector.size(), 1);
        
        // Check if needs sync
        let needs_sync = manager.needs_sync(&entity_type, entity_id).await.unwrap();
        assert!(needs_sync);
        
        // Test conflict detection
        let has_conflict = manager.detect_conflicts(
            &entity_type, 
            entity_id,
            Some(serde_json::json!({"title": "Canvas Title"})),
            Some(serde_json::json!({"title": "Discourse Title"}))
        ).await.unwrap();
        
        assert!(has_conflict);
        
        // Test conflict resolution
        manager.resolve_conflicts(&entity_type, entity_id, "merge").await.unwrap();
        
        // Check if still needs sync
        let needs_sync = manager.needs_sync(&entity_type, entity_id).await.unwrap();
        assert!(!needs_sync);
        
        // Load final version vector
        let vector_sync = manager.load_version_vector(&entity_type, entity_id).await.unwrap().unwrap();
        assert_eq!(vector_sync.status, SyncStatus::Synced);
    }
}