use automerge::Automerge;
use libp2p::PeerId;
use redb::Database;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::collections::BTreeSet;
use uuid::Uuid;
use tokio::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;
use crate::blockchain::config::ChainConfig;

mod config;
mod batching;
mod cache;
mod crdt_ops;
mod p2p;
mod wasm_bridge;
mod memory;
mod consensus;
mod anchoring;
mod governor;
mod sync;
mod metrics;
mod error;
mod storage;
mod core;

pub use config::ChainConfig;
pub use batching::start_batch_processor;
pub use cache::CachedVerifier;
pub use wasm_bridge::BlockchainAnchor;
pub use error::BlockchainError;
pub use core::HybridChain;
pub use anchoring::DifferentialAnchoring;
pub use batching::{AdaptiveBatcher, BatchConfig, SyncPriority};
pub use governor::{ResourceGovernor, TransactionGuard};
pub use sync::{AdaptiveSyncManager, UserEvent};
pub use metrics::PerformanceMetrics;

lazy_static! {
    static ref WASM_ANCHOR: Arc<Mutex<Option<BlockchainAnchor>>> = Arc::new(Mutex::new(None));
}

pub struct HybridChain {
    blockchain: Database, 
    crdt_store: Automerge,
    config: ChainConfig,
}

impl HybridChain {
    pub async fn new(config_path: Option<&str>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = match config_path {
            Some(path) => ChainConfig::load(path)?,
            None => ChainConfig::default(),
        };
        
        let db = Database::create("./chain-data")?;
        let crdt = Automerge::new();
        
        let mut anchor = WASM_ANCHOR.lock().await;
        if anchor.is_none() {
            *anchor = Some(BlockchainAnchor::new(None));
        }
        
        Ok(Self { 
            blockchain: db, 
            crdt_store: crdt,
            config,
        })
    }
    
    pub async fn create_block(&mut self) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
        let mut buffer = Vec::new();
        self.crdt_store.save(&mut buffer)?;
        
        let anchor = WASM_ANCHOR.lock().await;
        let state_hash = anchor.as_ref().unwrap().hash_data(&buffer);
        let timestamp = Utc::now().timestamp();
        
        let prev_hash = self.last_block_hash().await?;
        
        let signature = anchor.as_ref().unwrap().sign(&state_hash);
        
        let write_txn = self.blockchain.begin_write()?;
        let mut table = write_txn.open_table("blocks")?;
        
        #[derive(Serialize)]
        struct CompactBlock {
            t: i64,
            p: Vec<u8>,
            h: Vec<u8>,
            s: Vec<u8>,
        }
        
        let block = CompactBlock {
            t: timestamp,
            p: prev_hash.to_vec(),
            h: state_hash,
            s: signature,
        };
        
        let block_bytes = bincode::serialize(&block)?;
        table.insert(&timestamp.to_be_bytes(), block_bytes.as_slice())?;
        
        write_txn.commit()?;
        Ok(timestamp)
    }
    
    async fn last_block_hash(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let read_txn = self.blockchain.begin_read()?;
        if let Ok(table) = read_txn.open_table("blocks") {
            if let Ok(Some((_, block_data))) = table.last() {
                #[derive(Deserialize)]
                struct HashOnly {
                    h: Vec<u8>,
                }
                
                let hash: HashOnly = bincode::deserialize(block_data.value())?;
                return Ok(hash.h);
            }
        }
        
        Ok(vec![0u8; 32])
    }
    
    pub async fn compact_history(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.storage.compact_history {
            self.crdt_store.compact();
            println!("Compacted CRDT history");
        }
        
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LmsBlock {
    timestamp: i64,
    prev_hash: [u8; 32],
    state_hash: [u8; 32],
    signatures: Vec<[u8; 64]>,
}

pub async fn initialize_blockchain(
    db_pool: sqlx::SqlitePool,
) -> Result<HybridChain, BlockchainError> {
    let chain = core::HybridChain::new().await?;
    
    // Start metrics reporting
    let metrics = metrics::PerformanceMetrics::new();
    metrics.start_reporting_task(60); // Report every minute
    
    // Return initialized chain
    Ok(chain)
}

// Basic blockchain module structure
pub mod core;
pub mod storage;
pub mod error;

pub use error::BlockchainError;