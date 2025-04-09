use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use log::{info, debug};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct ProfilerEntry {
    calls: usize,
    total_duration_ns: u128,
    min_duration_ns: u128,
    max_duration_ns: u128,
}

#[derive(Debug, Default)]
pub struct Profiler {
    entries: Mutex<HashMap<String, ProfilerEntry>>,
}

impl Profiler {
    pub fn new() -> Self {
        Self::default()
    }
    
    // Start profiling an operation
    pub fn start(&self, name: &str) -> ProfilerGuard {
        ProfilerGuard {
            name: name.to_string(),
            start_time: Instant::now(),
            profiler: self,
        }
    }
    
    // Record a completed operation
    fn record(&self, name: &str, duration: Duration) {
        let duration_ns = duration.as_nanos();
        
        let mut entries = self.entries.lock().unwrap();
        let entry = entries.entry(name.to_string()).or_insert(ProfilerEntry {
            calls: 0,
            total_duration_ns: 0,
            min_duration_ns: u128::MAX,
            max_duration_ns: 0,
        });
        
        entry.calls += 1;
        entry.total_duration_ns += duration_ns;
        entry.min_duration_ns = entry.min_duration_ns.min(duration_ns);
        entry.max_duration_ns = entry.max_duration_ns.max(duration_ns);
    }
    
    // Print a summary of profiled operations
    pub fn print_summary(&self) {
        let entries = self.entries.lock().unwrap();
        if entries.is_empty() {
            info!("No profiling data available");
            return;
        }
        
        // Sort by total time
        let mut sorted_entries: Vec<_> = entries.iter().collect();
        sorted_entries.sort_by(|a, b| b.1.total_duration_ns.cmp(&a.1.total_duration_ns));
        
        info!("=== Performance Profile Summary ===");
        info!("{:<30} | {:<10} | {:<15} | {:<15} | {:<15}", 
              "Operation", "Calls", "Total (ms)", "Avg (ms)", "Max (ms)");
        info!("{:-<95}", "");
        
        for (name, entry) in sorted_entries {
            let total_ms = entry.total_duration_ns as f64 / 1_000_000.0;
            let avg_ms = if entry.calls > 0 {
                total_ms / entry.calls as f64
            } else {
                0.0
            };
            let max_ms = entry.max_duration_ns as f64 / 1_000_000.0;
            
            info!("{:<30} | {:<10} | {:<15.2} | {:<15.2} | {:<15.2}", 
                 name, entry.calls, total_ms, avg_ms, max_ms);
        }
        
        info!("=== End Profile Summary ===");
    }
    
    // Reset profiling data
    pub fn reset(&self) {
        self.entries.lock().unwrap().clear();
    }
    
    // Export profiling data
    pub fn export(&self) -> String {
        let entries = self.entries.lock().unwrap();
        match serde_json::to_string(&*entries) {
            Ok(json) => json,
            Err(e) => format!("{{\"error\": \"Failed to serialize profile data: {}\"}}", e),
        }
    }
}

// RAII guard for timing operations
pub struct ProfilerGuard<'a> {
    name: String,
    start_time: Instant,
    profiler: &'a Profiler,
}

impl<'a> Drop for ProfilerGuard<'a> {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        self.profiler.record(&self.name, duration);
    }
}

// Global profiler instance
lazy_static::lazy_static! {
    static ref GLOBAL_PROFILER: Profiler = Profiler::new();
}

// Get the global profiler instance
pub fn get_profiler() -> &'static Profiler {
    &GLOBAL_PROFILER
}

// Macro for easy profiling
#[macro_export]
macro_rules! profile {
    ($name:expr) => {
        let _guard = $crate::profiler::get_profiler().start($name);
    };
}