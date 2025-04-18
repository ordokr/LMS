use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context, anyhow};
use chrono::{DateTime, Utc};

/// Generic file metadata for incremental analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata<T> {
    /// File path relative to the base directory
    pub path: String,
    /// Last modified timestamp (seconds since epoch)
    #[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
    #[serde(bound(serialize = "T: Serialize"))]
    pub last_modified: u64,
    /// File size in bytes
    #[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
    #[serde(bound(serialize = "T: Serialize"))]
    pub size: u64,
    /// Hash of the file content (if enabled)
    #[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
    #[serde(bound(serialize = "T: Serialize"))]
    pub content_hash: Option<String>,
    /// Analysis results for the file
    #[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
    #[serde(bound(serialize = "T: Serialize"))]
    pub results: T,
}

/// Generic analysis cache for incremental analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCache<T> {
    /// When the cache was last updated
    pub last_updated: DateTime<Utc>,
    /// Version of the analyzer that created the cache
    pub analyzer_version: String,
    /// Hash of the analyzer configuration
    pub config_hash: String,
    /// File metadata by path
    #[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
    #[serde(bound(serialize = "T: Serialize"))]
    pub files: HashMap<String, FileMetadata<T>>,
}

impl<T> Default for AnalysisCache<T> where T: Clone {
    fn default() -> Self {
        Self {
            last_updated: Utc::now(),
            analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
            config_hash: String::new(),
            files: HashMap::new(),
        }
    }
}

/// Trait for analyzers that support incremental analysis
pub trait IncrementalAnalyzer<T> where T: Serialize + serde::de::DeserializeOwned + Clone {
    /// Get the base directory for analysis
    fn base_dir(&self) -> &Path;

    /// Get the cache path
    fn cache_path(&self) -> Option<&Path>;

    /// Check if incremental analysis is enabled
    fn use_incremental(&self) -> bool;

    /// Get a hash of the analyzer configuration
    fn config_hash(&self) -> String;

    /// Get the analyzer version
    fn analyzer_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Check if a file should be excluded from analysis
    fn should_exclude_file(&self, file_path: &Path) -> bool;

    /// Analyze a file and return the results
    fn analyze_file(&self, file_path: &Path) -> Result<T>;

    /// Get file metadata for incremental analysis
    fn get_file_metadata(&self, file_path: &Path) -> Result<FileMetadata<T>> {
        // Get file metadata
        let metadata = fs::metadata(file_path)
            .context(format!("Failed to get metadata for file: {}", file_path.display()))?;

        // Get last modified time
        let last_modified = metadata.modified()
            .context("Failed to get modification time")?;

        // Convert to seconds since epoch
        let last_modified_secs = last_modified.duration_since(UNIX_EPOCH)
            .context("Failed to calculate duration since epoch")?;

        // Get file size
        let size = metadata.len();

        // Get relative path
        let relative_path = file_path.strip_prefix(self.base_dir())
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        // Analyze the file
        let results = self.analyze_file(file_path)?;

        Ok(FileMetadata {
            path: relative_path,
            last_modified: last_modified_secs.as_secs(),
            size,
            content_hash: None, // We don't calculate content hash by default
            results,
        })
    }

    /// Check if a file has changed since the last analysis
    fn has_file_changed(&self, file_path: &Path, cache: &AnalysisCache<T>) -> Result<bool> {
        if !self.use_incremental() {
            return Ok(true); // Always analyze if incremental is disabled
        }

        // Get current file metadata
        let metadata = fs::metadata(file_path)
            .context(format!("Failed to get metadata for file: {}", file_path.display()))?;

        // Get last modified time
        let last_modified = metadata.modified()
            .context("Failed to get modification time")?;

        // Convert to seconds since epoch
        let last_modified_secs = last_modified.duration_since(UNIX_EPOCH)
            .context("Failed to calculate duration since epoch")?
            .as_secs();

        // Get file size
        let size = metadata.len();

        // Get relative path
        let relative_path = file_path.strip_prefix(self.base_dir())
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        // Check if the file is in the cache
        if let Some(cached_metadata) = cache.files.get(&relative_path) {
            // Check if the file has changed
            if cached_metadata.last_modified == last_modified_secs &&
               cached_metadata.size == size {
                return Ok(false); // File hasn't changed
            }
        }

        Ok(true) // File has changed or is not in the cache
    }

    /// Load the analysis cache from disk
    fn load_cache(&self) -> Result<AnalysisCache<T>> {
        if !self.use_incremental() {
            return Ok(AnalysisCache::default());
        }

        let cache_path = match self.cache_path() {
            Some(path) => path,
            None => return Ok(AnalysisCache::default()),
        };

        if !cache_path.exists() {
            return Ok(AnalysisCache::default());
        }

        // Read the cache file
        let mut file = File::open(cache_path)
            .context(format!("Failed to open cache file: {}", cache_path.display()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Failed to read cache file")?;

        // Parse the cache
        let cache: AnalysisCache<T> = serde_json::from_str(&contents)
            .context("Failed to parse cache file as JSON")?;

        // Check if the cache is valid
        if cache.analyzer_version != self.analyzer_version() || cache.config_hash != self.config_hash() {
            // Cache is invalid, use a new one
            return Ok(AnalysisCache {
                last_updated: Utc::now(),
                analyzer_version: self.analyzer_version(),
                config_hash: self.config_hash(),
                files: HashMap::new(),
            });
        }

        Ok(cache)
    }

    /// Save the analysis cache to disk
    fn save_cache(&self, cache: &AnalysisCache<T>) -> Result<()> {
        if !self.use_incremental() {
            return Ok(());
        }

        let cache_path = match self.cache_path() {
            Some(path) => path,
            None => return Ok(()),
        };

        // Update the cache metadata
        let updated_cache = AnalysisCache {
            last_updated: Utc::now(),
            analyzer_version: self.analyzer_version(),
            config_hash: self.config_hash(),
            files: cache.files.clone(),
        };

        // Serialize the cache
        let json = serde_json::to_string_pretty(&updated_cache)
            .context("Failed to serialize cache to JSON")?;

        // Create parent directories if they don't exist
        if let Some(parent) = cache_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .context(format!("Failed to create directory: {}", parent.display()))?;
            }
        }

        // Write the cache file
        let mut file = File::create(cache_path)
            .context(format!("Failed to create cache file: {}", cache_path.display()))?;

        file.write_all(json.as_bytes())
            .context("Failed to write cache file")?;

        Ok(())
    }

    /// Analyze files incrementally
    fn analyze_files_incrementally(&self, files: &[PathBuf]) -> Result<Vec<T>> {
        if !self.use_incremental() {
            // If incremental analysis is disabled, just analyze all files
            return self.analyze_files(files);
        }

        // Load the cache
        let mut cache = self.load_cache()?;

        // Collect files that need to be analyzed
        let mut files_to_analyze = Vec::new();
        for file_path in files {
            if self.has_file_changed(file_path, &cache)? {
                files_to_analyze.push(file_path.clone());
            }
        }

        // Analyze files that have changed
        let mut results = Vec::new();
        for file_path in &files_to_analyze {
            match self.get_file_metadata(file_path) {
                Ok(metadata) => {
                    // Add results
                    results.push(metadata.results.clone());

                    // Update the cache
                    cache.files.insert(metadata.path.clone(), metadata);
                },
                Err(e) => eprintln!("Error analyzing file {}: {}", file_path.display(), e),
            }
        }

        // Add results from unchanged files
        for file_path in files {
            let relative_path = file_path.strip_prefix(self.base_dir())
                .unwrap_or(file_path)
                .to_string_lossy()
                .to_string();

            if !files_to_analyze.contains(file_path) {
                if let Some(metadata) = cache.files.get(&relative_path) {
                    results.push(metadata.results.clone());
                }
            }
        }

        // Save the updated cache
        self.save_cache(&cache)?;

        Ok(results)
    }

    /// Analyze files without incremental analysis
    fn analyze_files(&self, files: &[PathBuf]) -> Result<Vec<T>> {
        let mut results = Vec::new();

        for file_path in files {
            if !self.should_exclude_file(file_path) {
                match self.analyze_file(file_path) {
                    Ok(result) => results.push(result),
                    Err(e) => eprintln!("Error analyzing file {}: {}", file_path.display(), e),
                }
            }
        }

        Ok(results)
    }
}

/// Helper function to collect files for analysis
pub fn collect_files_for_analysis(
    base_dir: &Path,
    include_extensions: &[String],
    exclude_dirs: &[String],
    should_exclude_file: impl Fn(&Path) -> bool,
) -> Vec<PathBuf> {
    use walkdir::WalkDir;

    let mut files = Vec::new();

    for entry in WalkDir::new(base_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();

        // Skip excluded files
        if should_exclude_file(file_path) {
            continue;
        }

        // Skip excluded directories
        let dir_name = file_path.parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        if exclude_dirs.iter().any(|d| dir_name.contains(d)) {
            continue;
        }

        // Check file extension
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy().to_string();
            if include_extensions.contains(&ext_str) {
                files.push(file_path.to_path_buf());
            }
        }
    }

    files
}

/// Helper function to calculate a simple hash of a serializable object
pub fn calculate_hash<T: Serialize>(obj: &T) -> String {
    // Create a string representation of the object
    let obj_str = serde_json::to_string(obj).unwrap_or_default();

    // Calculate a simple hash
    let mut hash = 0u64;
    for byte in obj_str.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }

    format!("{:016x}", hash)
}
