use std::time::{Duration, Instant};
use hdrhistogram::Histogram;
use std::sync::Arc;
use parking_lot::Mutex;
use tracing::{info, warn};
use serde::Serialize;

pub struct PerformanceMetrics {
    tx_process: Arc<Mutex<Histogram<u64>>>,
    block_creation: Arc<Mutex<Histogram<u64>>>,
    sync_latency: Arc<Mutex<Histogram<u64>>>,
    memory_usage: Arc<Mutex<Vec<(Instant, usize)>>>,
    tx_count: Arc<std::sync::atomic::AtomicUsize>,
    block_count: Arc<std::sync::atomic::AtomicUsize>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            tx_process: Arc::new(Mutex::new(Histogram::<u64>::new(3).unwrap())),
            block_creation: Arc::new(Mutex::new(Histogram::<u64>::new(3).unwrap())),
            sync_latency: Arc::new(Mutex::new(Histogram::<u64>::new(3).unwrap())),
            memory_usage: Arc::new(Mutex::new(Vec::new())),
            tx_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            block_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
    
    pub fn record_tx_process(&self, duration: Duration) {
        if let Err(e) = self.tx_process.lock().record(duration.as_micros() as u64) {
            warn!(event = "metrics_error", category = "tx_process", error = %e);
        }
        self.tx_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn record_block_creation(&self, duration: Duration) {
        if let Err(e) = self.block_creation.lock().record(duration.as_micros() as u64) {
            warn!(event = "metrics_error", category = "block_creation", error = %e);
        }
        self.block_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn record_sync_latency(&self, duration: Duration) {
        if let Err(e) = self.sync_latency.lock().record(duration.as_micros() as u64) {
            warn!(event = "metrics_error", category = "sync_latency", error = %e);
        }
    }
    
    pub fn record_memory_usage(&self, bytes: usize) {
        let mut memory_usage = self.memory_usage.lock();
        let now = Instant::now();
        
        // Add new data point
        memory_usage.push((now, bytes));
        
        // Remove data points older than 1 hour
        let one_hour_ago = now - Duration::from_secs(3600);
        memory_usage.retain(|(time, _)| *time >= one_hour_ago);
    }
    
    pub fn report_metrics(&self) {
        // Transaction processing metrics
        let tx_hist = self.tx_process.lock();
        info!(
            event = "performance_metrics",
            category = "tx_process",
            p50_us = tx_hist.value_at_percentile(50.0),
            p95_us = tx_hist.value_at_percentile(95.0),
            p99_us = tx_hist.value_at_percentile(99.0),
            min_us = tx_hist.min(),
            max_us = tx_hist.max(),
            count = self.tx_count.load(std::sync::atomic::Ordering::Relaxed),
        );
        
        // Block creation metrics
        let block_hist = self.block_creation.lock();
        info!(
            event = "performance_metrics",
            category = "block_creation",
            p50_us = block_hist.value_at_percentile(50.0),
            p95_us = block_hist.value_at_percentile(95.0),
            p99_us = block_hist.value_at_percentile(99.0),
            min_us = block_hist.min(),
            max_us = block_hist.max(),
            count = self.block_count.load(std::sync::atomic::Ordering::Relaxed),
        );
        
        // Sync latency metrics
        let sync_hist = self.sync_latency.lock();
        info!(
            event = "performance_metrics",
            category = "sync_latency",
            p50_us = sync_hist.value_at_percentile(50.0),
            p95_us = sync_hist.value_at_percentile(95.0),
            p99_us = sync_hist.value_at_percentile(99.0),
            min_us = sync_hist.min(),
            max_us = sync_hist.max(),
        );
        
        // Memory usage metrics
        let memory_usage = self.memory_usage.lock();
        if !memory_usage.is_empty() {
            let sum: usize = memory_usage.iter().map(|(_, bytes)| bytes).sum();
            let avg = sum / memory_usage.len();
            let max = memory_usage.iter().map(|(_, bytes)| *bytes).max().unwrap_or(0);
            
            info!(
                event = "performance_metrics",
                category = "memory_usage",
                avg_bytes = avg,
                max_bytes = max,
                samples = memory_usage.len(),
            );
        }
    }
    
    pub fn start_reporting_task(&self, interval_secs: u64) {
        let metrics = self.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
            
            loop {
                interval.tick().await;
                metrics.report_metrics();
            }
        });
    }
    
    // Get a snapshot of current metrics that can be exposed via API
    pub fn get_metrics_snapshot(&self) -> MetricsSnapshot {
        let tx_hist = self.tx_process.lock();
        let block_hist = self.block_creation.lock();
        let sync_hist = self.sync_latency.lock();
        
        let memory_usage = self.memory_usage.lock();
        let avg_memory = if !memory_usage.is_empty() {
            let sum: usize = memory_usage.iter().map(|(_, bytes)| bytes).sum();
            sum / memory_usage.len()
        } else {
            0
        };
        
        MetricsSnapshot {
            transaction_count: self.tx_count.load(std::sync::atomic::Ordering::Relaxed),
            block_count: self.block_count.load(std::sync::atomic::Ordering::Relaxed),
            tx_p99_us: tx_hist.value_at_percentile(99.0),
            block_p99_us: block_hist.value_at_percentile(99.0),
            sync_p99_us: sync_hist.value_at_percentile(99.0),
            avg_memory_bytes: avg_memory,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct MetricsSnapshot {
    pub transaction_count: usize,
    pub block_count: usize,
    pub tx_p99_us: u64,
    pub block_p99_us: u64,
    pub sync_p99_us: u64,
    pub avg_memory_bytes: usize,
    pub timestamp: i64,
}

impl Clone for PerformanceMetrics {
    fn clone(&self) -> Self {
        Self {
            tx_process: Arc::clone(&self.tx_process),
            block_creation: Arc::clone(&self.block_creation),
            sync_latency: Arc::clone(&self.sync_latency),
            memory_usage: Arc::clone(&self.memory_usage),
            tx_count: Arc::clone(&self.tx_count),
            block_count: Arc::clone(&self.block_count),
        }
    }
}

// Performance measurement helper
#[must_use = "This times a block, so the result should be used"]
pub struct BlockTimer {
    start: Instant,
    operation: Operation,
    metrics: Arc<PerformanceMetrics>,
}

#[derive(Clone, Copy)]
pub enum Operation {
    BlockchainWrite,
    BlockchainRead,
    BlockCreation,
    Synchronization,
}

impl BlockTimer {
    pub fn new(operation: Operation, metrics: Arc<PerformanceMetrics>) -> Self {
        Self {
            start: Instant::now(),
            operation,
            metrics,
        }
    }
    
    pub fn finish(self) {
        let duration = self.start.elapsed();
        
        match self.operation {
            Operation::BlockchainWrite => self.metrics.record_tx_process(duration),
            Operation::BlockchainRead => (), // Not currently tracked separately
            Operation::BlockCreation => self.metrics.record_block_creation(duration),
            Operation::Synchronization => self.metrics.record_sync_latency(duration),
        }
    }
}