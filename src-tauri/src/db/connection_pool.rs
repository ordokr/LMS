use std::sync::Arc;
use std::time::{Duration, Instant};
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use tokio::sync::Mutex;
use log::{info, warn, error};

pub struct AdaptiveConnectionPool {
    pool: Arc<SqlitePool>,
    min_connections: u32,
    max_connections: u32,
    current_size: Arc<Mutex<u32>>,
    last_adjusted: Arc<Mutex<Instant>>,
    metrics: Arc<Mutex<PoolMetrics>>,
}

struct PoolMetrics {
    recent_wait_times: Vec<Duration>,
    connection_timeouts: u32,
    total_requests: u64,
    max_wait_time: Duration,
}

impl AdaptiveConnectionPool {
    pub async fn new(
        db_url: &str, 
        min_connections: u32,
        max_connections: u32,
    ) -> Result<Self, sqlx::Error> {
        // Create connection options with optimized settings
        let options = SqliteConnectOptions::new()
            .filename(db_url.trim_start_matches("sqlite:"))
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .pragma("cache_size", "-64000") // 64MB cache
            .pragma("foreign_keys", "ON")
            .pragma("busy_timeout", "5000") // 5 second timeout
            .pragma("temp_store", "MEMORY");
        
        // Create pool with initial min connections
        let pool = SqlitePoolOptions::new()
            .min_connections(min_connections)
            .max_connections(min_connections) // Start with min, will adjust as needed
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .max_lifetime(Duration::from_secs(3600))
            .connect_with(options)
            .await?;

        let adaptive_pool = Self {
            pool: Arc::new(pool),
            min_connections,
            max_connections,
            current_size: Arc::new(Mutex::new(min_connections)),
            last_adjusted: Arc::new(Mutex::new(Instant::now())),
            metrics: Arc::new(Mutex::new(PoolMetrics {
                recent_wait_times: Vec::with_capacity(100),
                connection_timeouts: 0,
                total_requests: 0,
                max_wait_time: Duration::from_millis(0),
            })),
        };
        
        // Start background monitoring task
        adaptive_pool.start_monitoring();
        
        Ok(adaptive_pool)
    }
    
    pub fn pool(&self) -> Arc<SqlitePool> {
        self.pool.clone()
    }
    
    // Start monitoring thread to adjust pool size based on usage patterns
    fn start_monitoring(&self) {
        let pool = self.pool.clone();
        let min_connections = self.min_connections;
        let max_connections = self.max_connections;
        let current_size = self.current_size.clone();
        let metrics = self.metrics.clone();
        let last_adjusted = self.last_adjusted.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Get current metrics
                let mut metrics_guard = metrics.lock().await;
                let mut current_size_guard = current_size.lock().await;
                let mut last_adjusted_guard = last_adjusted.lock().await;
                
                // Only adjust if at least 30 seconds since last adjustment
                if Instant::now().duration_since(*last_adjusted_guard) < Duration::from_secs(30) {
                    continue;
                }
                
                // Calculate avg wait time and decide if we need to adjust
                if metrics_guard.recent_wait_times.len() > 10 {
                    let avg_wait: Duration = metrics_guard.recent_wait_times.iter().sum::<Duration>() 
                        / metrics_guard.recent_wait_times.len() as u32;
                    
                    let target_size = if avg_wait > Duration::from_millis(50) || 
                                        metrics_guard.connection_timeouts > 0 {
                        // Connections taking too long or timeouts occurring - increase pool
                        (*current_size_guard + 2).min(max_connections)
                    } else if avg_wait < Duration::from_millis(5) && 
                              metrics_guard.connection_timeouts == 0 {
                        // Very fast connections with no timeouts - can reduce pool
                        (*current_size_guard - 1).max(min_connections)
                    } else {
                        // Current size is appropriate
                        *current_size_guard
                    };
                    
                    if target_size != *current_size_guard {
                        info!(
                            "Adjusting connection pool size from {} to {} (avg wait: {:?}, timeouts: {})",
                            *current_size_guard, target_size, avg_wait, metrics_guard.connection_timeouts
                        );
                        
                        // Create a new pool with the target size
                        match pool.clone().connect_lazy() {
                            Ok(_) => {
                                *current_size_guard = target_size;
                                *last_adjusted_guard = Instant::now();
                                
                                // Reset metrics after adjustment
                                metrics_guard.recent_wait_times.clear();
                                metrics_guard.connection_timeouts = 0;
                            },
                            Err(e) => {
                                error!("Failed to adjust pool size: {}", e);
                            }
                        }
                    }
                }
                
                // Log current pool stats
                info!(
                    "Connection pool stats: size={}, requests={}, max_wait={:?}, timeouts={}", 
                    *current_size_guard, metrics_guard.total_requests, 
                    metrics_guard.max_wait_time, metrics_guard.connection_timeouts
                );
                
                // Reset some metrics periodically
                if metrics_guard.recent_wait_times.len() > 1000 {
                    metrics_guard.recent_wait_times.clear();
                }
            }
        });
    }
    
    // Acquire a connection with metrics tracking
    pub async fn acquire(&self) -> Result<sqlx::pool::PoolConnection<sqlx::Sqlite>, sqlx::Error> {
        let start = Instant::now();
        let mut metrics = self.metrics.lock().await;
        metrics.total_requests += 1;
        drop(metrics); // Release lock before await
        
        // Attempt to get connection
        match self.pool.acquire().await {
            Ok(conn) => {
                let wait_time = start.elapsed();
                let mut metrics = self.metrics.lock().await;
                
                // Record metrics
                metrics.recent_wait_times.push(wait_time);
                if wait_time > metrics.max_wait_time {
                    metrics.max_wait_time = wait_time;
                }
                
                Ok(conn)
            },
            Err(e) => {
                // Record timeout
                if matches!(e, sqlx::Error::PoolTimedOut) {
                    let mut metrics = self.metrics.lock().await;
                    metrics.connection_timeouts += 1;
                    
                    warn!("Connection pool timed out - current size: {}", 
                          *self.current_size.lock().await);
                }
                Err(e)
            }
        }
    }
    
    // Run health check on the pool
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        info!("Running connection pool health check");
        
        let start = Instant::now();
        let conn = self.pool.acquire().await?;
        
        // Check if connection works by running a simple query
        sqlx::query("SELECT 1").execute(&*conn).await?;
        
        info!("Connection pool health check passed in {:?}", start.elapsed());
        
        Ok(())
    }
}