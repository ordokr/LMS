use crate::services::integration::sync_state::SyncState;
use crate::services::integration::sync_transaction::SyncTransaction;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use thiserror::Error;

/// Error types for the sync monitor
#[derive(Error, Debug)]
pub enum SyncMonitorError {
    #[error("Failed to get entity statistics: {0}")]
    EntityStatsError(String),
    
    #[error("Failed to get transaction statistics: {0}")]
    TransactionStatsError(String),
    
    #[error("Failed to get sync status: {0}")]
    SyncStatusError(String),
    
    #[error("Invalid entity type: {0}")]
    InvalidEntityType(String),
    
    #[error("Entity not found: {0} {1}")]
    EntityNotFound(String, String),
}

/// Entity synchronization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityStats {
    pub total: usize,
    pub synced: usize,
    pub failed: usize,
    pub pending: usize,
    pub success_rate: f64,
}

/// Transaction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStats {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub average_duration_ms: f64,
}

/// Overall synchronization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    pub entities: EntityTypeStats,
    pub transactions: TransactionStats,
    pub overall: OverallStats,
}

/// Entity type statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityTypeStats {
    pub users: EntityStats,
    pub courses: EntityStats,
    pub assignments: EntityStats,
    pub discussions: EntityStats,
    pub posts: EntityStats,
}

/// Overall synchronization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallStats {
    pub last_sync: Option<u64>,
    pub error_rate: f64,
}

/// Entity synchronization history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySyncHistory {
    pub current_status: serde_json::Value,
    pub sync_history: Vec<SyncHistoryItem>,
}

/// Individual sync history item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncHistoryItem {
    pub id: String,
    pub timestamp: u64,
    pub operation: String,
    pub status: String,
    pub duration: Option<u64>,
    pub error: Option<String>,
}

/// Synchronization Monitor Service
pub struct SyncMonitor {
    sync_state: Arc<SyncState>,
    sync_transaction: Arc<SyncTransaction>,
    cached_stats: RwLock<Option<(SyncStats, Instant)>>,
    cache_ttl: Duration,
}

impl SyncMonitor {
    /// Create a new synchronization monitoring service
    pub fn new(sync_state: Arc<SyncState>, sync_transaction: Arc<SyncTransaction>) -> Self {
        SyncMonitor {
            sync_state,
            sync_transaction,
            cached_stats: RwLock::new(None),
            cache_ttl: Duration::from_secs(60), // 1 minute cache
        }
    }

    /// Get overall synchronization statistics
    pub async fn get_statistics(&self) -> Result<SyncStats, SyncMonitorError> {
        // Check cache first
        {
            let cached = self.cached_stats.read().await;
            if let Some((stats, cache_time)) = &*cached {
                if cache_time.elapsed() < self.cache_ttl {
                    return Ok(stats.clone());
                }
            }
        }

        // Cache miss or expired, compute fresh stats
        let users = self.get_entity_stats("user").await?;
        let courses = self.get_entity_stats("course").await?;
        let assignments = self.get_entity_stats("assignment").await?;
        let discussions = self.get_entity_stats("discussion").await?;
        let posts = self.get_entity_stats("post").await?;
        
        let entity_stats = EntityTypeStats {
            users,
            courses,
            assignments,
            discussions,
            posts,
        };
        
        let transaction_stats = self.get_transaction_stats().await?;
        
        let last_sync = self.get_last_sync_time().await?;
        let error_rate = self.calculate_error_rate().await?;
        
        let overall = OverallStats {
            last_sync,
            error_rate,
        };
        
        let stats = SyncStats {
            entities: entity_stats,
            transactions: transaction_stats,
            overall,
        };
        
        // Update cache
        let mut cache = self.cached_stats.write().await;
        *cache = Some((stats.clone(), Instant::now()));
        
        Ok(stats)
    }

    /// Get pending synchronization items that need attention
    pub async fn get_pending_items(&self, limit: usize) -> Result<Vec<serde_json::Value>, SyncMonitorError> {
        self.sync_state.get_failed_syncs(limit)
            .await
            .map_err(|e| SyncMonitorError::SyncStatusError(e.to_string()))
    }

    /// Get synchronization history for a specific entity
    pub async fn get_entity_sync_history(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<EntitySyncHistory, SyncMonitorError> {
        // Get transaction history
        let transactions = self.sync_transaction.get_transactions_for_entity(entity_type, entity_id)
            .await
            .map_err(|e| SyncMonitorError::TransactionStatsError(e.to_string()))?;
        
        // Get current sync state
        let sync_state = self.sync_state.get_sync_status(entity_type, entity_id)
            .await
            .map_err(|e| SyncMonitorError::SyncStatusError(e.to_string()))?;
        
        // Convert transactions to history items
        let sync_history = transactions.into_iter()
            .map(|t| {
                let duration = if let (Some(start), Some(end)) = (t.start_time, t.end_time) {
                    Some(end - start)
                } else {
                    None
                };
                
                SyncHistoryItem {
                    id: t.id,
                    timestamp: t.timestamp,
                    operation: t.operation,
                    status: t.status,
                    duration,
                    error: t.error,
                }
            })
            .collect();
        
        Ok(EntitySyncHistory {
            current_status: sync_state,
            sync_history,
        })
    }

    /// Trigger a manual resync for a failing entity
    pub async fn trigger_resync(
        &self,
        entity_type: &str,
        entity_id: &str,
        priority: Option<&str>,
    ) -> Result<bool, SyncMonitorError> {
        // Validate entity type
        match entity_type {
            "user" | "course" | "assignment" | "discussion" | "post" => {},
            _ => return Err(SyncMonitorError::InvalidEntityType(entity_type.to_string())),
        }
        
        // Reset the sync state
        self.sync_state.reset_sync_state(entity_type, entity_id)
            .await
            .map_err(|e| SyncMonitorError::SyncStatusError(e.to_string()))?;
        
        // Create a new sync transaction
        let priority = priority.unwrap_or("high").to_string();
        
        self.sync_transaction.create_transaction(entity_type, entity_id, "resync", Some(priority))
            .await
            .map_err(|e| SyncMonitorError::TransactionStatsError(e.to_string()))?;
        
        Ok(true)
    }

    // Private helper methods
    
    /// Get statistics for a specific entity type
    async fn get_entity_stats(&self, entity_type: &str) -> Result<EntityStats, SyncMonitorError> {
        let stats = self.sync_state.get_entity_stats(entity_type)
            .await
            .map_err(|e| SyncMonitorError::EntityStatsError(e.to_string()))?;
        
        let total = stats["total"].as_u64().unwrap_or(0) as usize;
        let synced = stats["synced"].as_u64().unwrap_or(0) as usize;
        let failed = stats["failed"].as_u64().unwrap_or(0) as usize;
        let pending = stats["pending"].as_u64().unwrap_or(0) as usize;
        
        let success_rate = if total > 0 {
            synced as f64 / total as f64
        } else {
            0.0
        };
        
        Ok(EntityStats {
            total,
            synced,
            failed,
            pending,
            success_rate,
        })
    }

    /// Get statistics about transactions
    async fn get_transaction_stats(&self) -> Result<TransactionStats, SyncMonitorError> {
        let stats = self.sync_transaction.get_transaction_stats()
            .await
            .map_err(|e| SyncMonitorError::TransactionStatsError(e.to_string()))?;
        
        let total = stats["total"].as_u64().unwrap_or(0) as usize;
        let success = stats["success"].as_u64().unwrap_or(0) as usize;
        let failed = stats["failed"].as_u64().unwrap_or(0) as usize;
        let in_progress = stats["in_progress"].as_u64().unwrap_or(0) as usize;
        let average_duration_ms = stats["average_duration_ms"].as_f64().unwrap_or(0.0);
        
        Ok(TransactionStats {
            total,
            success,
            failed,
            in_progress,
            average_duration_ms,
        })
    }

    /// Get the timestamp of the last successful sync
    async fn get_last_sync_time(&self) -> Result<Option<u64>, SyncMonitorError> {
        let last_sync = self.sync_transaction.get_last_successful_sync()
            .await
            .map_err(|e| SyncMonitorError::TransactionStatsError(e.to_string()))?;
        
        Ok(last_sync)
    }

    /// Calculate the overall error rate in synchronization
    async fn calculate_error_rate(&self) -> Result<f64, SyncMonitorError> {
        let stats = self.sync_transaction.get_transaction_stats()
            .await
            .map_err(|e| SyncMonitorError::TransactionStatsError(e.to_string()))?;
        
        let total = stats["total"].as_u64().unwrap_or(0) as f64;
        let failed = stats["failed"].as_u64().unwrap_or(0) as f64;
        
        if total > 0.0 {
            Ok(failed / total)
        } else {
            Ok(0.0)
        }
    }
}
