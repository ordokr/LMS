use sqlx::{sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions}, SqliteExecutor};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use log::{info, warn, error, debug};

/// Enhanced connection pool with metrics and self-tuning capabilities
pub struct EnhancedConnectionPool {
    pool: SqlitePool,
    metrics: Arc<Mutex<PoolMetrics>>,
}

struct PoolMetrics {
    total_connections: u32,
    active_connections: u32,
    idle_connections: u32,
    wait_times: Vec<Duration>,
    connection_lifetime: Vec<Duration>,
    last_diagnostics: Instant,
}

impl EnhancedConnectionPool {
    /// Create a new optimized connection pool
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        // Parse connection options with optimized settings
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(Duration::from_secs(10))
            .pragma("cache_size", "-8000")   // 8MB cache
            .pragma("foreign_keys", "ON")
            .pragma("temp_store", "MEMORY")  // Store temp tables in memory
            .pragma("mmap_size", "30000000") // 30MB mmap
            .pragma("page_size", "4096");    // Optimize page size
            
        // Create pool with optimal settings for desktop/local use
        let pool = SqlitePoolOptions::new()
            .min_connections(2)        // Min connections in pool
            .max_connections(8)        // Max connections in pool - prevent resource exhaustion
            .max_lifetime(Duration::from_secs(1800)) // 30-minute maximum connection lifetime
            .idle_timeout(Duration::from_secs(600))  // 10-minute idle timeout
            .acquire_timeout(Duration::from_secs(5)) // 5-second timeout for connection acquisition
            .test_before_acquire(true) // Ensure connections are valid before use
            .connect_with(options)
            .await?;
            
        // Initialize pool metrics
        let metrics = Arc::new(Mutex::new(PoolMetrics {
            total_connections: 0,
            active_connections: 0,
            idle_connections: 0,
            wait_times: Vec::with_capacity(100),
            connection_lifetime: Vec::with_capacity(100),
            last_diagnostics: Instant::now(),
        }));
        
        let pool_obj = Self { pool, metrics };
        
        // Start background monitoring
        pool_obj.start_metrics_collector();
        
        // Run initial optimization
        pool_obj.optimize_database().await?;
            
        Ok(pool_obj)
    }
    
    /// Get the underlying SQLite pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    /// Run initial database optimizations
    async fn optimize_database(&self) -> Result<(), sqlx::Error> {
        info!("Running initial database optimizations");
        
        // Ensure we're using WAL mode
        sqlx::query("PRAGMA journal_mode = WAL")
            .execute(&self.pool).await?;
            
        // Run ANALYZE to collect statistics for query planner
        sqlx::query("ANALYZE")
            .execute(&self.pool).await?;
            
        // Run integrity check in the background
        let pool = self.pool.clone();
        tokio::spawn(async move {
            match sqlx::query("PRAGMA integrity_check")
                .fetch_one(&pool).await {
                Ok(row) => {
                    let result: String = row.try_get(0).unwrap_or_else(|_| "failed".into());
                    if result != "ok" {
                        warn!("Database integrity check failed: {}", result);
                    } else {
                        debug!("Database integrity check passed");
                    }
                },
                Err(e) => error!("Database integrity check error: {}", e),
            }
        });
        
        info!("Database optimization complete");
        Ok(())
    }
    
    /// Start background metrics collection
    fn start_metrics_collector(&self) {
        let pool = self.pool.clone();
        let metrics = self.metrics.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                // Get pool status
                let mut conn = match pool.acquire().await {
                    Ok(conn) => conn,
                    Err(_) => continue,
                };
                
                // Get connection stats using SQLite pragmas
                let sqlite_stats = match collect_sqlite_stats(&mut conn).await {
                    Ok(stats) => stats,
                    Err(e) => {
                        error!("Failed to collect SQLite stats: {}", e);
                        continue;
                    }
                };
                
                let mut metrics_guard = metrics.lock().await;
                
                // Update metrics
                metrics_guard.total_connections = sqlite_stats.0;
                
                // Log diagnostics every 5 minutes
                if metrics_guard.last_diagnostics.elapsed() > Duration::from_secs(300) {
                    log_pool_diagnostics(&metrics_guard, sqlite_stats).await;
                    metrics_guard.last_diagnostics = Instant::now();
                    
                    // Reset metrics collection
                    if metrics_guard.wait_times.len() > 1000 {
                        metrics_guard.wait_times.clear();
                        metrics_guard.connection_lifetime.clear();
                    }
                }
            }
        });
    }
    
    /// Acquire a connection with metrics tracking
    pub async fn acquire(&self) -> Result<PoolConnection, sqlx::Error> {
        let start = Instant::now();
        
        match self.pool.acquire().await {
            Ok(conn) => {
                let wait_time = start.elapsed();
                
                // Record metrics
                let mut metrics_guard = self.metrics.lock().await;
                metrics_guard.wait_times.push(wait_time);
                metrics_guard.active_connections += 1;
                
                // Return wrapped connection that updates metrics on drop
                Ok(PoolConnection {
                    conn: Some(conn),
                    created_at: Instant::now(),
                    metrics: self.metrics.clone(),
                })
            },
            Err(e) => Err(e),
        }
    }
    
    /// Perform maintenance operations on the database
    pub async fn maintenance(&self) -> Result<(), sqlx::Error> {
        info!("Running database maintenance");
        
        // Run in sequence to avoid overloading the database
        sqlx::query("PRAGMA optimize").execute(&self.pool).await?;
        sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)").execute(&self.pool).await?;
        sqlx::query("ANALYZE").execute(&self.pool).await?;
        
        info!("Database maintenance complete");
        Ok(())
    }
}

/// Connection wrapper that updates metrics when dropped
pub struct PoolConnection<'a> {
    conn: Option<sqlx::pool::PoolConnection<sqlx::Sqlite>>,
    created_at: Instant,
    metrics: Arc<Mutex<PoolMetrics>>,
}

impl<'a> Drop for PoolConnection<'a> {
    fn drop(&mut self) {
        if self.conn.is_some() {
            let lifetime = self.created_at.elapsed();
            let metrics = self.metrics.clone();
            
            tokio::spawn(async move {
                let mut metrics_guard = metrics.lock().await;
                metrics_guard.connection_lifetime.push(lifetime);
                metrics_guard.active_connections -= 1;
            });
        }
    }
}

impl<'a> std::ops::Deref for PoolConnection<'a> {
    type Target = sqlx::pool::PoolConnection<sqlx::Sqlite>;
    
    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}

impl<'a> std::ops::DerefMut for PoolConnection<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().unwrap()
    }
}

async fn collect_sqlite_stats(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>
) -> Result<(u32, u32, u32), sqlx::Error> {
    // Get database stats
    let cache_stats = sqlx::query("PRAGMA cache_stats").fetch_all(&mut *conn).await?;
    let total_connections = sqlx::query_scalar::<_, i64>("PRAGMA connection_count").fetch_one(&mut *conn).await? as u32;
    
    // Parse cache stats for memory usage
    let mut cache_used = 0;
    let mut cache_hits = 0;
    
    for row in cache_stats {
        let name: String = row.try_get("name")?;
        if name == "cache_used" {
            cache_used = row.try_get::<i64, _>("val")? as u32;
        }
        if name == "cache_hit" {
            cache_hits = row.try_get::<i64, _>("val")? as u32;
        }
    }
    
    Ok((total_connections, cache_hits, cache_used))
}

async fn log_pool_diagnostics(metrics: &PoolMetrics, sqlite_stats: (u32, u32, u32)) {
    // Calculate average wait time
    let avg_wait = if !metrics.wait_times.is_empty() {
        metrics.wait_times.iter().sum::<Duration>().as_millis() / metrics.wait_times.len() as u128
    } else {
        0
    };
    
    // Calculate average connection lifetime
    let avg_lifetime = if !metrics.connection_lifetime.is_empty() {
        metrics.connection_lifetime.iter().sum::<Duration>().as_secs() / metrics.connection_lifetime.len() as u64
    } else {
        0
    };
    
    info!(
        "DB Pool stats: connections={}, active={}, idle={}, avg_wait={}ms, avg_lifetime={}s, cache_hits={}, cache_used={}",
        metrics.total_connections,
        metrics.active_connections,
        metrics.idle_connections,
        avg_wait,
        avg_lifetime,
        sqlite_stats.1,
        sqlite_stats.2
    );
}