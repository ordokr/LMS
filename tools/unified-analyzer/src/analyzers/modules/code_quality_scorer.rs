use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow, Context};
use walkdir::WalkDir;
use regex::Regex;
use thiserror::Error;

/// Custom error type for code quality scoring operations
#[derive(Error, Debug)]
pub enum CodeQualityError {
    /// Error reading file
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    /// Error parsing regex
    #[error("Failed to parse regex: {0}")]
    RegexError(#[from] regex::Error),

    /// Error serializing to JSON
    #[error("Failed to serialize to JSON: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// General error
    #[error("{0}")]
    General(String),
}

/// Represents code metrics for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    /// File path
    pub file_path: String,
    /// Lines of code
    pub loc: usize,
    /// Cyclomatic complexity
    pub complexity: u32,
    /// Comment coverage (percentage)
    pub comment_coverage: f32,
    /// Cohesion score (0.0 to 1.0)
    pub cohesion: f32,
    /// Overall usefulness score (0 to 100)
    pub usefulness_score: u8,
    /// Recommendation (reuse, partial, rebuild)
    pub recommendation: String,
}

/// Cache entry for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileCache {
    /// Last modified time
    last_modified: u64,
    /// Code metrics
    metrics: CodeMetrics,
}

/// Code Quality Scorer for analyzing code quality and usefulness
pub struct CodeQualityScorer {
    /// Code metrics by file path
    metrics: HashMap<String, CodeMetrics>,
    /// Thresholds for usefulness scores
    usefulness_threshold_high: u8,
    usefulness_threshold_medium: u8,
    /// Weights for different metrics
    complexity_weight: f32,
    loc_weight: f32,
    comment_coverage_weight: f32,
    cohesion_weight: f32,
    /// File cache to avoid re-analyzing unchanged files
    file_cache: HashMap<String, FileCache>,
    /// Whether to use caching
    use_cache: bool,
}

impl CodeQualityScorer {
    /// Create a new CodeQualityScorer with default settings
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            usefulness_threshold_high: 80,
            usefulness_threshold_medium: 50,
            complexity_weight: 0.4,
            loc_weight: 0.2,
            comment_coverage_weight: 0.2,
            cohesion_weight: 0.2,
            file_cache: HashMap::new(),
            use_cache: true,
        }
    }

    /// Create a new CodeQualityScorer with caching disabled
    pub fn new_without_cache() -> Self {
        let mut scorer = Self::new();
        scorer.use_cache = false;
        scorer
    }

    /// Create a new CodeQualityScorer with custom settings
    pub fn with_config(
        usefulness_threshold_high: u8,
        usefulness_threshold_medium: u8,
        complexity_weight: f32,
        loc_weight: f32,
        comment_coverage_weight: f32,
        cohesion_weight: f32,
    ) -> Self {
        Self {
            metrics: HashMap::new(),
            usefulness_threshold_high,
            usefulness_threshold_medium,
            complexity_weight,
            loc_weight,
            comment_coverage_weight,
            cohesion_weight,
            file_cache: HashMap::new(),
            use_cache: true,
        }
    }

    /// Get all code metrics
    pub fn get_metrics(&self) -> &HashMap<String, CodeMetrics> {
        &self.metrics
    }

    /// Analyze code quality for a codebase
    pub fn analyze_codebase(&mut self, path: &Path, source: &str) -> Result<()> {
        println!("Analyzing code quality for {} codebase at: {}", source, path.display());

        // Check if path exists
        if !path.exists() {
            return Err(anyhow!("Path does not exist: {}", path.display()));
        }

        // Check if path is a directory
        if !path.is_dir() {
            return Err(anyhow!("Path is not a directory: {}", path.display()));
        }

        // Supported file extensions
        let supported_extensions = ["rb", "js", "ts", "jsx", "tsx", "rs", "hs"];
        let mut analyzed_count = 0;
        let mut cached_count = 0;
        let mut error_count = 0;

        // Walk the directory tree
        for entry_result in WalkDir::new(path) {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Error accessing directory entry: {}", e);
                    error_count += 1;
                    continue;
                }
            };

            let file_path = entry.path();

            // Skip if not a file
            if !file_path.is_file() {
                continue;
            }

            // Check file extension
            let extension = match file_path.extension() {
                Some(ext) => ext.to_string_lossy().to_lowercase(),
                None => continue, // Skip files without extension
            };

            // Skip unsupported file types
            if !supported_extensions.contains(&extension.as_str()) {
                continue;
            }

            // Get file path as string
            let file_path_str = file_path.to_string_lossy().to_string();

            // Check if we should analyze this file or use cached results
            let should_analyze = if !self.use_cache {
                true
            } else {
                self.should_analyze_file(file_path, &file_path_str)
            };

            if !should_analyze {
                cached_count += 1;
                continue;
            }

            // Read and analyze file
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    let metrics = self.calculate_metrics(file_path, &content);

                    // Update cache if enabled
                    if self.use_cache {
                        self.update_cache(file_path, &file_path_str, &metrics);
                    }

                    // Store metrics
                    self.metrics.insert(file_path_str, metrics);
                    analyzed_count += 1;
                },
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file_path.display(), e);
                    error_count += 1;
                }
            }
        }

        println!("Code quality analysis complete:");
        println!("  - Analyzed: {} files", analyzed_count);
        println!("  - Used cache: {} files", cached_count);
        println!("  - Errors: {} files", error_count);
        println!("  - Total processed: {} files", self.metrics.len());

        Ok(())
    }

    /// Determine if a file should be analyzed or if cached results can be used
    fn should_analyze_file(&self, file_path: &Path, file_path_str: &str) -> bool {
        // Get file metadata
        let metadata = match fs::metadata(file_path) {
            Ok(meta) => meta,
            Err(_) => return true, // Analyze if we can't get metadata
        };

        // Get modification time
        let modified_time = match metadata.modified() {
            Ok(time) => time,
            Err(_) => return true, // Analyze if we can't get modification time
        };

        // Convert to seconds since epoch
        let modified_secs = match modified_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => return true, // Analyze if we can't convert time
        };

        // Check cache
        if let Some(cache) = self.file_cache.get(file_path_str) {
            if cache.last_modified >= modified_secs {
                // File hasn't been modified, use cached metrics
                self.metrics.insert(file_path_str.to_string(), cache.metrics.clone());
                return false; // Don't need to analyze
            }
        }

        true // Need to analyze
    }

    /// Update the cache with new metrics
    fn update_cache(&mut self, file_path: &Path, file_path_str: &str, metrics: &CodeMetrics) {
        // Get file metadata
        let metadata = match fs::metadata(file_path) {
            Ok(meta) => meta,
            Err(_) => return, // Skip caching if we can't get metadata
        };

        // Get modification time
        let modified_time = match metadata.modified() {
            Ok(time) => time,
            Err(_) => return, // Skip caching if we can't get modification time
        };

        // Convert to seconds since epoch
        let modified_secs = match modified_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => return, // Skip caching if we can't convert time
        };

        // Update cache
        self.file_cache.insert(file_path_str.to_string(), FileCache {
            last_modified: modified_secs,
            metrics: metrics.clone(),
        });
    }

    /// Calculate metrics for a file
    fn calculate_metrics(&self, file_path: &Path, content: &str) -> CodeMetrics {
        // Calculate lines of code
        let lines: Vec<&str> = content.lines().collect();
        let loc = lines.len();

        // Calculate cyclomatic complexity
        let complexity = self.calculate_complexity(file_path, content);

        // Calculate comment coverage
        let comment_coverage = self.calculate_comment_coverage(file_path, content);

        // Calculate cohesion
        let cohesion = self.calculate_cohesion(file_path, content);

        // Calculate usefulness score
        let usefulness_score = self.calculate_usefulness_score(complexity, loc, comment_coverage, cohesion);

        // Determine recommendation
        let recommendation = if usefulness_score >= self.usefulness_threshold_high {
            "reuse".to_string()
        } else if usefulness_score >= self.usefulness_threshold_medium {
            "partial".to_string()
        } else {
            "rebuild".to_string()
        };

        CodeMetrics {
            file_path: file_path.to_string_lossy().to_string(),
            loc,
            complexity,
            comment_coverage,
            cohesion,
            usefulness_score,
            recommendation,
        }
    }

    /// Calculate cyclomatic complexity
    fn calculate_complexity(&self, file_path: &Path, content: &str) -> u32 {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Base complexity is 1
        let mut complexity = 1;

        // Define regex patterns
        let ruby_control_flow = r"\b(if|else|elsif|case|when|while|until|for|rescue)\b";
        let rust_control_flow = r"\b(if|else|match|while|loop|for)\b";
        let js_control_flow = r"\b(if|else|switch|case|while|do|for|try|catch)\b";
        let haskell_control_flow = r"\b(case|if|of)\b";
        let generic_control_flow = r"\b(if|else|switch|case|while|for)\b";
        let logical_operators = r"&&|\|\|";
        let ternary_operator = r"\?.*:";
        let haskell_guards = r"\|";

        // Helper function to safely compile and apply regex
        let count_matches = |pattern: &str| -> u32 {
            match Regex::new(pattern) {
                Ok(regex) => regex.find_iter(content).count() as u32,
                Err(_) => {
                    // Log error but continue with 0 count
                    eprintln!("Error compiling regex pattern: {}", pattern);
                    0
                }
            }
        };

        match ext {
            "rb" => {
                // Ruby complexity
                complexity += count_matches(ruby_control_flow);
                complexity += count_matches(logical_operators);
            },
            "rs" => {
                // Rust complexity
                complexity += count_matches(rust_control_flow);
                complexity += count_matches(logical_operators);
            },
            "js" | "ts" | "jsx" | "tsx" => {
                // JavaScript/TypeScript complexity
                complexity += count_matches(js_control_flow);
                complexity += count_matches(logical_operators);
                complexity += count_matches(ternary_operator);
            },
            "hs" => {
                // Haskell complexity
                complexity += count_matches(haskell_control_flow);
                complexity += count_matches(haskell_guards);
            },
            _ => {
                // Generic complexity
                complexity += count_matches(generic_control_flow);
            }
        }

        complexity
    }

    /// Calculate comment coverage
    fn calculate_comment_coverage(&self, file_path: &Path, content: &str) -> f32 {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        if total_lines == 0.0 {
            return 0.0;
        }

        let mut comment_lines = 0;

        match ext {
            "rb" => {
                // Ruby comments: # and =begin/=end blocks
                let mut in_block_comment = false;

                for line in &lines {
                    let trimmed = line.trim();

                    if trimmed.starts_with("=begin") {
                        in_block_comment = true;
                        comment_lines += 1;
                    } else if trimmed.starts_with("=end") {
                        in_block_comment = false;
                        comment_lines += 1;
                    } else if in_block_comment || trimmed.starts_with("#") {
                        comment_lines += 1;
                    }
                }
            },
            "rs" => {
                // Rust comments: // and /* */ blocks
                let mut in_block_comment = false;

                for line in &lines {
                    let trimmed = line.trim();

                    if !in_block_comment && trimmed.contains("/*") {
                        in_block_comment = true;
                    }

                    if in_block_comment || trimmed.starts_with("//") || trimmed.starts_with("///") {
                        comment_lines += 1;
                    }

                    if in_block_comment && trimmed.contains("*/") {
                        in_block_comment = false;
                    }
                }
            },
            "js" | "ts" | "jsx" | "tsx" => {
                // JavaScript/TypeScript comments: // and /* */ blocks
                let mut in_block_comment = false;

                for line in &lines {
                    let trimmed = line.trim();

                    if !in_block_comment && trimmed.contains("/*") {
                        in_block_comment = true;
                    }

                    if in_block_comment || trimmed.starts_with("//") {
                        comment_lines += 1;
                    }

                    if in_block_comment && trimmed.contains("*/") {
                        in_block_comment = false;
                    }
                }
            },
            "hs" => {
                // Haskell comments: -- and {- -} blocks
                let mut in_block_comment = false;

                for line in &lines {
                    let trimmed = line.trim();

                    if !in_block_comment && trimmed.contains("{-") {
                        in_block_comment = true;
                    }

                    if in_block_comment || trimmed.starts_with("--") {
                        comment_lines += 1;
                    }

                    if in_block_comment && trimmed.contains("-}") {
                        in_block_comment = false;
                    }
                }
            },
            _ => {
                // Generic comments: // and /* */ blocks
                let mut in_block_comment = false;

                for line in &lines {
                    let trimmed = line.trim();

                    if !in_block_comment && trimmed.contains("/*") {
                        in_block_comment = true;
                    }

                    if in_block_comment || trimmed.starts_with("//") || trimmed.starts_with("#") {
                        comment_lines += 1;
                    }

                    if in_block_comment && trimmed.contains("*/") {
                        in_block_comment = false;
                    }
                }
            }
        }

        (comment_lines as f32) / total_lines
    }

    /// Calculate cohesion
    fn calculate_cohesion(&self, file_path: &Path, content: &str) -> f32 {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Cohesion is a measure of how related the methods in a class are
        // For simplicity, we'll use a heuristic based on:
        // 1. Number of methods/functions
        // 2. Shared variable usage
        // 3. File size (smaller files tend to be more cohesive)

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        if total_lines == 0.0 {
            return 0.0;
        }

        // Penalize large files
        let size_factor = if total_lines > 500.0 {
            0.5
        } else if total_lines > 300.0 {
            0.7
        } else if total_lines > 100.0 {
            0.9
        } else {
            1.0
        };

        // Define regex patterns for method/function detection
        let ruby_method = r"\bdef\s+\w+";
        let rust_function = r"\bfn\s+\w+";
        let js_function = r"\bfunction\s+\w+|\w+\s*=\s*function|\w+\s*=\s*\(.*\)\s*=>";
        let haskell_function = r"^[a-z]\w*\s+::|^[a-z]\w*\s+[a-z]";
        let generic_function = r"\bfunction\s+\w+|\bdef\s+\w+";

        // Count methods/functions
        let method_count = match ext {
            "rb" => self.count_pattern_matches(ruby_method, content),
            "rs" => self.count_pattern_matches(rust_function, content),
            "js" | "ts" | "jsx" | "tsx" => self.count_pattern_matches(js_function, content),
            "hs" => {
                // For Haskell, we need to check line by line
                match Regex::new(haskell_function) {
                    Ok(regex) => {
                        lines.iter().filter(|line| regex.is_match(line)).count()
                    },
                    Err(e) => {
                        eprintln!("Error compiling Haskell function regex: {}", e);
                        0
                    }
                }
            },
            _ => self.count_pattern_matches(generic_function, content),
        };

        // Method count factor
        let method_factor = if method_count > 20 {
            0.5
        } else if method_count > 10 {
            0.7
        } else if method_count > 5 {
            0.9
        } else if method_count > 0 {
            1.0
        } else {
            0.5 // No methods is not good
        };

        // Calculate cohesion score
        let cohesion: f32 = size_factor * method_factor;

        // Normalize to 0.0-1.0 range
        cohesion.max(0.0).min(1.0)
    }

    /// Helper method to safely count regex pattern matches
    fn count_pattern_matches(&self, pattern: &str, content: &str) -> usize {
        match Regex::new(pattern) {
            Ok(regex) => regex.find_iter(content).count(),
            Err(e) => {
                eprintln!("Error compiling regex pattern '{}': {}", pattern, e);
                0
            }
        }
    }

    /// Calculate usefulness score
    fn calculate_usefulness_score(&self, complexity: u32, loc: usize, comment_coverage: f32, cohesion: f32) -> u8 {
        // Normalize complexity (lower is better)
        let complexity_score = if complexity > 100 {
            0.0
        } else if complexity > 50 {
            0.3
        } else if complexity > 20 {
            0.6
        } else {
            1.0
        };

        // Normalize LOC (moderate is better)
        let loc_score = if loc > 1000 {
            0.2
        } else if loc > 500 {
            0.4
        } else if loc > 200 {
            0.7
        } else if loc > 50 {
            1.0
        } else {
            0.8 // Very small files might be too simple
        };

        // Comment coverage score (higher is better)
        let comment_score = if comment_coverage > 0.3 {
            1.0
        } else if comment_coverage > 0.2 {
            0.8
        } else if comment_coverage > 0.1 {
            0.6
        } else if comment_coverage > 0.05 {
            0.4
        } else {
            0.2
        };

        // Calculate weighted score
        let weighted_score =
            complexity_score * self.complexity_weight +
            loc_score * self.loc_weight +
            comment_score * self.comment_coverage_weight +
            cohesion * self.cohesion_weight;

        // Convert to 0-100 scale
        (weighted_score * 100.0).round() as u8
    }

    /// Generate a JSON report of code metrics
    pub fn generate_metrics_report(&self) -> Result<String> {
        let report = serde_json::to_string_pretty(&self.metrics)?;
        Ok(report)
    }

    /// Generate a Markdown report of code quality
    /// Enable or disable caching
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    /// Clear the file cache
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
    }

    pub fn generate_quality_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Code Quality Analysis Report\n\n");

        // Summary statistics
        let total_files = self.metrics.len();
        let reuse_count = self.metrics.values().filter(|m| m.recommendation == "reuse").count();
        let partial_count = self.metrics.values().filter(|m| m.recommendation == "partial").count();
        let rebuild_count = self.metrics.values().filter(|m| m.recommendation == "rebuild").count();

        markdown.push_str("## Summary\n\n");
        markdown.push_str(&format!("- Total Files Analyzed: {}\n", total_files));
        markdown.push_str(&format!("- Recommended for Reuse: {} ({:.1}%)\n",
            reuse_count,
            if total_files > 0 { reuse_count as f32 / total_files as f32 * 100.0 } else { 0.0 }));
        markdown.push_str(&format!("- Recommended for Partial Reuse: {} ({:.1}%)\n",
            partial_count,
            if total_files > 0 { partial_count as f32 / total_files as f32 * 100.0 } else { 0.0 }));
        markdown.push_str(&format!("- Recommended for Rebuild: {} ({:.1}%)\n\n",
            rebuild_count,
            if total_files > 0 { rebuild_count as f32 / total_files as f32 * 100.0 } else { 0.0 }));

        // Top files for reuse
        markdown.push_str("## Top Files for Reuse\n\n");
        markdown.push_str("| File | Score | LOC | Complexity | Comment Coverage |\n");
        markdown.push_str("|------|-------|-----|------------|------------------|\n");

        let mut reuse_files: Vec<&CodeMetrics> = self.metrics.values()
            .filter(|m| m.recommendation == "reuse")
            .collect();

        reuse_files.sort_by(|a, b| b.usefulness_score.cmp(&a.usefulness_score));

        for metrics in reuse_files.iter().take(10) {
            markdown.push_str(&format!(
                "| {} | {} | {} | {} | {:.1}% |\n",
                metrics.file_path,
                metrics.usefulness_score,
                metrics.loc,
                metrics.complexity,
                metrics.comment_coverage * 100.0
            ));
        }

        markdown.push_str("\n");

        // Files needing rebuild
        markdown.push_str("## Files Recommended for Rebuild\n\n");
        markdown.push_str("| File | Score | LOC | Complexity | Comment Coverage |\n");
        markdown.push_str("|------|-------|-----|------------|------------------|\n");

        let mut rebuild_files: Vec<&CodeMetrics> = self.metrics.values()
            .filter(|m| m.recommendation == "rebuild")
            .collect();

        rebuild_files.sort_by(|a, b| a.usefulness_score.cmp(&b.usefulness_score));

        for metrics in rebuild_files.iter().take(10) {
            markdown.push_str(&format!(
                "| {} | {} | {} | {} | {:.1}% |\n",
                metrics.file_path,
                metrics.usefulness_score,
                metrics.loc,
                metrics.complexity,
                metrics.comment_coverage * 100.0
            ));
        }

        markdown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_new() {
        let scorer = CodeQualityScorer::new();
        assert_eq!(scorer.metrics.len(), 0);
        assert_eq!(scorer.usefulness_threshold_high, 80);
        assert_eq!(scorer.usefulness_threshold_medium, 50);
        assert!(scorer.use_cache);
    }

    #[test]
    fn test_new_without_cache() {
        let scorer = CodeQualityScorer::new_without_cache();
        assert!(!scorer.use_cache);
    }

    #[test]
    fn test_with_config() {
        let scorer = CodeQualityScorer::with_config(90, 60, 0.3, 0.3, 0.2, 0.2);
        assert_eq!(scorer.usefulness_threshold_high, 90);
        assert_eq!(scorer.usefulness_threshold_medium, 60);
        assert_eq!(scorer.complexity_weight, 0.3);
        assert_eq!(scorer.loc_weight, 0.3);
        assert_eq!(scorer.comment_coverage_weight, 0.2);
        assert_eq!(scorer.cohesion_weight, 0.2);
    }

    #[test]
    fn test_calculate_metrics() {
        let scorer = CodeQualityScorer::new();
        let content = r#"
        fn test_function() {
            // This is a test function
            if true {
                println!("Hello, world!");
            }
        }
        "#;

        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        let path = file_path.as_path();

        let metrics = scorer.calculate_metrics(path, content);

        assert_eq!(metrics.file_path, path.to_string_lossy().to_string());
        assert_eq!(metrics.loc, 8);
        assert!(metrics.complexity > 1); // Should count at least the if statement
        assert!(metrics.comment_coverage > 0.0); // Should detect the comment
    }

    #[test]
    fn test_count_pattern_matches() {
        let scorer = CodeQualityScorer::new();
        let content = "if true { println!(\"test\"); } else { println!(\"else\"); }";

        let count = scorer.count_pattern_matches(r"\b(if|else)\b", content);
        assert_eq!(count, 2); // Should find 'if' and 'else'
    }

    #[test]
    fn test_calculate_usefulness_score() {
        let scorer = CodeQualityScorer::new();

        // Low complexity, moderate size, good comments, high cohesion should score well
        let score1 = scorer.calculate_usefulness_score(10, 100, 0.25, 0.9);

        // High complexity, large size, poor comments, low cohesion should score poorly
        let score2 = scorer.calculate_usefulness_score(60, 600, 0.05, 0.3);

        assert!(score1 > score2);
        assert!(score1 >= 80); // Should be recommended for reuse
        assert!(score2 < 50);  // Should be recommended for rebuild
    }

    #[test]
    fn test_analyze_codebase() -> Result<()> {
        let mut scorer = CodeQualityScorer::new_without_cache();

        // Create a temporary directory with test files
        let temp_dir = tempdir()?;

        // Create a Rust file
        let rs_file_path = temp_dir.path().join("test.rs");
        let mut rs_file = File::create(&rs_file_path)?;
        writeln!(rs_file, "fn main() {{
    // This is a test
    println!(\"Hello, world!\");
}}")?;

        // Create a JavaScript file
        let js_file_path = temp_dir.path().join("test.js");
        let mut js_file = File::create(&js_file_path)?;
        writeln!(js_file, "function test() {{
    // This is a test
    console.log(\"Hello, world!\");
}}")?;

        // Analyze the codebase
        scorer.analyze_codebase(temp_dir.path(), "test")?;

        // Should have analyzed both files
        assert_eq!(scorer.metrics.len(), 2);

        // Check that metrics were calculated for both files
        assert!(scorer.metrics.contains_key(&rs_file_path.to_string_lossy().to_string()));
        assert!(scorer.metrics.contains_key(&js_file_path.to_string_lossy().to_string()));

        Ok(())
    }

    #[test]
    fn test_generate_metrics_report() -> Result<()> {
        let mut scorer = CodeQualityScorer::new();

        // Add a test metric
        let metrics = CodeMetrics {
            file_path: "test.rs".to_string(),
            loc: 100,
            complexity: 10,
            comment_coverage: 0.2,
            cohesion: 0.8,
            usefulness_score: 85,
            recommendation: "reuse".to_string(),
        };

        scorer.metrics.insert("test.rs".to_string(), metrics);

        // Generate report
        let report = scorer.generate_metrics_report()?;

        // Check that the report contains the expected data
        assert!(report.contains("test.rs"));
        assert!(report.contains("85"));
        assert!(report.contains("reuse"));

        Ok(())
    }

    #[test]
    fn test_generate_quality_markdown() {
        let mut scorer = CodeQualityScorer::new();

        // Add test metrics
        let reuse_metrics = CodeMetrics {
            file_path: "good.rs".to_string(),
            loc: 100,
            complexity: 10,
            comment_coverage: 0.2,
            cohesion: 0.8,
            usefulness_score: 85,
            recommendation: "reuse".to_string(),
        };

        let rebuild_metrics = CodeMetrics {
            file_path: "bad.rs".to_string(),
            loc: 500,
            complexity: 50,
            comment_coverage: 0.05,
            cohesion: 0.3,
            usefulness_score: 40,
            recommendation: "rebuild".to_string(),
        };

        scorer.metrics.insert("good.rs".to_string(), reuse_metrics);
        scorer.metrics.insert("bad.rs".to_string(), rebuild_metrics);

        // Generate markdown
        let markdown = scorer.generate_quality_markdown();

        // Check that the markdown contains the expected sections and data
        assert!(markdown.contains("# Code Quality Analysis Report"));
        assert!(markdown.contains("## Summary"));
        assert!(markdown.contains("Total Files Analyzed: 2"));
        assert!(markdown.contains("Recommended for Reuse: 1"));
        assert!(markdown.contains("Recommended for Rebuild: 1"));
        assert!(markdown.contains("## Top Files for Reuse"));
        assert!(markdown.contains("good.rs"));
        assert!(markdown.contains("## Files Recommended for Rebuild"));
        assert!(markdown.contains("bad.rs"));
    }
}
