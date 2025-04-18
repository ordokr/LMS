use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use regex::Regex;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use rayon::prelude::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use toml;
use anyhow::{Result, anyhow, Context};
use chrono::{self, DateTime, Utc};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Read, Write};
use std::fs::File;

/// Severity level for technical debt items
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TechDebtSeverity {
    /// Critical severity - must be fixed immediately
    Critical,
    /// High severity - should be fixed soon
    High,
    /// Medium severity - should be fixed when possible
    Medium,
    /// Low severity - can be fixed when convenient
    Low,
}

/// Technical debt item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtItem {
    /// File path
    pub file: String,
    /// Line number
    pub line: usize,
    /// Category
    pub category: String,
    /// Description
    pub description: String,
    /// Severity
    pub severity: TechDebtSeverity,
    /// Fix suggestion
    pub fix_suggestion: String,
    /// Impact score (0-100)
    pub impact_score: u8,
    /// Effort to fix (0-100)
    pub effort_to_fix: u8,
    /// Tags
    pub tags: Vec<String>,
}

/// Rule definition for detecting technical debt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDebtRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Pattern to match (regex)
    pub pattern: String,
    /// Default severity
    pub severity: TechDebtSeverity,
    /// Default fix suggestion
    pub fix_suggestion: String,
    /// Default impact score (0-100)
    pub impact_score: u8,
    /// Default effort to fix (0-100)
    pub effort_to_fix: u8,
    /// Tags
    pub tags: Vec<String>,
    /// Whether to match whole file or line by line
    pub whole_file_match: bool,
}

/// Global regex pattern cache to avoid recompiling patterns
static REGEX_CACHE: Lazy<Mutex<HashMap<String, Regex>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Get or create a regex pattern from the cache
fn get_cached_regex(pattern: &str) -> Option<Regex> {
    let mut cache = REGEX_CACHE.lock().unwrap();
    if let Some(regex) = cache.get(pattern) {
        return Some(regex.clone());
    }

    match Regex::new(pattern) {
        Ok(regex) => {
            cache.insert(pattern.to_string(), regex.clone());
            Some(regex)
        },
        Err(e) => {
            eprintln!("Error compiling regex pattern '{}': {}", pattern, e);
            None
        }
    }
}

/// File metadata for incremental analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileMetadata {
    /// File path relative to the base directory
    path: String,
    /// Last modified timestamp (seconds since epoch)
    last_modified: u64,
    /// File size in bytes
    size: u64,
    /// Hash of the file content (if enabled)
    content_hash: Option<String>,
    /// Technical debt items found in the file
    debt_items: Vec<TechDebtItem>,
}

/// Analysis cache for incremental analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnalysisCache {
    /// When the cache was last updated
    last_updated: DateTime<Utc>,
    /// Version of the analyzer that created the cache
    analyzer_version: String,
    /// Rules used for the analysis
    rules_hash: String,
    /// File metadata by path
    files: HashMap<String, FileMetadata>,
}

impl Default for AnalysisCache {
    fn default() -> Self {
        Self {
            last_updated: Utc::now(),
            analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
            rules_hash: String::new(),
            files: HashMap::new(),
        }
    }
}

/// Enhanced technical debt analyzer
#[derive(Clone)]
pub struct EnhancedTechDebtAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    /// Custom rules for detecting technical debt
    rules: Vec<TechDebtRule>,
    /// Directories to exclude from analysis
    exclude_dirs: Vec<String>,
    /// File extensions to analyze
    include_extensions: Vec<String>,
    /// Whether to use parallel processing
    use_parallel: bool,
    /// Whether to use incremental analysis
    use_incremental: bool,
    /// Path to the cache file
    cache_path: Option<PathBuf>,
    /// Analysis cache for incremental analysis
    cache: AnalysisCache,
}

impl EnhancedTechDebtAnalyzer {
    /// Create a new technical debt analyzer with default rules
    pub fn new(base_dir: PathBuf) -> Self {
        let default_rules = Self::default_rules();
        let default_exclude_dirs = vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "build-output".to_string(),
            "dist".to_string(),
        ];
        let default_extensions = vec![
            "rs".to_string(),
            "hs".to_string(),
            "toml".to_string(),
            "md".to_string(),
        ];

        Self {
            base_dir: base_dir.clone(),
            rules: default_rules,
            exclude_dirs: default_exclude_dirs,
            include_extensions: default_extensions,
            use_parallel: true,
            use_incremental: true, // Enable incremental analysis by default
            cache_path: Some(base_dir.join(".tech_debt_cache.json")),
            cache: AnalysisCache::default(),
        }
    }

    /// Enable or disable incremental analysis
    pub fn with_incremental(mut self, use_incremental: bool) -> Self {
        self.use_incremental = use_incremental;
        self
    }

    /// Set the cache file path
    pub fn with_cache_path(mut self, cache_path: PathBuf) -> Self {
        self.cache_path = Some(cache_path);
        self
    }

    /// Calculate a hash of the rules for cache invalidation
    fn calculate_rules_hash(&self) -> String {
        // Create a string representation of the rules
        let rules_str = serde_json::to_string(&self.rules).unwrap_or_default();

        // Calculate a simple hash
        let mut hash = 0u64;
        for byte in rules_str.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }

        format!("{:016x}", hash)
    }

    /// Load the analysis cache from disk
    fn load_cache(&mut self) -> Result<()> {
        if !self.use_incremental {
            return Ok(());
        }

        let cache_path = match &self.cache_path {
            Some(path) => path,
            None => return Ok(()),
        };

        if !cache_path.exists() {
            return Ok(());
        }

        // Read the cache file
        let mut file = File::open(cache_path)
            .context(format!("Failed to open cache file: {}", cache_path.display()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Failed to read cache file")?;

        // Parse the cache
        let cache: AnalysisCache = serde_json::from_str(&contents)
            .context("Failed to parse cache file as JSON")?;

        // Check if the cache is valid
        let current_rules_hash = self.calculate_rules_hash();
        if cache.analyzer_version != env!("CARGO_PKG_VERSION") || cache.rules_hash != current_rules_hash {
            // Cache is invalid, use a new one
            self.cache = AnalysisCache {
                last_updated: Utc::now(),
                analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
                rules_hash: current_rules_hash,
                files: HashMap::new(),
            };
        } else {
            // Cache is valid, update self.cache
            self.cache = cache;
        }

        println!("Loaded analysis cache with {} files", self.cache.files.len());

        Ok(())
    }

    /// Save the analysis cache to disk
    fn save_cache(&self) -> Result<()> {
        if !self.use_incremental {
            return Ok(());
        }

        let cache_path = match &self.cache_path {
            Some(path) => path,
            None => return Ok(()),
        };

        // Create a new cache with updated metadata
        let cache = AnalysisCache {
            last_updated: Utc::now(),
            analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
            rules_hash: self.calculate_rules_hash(),
            files: self.cache.files.clone(),
        };

        // Serialize the cache
        let json = serde_json::to_string_pretty(&cache)
            .context("Failed to serialize cache to JSON")?;

        // Write the cache file
        let mut file = File::create(cache_path)
            .context(format!("Failed to create cache file: {}", cache_path.display()))?;

        file.write_all(json.as_bytes())
            .context("Failed to write cache file")?;

        println!("Saved analysis cache with {} files to {}", cache.files.len(), cache_path.display());

        Ok(())
    }

    /// Get file metadata for incremental analysis
    fn get_file_metadata(&self, file_path: &Path) -> Result<FileMetadata> {
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
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        Ok(FileMetadata {
            path: relative_path,
            last_modified: last_modified_secs.as_secs(),
            size,
            content_hash: None, // We don't calculate content hash by default
            debt_items: Vec::new(),
        })
    }

    /// Check if a file has changed since the last analysis
    fn has_file_changed(&self, file_path: &Path) -> Result<bool> {
        if !self.use_incremental {
            return Ok(true); // Always analyze if incremental is disabled
        }

        // Get current file metadata
        let current_metadata = self.get_file_metadata(file_path)?;

        // Get relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        // Check if the file is in the cache
        if let Some(cached_metadata) = self.cache.files.get(&relative_path) {
            // Check if the file has changed
            if cached_metadata.last_modified == current_metadata.last_modified &&
               cached_metadata.size == current_metadata.size {
                return Ok(false); // File hasn't changed
            }
        }

        Ok(true) // File has changed or is not in the cache
    }

    /// Create default rules for detecting technical debt
    fn default_rules() -> Vec<TechDebtRule> {
        vec![
            // Comment-based markers
            TechDebtRule {
                id: "todo".to_string(),
                name: "TODO Comment".to_string(),
                description: "TODO comment indicating unfinished work".to_string(),
                pattern: r"(?i)//\s*TODO".to_string(),
                severity: TechDebtSeverity::Low,
                fix_suggestion: "Implement the TODO item".to_string(),
                impact_score: 20,
                effort_to_fix: 40,
                tags: vec!["comment".to_string(), "incomplete".to_string()],
                whole_file_match: false,
            },
            TechDebtRule {
                id: "fixme".to_string(),
                name: "FIXME Comment".to_string(),
                description: "FIXME comment indicating broken code".to_string(),
                pattern: r"(?i)//\s*FIXME".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Fix the noted issue".to_string(),
                impact_score: 50,
                effort_to_fix: 40,
                tags: vec!["comment".to_string(), "broken".to_string()],
                whole_file_match: false,
            },
            TechDebtRule {
                id: "hack".to_string(),
                name: "HACK Comment".to_string(),
                description: "HACK comment indicating a workaround".to_string(),
                pattern: r"(?i)//\s*HACK".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Refactor the hack with a proper solution".to_string(),
                impact_score: 60,
                effort_to_fix: 60,
                tags: vec!["comment".to_string(), "workaround".to_string()],
                whole_file_match: false,
            },
            TechDebtRule {
                id: "bug".to_string(),
                name: "BUG Comment".to_string(),
                description: "BUG comment indicating a known bug".to_string(),
                pattern: r"(?i)//\s*BUG".to_string(),
                severity: TechDebtSeverity::Critical,
                fix_suggestion: "Fix the bug".to_string(),
                impact_score: 90,
                effort_to_fix: 50,
                tags: vec!["comment".to_string(), "bug".to_string()],
                whole_file_match: false,
            },

            // Code patterns
            TechDebtRule {
                id: "unwrap".to_string(),
                name: "Unwrap Usage".to_string(),
                description: "Using unwrap() might lead to panics in production".to_string(),
                pattern: r"\.unwrap\(\)".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Replace with proper error handling using match or if let".to_string(),
                impact_score: 70,
                effort_to_fix: 30,
                tags: vec!["error_handling".to_string(), "panic".to_string()],
                whole_file_match: false,
            },
            TechDebtRule {
                id: "expect".to_string(),
                name: "Expect Usage".to_string(),
                description: "Using expect() might lead to panics in production".to_string(),
                pattern: r"\.expect\([^)]+\)".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Replace with proper error handling using match or if let".to_string(),
                impact_score: 65,
                effort_to_fix: 30,
                tags: vec!["error_handling".to_string(), "panic".to_string()],
                whole_file_match: false,
            },
            TechDebtRule {
                id: "panic".to_string(),
                name: "Panic Usage".to_string(),
                description: "Using panic! in production code".to_string(),
                pattern: r"panic!\([^)]+\)".to_string(),
                severity: TechDebtSeverity::High,
                fix_suggestion: "Replace with proper error handling and propagation".to_string(),
                impact_score: 80,
                effort_to_fix: 40,
                tags: vec!["error_handling".to_string(), "panic".to_string()],
                whole_file_match: false,
            },
            TechDebtRule {
                id: "println_debug".to_string(),
                name: "Debug Print".to_string(),
                description: "Debug println in code".to_string(),
                pattern: r"println!\([^)]+\)".to_string(),
                severity: TechDebtSeverity::Low,
                fix_suggestion: "Replace with proper logging or remove".to_string(),
                impact_score: 20,
                effort_to_fix: 10,
                tags: vec!["debugging".to_string()],
                whole_file_match: false,
            },
            TechDebtRule {
                id: "unsafe_block".to_string(),
                name: "Unsafe Code".to_string(),
                description: "Unsafe block used in the file".to_string(),
                pattern: r"unsafe\s*\{".to_string(),
                severity: TechDebtSeverity::High,
                fix_suggestion: "Reconsider if unsafe is necessary and document the safety invariants".to_string(),
                impact_score: 75,
                effort_to_fix: 70,
                tags: vec!["unsafe".to_string(), "security".to_string()],
                whole_file_match: false,
            },

            // Whole file patterns
            TechDebtRule {
                id: "large_function".to_string(),
                name: "Large Function".to_string(),
                description: "File contains very large functions (>300 lines)".to_string(),
                pattern: r"fn\s+\w+[^}]*\{[^}]{300,}\}".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Refactor large functions into smaller, more focused functions".to_string(),
                impact_score: 60,
                effort_to_fix: 60,
                tags: vec!["complexity".to_string(), "maintainability".to_string()],
                whole_file_match: true,
            },
            TechDebtRule {
                id: "large_enum".to_string(),
                name: "Large Enum".to_string(),
                description: "File contains very large enum definitions".to_string(),
                pattern: r"enum\s+\w+\s*\{[^}]{1000,}\}".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Consider splitting the enum or using a different data structure".to_string(),
                impact_score: 50,
                effort_to_fix: 70,
                tags: vec!["complexity".to_string(), "maintainability".to_string()],
                whole_file_match: true,
            },
            TechDebtRule {
                id: "nested_match".to_string(),
                name: "Nested Match".to_string(),
                description: "Nested match statements detected".to_string(),
                pattern: r"match\s+.*\{\s*.*match\s+".to_string(),
                severity: TechDebtSeverity::Low,
                fix_suggestion: "Extract inner match to a separate function or use if let".to_string(),
                impact_score: 40,
                effort_to_fix: 30,
                tags: vec!["complexity".to_string(), "readability".to_string()],
                whole_file_match: true,
            },
            TechDebtRule {
                id: "clone_clone".to_string(),
                name: "Double Clone".to_string(),
                description: "Double clone() call detected".to_string(),
                pattern: r"\.clone\(\)\.clone\(\)".to_string(),
                severity: TechDebtSeverity::Medium,
                fix_suggestion: "Simplify to a single clone or refactor to avoid cloning".to_string(),
                impact_score: 30,
                effort_to_fix: 20,
                tags: vec!["performance".to_string(), "inefficiency".to_string()],
                whole_file_match: false,
            },
        ]
    }

    /// Set custom rules
    pub fn with_rules(mut self, rules: Vec<TechDebtRule>) -> Self {
        self.rules = rules;
        self
    }

    /// Add a custom rule
    pub fn add_rule(&mut self, rule: TechDebtRule) -> &mut Self {
        self.rules.push(rule);
        self
    }

    /// Set directories to exclude
    pub fn with_exclude_dirs(mut self, dirs: Vec<String>) -> Self {
        self.exclude_dirs = dirs;
        self
    }

    /// Set file extensions to include
    pub fn with_include_extensions(mut self, extensions: Vec<String>) -> Self {
        self.include_extensions = extensions;
        self
    }

    /// Enable or disable parallel processing
    pub fn with_parallel(mut self, use_parallel: bool) -> Self {
        self.use_parallel = use_parallel;
        self
    }

    /// Load configuration from a TOML file
    pub fn load_config(&mut self, config_path: &Path) -> Result<()> {
        // Check if config file exists
        if !config_path.exists() {
            return Ok(());
        }

        // Read config file
        let config_content = fs::read_to_string(config_path)
            .context(format!("Failed to read config file: {}", config_path.display()))?;

        // Parse TOML
        let config: toml::Value = toml::from_str(&config_content)
            .context("Failed to parse config file as TOML")?;

        // Extract tech debt analyzer configuration
        if let Some(tech_debt) = config.get("tech_debt_analyzer") {
            // Extract exclude directories
            if let Some(exclude_dirs) = tech_debt.get("exclude_dirs") {
                if let Some(exclude_dirs_array) = exclude_dirs.as_array() {
                    let dirs: Vec<String> = exclude_dirs_array.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect();

                    if !dirs.is_empty() {
                        self.exclude_dirs = dirs;
                        println!("Loaded {} exclude directories from config", self.exclude_dirs.len());
                    }
                }
            }

            // Extract include extensions
            if let Some(include_extensions) = tech_debt.get("include_extensions") {
                if let Some(include_extensions_array) = include_extensions.as_array() {
                    let extensions: Vec<String> = include_extensions_array.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect();

                    if !extensions.is_empty() {
                        self.include_extensions = extensions;
                        println!("Loaded {} include extensions from config", self.include_extensions.len());
                    }
                }
            }

            // Extract parallel processing setting
            if let Some(use_parallel) = tech_debt.get("use_parallel") {
                if let Some(use_parallel_bool) = use_parallel.as_bool() {
                    self.use_parallel = use_parallel_bool;
                    println!("Set parallel processing to: {}", self.use_parallel);
                }
            }

            // Extract incremental analysis setting
            if let Some(use_incremental) = tech_debt.get("use_incremental") {
                if let Some(use_incremental_bool) = use_incremental.as_bool() {
                    self.use_incremental = use_incremental_bool;
                    println!("Set incremental analysis to: {}", self.use_incremental);
                }
            }

            // Extract cache path
            if let Some(cache_path) = tech_debt.get("cache_path") {
                if let Some(cache_path_str) = cache_path.as_str() {
                    self.cache_path = Some(self.base_dir.join(cache_path_str));
                    println!("Set cache path to: {}", self.cache_path.as_ref().unwrap().display());
                }
            }

            // Extract custom rules
            if let Some(custom_rules) = tech_debt.get("custom_rules") {
                if let Some(custom_rules_array) = custom_rules.as_array() {
                    for rule_value in custom_rules_array {
                        if let Some(rule_table) = rule_value.as_table() {
                            // Extract rule fields
                            let id = rule_table.get("id").and_then(|v| v.as_str()).unwrap_or("custom").to_string();
                            let name = rule_table.get("name").and_then(|v| v.as_str()).unwrap_or("Custom Rule").to_string();
                            let description = rule_table.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let pattern = rule_table.get("pattern").and_then(|v| v.as_str());

                            if let Some(pattern_str) = pattern {
                                // Extract severity
                                let severity = match rule_table.get("severity").and_then(|v| v.as_str()) {
                                    Some("critical") => TechDebtSeverity::Critical,
                                    Some("high") => TechDebtSeverity::High,
                                    Some("medium") => TechDebtSeverity::Medium,
                                    Some("low") => TechDebtSeverity::Low,
                                    _ => TechDebtSeverity::Medium,
                                };

                                // Extract fix suggestion
                                let fix_suggestion = rule_table.get("fix_suggestion")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Fix the issue")
                                    .to_string();

                                // Extract impact score
                                let impact_score = rule_table.get("impact_score")
                                    .and_then(|v| v.as_integer())
                                    .unwrap_or(50) as u8;

                                // Extract effort to fix
                                let effort_to_fix = rule_table.get("effort_to_fix")
                                    .and_then(|v| v.as_integer())
                                    .unwrap_or(50) as u8;

                                // Extract tags
                                let tags = rule_table.get("tags")
                                    .and_then(|v| v.as_array())
                                    .map(|arr| arr.iter()
                                        .filter_map(|tag| tag.as_str())
                                        .map(|s| s.to_string())
                                        .collect())
                                    .unwrap_or_else(Vec::new);

                                // Extract whole file match
                                let whole_file_match = rule_table.get("whole_file_match")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(false);

                                // Create and add the rule
                                let rule = TechDebtRule {
                                    id,
                                    name,
                                    description,
                                    pattern: pattern_str.to_string(),
                                    severity,
                                    fix_suggestion,
                                    impact_score,
                                    effort_to_fix,
                                    tags,
                                    whole_file_match,
                                };

                                self.rules.push(rule);
                            }
                        }
                    }

                    println!("Loaded {} custom rules from config", self.rules.len() - Self::default_rules().len());
                }
            }
        }

        Ok(())
    }

    /// Check if a file should be excluded based on patterns
    fn should_exclude_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();

        // Check exclude directories
        for dir in &self.exclude_dirs {
            if path_str.contains(dir) {
                return true;
            }
        }

        // Check file extension
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !self.include_extensions.iter().any(|e| e == &ext_str) {
                return true;
            }
        } else {
            // No extension, exclude
            return true;
        }

        false
    }

    /// Calculate severity score based on multiple factors
    fn calculate_severity_score(&self, rule: &TechDebtRule, content: &str, line: &str) -> TechDebtSeverity {
        // Start with the rule's default severity
        let base_severity = &rule.severity;

        // Adjust based on content and context
        let mut severity_factors = 0;

        // Check for critical keywords that might increase severity
        if line.contains("CRITICAL") || line.contains("SEVERE") || line.contains("URGENT") {
            severity_factors += 2;
        }

        // Check for security-related issues
        if line.contains("SECURITY") || line.contains("VULNERABILITY") || line.contains("EXPLOIT") {
            severity_factors += 2;
        }

        // Check for data-related issues
        if line.contains("DATA LOSS") || line.contains("CORRUPTION") {
            severity_factors += 2;
        }

        // Adjust severity based on factors
        match base_severity {
            TechDebtSeverity::Low => {
                if severity_factors >= 2 {
                    TechDebtSeverity::Medium
                } else {
                    TechDebtSeverity::Low
                }
            },
            TechDebtSeverity::Medium => {
                if severity_factors >= 2 {
                    TechDebtSeverity::High
                } else {
                    TechDebtSeverity::Medium
                }
            },
            TechDebtSeverity::High => {
                if severity_factors >= 2 {
                    TechDebtSeverity::Critical
                } else {
                    TechDebtSeverity::High
                }
            },
            TechDebtSeverity::Critical => TechDebtSeverity::Critical,
        }
    }

    /// Extract comment text from a line
    fn extract_comment(&self, line: &str) -> String {
        if let Some(comment_start) = line.find("//") {
            let comment = line[comment_start + 2..].trim();

            // Remove common markers
            let markers = ["TODO", "FIXME", "HACK", "BUG", "WORKAROUND", "OPTIMIZE", "REFACTOR", "SECURITY"];
            let mut result = comment.to_string();

            for marker in &markers {
                if result.starts_with(marker) {
                    result = result[marker.len()..].trim_start_matches(|c| c == ':' || c == '-' || c == ' ').to_string();
                    break;
                }
            }

            if result.is_empty() {
                "No description provided".to_string()
            } else {
                result
            }
        } else {
            "No description provided".to_string()
        }
    }

    /// Analyze a file for technical debt
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<Vec<TechDebtItem>> {
        // Skip excluded files
        if self.should_exclude_file(file_path) {
            return Ok(Vec::new());
        }

        // Get relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        // Check if we can use cached results
        if self.use_incremental {
            // Check if the file has changed
            if let Ok(has_changed) = self.has_file_changed(file_path) {
                if !has_changed {
                    // Use cached results
                    if let Some(cached_metadata) = self.cache.files.get(&relative_path) {
                        return Ok(cached_metadata.debt_items.clone());
                    }
                }
            }
        }

        // Read file content
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file {}", file_path.display()))?;

        let mut debt_items = Vec::new();

        // Apply whole-file rules first
        for rule in &self.rules {
            if rule.whole_file_match {
                if let Some(regex) = get_cached_regex(&rule.pattern) {
                    if regex.is_match(&content) {
                        // Calculate line number (0 for whole file matches)
                        let line_num = 0;

                        // Calculate severity based on multiple factors
                        let severity = self.calculate_severity_score(rule, &content, "");

                        // Create tech debt item
                        debt_items.push(TechDebtItem {
                            file: relative_path.clone(),
                            line: line_num,
                            category: rule.name.clone(),
                            description: rule.description.clone(),
                            severity,
                            fix_suggestion: rule.fix_suggestion.clone(),
                            impact_score: rule.impact_score,
                            effort_to_fix: rule.effort_to_fix,
                            tags: rule.tags.clone(),
                        });
                    }
                }
            }
        }

        // Apply line-by-line rules
        for (i, line) in content.lines().enumerate() {
            for rule in &self.rules {
                if !rule.whole_file_match {
                    if let Some(regex) = get_cached_regex(&rule.pattern) {
                        if regex.is_match(line) {
                            // Extract description from comment if available
                            let description = if line.contains("//") {
                                self.extract_comment(line)
                            } else {
                                rule.description.clone()
                            };

                            // Calculate severity based on multiple factors
                            let severity = self.calculate_severity_score(rule, &content, line);

                            // Create tech debt item
                            debt_items.push(TechDebtItem {
                                file: relative_path.clone(),
                                line: i + 1,
                                category: rule.name.clone(),
                                description,
                                severity,
                                fix_suggestion: rule.fix_suggestion.clone(),
                                impact_score: rule.impact_score,
                                effort_to_fix: rule.effort_to_fix,
                                tags: rule.tags.clone(),
                            });
                        }
                    }
                }
            }
        }

        // Update the cache if incremental analysis is enabled
        if self.use_incremental {
            // Get file metadata
            if let Ok(mut metadata) = self.get_file_metadata(file_path) {
                // Update debt items
                metadata.debt_items = debt_items.clone();

                // Update the cache
                let mut cache = self.cache.clone();
                cache.files.insert(relative_path, metadata);
                self.cache = cache;
            }
        }

        Ok(debt_items)
    }

    /// Analyze a directory for technical debt
    fn analyze_directory(&mut self, dir_path: &Path, results: &mut Vec<TechDebtItem>) -> Result<()> {
        // Skip excluded directories
        let dir_name = dir_path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
        if self.exclude_dirs.iter().any(|d| dir_name.contains(d)) {
            return Ok(());
        }

        match fs::read_dir(dir_path) {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();

                    if path.is_dir() {
                        self.analyze_directory(&path, results)?;
                    } else if path.is_file() {
                        match self.analyze_file(&path) {
                            Ok(mut debt_items) => results.extend(debt_items),
                            Err(e) => eprintln!("Error analyzing file {}: {}", path.display(), e),
                        }
                    }
                }
            },
            Err(e) => {
                return Err(anyhow!("Failed to read directory {}: {}", dir_path.display(), e));
            }
        }

        Ok(())
    }

    /// Analyze the entire codebase for technical debt
    pub fn analyze_codebase(&mut self) -> Result<Vec<TechDebtItem>> {
        // Load the cache if incremental analysis is enabled
        if self.use_incremental {
            self.load_cache()?;
        }

        let result = if self.use_parallel {
            self.analyze_codebase_parallel()
        } else {
            self.analyze_codebase_sequential()
        };

        // Save the cache if incremental analysis is enabled
        if self.use_incremental {
            self.save_cache()?;
        }

        result
    }

    /// Analyze the codebase sequentially
    fn analyze_codebase_sequential(&mut self) -> Result<Vec<TechDebtItem>> {
        let mut all_debt = Vec::new();

        // Analyze the base directory
        let base_dir = self.base_dir.clone();
        self.analyze_directory(&base_dir, &mut all_debt)?;

        // Sort by severity (higher severity first)
        all_debt.sort_by(|a, b| {
            let a_severity = match a.severity {
                TechDebtSeverity::Critical => 3,
                TechDebtSeverity::High => 2,
                TechDebtSeverity::Medium => 1,
                TechDebtSeverity::Low => 0,
            };

            let b_severity = match b.severity {
                TechDebtSeverity::Critical => 3,
                TechDebtSeverity::High => 2,
                TechDebtSeverity::Medium => 1,
                TechDebtSeverity::Low => 0,
            };

            b_severity.cmp(&a_severity)
        });

        Ok(all_debt)
    }

    /// Analyze the codebase in parallel
    fn analyze_codebase_parallel(&mut self) -> Result<Vec<TechDebtItem>> {
        // Collect all files to analyze
        let mut files_to_analyze = Vec::new();

        // Walk the directory tree to collect files
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_path = entry.path();

            // Skip excluded files
            if !self.should_exclude_file(file_path) {
                // Check if the file has changed (if incremental analysis is enabled)
                if self.use_incremental {
                    match self.has_file_changed(file_path) {
                        Ok(true) => files_to_analyze.push(file_path.to_path_buf()),
                        Ok(false) => {}, // Skip unchanged files
                        Err(_) => files_to_analyze.push(file_path.to_path_buf()), // Analyze on error
                    }
                } else {
                    files_to_analyze.push(file_path.to_path_buf());
                }
            }
        }

        println!("Found {} files to analyze", files_to_analyze.len());

        // Create a thread-safe results collection
        let results = Arc::new(Mutex::new(Vec::new()));
        let cache_mutex = Arc::new(Mutex::new(self.cache.clone()));

        // Process files in parallel
        files_to_analyze.par_iter().for_each(|file_path| {
            // Create a clone of self for each iteration
            let mut analyzer = self.clone();

            match analyzer.analyze_file(file_path) {
                Ok(debt_items) => {
                    if !debt_items.is_empty() {
                        let mut results_guard = results.lock().unwrap();
                        results_guard.extend(debt_items.clone());
                    }
                },
                Err(e) => eprintln!("Error analyzing file {}: {}", file_path.display(), e),
            }
        });

        // Get the results
        let mut all_debt = results.lock().unwrap().clone();

        // We can't update self.cache since self is immutable

        // Add cached results for unchanged files
        if self.use_incremental {
            for (_, metadata) in &self.cache.files {
                all_debt.extend(metadata.debt_items.clone());
            }
        }

        // Sort by severity (higher severity first)
        all_debt.sort_by(|a, b| {
            let a_severity = match a.severity {
                TechDebtSeverity::Critical => 3,
                TechDebtSeverity::High => 2,
                TechDebtSeverity::Medium => 1,
                TechDebtSeverity::Low => 0,
            };

            let b_severity = match b.severity {
                TechDebtSeverity::Critical => 3,
                TechDebtSeverity::High => 2,
                TechDebtSeverity::Medium => 1,
                TechDebtSeverity::Low => 0,
            };

            b_severity.cmp(&a_severity)
        });

        // Remove duplicates (same file and line)
        let mut unique_debt = Vec::new();
        let mut seen = HashMap::new();

        for item in all_debt {
            let key = format!("{}-{}", item.file, item.line);
            if !seen.contains_key(&key) {
                seen.insert(key, true);
                unique_debt.push(item);
            }
        }

        Ok(unique_debt)
    }

    /// Generate a technical debt report in Markdown format
    pub fn generate_report(&mut self) -> Result<String> {
        let debt_items = self.analyze_codebase()?;

        if debt_items.is_empty() {
            return Ok("# Technical Debt Report\n\nNo technical debt items found. Great job!".to_string());
        }

        let mut report = String::new();

        // Header
        report.push_str("# Technical Debt Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", chrono::Local::now().format("%Y-%m-%d")));

        // Summary
        report.push_str("## Summary\n\n");

        let critical = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Critical)).count();
        let high = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::High)).count();
        let medium = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Medium)).count();
        let low = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Low)).count();

        report.push_str("| Severity | Count |\n");
        report.push_str("|----------|-------|\n");
        report.push_str(&format!("| 🔴 Critical | {} |\n", critical));
        report.push_str(&format!("| 🟠 High | {} |\n", high));
        report.push_str(&format!("| 🟡 Medium | {} |\n", medium));
        report.push_str(&format!("| 🟢 Low | {} |\n", low));
        report.push_str(&format!("| **Total** | **{}** |\n", debt_items.len()));

        report.push_str("\n");

        // Items by category
        report.push_str("## Items by Category\n\n");

        let mut categories = HashMap::new();
        for item in &debt_items {
            categories.entry(item.category.clone()).or_insert_with(Vec::new).push(item);
        }

        let mut category_names: Vec<String> = categories.keys().cloned().collect();
        category_names.sort();

        for category in &category_names {
            let items = categories.get(category).unwrap();
            report.push_str(&format!("### {}\n\n", category));

            report.push_str("| File | Line | Severity | Description | Suggestion | Impact | Effort |\n");
            report.push_str("|------|------|----------|-------------|------------|--------|--------|\n");

            for item in items {
                let severity_icon = match item.severity {
                    TechDebtSeverity::Critical => "🔴",
                    TechDebtSeverity::High => "🟠",
                    TechDebtSeverity::Medium => "🟡",
                    TechDebtSeverity::Low => "🟢",
                };

                report.push_str(&format!("| `{}` | {} | {} | {} | {} | {} | {} |\n",
                    item.file,
                    if item.line == 0 { "N/A".to_string() } else { item.line.to_string() },
                    severity_icon,
                    item.description,
                    item.fix_suggestion,
                    item.impact_score,
                    item.effort_to_fix
                ));
            }

            report.push_str("\n");
        }

        // Highest severity items
        report.push_str("## Critical and High Severity Items\n\n");

        let high_severity_items: Vec<&TechDebtItem> = debt_items.iter()
            .filter(|item| matches!(item.severity, TechDebtSeverity::Critical | TechDebtSeverity::High))
            .collect();

        if high_severity_items.is_empty() {
            report.push_str("No critical or high severity items found.\n\n");
        } else {
            report.push_str("| File | Line | Category | Description | Suggestion | Impact | Effort |\n");
            report.push_str("|------|------|----------|-------------|------------|--------|--------|\n");

            for item in high_severity_items {
                let severity_icon = match item.severity {
                    TechDebtSeverity::Critical => "🔴",
                    TechDebtSeverity::High => "🟠",
                    _ => "",
                };

                report.push_str(&format!("| `{}` | {} | {} {} | {} | {} | {} | {} |\n",
                    item.file,
                    if item.line == 0 { "N/A".to_string() } else { item.line.to_string() },
                    severity_icon,
                    item.category,
                    item.description,
                    item.fix_suggestion,
                    item.impact_score,
                    item.effort_to_fix
                ));
            }

            report.push_str("\n");
        }

        // Hotspots (files with most tech debt)
        report.push_str("## Technical Debt Hotspots\n\n");

        let mut file_counts = HashMap::new();
        for item in &debt_items {
            *file_counts.entry(item.file.clone()).or_insert(0) += 1;
        }

        let mut files: Vec<(String, usize)> = file_counts.into_iter().collect();
        files.sort_by(|a, b| b.1.cmp(&a.1));

        report.push_str("| File | Debt Items |\n");
        report.push_str("|------|------------|\n");

        for (file, count) in files.iter().take(10) {
            report.push_str(&format!("| `{}` | {} |\n", file, count));
        }

        report.push_str("\n");

        // Tags analysis
        report.push_str("## Technical Debt by Tag\n\n");

        let mut tags_count = HashMap::new();
        for item in &debt_items {
            for tag in &item.tags {
                *tags_count.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        let mut tags: Vec<(String, usize)> = tags_count.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1));

        report.push_str("| Tag | Count |\n");
        report.push_str("|-----|-------|\n");

        for (tag, count) in tags {
            report.push_str(&format!("| {} | {} |\n", tag, count));
        }

        report.push_str("\n");

        // Recommendations
        report.push_str("## Recommendations\n\n");

        if critical > 0 {
            report.push_str("1. **Address Critical Issues First**: Focus on resolving the critical issues that could lead to severe bugs or security vulnerabilities.\n");
        }

        if high > 0 {
            report.push_str("2. **Tackle High Severity Items**: Address high severity items which can improve code quality significantly.\n");
        }

        let top_categories: Vec<&String> = category_names.iter()
            .filter(|c| {
                let items = categories.get(*c).unwrap();
                items.len() >= 5
            })
            .collect();

        if !top_categories.is_empty() {
            report.push_str("3. **Focus on Common Categories**: Address these common issues:\n");
            for category in top_categories {
                report.push_str(&format!("   - {}: {} instances\n",
                    category,
                    categories.get(category).unwrap().len()));
            }
        }

        // Impact vs. Effort Analysis
        report.push_str("\n## Impact vs. Effort Analysis\n\n");
        report.push_str("The following items have high impact but relatively low effort to fix (high ROI):\n\n");

        let high_roi_items: Vec<&TechDebtItem> = debt_items.iter()
            .filter(|item| item.impact_score > 60 && item.effort_to_fix < 40)
            .collect();

        if high_roi_items.is_empty() {
            report.push_str("No high ROI items found.\n\n");
        } else {
            report.push_str("| File | Line | Category | Description | Impact | Effort |\n");
            report.push_str("|------|------|----------|-------------|--------|--------|\n");

            for item in high_roi_items {
                report.push_str(&format!("| `{}` | {} | {} | {} | {} | {} |\n",
                    item.file,
                    if item.line == 0 { "N/A".to_string() } else { item.line.to_string() },
                    item.category,
                    item.description,
                    item.impact_score,
                    item.effort_to_fix
                ));
            }
        }

        Ok(report)
    }

    /// Export technical debt items to JSON
    pub fn export_to_json(&mut self) -> Result<String> {
        let debt_items = self.analyze_codebase()?;

        // Create a summary object
        let critical = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Critical)).count();
        let high = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::High)).count();
        let medium = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Medium)).count();
        let low = debt_items.iter().filter(|item| matches!(item.severity, TechDebtSeverity::Low)).count();

        let summary = serde_json::json!({
            "total": debt_items.len(),
            "critical": critical,
            "high": high,
            "medium": medium,
            "low": low,
            "generated_at": chrono::Local::now().to_rfc3339(),
        });

        // Create the full export object
        let export = serde_json::json!({
            "summary": summary,
            "items": debt_items,
        });

        // Convert to JSON string
        let json_string = serde_json::to_string_pretty(&export)
            .context("Failed to serialize tech debt items to JSON")?;

        Ok(json_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};
    use std::io::Write;
    use std::fs::File;

    fn create_test_file(dir: &Path, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.join(filename);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", content).unwrap();
        file_path
    }

    fn setup_test_directory() -> (TempDir, EnhancedTechDebtAnalyzer) {
        let dir = tempdir().unwrap();
        let analyzer = EnhancedTechDebtAnalyzer::new(dir.path().to_path_buf());
        (dir, analyzer)
    }

    #[test]
    fn test_analyze_file_with_todos() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        let file_path = create_test_file(
            dir.path(),
            "todo_file.rs",
            r#"
            // TODO: Fix this function
            fn broken_function() {
                // FIXME: This is not implemented yet
                unimplemented!();
            }
            "#
        );

        let result = analyzer.analyze_file(&file_path)?;

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|item| item.category == "TODO Comment"));
        assert!(result.iter().any(|item| item.category == "FIXME Comment"));

        Ok(())
    }

    #[test]
    fn test_analyze_file_with_unwraps() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        let file_path = create_test_file(
            dir.path(),
            "unwraps.rs",
            r#"
            fn unsafe_function() -> Option<i32> {
                let result = Some(42);
                result.unwrap() // This should be detected
            }
            "#
        );

        let result = analyzer.analyze_file(&file_path)?;

        assert!(result.iter().any(|item| item.category == "Unwrap Usage"));

        Ok(())
    }

    #[test]
    fn test_severity_scoring() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        let file_path = create_test_file(
            dir.path(),
            "severity.rs",
            r#"
            // TODO: Normal todo
            fn normal_todo() {}

            // TODO: CRITICAL security issue
            fn critical_todo() {}

            // FIXME: SECURITY vulnerability
            fn security_fixme() {}
            "#
        );

        let result = analyzer.analyze_file(&file_path)?;

        // Find the normal TODO
        let normal_todo = result.iter().find(|item|
            item.category == "TODO Comment" && item.description.contains("Normal todo"));

        // Find the critical TODO
        let critical_todo = result.iter().find(|item|
            item.category == "TODO Comment" && item.description.contains("CRITICAL security"));

        // Find the security FIXME
        let security_fixme = result.iter().find(|item|
            item.category == "FIXME Comment" && item.description.contains("SECURITY vulnerability"));

        assert!(normal_todo.is_some());
        assert!(critical_todo.is_some());
        assert!(security_fixme.is_some());

        // Check that severity was adjusted correctly
        assert!(matches!(normal_todo.unwrap().severity, TechDebtSeverity::Low));
        assert!(matches!(critical_todo.unwrap().severity, TechDebtSeverity::Medium));
        assert!(matches!(security_fixme.unwrap().severity, TechDebtSeverity::High));

        Ok(())
    }

    #[test]
    fn test_custom_rules() -> Result<()> {
        let (dir, mut analyzer) = setup_test_directory();

        // Add a custom rule
        let custom_rule = TechDebtRule {
            id: "custom_rule".to_string(),
            name: "Custom Rule".to_string(),
            description: "Custom rule for testing".to_string(),
            pattern: r"CUSTOM_PATTERN".to_string(),
            severity: TechDebtSeverity::High,
            fix_suggestion: "Fix the custom pattern".to_string(),
            impact_score: 80,
            effort_to_fix: 20,
            tags: vec!["custom".to_string(), "test".to_string()],
            whole_file_match: false,
        };

        analyzer.add_rule(custom_rule);

        let file_path = create_test_file(
            dir.path(),
            "custom_rule.rs",
            r#"
            fn test_function() {
                // This has a CUSTOM_PATTERN that should be detected
                println!("Hello, world!");
            }
            "#
        );

        let result = analyzer.analyze_file(&file_path)?;

        assert!(result.iter().any(|item| item.category == "Custom Rule"));

        Ok(())
    }

    #[test]
    fn test_load_config() -> Result<()> {
        let (dir, mut analyzer) = setup_test_directory();

        // Create a config file
        let config_content = r#"
        [tech_debt_analyzer]
        exclude_dirs = ["test_exclude", "another_exclude"]
        include_extensions = ["rs", "hs", "toml"]
        use_parallel = true

        [[tech_debt_analyzer.custom_rules]]
        id = "config_rule"
        name = "Config Rule"
        description = "Rule loaded from config"
        pattern = "CONFIG_PATTERN"
        severity = "high"
        fix_suggestion = "Fix the config pattern"
        impact_score = 70
        effort_to_fix = 30
        tags = ["config", "test"]
        whole_file_match = false
        "#;

        let config_path = create_test_file(dir.path(), "config.toml", config_content);

        // Load the config
        analyzer.load_config(&config_path)?;

        // Check that the config was loaded correctly
        assert!(analyzer.exclude_dirs.contains(&"test_exclude".to_string()));
        assert!(analyzer.exclude_dirs.contains(&"another_exclude".to_string()));

        // Create a file with the custom pattern
        let file_path = create_test_file(
            dir.path(),
            "config_rule.rs",
            r#"
            fn test_function() {
                // This has a CONFIG_PATTERN that should be detected
                println!("Hello, world!");
            }
            "#
        );

        let result = analyzer.analyze_file(&file_path)?;

        assert!(result.iter().any(|item| item.category == "Config Rule"));

        Ok(())
    }

    #[test]
    fn test_incremental_analysis() -> Result<()> {
        let (dir, mut analyzer) = setup_test_directory();

        // Enable incremental analysis
        analyzer = analyzer.with_incremental(true);

        // Set the cache path
        let cache_path = dir.path().join(".tech_debt_cache.json");
        analyzer = analyzer.with_cache_path(cache_path.clone());

        // Create a file with a TODO
        let file_path = create_test_file(
            dir.path(),
            "todo_file.rs",
            r#"
            // TODO: Fix this function
            fn broken_function() {
                // This is a test
                println!("Hello, world!");
            }
            "#
        );

        // First analysis - should analyze the file
        let result1 = analyzer.analyze_codebase()?;

        // Check that the TODO was detected
        assert!(result1.iter().any(|item| item.category == "TODO Comment"));

        // Check that the cache file was created
        assert!(cache_path.exists());

        // Create a new analyzer with the same cache path
        let mut analyzer2 = EnhancedTechDebtAnalyzer::new(dir.path().to_path_buf())
            .with_incremental(true)
            .with_cache_path(cache_path);

        // Second analysis - should use the cache
        let result2 = analyzer2.analyze_codebase()?;

        // Check that the TODO was still detected
        assert!(result2.iter().any(|item| item.category == "TODO Comment"));

        // Modify the file
        let file_path = create_test_file(
            dir.path(),
            "todo_file.rs",
            r#"
            // TODO: Fix this function
            // FIXME: This is a new issue
            fn broken_function() {
                // This is a test
                println!("Hello, world!");
            }
            "#
        );

        // Third analysis - should analyze the modified file
        let result3 = analyzer2.analyze_codebase()?;

        // Check that both the TODO and FIXME were detected
        assert!(result3.iter().any(|item| item.category == "TODO Comment"));
        assert!(result3.iter().any(|item| item.category == "FIXME Comment"));

        Ok(())
    }
}
