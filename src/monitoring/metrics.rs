use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt};

use crate::utils::logger::create_logger;

/// Types of metrics supported by the metrics collector
#[derive(Debug, Clone)]
pub enum MetricValue {
    /// Counter metrics track incrementing/decrementing values
    Counter(i64),
    
    /// Gauge metrics track values that can go up or down
    Gauge(f64),
    
    /// Timer metrics track durations of operations
    Timer(Vec<u64>),
}

/// Summary of a metric for reporting
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum MetricSummary {
    #[serde(rename = "counter")]
    Counter { value: i64 },
    
    #[serde(rename = "gauge")]
    Gauge { value: f64 },
    
    #[serde(rename = "timer")]
    Timer {
        count: usize,
        min: u64,
        max: u64,
        avg: f64,
    },
}

/// Metrics collector for the application
pub struct MetricsCollector {
    metrics: HashMap<String, MetricValue>,
    counters: HashMap<String, i64>,
    timers: HashMap<String, Vec<u64>>,
    gauges: HashMap<String, f64>,
    logger: slog::Logger,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        let logger = create_logger("metrics-collector");
        
        let mut collector = Self {
            metrics: HashMap::new(),
            counters: HashMap::new(),
            timers: HashMap::new(),
            gauges: HashMap::new(),
            logger,
        };
        
        collector.initialize_system_metrics();
        collector
    }
    
    /// Initialize system metrics with baseline values
    fn initialize_system_metrics(&mut self) {
        // Get system information
        let mut system = System::new_all();
        system.refresh_all();
        
        // Add system metrics
        self.gauge("system.memory.free", system.available_memory() as f64);
        self.gauge("system.memory.total", system.total_memory() as f64);
        
        // Load average might not be available on all platforms
        if let Some(load_avg) = system.load_average().one {
            self.gauge("system.load.1m", load_avg);
        }
        
        // Add sample API metrics for demonstration
        self.timing("api.canvas.courses.get", 245);
        self.timing("api.canvas.announcements.list", 189);
        self.timing("api.discourse.topics.create", 320);
        self.timing("integration.announcements-sync", 550);
        
        self.increment("api.success.count", 42);
        self.increment("api.error.count", 3);
    }
    
    /// Increment a counter metric
    pub fn increment(&mut self, metric_name: &str, value: i64) -> i64 {
        let current = *self.counters.get(metric_name).unwrap_or(&0);
        let new_value = current + value;
        self.counters.insert(metric_name.to_string(), new_value);
        new_value
    }
    
    /// Decrement a counter metric
    pub fn decrement(&mut self, metric_name: &str, value: i64) -> i64 {
        let current = *self.counters.get(metric_name).unwrap_or(&0);
        let new_value = current - value;
        self.counters.insert(metric_name.to_string(), new_value);
        new_value
    }
    
    /// Set a gauge metric
    pub fn gauge(&mut self, metric_name: &str, value: f64) -> f64 {
        self.gauges.insert(metric_name.to_string(), value);
        value
    }
    
    /// Record a timing metric
    pub fn timing(&mut self, metric_name: &str, time_ms: u64) -> u64 {
        let timings = self.timers.entry(metric_name.to_string()).or_insert_with(Vec::new);
        timings.push(time_ms);
        
        // Keep only the last 100 timings
        if timings.len() > 100 {
            timings.remove(0);
        }
        
        time_ms
    }
    
    /// Start timing an operation
    pub fn start_timer(&self, metric_name: &str) -> Timer {
        Timer::new(metric_name.to_string())
    }
    
    /// Get a summary of a specific metric
    pub fn get_metric_summary(&self, metric_name: &str) -> Option<MetricSummary> {
        if let Some(value) = self.counters.get(metric_name) {
            return Some(MetricSummary::Counter { value: *value });
        }
        
        if let Some(value) = self.gauges.get(metric_name) {
            return Some(MetricSummary::Gauge { value: *value });
        }
        
        if let Some(timings) = self.timers.get(metric_name) {
            if timings.is_empty() {
                return None;
            }
            
            let min = *timings.iter().min().unwrap();
            let max = *timings.iter().max().unwrap();
            let sum: u64 = timings.iter().sum();
            let avg = sum as f64 / timings.len() as f64;
            
            return Some(MetricSummary::Timer {
                count: timings.len(),
                min,
                max,
                avg,
            });
        }
        
        None
    }
    
    /// Get all metrics
    pub fn get_all_metrics(&self) -> HashMap<String, MetricSummary> {
        let mut metrics = HashMap::new();
        
        // Add counters
        for (name, value) in &self.counters {
            metrics.insert(name.clone(), MetricSummary::Counter { value: *value });
        }
        
        // Add gauges
        for (name, value) in &self.gauges {
            metrics.insert(name.clone(), MetricSummary::Gauge { value: *value });
        }
        
        // Add timers
        for (name, timings) in &self.timers {
            if !timings.is_empty() {
                let min = *timings.iter().min().unwrap();
                let max = *timings.iter().max().unwrap();
                let sum: u64 = timings.iter().sum();
                let avg = sum as f64 / timings.len() as f64;
                
                metrics.insert(name.clone(), MetricSummary::Timer {
                    count: timings.len(),
                    min,
                    max,
                    avg,
                });
            }
        }
        
        metrics
    }
    
    /// Get a formatted report of all metrics
    pub fn get_metrics_report(&self) -> String {
        let metrics = self.get_all_metrics();
        let mut report = String::new();
        
        report.push_str("=== Metrics Report ===\n\n");
        
        // Counter metrics
        report.push_str("== Counters ==\n");
        for (name, summary) in &metrics {
            if let MetricSummary::Counter { value } = summary {
                report.push_str(&format!("{}: {}\n", name, value));
            }
        }
        
        // Gauge metrics
        report.push_str("\n== Gauges ==\n");
        for (name, summary) in &metrics {
            if let MetricSummary::Gauge { value } = summary {
                report.push_str(&format!("{}: {:.2}\n", name, value));
            }
        }
        
        // Timer metrics
        report.push_str("\n== Timers ==\n");
        for (name, summary) in &metrics {
            if let MetricSummary::Timer { count, min, max, avg } = summary {
                report.push_str(&format!("{}: count={}, min={}ms, max={}ms, avg={:.2}ms\n",
                    name, count, min, max, avg));
            }
        }
        
        report
    }
}

/// Default implementation for MetricsCollector
impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer for measuring operation duration
pub struct Timer {
    name: String,
    start: Instant,
}

impl Timer {
    /// Create a new timer
    pub fn new(name: String) -> Self {
        Self {
            name,
            start: Instant::now(),
        }
    }
    
    /// Stop the timer and record the duration
    pub fn stop(self, metrics: &mut MetricsCollector) -> u64 {
        let duration = self.start.elapsed();
        let ms = duration.as_millis() as u64;
        metrics.timing(&self.name, ms);
        ms
    }
}

/// Create a thread-safe metrics collector
pub fn create_metrics_collector() -> Arc<Mutex<MetricsCollector>> {
    Arc::new(Mutex::new(MetricsCollector::new()))
}

/// Get and return all metrics as JSON
pub fn get_metrics_json(metrics: &Arc<Mutex<MetricsCollector>>) -> String {
    match metrics.lock() {
        Ok(metrics) => {
            let all_metrics = metrics.get_all_metrics();
            match serde_json::to_string_pretty(&all_metrics) {
                Ok(json) => json,
                Err(_) => String::from("{\"error\": \"Failed to serialize metrics\"}"),
            }
        },
        Err(_) => String::from("{\"error\": \"Failed to acquire metrics lock\"}"),
    }
}
