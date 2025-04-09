use anyhow::Result;
use log::{info, error, warn};
use std::time::{Duration, Instant};
use reqwest::Client;

use crate::monitoring::sync_metrics::SyncMonitor;

pub struct ApiHealthCheck {
    client: Client,
    canvas_health_url: String,
    discourse_health_url: String,
    sync_monitor: SyncMonitor,
    timeout: Duration,
}

impl ApiHealthCheck {
    pub fn new(
        canvas_health_url: &str,
        discourse_health_url: &str,
        sync_monitor: SyncMonitor,
        timeout_seconds: u64,
    ) -> Self {
        Self {
            client: Client::new(),
            canvas_health_url: canvas_health_url.to_string(),
            discourse_health_url: discourse_health_url.to_string(),
            sync_monitor,
            timeout: Duration::from_secs(timeout_seconds),
        }
    }
    
    pub async fn check_canvas_health(&self) -> Result<bool> {
        let start = Instant::now();
        
        let result = match tokio::time::timeout(
            self.timeout,
            self.client.get(&self.canvas_health_url).send()
        ).await {
            Ok(response_result) => {
                match response_result {
                    Ok(response) => response.status().is_success(),
                    Err(e) => {
                        warn!("Canvas API health check failed: {}", e);
                        false
                    }
                }
            },
            Err(_) => {
                warn!("Canvas API health check timed out after {} seconds", self.timeout.as_secs());
                false
            }
        };
        
        info!("Canvas API health check: {} (took {:?})", 
            if result { "healthy" } else { "unhealthy" }, 
            start.elapsed()
        );
        
        Ok(result)
    }
    
    pub async fn check_discourse_health(&self) -> Result<bool> {
        let start = Instant::now();
        
        let result = match tokio::time::timeout(
            self.timeout,
            self.client.get(&self.discourse_health_url).send()
        ).await {
            Ok(response_result) => {
                match response_result {
                    Ok(response) => response.status().is_success(),
                    Err(e) => {
                        warn!("Discourse API health check failed: {}", e);
                        false
                    }
                }
            },
            Err(_) => {
                warn!("Discourse API health check timed out after {} seconds", self.timeout.as_secs());
                false
            }
        };
        
        info!("Discourse API health check: {} (took {:?})", 
            if result { "healthy" } else { "unhealthy" }, 
            start.elapsed()
        );
        
        Ok(result)
    }
    
    pub async fn check_database_health(&self, db_pool: &sqlx::Pool<sqlx::Postgres>) -> Result<bool> {
        let start = Instant::now();
        
        let result = match tokio::time::timeout(
            self.timeout,
            sqlx::query("SELECT 1").execute(db_pool)
        ).await {
            Ok(query_result) => {
                match query_result {
                    Ok(_) => true,
                    Err(e) => {
                        warn!("Database health check failed: {}", e);
                        false
                    }
                }
            },
            Err(_) => {
                warn!("Database health check timed out after {} seconds", self.timeout.as_secs());
                false
            }
        };
        
        info!("Database health check: {} (took {:?})", 
            if result { "healthy" } else { "unhealthy" }, 
            start.elapsed()
        );
        
        Ok(result)
    }
    
    pub async fn run_all_health_checks(&self, db_pool: &sqlx::Pool<sqlx::Postgres>) -> Result<bool> {
        let canvas_healthy = self.check_canvas_health().await?;
        let discourse_healthy = self.check_discourse_health().await?;
        let db_healthy = self.check_database_health(db_pool).await?;
        
        // Update health status in the monitor
        self.sync_monitor.update_api_health(canvas_healthy, discourse_healthy).await;
        self.sync_monitor.update_database_health(db_healthy).await;
        
        // System is healthy if all components are healthy
        let all_healthy = canvas_healthy && discourse_healthy && db_healthy;
        
        info!("System health check complete. Overall status: {}", 
            if all_healthy { "HEALTHY" } else { "UNHEALTHY" }
        );
        
        Ok(all_healthy)
    }
    
    pub async fn start_health_check_scheduler(&self, db_pool: sqlx::Pool<sqlx::Postgres>, interval_seconds: u64) {
        let db_pool = db_pool.clone();
        let self_clone = self.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval_seconds));
            
            loop {
                interval.tick().await;
                info!("Running scheduled health checks...");
                
                if let Err(e) = self_clone.run_all_health_checks(&db_pool).await {
                    error!("Error during health checks: {}", e);
                }
            }
        });
        
        info!("Health check scheduler started with interval of {} seconds", interval_seconds);
    }
}

// Make ApiHealthCheck cloneable
impl Clone for ApiHealthCheck {
    fn clone(&self) -> Self {
        Self {
            client: Client::new(), // Create a new client
            canvas_health_url: self.canvas_health_url.clone(),
            discourse_health_url: self.discourse_health_url.clone(),
            sync_monitor: self.sync_monitor.clone(),
            timeout: self.timeout,
        }
    }
}