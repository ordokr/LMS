use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use regex::Regex;

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

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let file_path = entry.path();

            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ["rb", "js", "ts", "jsx", "tsx", "rs", "hs"].contains(&ext_str.as_str()) {
                        // Check if file is in cache and hasn't been modified
                        let file_path_str = file_path.to_string_lossy().to_string();
                        let should_analyze = if self.use_cache {
                            match fs::metadata(file_path) {
                                Ok(metadata) => {
                                    if let Ok(modified_time) = metadata.modified() {
                                        if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                                            let modified_secs = modified_secs.as_secs();
                                            if let Some(cache) = self.file_cache.get(&file_path_str) {
                                                if cache.last_modified >= modified_secs {
                                                    // File hasn't been modified, use cached metrics
                                                    self.metrics.insert(file_path_str.clone(), cache.metrics.clone());
                                                    println!("Using cached metrics for: {}", file_path_str);
                                                    false
                                                } else {
                                                    true
                                                }
                                            } else {
                                                true
                                            }
                                        } else {
                                            true
                                        }
                                    } else {
                                        true
                                    }
                                },
                                Err(_) => true,
                            }
                        } else {
                            true
                        };

                        if should_analyze {
                            if let Ok(content) = fs::read_to_string(file_path) {
                                let metrics = self.calculate_metrics(file_path, &content);

                                // Update cache
                                if self.use_cache {
                                    if let Ok(metadata) = fs::metadata(file_path) {
                                        if let Ok(modified_time) = metadata.modified() {
                                            if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                                                let modified_secs = modified_secs.as_secs();
                                                self.file_cache.insert(file_path_str.clone(), FileCache {
                                                    last_modified: modified_secs,
                                                    metrics: metrics.clone(),
                                                });
                                            }
                                        }
                                    }
                                }

                                self.metrics.insert(file_path_str, metrics);
                            }
                        }
                    }
                }
            }
        }

        println!("Analyzed {} files for code quality", self.metrics.len());

        Ok(())
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

        match ext {
            "rb" => {
                // Ruby complexity: count if, else, elsif, case, when, while, until, for, rescue
                let control_flow_regex = Regex::new(r"\b(if|else|elsif|case|when|while|until|for|rescue)\b").unwrap();
                complexity += control_flow_regex.find_iter(content).count() as u32;

                // Count && and || operators
                let logical_op_regex = Regex::new(r"&&|\|\|").unwrap();
                complexity += logical_op_regex.find_iter(content).count() as u32;
            },
            "rs" => {
                // Rust complexity: count if, else, match, while, loop, for
                let control_flow_regex = Regex::new(r"\b(if|else|match|while|loop|for)\b").unwrap();
                complexity += control_flow_regex.find_iter(content).count() as u32;

                // Count && and || operators
                let logical_op_regex = Regex::new(r"&&|\|\|").unwrap();
                complexity += logical_op_regex.find_iter(content).count() as u32;
            },
            "js" | "ts" | "jsx" | "tsx" => {
                // JavaScript/TypeScript complexity: count if, else, switch, case, while, do, for, try, catch
                let control_flow_regex = Regex::new(r"\b(if|else|switch|case|while|do|for|try|catch)\b").unwrap();
                complexity += control_flow_regex.find_iter(content).count() as u32;

                // Count && and || operators
                let logical_op_regex = Regex::new(r"&&|\|\|").unwrap();
                complexity += logical_op_regex.find_iter(content).count() as u32;

                // Count ternary operators
                let ternary_regex = Regex::new(r"\?.*:").unwrap();
                complexity += ternary_regex.find_iter(content).count() as u32;
            },
            "hs" => {
                // Haskell complexity: count case, if, guards (|)
                let control_flow_regex = Regex::new(r"\b(case|if|of)\b").unwrap();
                complexity += control_flow_regex.find_iter(content).count() as u32;

                // Count guards
                let guard_regex = Regex::new(r"\|").unwrap();
                complexity += guard_regex.find_iter(content).count() as u32;
            },
            _ => {
                // Generic complexity: count if, else, switch, case, while, for
                let control_flow_regex = Regex::new(r"\b(if|else|switch|case|while|for)\b").unwrap();
                complexity += control_flow_regex.find_iter(content).count() as u32;
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

        // Count methods/functions
        let mut method_count = 0;

        match ext {
            "rb" => {
                // Ruby methods: def
                let method_regex = Regex::new(r"\bdef\s+\w+").unwrap();
                method_count = method_regex.find_iter(content).count();
            },
            "rs" => {
                // Rust functions: fn
                let method_regex = Regex::new(r"\bfn\s+\w+").unwrap();
                method_count = method_regex.find_iter(content).count();
            },
            "js" | "ts" | "jsx" | "tsx" => {
                // JavaScript/TypeScript functions: function or =>
                let function_regex = Regex::new(r"\bfunction\s+\w+|\w+\s*=\s*function|\w+\s*=\s*\(.*\)\s*=>").unwrap();
                method_count = function_regex.find_iter(content).count();
            },
            "hs" => {
                // Haskell functions: function definitions
                let function_regex = Regex::new(r"^[a-z]\w*\s+::|^[a-z]\w*\s+[a-z]").unwrap();
                for line in &lines {
                    if function_regex.is_match(line) {
                        method_count += 1;
                    }
                }
            },
            _ => {
                // Generic functions: function or def
                let function_regex = Regex::new(r"\bfunction\s+\w+|\bdef\s+\w+").unwrap();
                method_count = function_regex.find_iter(content).count();
            }
        }

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
