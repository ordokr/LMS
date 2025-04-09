use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use super::app_state::AppStore;

pub struct StateMetrics {
    update_count: AtomicUsize,
    batch_update_count: AtomicUsize,
    observer_count: AtomicUsize,
    last_update_time: std::sync::Mutex<Option<Instant>>,
    update_durations: std::sync::Mutex<Vec<Duration>>,
}

impl StateMetrics {
    pub fn new() -> Self {
        Self {
            update_count: AtomicUsize::new(0),
            batch_update_count: AtomicUsize::new(0),
            observer_count: AtomicUsize::new(0),
            last_update_time: std::sync::Mutex::new(None),
            update_durations: std::sync::Mutex::new(Vec::new()),
        }
    }
    
    pub fn track_update(&self) {
        self.update_count.fetch_add(1, Ordering::Relaxed);
        
        let mut last_time = self.last_update_time.lock().unwrap();
        let now = Instant::now();
        
        if let Some(last) = *last_time {
            let duration = now.duration_since(last);
            let mut durations = self.update_durations.lock().unwrap();
            durations.push(duration);
            
            // Keep last 100 measurements
            if durations.len() > 100 {
                durations.remove(0);
            }
        }
        
        *last_time = Some(now);
    }
    
    pub fn track_batch_update(&self) {
        self.batch_update_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn track_observer(&self, added: bool) {
        if added {
            self.observer_count.fetch_add(1, Ordering::Relaxed);
        } else {
            self.observer_count.fetch_sub(1, Ordering::Relaxed);
        }
    }
    
    pub fn get_metrics(&self) -> AppStateMetricsReport {
        let update_count = self.update_count.load(Ordering::Relaxed);
        let batch_update_count = self.batch_update_count.load(Ordering::Relaxed);
        let observer_count = self.observer_count.load(Ordering::Relaxed);
        
        let durations = self.update_durations.lock().unwrap();
        let avg_update_interval = if durations.len() > 1 {
            let sum: Duration = durations.iter().sum();
            sum / durations.len() as u32
        } else {
            Duration::from_secs(0)
        };
        
        AppStateMetricsReport {
            update_count,
            batch_update_count,
            observer_count,
            avg_update_interval_ms: avg_update_interval.as_millis() as u64,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AppStateMetricsReport {
    pub update_count: usize,
    pub batch_update_count: usize,
    pub observer_count: usize,
    pub avg_update_interval_ms: u64,
}

// Add metrics to AppStore
pub trait StateMonitoring: Sized {
    fn with_monitoring(self) -> (Self, Arc<StateMetrics>);
}

impl StateMonitoring for AppStore {
    fn with_monitoring(self) -> (Self, Arc<StateMetrics>) {
        let metrics = Arc::new(StateMetrics::new());
        
        // Hook up metrics to the AppStore
        // (In a real implementation, you'd modify the AppStore to track metrics)
        
        (self, metrics)
    }
}