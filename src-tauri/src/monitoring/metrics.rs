use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct MetricSummary {
    count: u64,
    min: Duration,
    max: Duration,
    avg: Duration,
    p95: Duration,
    p99: Duration,
}

#[derive(Debug, Clone)]
struct MetricData {
    name: String,
    durations: Vec<Duration>,
    last_summarized: Instant,
}

impl MetricData {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            durations: Vec::new(),
            last_summarized: Instant::now(),
        }
    }
    
    fn add_duration(&mut self, duration: Duration) {
        self.durations.push(duration);
    }
    
    fn summarize(&mut self) -> MetricSummary {
        if self.durations.is_empty() {
            return MetricSummary {
                count: 0,
                min: Duration::from_secs(0),
                max: Duration::from_secs(0),
                avg: Duration::from_secs(0),
                p95: Duration::from_secs(0),
                p99: Duration::from_secs(0),
            };
        }
        
        // Sort for percentile calculation
        self.durations.sort();
        
        let count = self.durations.len() as u64;
        let min = *self.durations.first().unwrap();
        let max = *self.durations.last().unwrap();
        
        // Calculate average
        let total = self.durations.iter().sum::<Duration>();
        let avg = if count > 0 {
            let avg_nanos = total.as_nanos() / count as u128;
            Duration::from_nanos(avg_nanos as u64)
        } else {
            Duration::from_secs(0)
        };
        
        // Calculate percentiles
        let p95_idx = ((count as f64) * 0.95) as usize;
        let p99_idx = ((count as f64) * 0.99) as usize;
        
        let p95 = self.durations[p95_idx.min(count as usize - 1)];
        let p99 = self.durations[p99_idx.min(count as usize - 1)];
        
        self.last_summarized = Instant::now();
        
        // Reset durations to prevent memory growth, but keep most recent
        // This prevents losing all data between reporting cycles
        if self.durations.len() > 1000 {
            let recent = self.durations.split_off(self.durations.len() - 100);
            self.durations = recent;
        }
        
        MetricSummary {
            count,
            min,
            max,
            avg,
            p95,
            p99,
        }
    }
}

pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, MetricData>>>,
    report_interval: Duration,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            report_interval: Duration::from_secs(60), // Report every minute
        }
    }
    
    pub async fn measure<F, T, E>(&self, name: &str, op: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let start = Instant::now();
        let result = op();
        let duration = start.elapsed();
        
        self.record_duration(name, duration).await;
        
        result
    }
    
    pub async fn measure_async<F, Fut, T, E>(&self, name: &str, op: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let start = Instant::now();
        let result = op().await;
        let duration = start.elapsed();
        
        self.record_duration(name, duration).await;
        
        result
    }
    
    async fn record_duration(&self, name: &str, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        
        if let Some(metric) = metrics.get_mut(name) {
            metric.add_duration(duration);
        } else {
            let mut metric = MetricData::new(name);
            metric.add_duration(duration);
            metrics.insert(name.to_string(), metric);
        }
    }
    
    pub fn start_reporting(&self) {
        let metrics = self.metrics.clone();
        let interval = self.report_interval;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                // Generate and log summaries
                let mut metrics_write = metrics.write().await;
                for (name, metric) in metrics_write.iter_mut() {
                    let summary = metric.summarize();
                    
                    // Only log if there's data
                    if summary.count > 0 {
                        log::info!(
                            "Metric: {}, Count: {}, Min: {:?}, Max: {:?}, Avg: {:?}, p95: {:?}, p99: {:?}",
                            name, summary.count, summary.min, summary.max, summary.avg, summary.p95, summary.p99
                        );
                    }
                }
            }
        });
    }
}

// Global metrics collector
pub static METRICS: once_cell::sync::Lazy<MetricsCollector> = once_cell::sync::Lazy::new(|| {
    let collector = MetricsCollector::new();
    
    // Start reporting in a background task
    collector.start_reporting();
    
    collector
});

// Convenience macro for timing operations
#[macro_export]
macro_rules! measure {
    ($name:expr, $op:expr) => {{
        crate::monitoring::metrics::METRICS.measure($name, || $op).await
    }};
}