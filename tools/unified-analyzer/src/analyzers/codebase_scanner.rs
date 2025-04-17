use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use anyhow::Result;

use crate::utils::file_system::FileSystemUtils;

/// Represents a file in the codebase
#[derive(Debug, Clone)]
pub struct CodeFile {
    pub path: PathBuf,
    pub extension: String,
    pub content: String,
    pub line_count: usize,
    pub is_test: bool,
    pub category: FileCategory,
}

/// Categories of files in the codebase
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileCategory {
    Model,
    Api,
    Component,
    Test,
    Database,
    Sync,
    Util,
    Config,
    Documentation,
    Other,
}

/// Statistics about the codebase
#[derive(Debug, Clone)]
pub struct CodebaseStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub files_by_extension: HashMap<String, usize>,
    pub files_by_category: HashMap<FileCategory, usize>,
    pub lines_by_category: HashMap<FileCategory, usize>,
    pub model_count: usize,
    pub api_endpoint_count: usize,
    pub component_count: usize,
    pub test_count: usize,
    pub test_coverage: f32,
    pub database_files: usize,
    pub sync_files: usize,
}

impl Default for CodebaseStats {
    fn default() -> Self {
        Self {
            total_files: 0,
            total_lines: 0,
            files_by_extension: HashMap::new(),
            files_by_category: HashMap::new(),
            lines_by_category: HashMap::new(),
            model_count: 0,
            api_endpoint_count: 0,
            component_count: 0,
            test_count: 0,
            test_coverage: 0.0,
            database_files: 0,
            sync_files: 0,
        }
    }
}

/// Scanner for analyzing the codebase
pub struct CodebaseScanner {
    base_dir: PathBuf,
    fs_utils: Arc<FileSystemUtils>,
    files: Arc<Mutex<Vec<CodeFile>>>,
    stats: Arc<Mutex<CodebaseStats>>,
    exclude_patterns: Vec<Regex>,
}

impl CodebaseScanner {
    pub fn new(base_dir: PathBuf, fs_utils: Arc<FileSystemUtils>) -> Self {
        let exclude_patterns = vec![
            Regex::new(r"node_modules").unwrap(),
            Regex::new(r"target").unwrap(),
            Regex::new(r"\.git").unwrap(),
            Regex::new(r"\.vscode").unwrap(),
            Regex::new(r"\.idea").unwrap(),
            Regex::new(r"dist").unwrap(),
            Regex::new(r"build").unwrap(),
        ];

        Self {
            base_dir,
            fs_utils,
            files: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(CodebaseStats::default())),
            exclude_patterns,
        }
    }

    /// Scan the codebase and analyze files
    pub async fn scan(&self) -> Result<CodebaseStats> {
        println!("Scanning codebase at {:?}...", self.base_dir);

        // Reset the files and stats
        let mut files = self.files.lock().await;
        files.clear();
        drop(files);

        let mut stats = self.stats.lock().await;
        *stats = CodebaseStats::default();
        drop(stats);

        // Scan the codebase
        self.scan_files().await?;

        // Analyze the files
        self.analyze_files().await?;

        // Calculate test coverage
        self.calculate_test_coverage().await?;

        // Return the stats
        let stats = self.stats.lock().await.clone();
        Ok(stats)
    }

    /// Scan files in the codebase
    async fn scan_files(&self) -> Result<()> {
        println!("Scanning files...");

        let mut files = Vec::new();

        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e.path()))
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();

            // Skip files larger than 1MB to avoid memory issues
            if let Ok(metadata) = fs::metadata(&path) {
                if metadata.len() > 1_000_000 {
                    continue;
                }
            }

            // Get the file extension
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();

            // Skip binary files and large files
            if extension.is_empty() ||
               ["exe", "dll", "so", "dylib", "bin", "obj", "o", "a", "lib", "png", "jpg", "jpeg", "gif", "svg", "ico"].contains(&extension.as_str()) {
                continue;
            }

            // Read the file content
            if let Ok(content) = fs::read_to_string(&path) {
                let line_count = content.lines().count();
                let is_test = self.is_test_file(&path, &content);
                let category = self.categorize_file(&path, &content);

                files.push(CodeFile {
                    path,
                    extension,
                    content,
                    line_count,
                    is_test,
                    category,
                });
            }
        }

        // Update the files
        let mut files_lock = self.files.lock().await;
        *files_lock = files;
        drop(files_lock);

        Ok(())
    }

    /// Analyze the scanned files
    async fn analyze_files(&self) -> Result<()> {
        println!("Analyzing files...");

        let files = self.files.lock().await.clone();
        let mut stats = self.stats.lock().await;

        stats.total_files = files.len();
        stats.total_lines = files.iter().map(|f| f.line_count).sum();

        // Count files by extension
        for file in &files {
            *stats.files_by_extension.entry(file.extension.clone()).or_insert(0) += 1;
            *stats.files_by_category.entry(file.category.clone()).or_insert(0) += 1;
            *stats.lines_by_category.entry(file.category.clone()).or_insert(0) += file.line_count;
        }

        // Count specific file types
        stats.model_count = files.iter().filter(|f| f.category == FileCategory::Model).count();
        stats.api_endpoint_count = files.iter().filter(|f| f.category == FileCategory::Api).count();
        stats.component_count = files.iter().filter(|f| f.category == FileCategory::Component).count();
        stats.test_count = files.iter().filter(|f| f.is_test).count();
        stats.database_files = files.iter().filter(|f| f.category == FileCategory::Database).count();
        stats.sync_files = files.iter().filter(|f| f.category == FileCategory::Sync).count();

        Ok(())
    }

    /// Calculate test coverage
    async fn calculate_test_coverage(&self) -> Result<()> {
        println!("Calculating test coverage...");

        let files = self.files.lock().await.clone();
        let mut stats = self.stats.lock().await;

        // Count the number of non-test files
        let non_test_files = files.iter().filter(|f| !f.is_test).count();

        // Count the number of files that have corresponding test files
        let mut tested_files = 0;
        let test_files: Vec<_> = files.iter().filter(|f| f.is_test).collect();

        for file in files.iter().filter(|f| !f.is_test) {
            let file_name = file.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

            // Check if there's a test file for this file
            let has_test = test_files.iter().any(|tf| {
                let test_file_name = tf.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                test_file_name.contains(file_name) ||
                (test_file_name.contains("test") && test_file_name.contains(file_name))
            });

            if has_test {
                tested_files += 1;
            }
        }

        // Calculate test coverage
        stats.test_coverage = if non_test_files > 0 {
            (tested_files as f32 / non_test_files as f32) * 100.0
        } else {
            0.0
        };

        Ok(())
    }

    /// Check if a file is excluded based on patterns
    fn is_excluded(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.exclude_patterns.iter().any(|pattern| pattern.is_match(&path_str))
    }

    /// Check if a file is a test file
    fn is_test_file(&self, path: &Path, content: &str) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("test") ||
        path_str.contains("spec") ||
        content.contains("#[test]") ||
        content.contains("describe(") ||
        content.contains("it(") ||
        content.contains("test(")
    }

    /// Categorize a file based on its path and content
    fn categorize_file(&self, path: &Path, content: &str) -> FileCategory {
        let path_str = path.to_string_lossy();

        if self.is_test_file(path, content) {
            return FileCategory::Test;
        }

        if path_str.contains("model") ||
           content.contains("struct") && (content.contains("Serialize") || content.contains("Deserialize")) {
            return FileCategory::Model;
        }

        if path_str.contains("api") ||
           path_str.contains("route") ||
           path_str.contains("controller") ||
           content.contains("get(") ||
           content.contains("post(") ||
           content.contains("put(") ||
           content.contains("delete(") {
            return FileCategory::Api;
        }

        if path_str.contains("component") ||
           path_str.contains("view") ||
           path_str.contains("ui") ||
           content.contains("view!") ||
           content.contains("component!") ||
           content.contains("function Component") {
            return FileCategory::Component;
        }

        if path_str.contains("db") ||
           path_str.contains("database") ||
           path_str.contains("migration") ||
           content.contains("sqlx") ||
           content.contains("Database") ||
           content.contains("Table") ||
           content.contains("Query") {
            return FileCategory::Database;
        }

        if path_str.contains("sync") ||
           path_str.contains("synchronization") ||
           content.contains("SyncEngine") ||
           content.contains("Synchronization") {
            return FileCategory::Sync;
        }

        if path_str.contains("util") ||
           path_str.contains("helper") ||
           path_str.contains("common") {
            return FileCategory::Util;
        }

        if path_str.contains("config") ||
           path_str.contains("settings") ||
           path_str.ends_with(".toml") ||
           path_str.ends_with(".json") ||
           path_str.ends_with(".yaml") ||
           path_str.ends_with(".yml") {
            return FileCategory::Config;
        }

        if path_str.ends_with(".md") ||
           path_str.contains("doc") ||
           path_str.contains("docs") {
            return FileCategory::Documentation;
        }

        FileCategory::Other
    }

    /// Get the list of models found in the codebase
    pub async fn get_models(&self) -> Vec<String> {
        let files = self.files.lock().await.clone();
        let mut models = Vec::new();

        for file in files.iter().filter(|f| f.category == FileCategory::Model) {
            // Use regex to find struct definitions
            let re = Regex::new(r"struct\s+([A-Za-z0-9_]+)").unwrap();
            for cap in re.captures_iter(&file.content) {
                if let Some(model_name) = cap.get(1) {
                    models.push(model_name.as_str().to_string());
                }
            }
        }

        models
    }

    /// Get the list of API endpoints found in the codebase
    pub async fn get_api_endpoints(&self) -> Vec<String> {
        let files = self.files.lock().await.clone();
        let mut endpoints = Vec::new();

        for file in files.iter().filter(|f| f.category == FileCategory::Api) {
            // Use regex to find route definitions
            // Simplified regex to avoid escaping issues
            let re = Regex::new(r"(get|post|put|delete|patch)\s*\(").unwrap();

            // Just count the matches for now
            let matches = re.find_iter(&file.content).count();

            // Add generic endpoints based on the count
            for i in 0..matches {
                endpoints.push(format!("API Endpoint {}", i + 1));
            }
        }

        endpoints
    }

    /// Get the list of components found in the codebase
    pub async fn get_components(&self) -> Vec<String> {
        let files = self.files.lock().await.clone();
        let mut components = Vec::new();

        for file in files.iter().filter(|f| f.category == FileCategory::Component) {
            // Use regex to find component definitions
            let re = Regex::new(r"fn\s+[A-Za-z0-9_]+").unwrap();

            // Just count the matches for now
            let matches = re.find_iter(&file.content).count();

            // Add generic components based on the count
            for i in 0..matches {
                components.push(format!("Component {}", i + 1));
            }
        }

        components
    }
}
