use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct ConsensusConfig {
    pub validation_threshold: u32,
    pub block_interval: u64,
    pub max_peers: u32,
    pub gossip_interval: u64,
}

#[derive(Deserialize, Clone)]
pub struct StorageConfig {
    pub compact_history: bool,
    pub history_retention_days: u32,
}

#[derive(Deserialize, Clone)]
pub struct PerformanceConfig {
    pub max_cache_size_mb: u32,
    pub max_pending_txs: u32,
}

#[derive(Deserialize, Clone)]
pub struct ChainConfig {
    pub consensus: ConsensusConfig,
    pub storage: StorageConfig,
    pub performance: PerformanceConfig,
}

impl ChainConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: ChainConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
    
    pub fn default() -> Self {
        Self {
            consensus: ConsensusConfig {
                validation_threshold: 1,
                block_interval: 300,
                max_peers: 5,
                gossip_interval: 10000,
            },
            storage: StorageConfig {
                compact_history: true,
                history_retention_days: 14,
            },
            performance: PerformanceConfig {
                max_cache_size_mb: 50,
                max_pending_txs: 1000,
            },
        }
    }
}