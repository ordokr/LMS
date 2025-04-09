use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tracing::{info, warn};
use once_cell::sync::Lazy;

// Global performance counters for continuous monitoring
static BLOCKCHAIN_WRITE_COUNT: AtomicUsize = AtomicUsize::new(0);
static BLOCKCHAIN_READ_COUNT: AtomicUsize = AtomicUsize::new(0);

// Performance metrics with percentile tracking
static METRICS: Lazy<Metrics> = Lazy::new(|| Metrics::new());

pub struct Metrics {
    // Histograms for tracking latency distributions
    blockchain_write_latency: parking_lot::Mutex<hdrhistogram::Histogram<u64>>,
    blockchain_read_latency: parking_lot::Mutex<hdrhistogram::Histogram<u64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            blockchain_write_latency: parking_lot::Mutex::new(
                hdrhistogram::Histogram::new(1).unwrap()
            ),
            blockchain_read_latency: parking_lot::Mutex::new(
                hdrhistogram::Histogram::new(1).unwrap()
            ),
        }
    }
    
    pub fn record_write_latency(&self, duration: Duration) {
        let micros = duration.as_micros() as u64;
        if let Err(e) = self.blockchain_write_latency.lock().record(micros) {
            warn!(event = "metrics_error", error = %e);
        }
        
        BLOCKCHAIN_WRITE_COUNT.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn record_read_latency(&self, duration: Duration) {
        let micros = duration.as_micros() as u64;
        if let Err(e) = self.blockchain_read_latency.lock().record(micros) {
            warn!(event = "metrics_error", error = %e);
        }
        
        BLOCKCHAIN_READ_COUNT.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn report_metrics(&self) {
        let write_hist = self.blockchain_write_latency.lock();
        let read_hist = self.blockchain_read_latency.lock();
        
        info!(
            event = "performance_report",
            write_ops = BLOCKCHAIN_WRITE_COUNT.load(Ordering::Relaxed),
            read_ops = BLOCKCHAIN_READ_COUNT.load(Ordering::Relaxed),
            write_p50_us = write_hist.value_at_percentile(50.0),
            write_p99_us = write_hist.value_at_percentile(99.0),
            read_p50_us = read_hist.value_at_percentile(50.0),
            read_p99_us = read_hist.value_at_percentile(99.0),
            throughput = (BLOCKCHAIN_WRITE_COUNT.load(Ordering::Relaxed) as f64) / 60.0,
        );
    }
}

// Performance measurement helper
#[must_use = "This times a block, so the result should be used"]
pub struct BlockTimer {
    start: Instant,
    operation: Operation,
}

#[derive(Clone, Copy)]
pub enum Operation {
    BlockchainWrite,
    BlockchainRead,
}

impl BlockTimer {
    pub fn new(operation: Operation) -> Self {
        Self {
            start: Instant::now(),
            operation,
        }
    }
    
    pub fn finish(self) {
        let duration = self.start.elapsed();
        
        match self.operation {
            Operation::BlockchainWrite => METRICS.record_write_latency(duration),
            Operation::BlockchainRead => METRICS.record_read_latency(duration),
        }
    }
}

// Start periodic reporting of performance metrics
pub fn start_metrics_reporter() {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            METRICS.report_metrics();
        }
    });
}