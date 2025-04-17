use crate::sync::sync_queue::SyncQueue;
use crate::error::Error;
use sqlx::Pool;
use sqlx::Sqlite;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use log::{info, warn, error};
use std::sync::atomic::{AtomicBool, Ordering};

/// Configuration for sync cleanup operations
#[derive(Debug, Clone)]
pub struct SyncCleanupConfig {
    /// Number of days to keep completed sync operations before removal
    pub completed_retention_days: u32,
    /// Number of days to keep failed sync operations before removal
    pub failed_retention_days: u32,
    /// Clean interval in hours
    pub cleanup_interval_hours: u32,
    /// Maximum items to clean in a single batch
    pub max_batch_size: u32,
    /// Whether to enable detailed logging
    pub enable_detailed_logging: bool,
}

impl Default for SyncCleanupConfig {
    fn default() -> Self {
        Self {
            completed_retention_days: 30,  // Keep completed operations for 30 days
            failed_retention_days: 90,     // Keep failed operations for 90 days
            cleanup_interval_hours: 24,    // Run cleanup once a day
            max_batch_size: 1000,          // Clean up to 1000 items at a time
            enable_detailed_logging: true, // Enable detailed logging
        }
    }
}

/// Service responsible for cleaning up old sync operations
pub struct SyncMaintenanceService {
    sync_queue: Arc<SyncQueue>,
    db_pool: Pool<Sqlite>,
    config: SyncCleanupConfig,
    running: Arc<AtomicBool>,
}

impl SyncMaintenanceService {
    /// Create a new sync maintenance service
    pub fn new(sync_queue: Arc<SyncQueue>, db_pool: Pool<Sqlite>, config: SyncCleanupConfig) -> Self {
        Self {
            sync_queue,
            db_pool,
            config,
            running: Arc::new(AtomicBool::new(false)),
        }
    }
    
    /// Start the maintenance service
    pub async fn start(&self) -> Result<(), Error> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(Error::Custom("Maintenance service is already running".to_string()));
        }
        
        let sync_queue = Arc::clone(&self.sync_queue);
        let config = self.config.clone();
        let running = Arc::clone(&self.running);
        
        tokio::spawn(async move {
            info!("Sync maintenance service started");
            
            // Cleanup interval in milliseconds
            let interval_ms = config.cleanup_interval_hours as u64 * 60 * 60 * 1000;
            let mut interval_timer = interval(Duration::from_millis(interval_ms));
            
            while running.load(Ordering::SeqCst) {
                interval_timer.tick().await;
                
                info!("Running scheduled sync queue cleanup");
                
                // Clean up completed operations
                match sync_queue.cleanup_completed(config.completed_retention_days).await {
                    Ok(count) => {
                        if config.enable_detailed_logging || count > 0 {
                            info!("Cleaned up {} completed sync operations older than {} days", 
                                 count, config.completed_retention_days);
                        }
                    },
                    Err(e) => {
                        error!("Error cleaning up completed sync operations: {}", e);
                    }
                }
                
                // Clean up failed operations
                match sync_queue.cleanup_failed(config.failed_retention_days).await {
                    Ok(count) => {
                        if config.enable_detailed_logging || count > 0 {
                            info!("Cleaned up {} failed sync operations older than {} days", 
                                 count, config.failed_retention_days);
                        }
                    },
                    Err(e) => {
                        error!("Error cleaning up failed sync operations: {}", e);
                    }
                }
                
                // Add other maintenance tasks here as needed
                // For example: archive sync history, consistency checks, etc.
            }
            
            info!("Sync maintenance service stopped");
        });
        
        Ok(())
    }
    
    /// Stop the maintenance service
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        info!("Sync maintenance service stopping");
    }
    
    /// Run an immediate cleanup (outside of the regular schedule)
    pub async fn run_immediate_cleanup(&self) -> Result<(u32, u32), Error> {
        info!("Running immediate sync queue cleanup");
        
        // Clean up completed operations
        let completed_count = self.sync_queue.cleanup_completed(self.config.completed_retention_days).await?;
        
        // Clean up failed operations
        let failed_count = self.sync_queue.cleanup_failed(self.config.failed_retention_days).await?;
        
        info!("Immediate cleanup complete: removed {} completed and {} failed operations", 
             completed_count, failed_count);
             
        Ok((completed_count, failed_count))
    }
    
    /// Run consistency checks on sync data
    pub async fn run_consistency_checks(&self) -> Result<(), Error> {
        info!("Running sync data consistency checks");
        
        // Check for operations stuck in "in progress" status
        let stuck_operations = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM sync_queue
            WHERE status = 'in_progress'
            AND updated_at < datetime('now', '-1 hour')
            "#
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        let stuck_count = stuck_operations.count;
        if stuck_count > 0 {
            warn!("Found {} sync operations stuck in 'in progress' state for more than 1 hour", stuck_count);
            
            // Reset stuck operations to 'pending' status so they can be retried
            let reset_count = sqlx::query!(
                r#"
                UPDATE sync_queue
                SET status = 'pending', updated_at = datetime('now')
                WHERE status = 'in_progress'
                AND updated_at < datetime('now', '-1 hour')
                "#
            )
            .execute(&self.db_pool)
            .await?
            .rows_affected();
            
            info!("Reset {} stuck operations to 'pending' status", reset_count);
        }
        
        // Additional consistency checks can be added here
        
        Ok(())
    }
    
    /// Archive old sync history to maintain performance
    pub async fn archive_old_sync_history(&self, days_threshold: u32) -> Result<u32, Error> {
        info!("Archiving sync history older than {} days", days_threshold);
        
        // Here we would implement archiving logic
        // For example, moving old entries to an archive table
        // This would depend on the specific schema and requirements
        
        Ok(0) // Placeholder - return the count of archived entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::sync_queue::{SyncQueue, SyncStatus};
    use sqlx::sqlite::SqlitePoolOptions;
    use chrono::Utc;
    use uuid::Uuid;
    
    async fn setup_test_db() -> Result<Pool<Sqlite>, Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await?;
        
        // Create the necessary tables for testing
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sync_queue (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                source_system TEXT NOT NULL,
                target_system TEXT NOT NULL,
                data TEXT,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                attempts INTEGER DEFAULT 0,
                error_message TEXT
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        Ok(pool)
    }
    
    #[tokio::test]
    async fn test_sync_maintenance_service() -> Result<(), Error> {
        let pool = setup_test_db().await?;
        let sync_queue = Arc::new(SyncQueue::new(pool.clone()));
        
        // Add some test operations
        for i in 0..5 {
            let op_id = sync_queue.enqueue(
                "course",
                &format!("course-{}", i),
                "update",
                "canvas",
                "discourse",
                serde_json::json!({"title": format!("Test Course {}", i)})
            ).await?;
            
            sync_queue.mark_completed(&op_id).await?;
            
            // Modify timestamp directly in DB to simulate older entries
            sqlx::query(
                "UPDATE sync_queue SET updated_at = datetime('now', '-40 days') WHERE id = ?"
            )
            .bind(&op_id)
            .execute(pool.clone())
            .await?;
        }
        
        // Add some failed operations
        for i in 5..10 {
            let op_id = sync_queue.enqueue(
                "topic",
                &format!("topic-{}", i),
                "create",
                "canvas",
                "discourse",
                serde_json::json!({"title": format!("Test Topic {}", i)})
            ).await?;
            
            sync_queue.mark_failed(&op_id, &format!("Test error {}", i)).await?;
            
            // Modify timestamp directly in DB to simulate older entries
            sqlx::query(
                "UPDATE sync_queue SET updated_at = datetime('now', '-100 days') WHERE id = ?"
            )
            .bind(&op_id)
            .execute(pool.clone())
            .await?;
        }
        
        // Create maintenance service with test config
        let config = SyncCleanupConfig {
            completed_retention_days: 30,
            failed_retention_days: 60,
            cleanup_interval_hours: 24,
            max_batch_size: 100,
            enable_detailed_logging: true,
        };
        
        let maintenance_service = SyncMaintenanceService::new(
            Arc::clone(&sync_queue),
            pool.clone(),
            config
        );
        
        // Run immediate cleanup
        let (completed_count, failed_count) = maintenance_service.run_immediate_cleanup().await?;
        
        // All 5 completed items should be cleaned up (they're 40 days old, retention is 30 days)
        assert_eq!(completed_count, 5);
        
        // All 5 failed items should be cleaned up (they're 100 days old, retention is 60 days)
        assert_eq!(failed_count, 5);
        
        // Verify no items remain in the queue
        let remaining = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sync_queue")
            .fetch_one(&pool)
            .await?;
            
        assert_eq!(remaining, 0);
        
        Ok(())
    }
}