use sqlx::SqlitePool;
use tokio::time::{self, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, error};
use crate::blockchain::core::HybridChain;
use crate::blockchain::anchoring::DifferentialAnchoring;
use crate::blockchain::error::BlockchainError;
use chrono::Utc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SyncPriority {
    Critical,    // Immediate blockchain commit
    High,        // Next batch window
    Background,  // Idle time processing
}

pub struct AdaptiveBatcher {
    db_pool: SqlitePool,
    chain: Arc<Mutex<HybridChain>>,
    differential_anchoring: Arc<Mutex<DifferentialAnchoring>>,
    config: BatchConfig,
}

pub struct BatchConfig {
    // Base batch interval in seconds
    pub base_interval: u64,
    
    // Maximum items per batch
    pub max_batch_size: usize,
    
    // Minimum items to trigger a batch before interval
    pub min_batch_threshold: usize,
    
    // Maximum batch wait time for critical items (seconds)
    pub critical_max_wait: u64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            base_interval: 300, // 5 minutes
            max_batch_size: 1000,
            min_batch_threshold: 50,
            critical_max_wait: 30, // 30 seconds
        }
    }
}

pub struct PendingChange {
    pub id: i64,
    pub change_type: String,
    pub data: Vec<u8>,
    pub priority: i32,
    pub created_at: i64,
}

impl AdaptiveBatcher {
    pub fn new(
        db_pool: SqlitePool,
        chain: Arc<Mutex<HybridChain>>,
        differential_anchoring: Arc<Mutex<DifferentialAnchoring>>,
        config: Option<BatchConfig>,
    ) -> Self {
        Self {
            db_pool,
            chain,
            differential_anchoring,
            config: config.unwrap_or_default(),
        }
    }
    
    pub async fn add_change(&self, change: &PendingChange, priority: SyncPriority) -> Result<(), sqlx::Error> {
        // Store in database with priority
        sqlx::query!(
            "INSERT INTO pending_changes (type, data, priority, created_at) VALUES (?, ?, ?, ?)",
            change.change_type,
            change.data,
            priority as i32,
            Utc::now().timestamp(),
        )
        .execute(&self.db_pool)
        .await?;
        
        // If this is a critical change, we might need to trigger batch early
        if priority == SyncPriority::Critical {
            self.check_critical_batch().await;
        }
        
        Ok(())
    }
    
    async fn check_critical_batch(&self) {
        // Check if we have critical items that need processing
        let critical_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM pending_changes WHERE priority = ?",
            SyncPriority::Critical as i32,
        )
        .fetch_one(&self.db_pool)
        .await
        .map(|row| row.count as usize)
        .unwrap_or(0);
        
        // If we have critical items, check their age
        if critical_count > 0 {
            let oldest_critical = sqlx::query!(
                "SELECT MIN(created_at) as oldest FROM pending_changes WHERE priority = ?",
                SyncPriority::Critical as i32,
            )
            .fetch_one(&self.db_pool)
            .await
            .map(|row| row.oldest as i64)
            .unwrap_or(0);
            
            let now = Utc::now().timestamp();
            let age_seconds = now - oldest_critical;
            
            // If critical items have been waiting too long, process immediately
            if age_seconds > self.config.critical_max_wait as i64 {
                tokio::spawn(self.process_batch());
            }
        }
    }
    
    async fn process_batch(&self) -> Result<(), BlockchainError> {
        info!(event = "batch_processing_start");
        let start_time = std::time::Instant::now();
        
        // Get changes up to max batch size, ordered by priority
        let changes = sqlx::query_as!(
            PendingChange,
            "SELECT id, type as change_type, data, priority, created_at FROM pending_changes 
             ORDER BY priority ASC LIMIT ?",
            self.config.max_batch_size,
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| BlockchainError::Database(e))?;
        
        if changes.is_empty() {
            return Ok(());
        }
        
        // Process the changes using differential anchoring
        {
            let mut anchoring = self.differential_anchoring.lock().await;
            
            // Process each change based on its type
            for change in &changes {
                match change.change_type.as_str() {
                    "forum_post" => {
                        // Example: Process forum post change
                        if let Ok((original, updated)) = bincode::deserialize::<(ForumPost, ForumPost)>(&change.data) {
                            anchoring.add_forum_diff(&original, &updated);
                        }
                    },
                    "course_achievement" => {
                        // Example: Process achievement change
                        if let Ok((original, updated)) = bincode::deserialize::<(CourseAchievement, CourseAchievement)>(&change.data) {
                            anchoring.add_achievement_diff(&original, &updated);
                        }
                    },
                    _ => {
                        warn!(event = "unknown_change_type", change_type = change.change_type);
                    }
                }
            }
            
            // Anchor the changes
            anchoring.anchor_changes().await?;
        }
        
        // Delete processed changes
        let ids: Vec<i64> = changes.iter().map(|c| c.id).collect();
        let ids_str = ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        
        let query = format!("DELETE FROM pending_changes WHERE id IN ({})", ids_str);
        sqlx::query(&query)
            .execute(&self.db_pool)
            .await
            .map_err(|e| BlockchainError::Database(e))?;
        
        let elapsed = start_time.elapsed();
        info!(
            event = "batch_processing_complete",
            changes_count = changes.len(),
            duration_ms = elapsed.as_millis() as u64,
        );
        
        Ok(())
    }
    
    pub async fn start_batch_loop(&self) {
        info!(event = "batch_loop_started", interval_secs = self.config.base_interval);
        
        let mut interval = time::interval(Duration::from_secs(self.config.base_interval));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.process_batch().await {
                error!(event = "batch_processing_error", error = %e);
            }
            
            // Check if we need to adjust the batch interval based on load
            self.adjust_batch_interval(&mut interval).await;
        }
    }
    
    async fn adjust_batch_interval(&self, interval: &mut time::Interval) {
        // Get stats on pending changes
        let stats = sqlx::query!(
            "SELECT COUNT(*) as count, MAX(created_at) as newest, MIN(created_at) as oldest FROM pending_changes"
        )
        .fetch_one(&self.db_pool)
        .await;
        
        if let Ok(stats) = stats {
            let count = stats.count as usize;
            
            // Adjust interval based on load
            let new_interval = if count > self.config.max_batch_size * 2 {
                // High load, reduce interval
                self.config.base_interval / 2
            } else if count < self.config.min_batch_threshold {
                // Low load, increase interval
                self.config.base_interval * 2
            } else {
                // Normal load, use base interval
                self.config.base_interval
            };
            
            // Cap the interval
            let new_interval = new_interval.max(30).min(600);
            
            // Update the interval if changed
            let current = interval.period().as_secs();
            if new_interval != current {
                *interval = time::interval(Duration::from_secs(new_interval));
                info!(event = "batch_interval_adjusted", new_interval_secs = new_interval);
            }
        }
    }
}

// These are needed for deserialization from the batching system
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForumPost {
    pub id: String,
    pub content: String,
    pub author: String,
    pub timestamp: i64,
    pub version: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CourseAchievement {
    pub id: String,
    pub user_id: String,
    pub course_id: String,
    pub achievement_type: String,
    pub timestamp: i64,
    pub version: u64,
}

// Minimal BatchProcessor with essential features
pub struct BatchProcessor {
    db: SqlitePool,
    chain: Arc<Mutex<HybridChain>>,
}

impl BatchProcessor {
    pub fn new(db: SqlitePool, chain: Arc<Mutex<HybridChain>>) -> Self {
        Self { db, chain }
    }

    // Process achievements in batches
    async fn process_batch(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Fetch all pending achievements using a minimal query
        let achievements = sqlx::query_as!(
            AchievementRecord,
            "SELECT user_id, achievement_id FROM temp_achievements LIMIT 1000"
        )
        .fetch_all(&self.db)
        .await?;
        
        if achievements.is_empty() {
            return Ok(());
        }
        
        // Create a batch record in the blockchain
        let mut chain = self.chain.lock().await;
        let timestamp = chain.create_block().await?;
        
        // Delete processed achievements
        sqlx::query!("DELETE FROM temp_achievements WHERE rowid IN (SELECT rowid FROM temp_achievements LIMIT 1000)")
            .execute(&self.db)
            .await?;
            
        Ok(())
    }
}

// Minimal record structure
struct AchievementRecord {
    user_id: String,
    achievement_id: String,
}

// Simplified batch processor starter
#[tokio::task]
pub async fn start_batch_processor(
    db: SqlitePool, 
    chain: Arc<Mutex<HybridChain>>
) {
    let processor = BatchProcessor::new(db, chain);
    let mut interval = time::interval(Duration::hours(1).to_std().unwrap());
    
    loop {
        interval.tick().await;
        if let Err(e) = processor.process_batch().await {
            eprintln!("Batch processing error: {}", e);
        }
    }
}