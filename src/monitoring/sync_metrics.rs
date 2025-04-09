use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SyncMetrics {
    // Overall statistics
    pub total_syncs_attempted: u64,
    pub total_syncs_succeeded: u64,
    pub total_syncs_failed: u64,
    pub last_sync_attempt: Option<DateTime<Utc>>,
    pub last_successful_sync: Option<DateTime<Utc>>,
    
    // Recent performance
    pub avg_sync_duration_ms: Option<u64>,
    pub recent_error_rate: f64,  // Percentage of failures in the last 100 attempts
    
    // System health
    pub canvas_api_healthy: bool,
    pub discourse_api_healthy: bool,
    pub database_healthy: bool,
}

impl SyncMetrics {
    pub fn new() -> Self {
        Self {
            total_syncs_attempted: 0,
            total_syncs_succeeded: 0,
            total_syncs_failed: 0,
            last_sync_attempt: None,
            last_successful_sync: None,
            avg_sync_duration_ms: None,
            recent_error_rate: 0.0,
            canvas_api_healthy: true,
            discourse_api_healthy: true,
            database_healthy: true,
        }
    }
}

#[derive(Debug)]
struct SyncAttempt {
    timestamp: DateTime<Utc>,
    success: bool,
    duration_ms: u64,
    error_message: Option<String>,
}

pub struct SyncMonitor {
    metrics: Arc<RwLock<SyncMetrics>>,
    recent_attempts: Arc<RwLock<Vec<SyncAttempt>>>,
    max_recent_attempts: usize,
}

impl SyncMonitor {
    pub fn new(max_recent_attempts: usize) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(SyncMetrics::new())),
            recent_attempts: Arc::new(RwLock::new(Vec::with_capacity(max_recent_attempts))),
            max_recent_attempts,
        }
    }
    
    pub async fn record_sync_attempt(&self, success: bool, duration_ms: u64, error_message: Option<String>) {
        let now = Utc::now();
        
        // Record the attempt
        let attempt = SyncAttempt {
            timestamp: now,
            success,
            duration_ms,
            error_message,
        };
        
        // Update recent attempts queue
        {
            let mut attempts = self.recent_attempts.write().await;
            attempts.push(attempt);
            
            // Keep only the most recent attempts
            if attempts.len() > self.max_recent_attempts {
                attempts.remove(0);
            }
        }
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_syncs_attempted += 1;
            metrics.last_sync_attempt = Some(now);
            
            if success {
                metrics.total_syncs_succeeded += 1;
                metrics.last_successful_sync = Some(now);
            } else {
                metrics.total_syncs_failed += 1;
            }
            
            // Calculate average duration
            let attempts = self.recent_attempts.read().await;
            let mut total_duration = 0;
            let mut count = 0;
            
            for attempt in attempts.iter() {
                total_duration += attempt.duration_ms;
                count += 1;
            }
            
            if count > 0 {
                metrics.avg_sync_duration_ms = Some(total_duration / count);
            }
            
            // Calculate recent error rate
            if !attempts.is_empty() {
                let failures = attempts.iter().filter(|a| !a.success).count();
                metrics.recent_error_rate = (failures as f64) / (attempts.len() as f64) * 100.0;
            }
        }
    }
    
    pub async fn update_api_health(&self, canvas_healthy: bool, discourse_healthy: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.canvas_api_healthy = canvas_healthy;
        metrics.discourse_api_healthy = discourse_healthy;
    }
    
    pub async fn update_database_health(&self, healthy: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.database_healthy = healthy;
    }
    
    pub async fn get_metrics(&self) -> SyncMetrics {
        self.metrics.read().await.clone()
    }
    
    pub async fn check_system_health(&self) -> bool {
        let metrics = self.metrics.read().await;
        metrics.canvas_api_healthy && metrics.discourse_api_healthy && metrics.database_healthy
    }
}