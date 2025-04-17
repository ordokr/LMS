use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use log::{warn, debug};
use serde::{Serialize, Deserialize};

/// Cache for storing analysis results
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AnalysisCache {
    /// Map of file paths to their last modified time and cached analysis result
    file_cache: HashMap<String, (u64, String)>,
    /// Map of directory paths to their cached analysis result
    directory_cache: HashMap<String, String>,
    /// Last time the cache was updated
    last_updated: u64,
}

impl AnalysisCache {
    /// Load the cache from disk
    pub fn load(cache_path: &Path) -> Self {
        if cache_path.exists() {
            match File::open(cache_path) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        if let Ok(cache) = serde_json::from_str(&contents) {
                            return cache;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to open cache file: {}", e);
                }
            }
        }

        Self::default()
    }

    /// Save the cache to disk
    pub fn save(&self, cache_path: &Path) -> Result<(), String> {
        let parent_dir = cache_path.parent().ok_or_else(|| "Invalid cache path".to_string())?;
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }

        let contents = serde_json::to_string(self).map_err(|e| format!("Failed to serialize cache: {}", e))?;
        let mut file = File::create(cache_path).map_err(|e| format!("Failed to create cache file: {}", e))?;
        file.write_all(contents.as_bytes()).map_err(|e| format!("Failed to write cache file: {}", e))?;

        Ok(())
    }

    /// Get a cached file analysis result
    pub fn get_file_cache(&self, file_path: &str, modified_time: u64) -> Option<String> {
        if let Some((cached_time, result)) = self.file_cache.get(file_path) {
            if *cached_time == modified_time {
                return Some(result.clone());
            }
        }
        None
    }

    /// Set a cached file analysis result
    pub fn set_file_cache(&mut self, file_path: String, modified_time: u64, result: String) {
        self.file_cache.insert(file_path, (modified_time, result));
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Get a cached directory analysis result
    pub fn get_directory_cache(&self, dir_path: &str) -> Option<String> {
        self.directory_cache.get(dir_path).cloned()
    }

    /// Set a cached directory analysis result
    pub fn set_directory_cache(&mut self, dir_path: String, result: String) {
        self.directory_cache.insert(dir_path, result);
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.file_cache.clear();
        self.directory_cache.clear();
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }
}

/// Performance metrics for tracking analysis performance
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    /// Map of analyzer names to their execution times
    analyzer_times: HashMap<String, Duration>,
    /// Total execution time
    total_time: Duration,
    /// Number of files processed
    files_processed: usize,
    /// Number of files skipped due to caching
    files_skipped: usize,
    /// Start time of the analysis
    start_time: Option<Instant>,
}

impl PerformanceMetrics {
    /// Create a new PerformanceMetrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Start tracking the total execution time
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Stop tracking the total execution time
    pub fn stop(&mut self) {
        if let Some(start_time) = self.start_time {
            self.total_time = start_time.elapsed();
            self.start_time = None;
        }
    }

    /// Record the execution time for an analyzer
    pub fn record_analyzer_time(&mut self, analyzer_name: &str, duration: Duration) {
        self.analyzer_times.insert(analyzer_name.to_string(), duration);
    }

    /// Increment the number of files processed
    pub fn increment_files_processed(&mut self) {
        self.files_processed += 1;
    }

    /// Increment the number of files skipped
    pub fn increment_files_skipped(&mut self) {
        self.files_skipped += 1;
    }

    /// Get the total execution time
    pub fn total_time(&self) -> Duration {
        self.total_time
    }

    /// Get the execution times for all analyzers
    pub fn analyzer_times(&self) -> &HashMap<String, Duration> {
        &self.analyzer_times
    }

    /// Get the number of files processed
    pub fn files_processed(&self) -> usize {
        self.files_processed
    }

    /// Get the number of files skipped
    pub fn files_skipped(&self) -> usize {
        self.files_skipped
    }

    /// Generate a performance report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# Performance Report\n\n");
        report.push_str(&format!("Total execution time: {:.2?}\n", self.total_time));
        report.push_str(&format!("Files processed: {}\n", self.files_processed));
        report.push_str(&format!("Files skipped (cached): {}\n", self.files_skipped));
        report.push_str("\n## Analyzer Execution Times\n\n");

        let mut analyzer_times: Vec<(&String, &Duration)> = self.analyzer_times.iter().collect();
        analyzer_times.sort_by(|a, b| b.1.cmp(a.1));

        for (analyzer, duration) in analyzer_times {
            report.push_str(&format!("- {}: {:.2?}\n", analyzer, duration));
        }

        report
    }
}

/// Shared performance metrics that can be accessed from multiple threads
pub type SharedPerformanceMetrics = Arc<Mutex<PerformanceMetrics>>;

/// Create a new shared performance metrics instance
pub fn new_shared_metrics() -> SharedPerformanceMetrics {
    Arc::new(Mutex::new(PerformanceMetrics::new()))
}

/// Process files in parallel using Rayon
pub fn process_files_in_parallel<F, T>(
    files: Vec<PathBuf>,
    processor: F,
    metrics: SharedPerformanceMetrics,
) -> Vec<T>
where
    F: Fn(&Path) -> T + Send + Sync,
    T: Send,
{
    files.par_iter().map(|file_path| {
        let result = processor(file_path);

        // Update metrics
        if let Ok(mut metrics) = metrics.lock() {
            metrics.increment_files_processed();
        }

        result
    }).collect()
}

/// Measure the execution time of a function
pub fn measure_execution_time<F, T>(name: &str, f: F, metrics: &mut PerformanceMetrics) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();

    metrics.record_analyzer_time(name, duration);
    debug!("{} took {:.2?}", name, duration);

    result
}

/// Check if a file has been modified since the last analysis
pub fn is_file_modified(file_path: &Path, cache: &AnalysisCache) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(modified_time) = modified.duration_since(std::time::UNIX_EPOCH) {
                let modified_secs = modified_time.as_secs();

                if let Some(_cached_result) = cache.get_file_cache(
                    &file_path.to_string_lossy().to_string(),
                    modified_secs
                ) {
                    return false;
                }
            }
        }
    }

    true
}
