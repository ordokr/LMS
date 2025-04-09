use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use sqlx::SqlitePool;
use log::{info, warn};
use tokio::time;

pub struct MemoryMonitor {
    threshold_mb: usize,
    poll_interval: Duration,
    last_warning: Arc<AtomicUsize>, // timestamp
    db_pool: Arc<SqlitePool>,
}

impl MemoryMonitor {
    pub fn new(db_pool: Arc<SqlitePool>, threshold_mb: usize) -> Self {
        Self {
            threshold_mb,
            poll_interval: Duration::from_secs(30),
            last_warning: Arc::new(AtomicUsize::new(0)),
            db_pool,
        }
    }
    
    pub fn start_monitoring(self) {
        let self_arc = Arc::new(self);
        let monitor_clone = self_arc.clone();
        
        tokio::spawn(async move {
            let mut interval = time::interval(monitor_clone.poll_interval);
            
            loop {
                interval.tick().await;
                monitor_clone.check_memory().await;
            }
        });
    }
    
    async fn check_memory(&self) {
        // Get SQLite memory stats
        let memory_used = self.get_sqlite_memory_usage().await.unwrap_or(0);
        let memory_mb = memory_used / 1024 / 1024;
        
        // Log memory usage periodically
        info!("SQLite memory usage: {}MB", memory_mb);
        
        // Check threshold
        if memory_mb > self.threshold_mb {
            // Get current time as seconds since UNIX epoch
            let now = Instant::now()
                .duration_since(std::time::UNIX_EPOCH.into())
                .unwrap_or(Duration::from_secs(0))
                .as_secs() as usize;
                
            // Only warn once per hour
            let last = self.last_warning.load(Ordering::Relaxed);
            if now - last > 3600 {
                warn!("SQLite memory usage is high: {}MB (threshold: {}MB)", 
                      memory_mb, self.threshold_mb);
                      
                // Try to free memory
                self.reduce_memory_pressure().await;
                
                // Update last warning time
                self.last_warning.store(now, Ordering::Relaxed);
            }
        }
    }
    
    async fn get_sqlite_memory_usage(&self) -> Result<usize, sqlx::Error> {
        let result = sqlx::query!("SELECT memory_used FROM pragma_memory_used")
            .fetch_one(&*self.db_pool)
            .await?;
            
        Ok(result.memory_used as usize)
    }
    
    async fn reduce_memory_pressure(&self) -> Result<(), sqlx::Error> {
        info!("Reducing memory pressure...");
        
        // Release unused memory
        sqlx::query("PRAGMA shrink_memory").execute(&*self.db_pool).await?;
        
        // Clear statement cache
        sqlx::query("PRAGMA optimize").execute(&*self.db_pool).await?;
        
        // Get new memory usage
        let new_usage = self.get_sqlite_memory_usage().await?;
        info!("Memory usage after optimization: {}KB", new_usage / 1024);
        
        Ok(())
    }
}