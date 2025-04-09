use crate::blockchain::error::BlockchainError;
use crate::blockchain::metrics::{BlockTimer, Operation, PerformanceMetrics};
use crate::blockchain::storage::BlockchainStorage;
use automerge::{Automerge, ObjType, ObjId, transaction::Transactable};
use blake3::Hasher;
use chrono::Utc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{info, warn, error};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Unique identifier for blockchain entities
pub type EntityId = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockchainEntity {
    pub id: EntityId,
    pub entity_type: String,
    pub data: HashMap<String, String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub version: u64,
}

pub struct HybridChain {
    // CRDT state
    crdt_store: Automerge,
    
    // Storage backend
    storage: BlockchainStorage,
    
    // Connection status
    is_online: Arc<AtomicBool>,
    
    // Performance metrics
    metrics: Arc<PerformanceMetrics>,
    
    // Last block timestamp
    last_block_time: i64,
}

impl HybridChain {
    pub async fn new() -> Result<Self, BlockchainError> {
        let storage = BlockchainStorage::open("./blockchain.db").await?;
        let is_online = Arc::new(AtomicBool::new(false));
        let metrics = Arc::new(PerformanceMetrics::new());
        
        // Initialize CRDT store
        let mut crdt_store = Automerge::new();
        
        // Initialize root objects in CRDT
        let tx = crdt_store.transaction();
        tx.put_object(&ObjId::Root, "entities", ObjType::Map)?;
        tx.put_object(&ObjId::Root, "blocks", ObjType::Map)?;
        tx.commit();
        
        let last_block_time = storage.get_last_block_time().await?;
        
        let chain = Self {
            crdt_store,
            storage,
            is_online,
            metrics,
            last_block_time,
        };
        
        // Restore state from storage
        chain.restore_state().await?;
        
        Ok(chain)
    }
    
    // Restore chain state from persistent storage
    async fn restore_state(&self) -> Result<(), BlockchainError> {
        let timer = BlockTimer::new(Operation::BlockchainRead, Arc::clone(&self.metrics));
        
        // Load entities from storage
        let entities = self.storage.get_all_entities().await?;
        
        // Apply to CRDT store
        let tx = self.crdt_store.transaction();
        
        for entity in entities {
            self.update_crdt_entity(&tx, &entity)?;
        }
        
        tx.commit();
        
        info!(event = "chain_state_restored", entity_count = entities.len());
        
        timer.finish();
        Ok(())
    }
    
    // Helper to update an entity in CRDT store
    fn update_crdt_entity(&self, tx: &automerge::Transaction, entity: &BlockchainEntity) -> Result<(), BlockchainError> {
        let entities_id = self.crdt_store.get_object_id(&ObjId::Root, "entities")
            .map_err(|e| BlockchainError::Storage(format!("Failed to get entities object: {}", e)))?;
        
        // Create or update entity in CRDT
        if !tx.has_object(&entities_id, &entity.id) {
            tx.put_object(&entities_id, &entity.id, ObjType::Map)
                .map_err(|e| BlockchainError::Storage(format!("Failed to create entity object: {}", e)))?;
        }
        
        let entity_id = tx.get_object_id(&entities_id, &entity.id)
            .map_err(|e| BlockchainError::Storage(format!("Failed to get entity ID: {}", e)))?;
        
        // Set fields
        tx.put(&entity_id, "id", entity.id.clone())
            .map_err(|e| BlockchainError::Storage(format!("Failed to set entity ID: {}", e)))?;
        
        tx.put(&entity_id, "entity_type", entity.entity_type.clone())
            .map_err(|e| BlockchainError::Storage(format!("Failed to set entity type: {}", e)))?;
        
        tx.put(&entity_id, "created_at", entity.created_at.to_string())
            .map_err(|e| BlockchainError::Storage(format!("Failed to set entity created_at: {}", e)))?;
        
        tx.put(&entity_id, "updated_at", entity.updated_at.to_string())
            .map_err(|e| BlockchainError::Storage(format!("Failed to set entity updated_at: {}", e)))?;
        
        tx.put(&entity_id, "version", entity.version.to_string())
            .map_err(|e| BlockchainError::Storage(format!("Failed to set entity version: {}", e)))?;
        
        // Create or get data map
        if !tx.has_object(&entity_id, "data") {
            tx.put_object(&entity_id, "data", ObjType::Map)
                .map_err(|e| BlockchainError::Storage(format!("Failed to create data object: {}", e)))?;
        }
        
        let data_id = tx.get_object_id(&entity_id, "data")
            .map_err(|e| BlockchainError::Storage(format!("Failed to get data ID: {}", e)))?;
        
        // Set data fields
        for (key, value) in &entity.data {
            tx.put(&data_id, key, value.clone())
                .map_err(|e| BlockchainError::Storage(format!("Failed to set data field {}: {}", key, e)))?;
        }
        
        Ok(())
    }
    
    // Create a new entity
    pub async fn create_entity(&mut self, entity_type: &str, data: HashMap<String, String>) -> Result<EntityId, BlockchainError> {
        let timer = BlockTimer::new(Operation::BlockchainWrite, Arc::clone(&self.metrics));
        
        let entity_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();
        
        let entity = BlockchainEntity {
            id: entity_id.clone(),
            entity_type: entity_type.to_string(),
            data,
            created_at: now,
            updated_at: now,
            version: 1,
        };
        
        // Update CRDT
        let tx = self.crdt_store.transaction();
        self.update_crdt_entity(&tx, &entity)?;
        tx.commit();
        
        // Store in database
        self.storage.save_entity(&entity).await?;
        
        // Create a block for this entity creation if enough time has passed
        if now - self.last_block_time >= 60 { // Create block at most once per minute
            self.create_block().await?;
        }
        
        timer.finish();
        info!(event = "entity_created", entity_id = entity_id, entity_type = entity_type);
        
        Ok(entity_id)
    }
    
    // Update an existing entity
    pub async fn update_entity(&mut self, entity_id: &str, data: HashMap<String, String>) -> Result<(), BlockchainError> {
        let timer = BlockTimer::new(Operation::BlockchainWrite, Arc::clone(&self.metrics));
        
        // Get existing entity
        let mut entity = self.storage.get_entity(entity_id).await?;
        
        // Update entity
        entity.data = data;
        entity.updated_at = Utc::now().timestamp();
        entity.version += 1;
        
        // Update CRDT
        let tx = self.crdt_store.transaction();
        self.update_crdt_entity(&tx, &entity)?;
        tx.commit();
        
        // Store in database
        self.storage.save_entity(&entity).await?;
        
        timer.finish();
        info!(event = "entity_updated", entity_id = entity_id, entity_type = entity.entity_type);
        
        Ok(())
    }
    
    // Get an entity
    pub async fn get_entity(&self, entity_id: &str) -> Result<BlockchainEntity, BlockchainError> {
        let timer = BlockTimer::new(Operation::BlockchainRead, Arc::clone(&self.metrics));
        
        let entity = self.storage.get_entity(entity_id).await?;
        
        timer.finish();
        Ok(entity)
    }
    
    // Create a block with current state
    pub async fn create_block(&mut self) -> Result<i64, BlockchainError> {
        let timer = BlockTimer::new(Operation::BlockCreation, Arc::clone(&self.metrics));
        
        // Generate state hash
        let mut buffer = Vec::new();
        self.crdt_store.save(&mut buffer)
            .map_err(|e| BlockchainError::Storage(format!("Failed to serialize CRDT state: {}", e)))?;
        
        let state_hash = blake3::hash(&buffer);
        let timestamp = Utc::now().timestamp();
        
        // Get previous block hash
        let prev_hash = self.storage.get_last_block_hash().await?;
        
        // Create block structure
        #[derive(Serialize)]
        struct BlockData {
            timestamp: i64,
            prev_hash: Vec<u8>,
            state_hash: Vec<u8>,
            entity_count: usize,
        }
        
        let entity_count = self.storage.count_entities().await?;
        
        let block_data = BlockData {
            timestamp,
            prev_hash: prev_hash.to_vec(),
            state_hash: state_hash.as_bytes().to_vec(),
            entity_count,
        };
        
        // Store block in database
        let block_id = self.storage.create_block(
            timestamp,
            &prev_hash,
            state_hash.as_bytes(),
            entity_count,
        ).await?;
        
        // Update last block time
        self.last_block_time = timestamp;
        
        timer.finish();
        info!(
            event = "block_created",
            block_id = block_id, 
            timestamp = timestamp,
            entity_count = entity_count
        );
        
        Ok(timestamp)
    }
    
    // Create a block with a provided hash (used for batch anchoring)
    pub async fn create_block_with_hash(&mut self, hash: &[u8]) -> Result<i64, BlockchainError> {
        let timer = BlockTimer::new(Operation::BlockCreation, Arc::clone(&self.metrics));
        
        let timestamp = Utc::now().timestamp();
        
        // Get previous block hash
        let prev_hash = self.storage.get_last_block_hash().await?;
        
        // Create block in storage
        let entity_count = self.storage.count_entities().await?;
        
        let block_id = self.storage.create_block(
            timestamp,
            &prev_hash,
            hash,
            entity_count,
        ).await?;
        
        // Update last block time
        self.last_block_time = timestamp;
        
        timer.finish();
        info!(
            event = "block_created_with_hash",
            block_id = block_id, 
            timestamp = timestamp,
            entity_count = entity_count
        );
        
        Ok(timestamp)
    }
    
    // Verify that an entity exists in the blockchain
    pub async fn verify_entity(&self, entity_id: &str) -> Result<bool, BlockchainError> {
        Ok(self.storage.entity_exists(entity_id).await?)
    }
    
    // Set online/offline status
    pub fn set_online_status(&self, is_online: bool) {
        self.is_online.store(is_online, Ordering::SeqCst);
        info!(event = "connectivity_changed", online = is_online);
    }
    
    // Get current online/offline status
    pub fn is_online(&self) -> bool {
        self.is_online.load(Ordering::SeqCst)
    }
    
    // Get metrics for monitoring
    pub fn metrics(&self) -> Arc<PerformanceMetrics> {
        Arc::clone(&self.metrics)
    }
    
    // Get block count
    pub async fn block_count(&self) -> Result<usize, BlockchainError> {
        Ok(self.storage.count_blocks().await?)
    }
}