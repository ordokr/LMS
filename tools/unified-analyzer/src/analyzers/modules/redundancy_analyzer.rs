use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use similar::{ChangeTag, TextDiff};

/// Represents a group of redundant implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedundancyGroup {
    /// The type of redundancy (e.g., "API Client", "Repository", "Error Handling")
    pub redundancy_type: String,
    /// The name of the component (e.g., "Canvas API Client", "User Repository")
    pub component_name: String,
    /// The list of files that contain redundant implementations
    pub files: Vec<RedundantFile>,
    /// Estimated impact of consolidation (1-10, higher is more impactful)
    pub impact_score: u8,
    /// Estimated effort to consolidate (1-10, higher is more effort)
    pub effort_score: u8,
    /// Suggested approach for consolidation
    pub consolidation_approach: String,
}

/// Represents a file with redundant implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedundantFile {
    /// The path to the file
    pub path: String,
    /// The size of the file in bytes
    pub size: u64,
    /// The number of lines in the file
    pub line_count: usize,
    /// The similarity score with other files in the group (0-100)
    pub similarity_score: u8,
    /// Key functions or methods in the file
    pub key_elements: Vec<String>,
}

/// Result of redundancy analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedundancyAnalysisResult {
    /// The list of redundancy groups
    pub redundancy_groups: Vec<RedundancyGroup>,
    /// Total number of redundant files
    pub total_redundant_files: usize,
    /// Total number of redundant lines
    pub total_redundant_lines: usize,
    /// Estimated reduction in code size if all redundancies are consolidated
    pub estimated_reduction_percentage: f32,
    /// Timestamp of the analysis
    pub timestamp: String,
}

/// Analyzer for detecting redundant implementations
pub struct RedundancyAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    /// Configuration for the analyzer
    config: RedundancyAnalyzerConfig,
    /// Result of the analysis
    result: RedundancyAnalysisResult,
}

/// Configuration for the redundancy analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedundancyAnalyzerConfig {
    /// Minimum similarity score to consider files redundant (0-100)
    pub min_similarity_score: u8,
    /// File extensions to analyze
    pub file_extensions: Vec<String>,
    /// Directories to exclude from analysis
    pub exclude_dirs: Vec<String>,
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: u64,
    /// Known redundancy patterns to look for
    pub known_patterns: HashMap<String, Vec<String>>,
}

impl Default for RedundancyAnalyzerConfig {
    fn default() -> Self {
        let mut known_patterns = HashMap::new();

        // API Client patterns
        known_patterns.insert(
            "API Client".to_string(),
            vec![
                "canvas_client".to_string(),
                "discourse_client".to_string(),
                "api_client".to_string(),
                "http_client".to_string(),
            ],
        );

        // Repository patterns
        known_patterns.insert(
            "Repository".to_string(),
            vec![
                "user_repository".to_string(),
                "course_repository".to_string(),
                "forum_repository".to_string(),
                "module_repository".to_string(),
            ],
        );

        // Error handling patterns
        known_patterns.insert(
            "Error Handling".to_string(),
            vec![
                "error.rs".to_string(),
                "errors.rs".to_string(),
                "error_handler".to_string(),
                "error_handling".to_string(),
            ],
        );

        // Utility function patterns
        known_patterns.insert(
            "Utility".to_string(),
            vec![
                "utils".to_string(),
                "helpers".to_string(),
                "util.rs".to_string(),
                "helper.rs".to_string(),
            ],
        );

        Self {
            min_similarity_score: 70,
            file_extensions: vec![".rs".to_string(), ".ts".to_string(), ".js".to_string()],
            exclude_dirs: vec![
                "node_modules".to_string(),
                "target".to_string(),
                "dist".to_string(),
                ".git".to_string(),
            ],
            max_file_size: 1024 * 1024, // 1 MB
            known_patterns,
        }
    }
}

impl RedundancyAnalyzer {
    /// Create a new redundancy analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            config: RedundancyAnalyzerConfig::default(),
            result: RedundancyAnalysisResult {
                redundancy_groups: Vec::new(),
                total_redundant_files: 0,
                total_redundant_lines: 0,
                estimated_reduction_percentage: 0.0,
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        }
    }

    /// Load configuration from a file
    pub fn load_config(&mut self, config_path: &Path) -> Result<()> {
        println!("Loading redundancy analyzer configuration from {}", config_path.display());

        // Read the file content
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        // Parse the TOML content
        let config: RedundancyAnalyzerConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        // Update the configuration
        self.config = config;

        println!("Loaded redundancy analyzer configuration");

        Ok(())
    }

    /// Set the configuration for the analyzer
    pub fn with_config(mut self, config: RedundancyAnalyzerConfig) -> Self {
        self.config = config;
        self
    }

    /// Run the analysis
    pub fn analyze(&mut self) -> Result<RedundancyAnalysisResult> {
        println!("Starting redundancy analysis...");

        // Reset the result
        self.result = RedundancyAnalysisResult {
            redundancy_groups: Vec::new(),
            total_redundant_files: 0,
            total_redundant_lines: 0,
            estimated_reduction_percentage: 0.0,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        // Find files to analyze
        let files = self.find_files()?;
        println!("Found {} files to analyze", files.len());

        // Group files by pattern
        let pattern_groups = self.group_files_by_pattern(&files)?;
        println!("Found {} potential redundancy groups", pattern_groups.len());

        // Analyze each group for redundancy
        let mut total_redundant_files = 0;
        let mut total_redundant_lines = 0;
        let mut total_lines = 0;

        for (pattern_type, pattern_files) in pattern_groups {
            // Skip groups with only one file
            if pattern_files.len() <= 1 {
                continue;
            }

            // Analyze the group
            if let Some(redundancy_group) = self.analyze_group(pattern_type, pattern_files)? {
                total_redundant_files += redundancy_group.files.len();
                total_redundant_lines += redundancy_group.files.iter().map(|f| f.line_count).sum::<usize>();
                self.result.redundancy_groups.push(redundancy_group);
            }
        }

        // Calculate total lines in the codebase
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip excluded directories
            if self.is_excluded(path) {
                continue;
            }

            // Only consider specified file extensions
            if let Some(ext) = path.extension() {
                if !self.config.file_extensions.iter().any(|e| e == &format!(".{}", ext.to_string_lossy())) {
                    continue;
                }
            } else {
                continue;
            }

            // Count lines in the file
            if let Ok(content) = fs::read_to_string(path) {
                total_lines += content.lines().count();
            }
        }

        // Update the result
        self.result.total_redundant_files = total_redundant_files;
        self.result.total_redundant_lines = total_redundant_lines;

        // Calculate estimated reduction percentage
        if total_lines > 0 {
            self.result.estimated_reduction_percentage = (total_redundant_lines as f32 / total_lines as f32) * 100.0;
        }

        println!("Redundancy analysis complete");
        println!("Found {} redundancy groups with {} redundant files",
            self.result.redundancy_groups.len(),
            self.result.total_redundant_files);
        println!("Estimated reduction: {:.2}% ({} lines)",
            self.result.estimated_reduction_percentage,
            self.result.total_redundant_lines);

        Ok(self.result.clone())
    }

    /// Find files to analyze
    fn find_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip excluded directories
            if self.is_excluded(path) {
                continue;
            }

            // Only consider specified file extensions
            if let Some(ext) = path.extension() {
                if !self.config.file_extensions.iter().any(|e| e == &format!(".{}", ext.to_string_lossy())) {
                    continue;
                }
            } else {
                continue;
            }

            // Skip files that are too large
            if let Ok(metadata) = fs::metadata(path) {
                if metadata.len() > self.config.max_file_size {
                    continue;
                }
            }

            files.push(path.to_path_buf());
        }

        Ok(files)
    }

    /// Check if a path should be excluded
    fn is_excluded(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        for exclude_dir in &self.config.exclude_dirs {
            if path_str.contains(exclude_dir) {
                return true;
            }
        }

        false
    }

    /// Group files by pattern
    fn group_files_by_pattern(&self, files: &[PathBuf]) -> Result<HashMap<String, Vec<PathBuf>>> {
        let mut pattern_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for file in files {
            let file_str = file.to_string_lossy();

            for (pattern_type, patterns) in &self.config.known_patterns {
                for pattern in patterns {
                    if file_str.contains(pattern) {
                        pattern_groups
                            .entry(pattern_type.clone())
                            .or_insert_with(Vec::new)
                            .push(file.clone());
                        break;
                    }
                }
            }
        }

        Ok(pattern_groups)
    }

    /// Analyze a group of files for redundancy
    fn analyze_group(&self, redundancy_type: String, files: Vec<PathBuf>) -> Result<Option<RedundancyGroup>> {
        // Extract component name from the first file
        let component_name = if let Some(file) = files.first() {
            let file_name = file.file_name().unwrap_or_default().to_string_lossy();

            if redundancy_type == "API Client" {
                if file_name.contains("canvas") {
                    "Canvas API Client".to_string()
                } else if file_name.contains("discourse") {
                    "Discourse API Client".to_string()
                } else {
                    "Generic API Client".to_string()
                }
            } else if redundancy_type == "Repository" {
                if file_name.contains("user") {
                    "User Repository".to_string()
                } else if file_name.contains("course") {
                    "Course Repository".to_string()
                } else if file_name.contains("forum") || file_name.contains("topic") {
                    "Forum Repository".to_string()
                } else if file_name.contains("module") {
                    "Module Repository".to_string()
                } else {
                    "Generic Repository".to_string()
                }
            } else if redundancy_type == "Error Handling" {
                "Error Handling".to_string()
            } else {
                "Utility Functions".to_string()
            }
        } else {
            "Unknown".to_string()
        };

        // Calculate similarity between files
        let mut redundant_files = Vec::new();
        let mut total_similarity = 0;
        let mut file_count = 0;

        for file in &files {
            let file_content = match fs::read_to_string(file) {
                Ok(content) => content,
                Err(_) => continue,
            };

            let line_count = file_content.lines().count();
            let key_elements = self.extract_key_elements(&file_content);

            // Calculate similarity with other files
            let mut max_similarity = 0;

            for other_file in &files {
                if file == other_file {
                    continue;
                }

                let other_content = match fs::read_to_string(other_file) {
                    Ok(content) => content,
                    Err(_) => continue,
                };

                let similarity = self.calculate_similarity(&file_content, &other_content);
                max_similarity = max_similarity.max(similarity);
            }

            // Only include files with similarity above threshold
            if max_similarity >= self.config.min_similarity_score {
                let metadata = fs::metadata(file)?;

                redundant_files.push(RedundantFile {
                    path: file.strip_prefix(&self.base_dir)
                        .unwrap_or(file)
                        .to_string_lossy()
                        .to_string(),
                    size: metadata.len(),
                    line_count,
                    similarity_score: max_similarity,
                    key_elements,
                });

                total_similarity += max_similarity as usize;
                file_count += 1;
            }
        }

        // Only create a group if there are at least 2 redundant files
        if redundant_files.len() >= 2 {
            // Calculate impact and effort scores
            let impact_score = match redundancy_type.as_str() {
                "API Client" => 8,
                "Repository" => 7,
                "Error Handling" => 6,
                "Utility" => 5,
                _ => 4,
            };

            let effort_score = match redundant_files.len() {
                2..=3 => 3,
                4..=6 => 5,
                7..=10 => 7,
                _ => 9,
            };

            // Generate consolidation approach
            let consolidation_approach = match redundancy_type.as_str() {
                "API Client" => "Create a unified API client with strategy pattern for different backends".to_string(),
                "Repository" => "Implement a base repository with common CRUD operations and extend for specific needs".to_string(),
                "Error Handling" => "Create a hierarchical error type system with clear categories and context".to_string(),
                "Utility" => "Consolidate utility functions into domain-specific modules with consistent APIs".to_string(),
                _ => "Analyze similarities and extract common functionality into a shared implementation".to_string(),
            };

            Ok(Some(RedundancyGroup {
                redundancy_type,
                component_name,
                files: redundant_files,
                impact_score,
                effort_score,
                consolidation_approach,
            }))
        } else {
            Ok(None)
        }
    }

    /// Extract key elements from file content
    fn extract_key_elements(&self, content: &str) -> Vec<String> {
        let mut elements = Vec::new();

        // Extract function and struct definitions
        for line in content.lines() {
            let line = line.trim();

            if (line.starts_with("fn ") || line.starts_with("pub fn ")) && line.contains("(") {
                if let Some(name) = line.split("fn ").nth(1).and_then(|s| s.split("(").next()) {
                    elements.push(format!("fn {}", name.trim()));
                }
            } else if (line.starts_with("struct ") || line.starts_with("pub struct ")) && !line.contains(";") {
                if let Some(name) = line.split("struct ").nth(1).and_then(|s| s.split("{").next()) {
                    elements.push(format!("struct {}", name.trim()));
                }
            } else if (line.starts_with("impl ") || line.starts_with("pub impl ")) && !line.contains(";") {
                if let Some(name) = line.split("impl ").nth(1).and_then(|s| s.split("{").next()) {
                    elements.push(format!("impl {}", name.trim()));
                }
            } else if (line.starts_with("trait ") || line.starts_with("pub trait ")) && !line.contains(";") {
                if let Some(name) = line.split("trait ").nth(1).and_then(|s| s.split("{").next()) {
                    elements.push(format!("trait {}", name.trim()));
                }
            }
        }

        // Limit to top 10 elements
        elements.truncate(10);

        elements
    }

    /// Calculate similarity between two strings (0-100)
    fn calculate_similarity(&self, a: &str, b: &str) -> u8 {
        let diff = TextDiff::from_lines(a, b);

        let mut same_count = 0;
        let mut total_count = 0;

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Equal => {
                    same_count += 1;
                    total_count += 1;
                }
                _ => {
                    total_count += 1;
                }
            }
        }

        if total_count == 0 {
            return 0;
        }

        ((same_count as f32 / total_count as f32) * 100.0) as u8
    }

    /// Generate a report of the analysis results
    pub fn generate_report(&self) -> Result<String> {
        let mut report = String::new();

        report.push_str("# Redundancy Analysis Report\n\n");
        report.push_str(&format!("Generated on: {}\n\n", self.result.timestamp));

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- Total redundancy groups: {}\n", self.result.redundancy_groups.len()));
        report.push_str(&format!("- Total redundant files: {}\n", self.result.total_redundant_files));
        report.push_str(&format!("- Total redundant lines: {}\n", self.result.total_redundant_lines));
        report.push_str(&format!("- Estimated reduction: {:.2}%\n\n", self.result.estimated_reduction_percentage));

        report.push_str("## Redundancy Groups\n\n");

        for (i, group) in self.result.redundancy_groups.iter().enumerate() {
            report.push_str(&format!("### Group {}: {} ({})\n\n", i + 1, group.component_name, group.redundancy_type));
            report.push_str(&format!("- Impact Score: {}/10\n", group.impact_score));
            report.push_str(&format!("- Effort Score: {}/10\n", group.effort_score));
            report.push_str(&format!("- Consolidation Approach: {}\n\n", group.consolidation_approach));

            report.push_str("| File | Size | Lines | Similarity | Key Elements |\n");
            report.push_str("|------|------|-------|------------|-------------|\n");

            for file in &group.files {
                let size_kb = file.size as f32 / 1024.0;
                let key_elements = file.key_elements.join(", ");

                report.push_str(&format!("| {} | {:.1} KB | {} | {}% | {} |\n",
                    file.path, size_kb, file.line_count, file.similarity_score, key_elements));
            }

            report.push_str("\n");
        }

        report.push_str("## Recommendations\n\n");

        // Sort groups by impact score (descending)
        let mut sorted_groups = self.result.redundancy_groups.clone();
        sorted_groups.sort_by(|a, b| b.impact_score.cmp(&a.impact_score));

        for group in sorted_groups.iter().take(5) {
            report.push_str(&format!("### {}\n\n", group.component_name));
            report.push_str(&format!("- **Priority**: {}/10\n", group.impact_score));
            report.push_str(&format!("- **Approach**: {}\n", group.consolidation_approach));
            report.push_str("- **Files to Consolidate**:\n");

            for file in &group.files {
                report.push_str(&format!("  - `{}`\n", file.path));
            }

            report.push_str("\n");
        }

        Ok(report)
    }

    /// Save the analysis results to a file
    pub fn save_results(&self, output_path: &Path) -> Result<()> {
        // Create the output directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Generate the report
        let report = self.generate_report()?;

        // Write the report to the file
        fs::write(output_path, report)?;

        println!("Saved redundancy analysis report to {}", output_path.display());

        Ok(())
    }
}
