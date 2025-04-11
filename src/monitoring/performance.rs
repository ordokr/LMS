use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::future::Future;
use anyhow::{Result, Context, anyhow};

use crate::utils::logger::create_logger;
use crate::monitoring::metrics::{MetricsCollector, create_metrics_collector};

/// Configuration options for the performance monitor
#[derive(Debug, Clone)]
pub struct PerformanceMonitorOptions {
    /// Threshold for API response time in milliseconds
    pub api_response_time: u64,
    
    /// Threshold for integration sync time in milliseconds
    pub integration_sync_time: u64,
    
    /// Additional custom thresholds
    pub thresholds: HashMap<String, u64>,
}

impl Default for PerformanceMonitorOptions {
    fn default() -> Self {
        Self {
            api_response_time: 2000, // 2 seconds
            integration_sync_time: 5000, // 5 seconds
            thresholds: HashMap::new(),
        }
    }
}

/// Performance monitor for tracking API and operation performance
pub struct PerformanceMonitor {
    logger: slog::Logger,
    thresholds: PerformanceMonitorOptions,
    metrics: Arc<Mutex<MetricsCollector>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(options: Option<PerformanceMonitorOptions>) -> Self {
        let logger = create_logger("performance-monitor");
        let thresholds = options.unwrap_or_default();
        let metrics = create_metrics_collector();
        
        Self {
            logger,
            thresholds,
            metrics,
        }
    }
    
    /// Monitor an asynchronous API call and log performance metrics
    /// 
    /// # Arguments
    /// 
    /// * `api_name` - Name of the API being called
    /// * `api_call` - The async function to monitor
    /// * `context` - Additional context information (optional)
    /// 
    /// # Returns
    /// 
    /// The result of the API call wrapped in a Result
    pub async fn monitor_api_call<F, Fut, T>(
        &self,
        api_name: &str,
        api_call: F,
        context: Option<HashMap<String, String>>,
    ) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let start_time = Instant::now();
        let mut success = false;
        
        let ctx = context.unwrap_or_default();
        
        let result = match api_call().await {
            Ok(res) => {
                success = true;
                Ok(res)
            },
            Err(err) => {
                slog::error!(
                    self.logger,
                    "API call to {} failed: {}", 
                    api_name, 
                    err; 
                    "context" => format!("{:?}", ctx)
                );
                
                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.increment(&format!("api.{}.error", api_name), 1);
                }
                
                Err(anyhow!("API call to {} failed: {}", api_name, err))
            }
        };
        
        let duration = start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.timing(&format!("api.{}.duration", api_name), duration_ms);
            
            if duration_ms > self.thresholds.api_response_time {
                slog::warn!(
                    self.logger,
                    "API call to {} exceeded threshold", 
                    api_name;
                    "duration" => duration_ms,
                    "threshold" => self.thresholds.api_response_time,
                    "context" => format!("{:?}", ctx)
                );
            }
            
            if success {
                metrics.increment(&format!("api.{}.success", api_name), 1);
            }
        }
        
        slog::debug!(
            self.logger,
            "API call to {} completed", 
            api_name;
            "duration" => duration_ms,
            "success" => success,
            "context" => format!("{:?}", ctx)
        );
        
        result
    }
    
    /// Monitor an integration sync operation
    /// 
    /// # Arguments
    /// 
    /// * `sync_type` - Type of sync operation
    /// * `sync_operation` - The async function to monitor
    /// * `context` - Additional context information (optional)
    /// 
    /// # Returns
    /// 
    /// The result of the sync operation wrapped in a Result
    pub async fn monitor_sync<F, Fut, T>(
        &self,
        sync_type: &str,
        sync_operation: F,
        context: Option<HashMap<String, String>>,
    ) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let start_time = Instant::now();
        let mut success = false;
        
        let ctx = context.unwrap_or_default();
        
        slog::info!(
            self.logger,
            "Starting {} sync operation", 
            sync_type;
            "context" => format!("{:?}", ctx)
        );
        
        let result = match sync_operation().await {
            Ok(res) => {
                success = true;
                Ok(res)
            },
            Err(err) => {
                slog::error!(
                    self.logger,
                    "{} sync operation failed: {}", 
                    sync_type, 
                    err; 
                    "context" => format!("{:?}", ctx)
                );
                
                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.increment(&format!("sync.{}.error", sync_type), 1);
                }
                
                Err(anyhow!("{} sync operation failed: {}", sync_type, err))
            }
        };
        
        let duration = start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.timing(&format!("sync.{}.duration", sync_type), duration_ms);
            
            if duration_ms > self.thresholds.integration_sync_time {
                slog::warn!(
                    self.logger,
                    "{} sync operation exceeded threshold", 
                    sync_type;
                    "duration" => duration_ms,
                    "threshold" => self.thresholds.integration_sync_time,
                    "context" => format!("{:?}", ctx)
                );
            }
            
            if success {
                metrics.increment(&format!("sync.{}.success", sync_type), 1);
            }
        }
        
        slog::info!(
            self.logger,
            "{} sync operation completed", 
            sync_type;
            "duration" => duration_ms,
            "success" => success,
            "context" => format!("{:?}", ctx)
        );
        
        result
    }
    
    /// Get the metrics collector
    pub fn get_metrics(&self) -> Arc<Mutex<MetricsCollector>> {
        self.metrics.clone()
    }
    
    /// Get performance report
    pub fn get_performance_report(&self) -> String {
        if let Ok(metrics) = self.metrics.lock() {
            let timer_metrics = metrics.get_all_metrics().into_iter()
                .filter(|(name, _)| name.contains(".duration"))
                .collect::<HashMap<_, _>>();
            
            let mut report = String::from("=== Performance Report ===\n\n");
            
            for (name, summary) in timer_metrics {
                report.push_str(&format!("{}: {:?}\n", name, summary));
            }
            
            report
        } else {
            String::from("Error: Could not generate performance report - failed to lock metrics")
        }
    }
}

/// Create a default performance monitor
pub fn create_performance_monitor() -> PerformanceMonitor {
    PerformanceMonitor::new(None)
}
