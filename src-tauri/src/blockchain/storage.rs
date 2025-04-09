use crate::blockchain::error::BlockchainError;
use crate::blockchain::core::BlockchainEntity;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;
use tracing::{info, warn, error};
use chrono::Utc;

const EMPTY_HASH: [u8; 32] = [0; 32];

pub struct BlockchainStorage {
    pool: SqlitePool,
}

impl BlockchainStorage {
    pub async fn open(path: &str) -> Result<Self, BlockchainError> {
        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| BlockchainError::Storage(format!("Failed to create directory: {}", e)))?;
            }
        }
        
        // Use connection URI format for SQLite
        let uri = format!("sqlite:{}", path);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&uri)
            .await
            .map_err(|e| BlockchainError::Storage(format!("Failed to open database: {}", e)))?;
        
        // Initialize schema
        Self::initialize_schema(&pool).await?;
        
        Ok(Self { pool })
    }
    
    async fn initialize_schema(pool: &SqlitePool) -> Result<(), BlockchainError> {
        // Create blocks table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                prev_hash BLOB NOT NULL,
                state_hash BLOB NOT NULL,
                entity_count INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            )"
        )
        .execute(pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to create blocks table: {}", e)))?;
        
        // Create entities table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS entities (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                data TEXT NOT NULL, -- JSON data
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                version INTEGER NOT NULL
            )"
        )
        .execute(pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to create entities table: {}", e)))?;
        
        // Create pending changes table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS pending_changes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                type TEXT NOT NULL,
                data BLOB NOT NULL,
                priority INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            )"
        )
        .execute(pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to create pending_changes table: {}", e)))?;
        
        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_entities_type ON entities (entity_type)")
            .execute(pool)
            .await
            .map_err(|e| BlockchainError::Storage(format!("Failed to create index: {}", e)))?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_pending_priority ON pending_changes (priority)")
            .execute(pool)
            .await
            .map_err(|e| BlockchainError::Storage(format!("Failed to create index: {}", e)))?;
        
        Ok(())
    }
    
    pub async fn get_last_block_hash(&self) -> Result<[u8; 32], BlockchainError> {
        let result = sqlx::query!(
            "SELECT state_hash FROM blocks ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to get last block hash: {}", e)))?;
        
        if let Some(row) = result {
            let hash_vec = row.state_hash;
            if hash_vec.len() == 32 {
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&hash_vec);
                return Ok(hash);
            }
        }
        
        // If no blocks or invalid hash, return empty hash
        Ok(EMPTY_HASH)
    }
    
    pub async fn get_last_block_time(&self) -> Result<i64, BlockchainError> {
        let result = sqlx::query!(
            "SELECT timestamp FROM blocks ORDER BY id DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to get last block time: {}", e)))?;
        
        if let Some(row) = result {
            return Ok(row.timestamp);
        }
        
        // If no blocks, return 0
        Ok(0)
    }
    
    pub async fn create_block(
        &self,
        timestamp: i64,
        prev_hash: &[u8],
        state_hash: &[u8],
        entity_count: usize,
    ) -> Result<i64, BlockchainError> {
        let result = sqlx::query!(
            "INSERT INTO blocks (timestamp, prev_hash, state_hash, entity_count, created_at) 
             VALUES (?, ?, ?, ?, ?)",
            timestamp,
            prev_hash,
            state_hash,
            entity_count as i64,
            Utc::now().timestamp()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to create block: {}", e)))?;
        
        Ok(result.last_insert_rowid())
    }
    
    pub async fn save_entity(&self, entity: &BlockchainEntity) -> Result<(), BlockchainError> {
        let data_json = serde_json::to_string(&entity.data)
            .map_err(|e| BlockchainError::Serialization(format!("Failed to serialize entity data: {}", e)))?;
        
        sqlx::query!(
            "INSERT OR REPLACE INTO entities (id, entity_type, data, created_at, updated_at, version) 
             VALUES (?, ?, ?, ?, ?, ?)",
            entity.id,
            entity.entity_type,
            data_json,
            entity.created_at,
            entity.updated_at,
            entity.version
        )
        .execute(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to save entity: {}", e)))?;
        
        Ok(())
    }
    
    pub async fn get_entity(&self, entity_id: &str) -> Result<BlockchainEntity, BlockchainError> {
        let row = sqlx::query!(
            "SELECT id, entity_type, data, created_at, updated_at, version 
             FROM entities WHERE id = ?",
            entity_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to get entity: {}", e)))?;
        
        if let Some(row) = row {
            let data: std::collections::HashMap<String, String> = serde_json::from_str(&row.data)
                .map_err(|e| BlockchainError::Serialization(format!("Failed to deserialize entity data: {}", e)))?;
            
            Ok(BlockchainEntity {
                id: row.id,
                entity_type: row.entity_type,
                data,
                created_at: row.created_at,
                updated_at: row.updated_at,
                version: row.version as u64,
            })
        } else {
            Err(BlockchainError::Storage(format!("Entity not found: {}", entity_id)))
        }
    }
    
    pub async fn get_all_entities(&self) -> Result<Vec<BlockchainEntity>, BlockchainError> {
        let rows = sqlx::query!(
            "SELECT id, entity_type, data, created_at, updated_at, version FROM entities"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to get entities: {}", e)))?;
        
        let mut entities = Vec::with_capacity(rows.len());
        
        for row in rows {
            let data: std::collections::HashMap<String, String> = serde_json::from_str(&row.data)
                .map_err(|e| BlockchainError::Serialization(format!("Failed to deserialize entity data: {}", e)))?;
            
            entities.push(BlockchainEntity {
                id: row.id,
                entity_type: row.entity_type,
                data,
                created_at: row.created_at,
                updated_at: row.updated_at,
                version: row.version as u64,
            });
        }
        
        Ok(entities)
    }
    
    pub async fn get_entities_by_type(&self, entity_type: &str) -> Result<Vec<BlockchainEntity>, BlockchainError> {
        let rows = sqlx::query!(
            "SELECT id, entity_type, data, created_at, updated_at, version 
             FROM entities WHERE entity_type = ?",
            entity_type
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to get entities by type: {}", e)))?;
        
        let mut entities = Vec::with_capacity(rows.len());
        
        for row in rows {
            let data: std::collections::HashMap<String, String> = serde_json::from_str(&row.data)
                .map_err(|e| BlockchainError::Serialization(format!("Failed to deserialize entity data: {}", e)))?;
            
            entities.push(BlockchainEntity {
                id: row.id,
                entity_type: row.entity_type,
                data,
                created_at: row.created_at,
                updated_at: row.updated_at,
                version: row.version as u64,
            });
        }
        
        Ok(entities)
    }
    
    pub async fn entity_exists(&self, entity_id: &str) -> Result<bool, BlockchainError> {
        let result = sqlx::query!(
            "SELECT 1 FROM entities WHERE id = ?",
            entity_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to check entity existence: {}", e)))?;
        
        Ok(result.is_some())
    }
    
    pub async fn count_entities(&self) -> Result<usize, BlockchainError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM entities"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to count entities: {}", e)))?;
        
        Ok(result.count as usize)
    }
    
    pub async fn count_blocks(&self) -> Result<usize, BlockchainError> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM blocks"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to count blocks: {}", e)))?;
        
        Ok(result.count as usize)
    }
    
    // Get database connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    // Verify the integrity of the blockchain
    pub async fn verify_blockchain_integrity(&self) -> Result<bool, BlockchainError> {
        let blocks = sqlx::query!(
            "SELECT id, timestamp, prev_hash, state_hash FROM blocks ORDER BY id ASC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to get blocks: {}", e)))?;
        
        if blocks.is_empty() {
            return Ok(true); // Empty blockchain is valid
        }
        
        let mut prev_hash = EMPTY_HASH.to_vec();
        
        for block in blocks {
            // For the first block, prev_hash should be empty
            if block.id == 1 {
                if block.prev_hash != prev_hash {
                    warn!(event = "blockchain_integrity_error", message = "Genesis block has invalid prev_hash");
                    return Ok(false);
                }
            } else {
                // For subsequent blocks, prev_hash should match previous block's state_hash
                if block.prev_hash != prev_hash {
                    warn!(
                        event = "blockchain_integrity_error",
                        message = "Block chain broken",
                        block_id = block.id
                    );
                    return Ok(false);
                }
            }
            
            // Update prev_hash for next iteration
            prev_hash = block.state_hash;
        }
        
        Ok(true)
    }
    
    // Compact the database (vacuum) to save space
    pub async fn compact_database(&self) -> Result<(), BlockchainError> {
        info!(event = "database_compact_started");
        
        sqlx::query("VACUUM")
            .execute(&self.pool)
            .await
            .map_err(|e| BlockchainError::Storage(format!("Failed to compact database: {}", e)))?;
        
        info!(event = "database_compact_completed");
        Ok(())
    }
    
    // Get database size in bytes
    pub async fn get_database_size(&self) -> Result<usize, BlockchainError> {
        sqlx::query!(
            "SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BlockchainError::Storage(format!("Failed to get database size: {}", e)))
        .map(|row| row.size as usize)
    }
}