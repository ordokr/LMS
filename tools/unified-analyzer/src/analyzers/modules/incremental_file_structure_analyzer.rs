use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, path::{Path, PathBuf}};
use walkdir::WalkDir;
use regex::Regex;
use lazy_static::lazy_static;
use anyhow::{Result, Context, anyhow};

use crate::utils::incremental_analyzer::{IncrementalAnalyzer, AnalysisCache};
use crate::analyzers::modules::file_structure_analyzer::{DirectoryPurpose, FileType};

lazy_static! {
    static ref RUST_IMPORT_REGEX: Regex = Regex::new(r#"use\s+([^;]+);|mod\s+([^;{]+)"#).unwrap();
    static ref JAVASCRIPT_IMPORT_REQUIRE_REGEX: Regex = Regex::new(r#"(?:import\s+.*?from\s+['"]([^'"]+)['"]|require\s*\(['"]([^'"]+)['"]\))"#).unwrap();
    static ref TYPESCRIPT_IMPORT_REGEX: Regex = Regex::new(r#"import\s+.*?from\s+['"]([^'"]+)['"]"#).unwrap();
    static ref RUBY_REQUIRE_REGEX: Regex = Regex::new(r#"require(?:_relative)?\s+['"]([^'"]+)['"]"#).unwrap();
    static ref PYTHON_IMPORT_REGEX: Regex = Regex::new(r#"(?:from\s+([^\s]+)\s+import|import\s+([^\s]+))"#).unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMetadata {
    pub name: String,
    pub parent_directory: Option<PathBuf>,
    pub relative_path: PathBuf,
    pub absolute_path: PathBuf,
    pub file_type: String,
    pub size: u64,
    pub modified_time: u64,
    pub dependencies: Vec<PathBuf>,
}

impl Default for FileMetadata {
    fn default() -> Self {
        FileMetadata {
            name: String::new(),
            parent_directory: None,
            relative_path: PathBuf::new(),
            absolute_path: PathBuf::new(),
            file_type: String::new(),
            size: 0,
            modified_time: 0,
            dependencies: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryMetadata {
    pub purpose: DirectoryPurpose,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileStructureAnalysisResult {
    pub directory_metadata: HashMap<PathBuf, DirectoryMetadata>,
    pub file_metadata: HashMap<PathBuf, FileMetadata>,
    pub file_dependency_graph: HashMap<PathBuf, Vec<PathBuf>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncrementalFileStructureAnalyzer {
    pub base_dir: PathBuf,
    pub use_incremental: bool,
    pub cache_path: Option<PathBuf>,
    pub exclude_dirs: Vec<String>,
    pub include_extensions: Vec<String>,
}

impl Default for IncrementalFileStructureAnalyzer {
    fn default() -> Self {
        Self {
            base_dir: PathBuf::new(),
            use_incremental: true, // Enable incremental analysis by default
            cache_path: None,
            exclude_dirs: vec![
                "node_modules".to_string(),
                "target".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".git".to_string(),
            ],
            include_extensions: vec![
                "rs".to_string(),
                "js".to_string(),
                "jsx".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "rb".to_string(),
                "py".to_string(),
            ],
        }
    }
}

impl IncrementalFileStructureAnalyzer {
    pub fn new(base_dir: PathBuf) -> Self {
        let mut analyzer = Self::default();
        analyzer.base_dir = base_dir.clone();
        analyzer.cache_path = Some(base_dir.join(".file_structure_analyzer_cache.json"));
        analyzer
    }

    pub fn with_incremental(mut self, use_incremental: bool) -> Self {
        self.use_incremental = use_incremental;
        self
    }

    pub fn with_cache_path(mut self, cache_path: PathBuf) -> Self {
        self.cache_path = Some(cache_path);
        self
    }

    pub fn analyze(&self) -> Result<FileStructureAnalysisResult> {
        let mut result = FileStructureAnalysisResult::default();

        // First pass: Collect directory metadata
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
        {
            let path = entry.path();
            if self.should_exclude_file(path) {
                continue;
            }

            let relative_path = path.strip_prefix(&self.base_dir)
                .unwrap_or(path)
                .to_path_buf();

            let purpose = self.categorize_directory(&relative_path);
            result.directory_metadata.insert(
                relative_path.clone(),
                DirectoryMetadata { purpose },
            );
        }

        // Second pass: Collect files to analyze
        let mut files_to_analyze = Vec::new();
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if !self.should_exclude_file(path) {
                files_to_analyze.push(path.to_path_buf());
            }
        }

        // Analyze files incrementally
        let file_results = self.analyze_files_incrementally(&files_to_analyze)?;

        // Process file results
        for file_result in file_results {
            // Add file metadata
            for (path, metadata) in file_result.file_metadata {
                result.file_metadata.insert(path, metadata);
            }

            // Add file dependencies
            for (path, deps) in file_result.file_dependency_graph {
                result.file_dependency_graph.insert(path, deps);
            }
        }

        Ok(result)
    }

    fn categorize_directory(&self, relative_path: &Path) -> DirectoryPurpose {
        match relative_path.to_string_lossy().as_ref() {
            "models" | "app/models" => DirectoryPurpose::Model,
            "controllers" | "app/controllers" => DirectoryPurpose::Controller,
            "views" | "app/views" => DirectoryPurpose::View,
            "routes" => DirectoryPurpose::Route,
            "helpers" | "app/helpers" => DirectoryPurpose::Helper,
            "mailers" | "app/mailers" => DirectoryPurpose::Mailer,
            "jobs" | "app/jobs" => DirectoryPurpose::Job,
            "serializers" | "app/serializers" => DirectoryPurpose::Serializer,
            "migrations" | "db/migrate" => DirectoryPurpose::Migration,
            "config" => DirectoryPurpose::Config,
            "lib" => DirectoryPurpose::Lib,
            "services" => DirectoryPurpose::Service,
            "components" => DirectoryPurpose::Component,
            "utils" | "libs" => DirectoryPurpose::Util,
            "styles" | "stylesheets" => DirectoryPurpose::Style,
            "tests" | "spec" => DirectoryPurpose::Test,
            _ => DirectoryPurpose::Unknown,
        }
    }

    fn analyze_file(&self, file_path: &Path) -> Result<FileStructureAnalysisResult> {
        let mut result = FileStructureAnalysisResult::default();

        // Get file metadata
        let metadata = std::fs::metadata(file_path)
            .context(format!("Failed to get metadata for file: {}", file_path.display()))?;

        let file_type = if metadata.is_file() {
            "file".to_string()
        } else if metadata.is_dir() {
            "directory".to_string()
        } else {
            "unknown".to_string()
        };

        let size = if file_type == "file" {
            metadata.len()
        } else {
            0
        };

        let modified_time = metadata
            .modified()
            .context("Failed to get modification time")?
            .duration_since(std::time::UNIX_EPOCH)
            .context("Failed to calculate duration since epoch")?
            .as_secs();

        // Get relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .unwrap_or(file_path)
            .to_path_buf();

        let parent_directory = relative_path.parent().map(PathBuf::from);

        let mut file_metadata = FileMetadata {
            name: relative_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            parent_directory: parent_directory.clone(),
            relative_path: relative_path.clone(),
            absolute_path: file_path.to_path_buf(),
            file_type,
            size,
            modified_time,
            dependencies: Vec::new(),
        };

        // Detect imports if it's a code file
        if file_metadata.file_type == "file" {
            if let Some(dependencies) = self.detect_imports(&file_path, &relative_path)? {
                file_metadata.dependencies = dependencies.clone();

                // Update file dependency graph
                result.file_dependency_graph.insert(
                    relative_path.clone(),
                    dependencies,
                );
            }
        }

        // Add file metadata to result
        result.file_metadata.insert(relative_path, file_metadata);

        Ok(result)
    }

    fn detect_imports(&self, file_path: &Path, relative_path: &Path) -> Result<Option<Vec<PathBuf>>> {
        let file_type = self.detect_file_type(file_path);
        if file_type == FileType::Unknown {
            return Ok(None);
        }

        // Read file content
        let content = std::fs::read_to_string(file_path)
            .context(format!("Failed to read file {}", file_path.display()))?;

        let regex = match file_type {
            FileType::Rust => &*RUST_IMPORT_REGEX,
            FileType::JavaScript | FileType::JSX => &*JAVASCRIPT_IMPORT_REQUIRE_REGEX,
            FileType::TypeScript | FileType::TSX => &*TYPESCRIPT_IMPORT_REGEX,
            FileType::Ruby => &*RUBY_REQUIRE_REGEX,
            FileType::Python => &*PYTHON_IMPORT_REGEX,
            _ => return Ok(None),
        };

        let imports = self.detect_imports_with_regex(&content, regex);

        if imports.is_empty() {
            return Ok(None);
        }

        let file_dir = file_path.parent().unwrap_or(Path::new(""));
        let import_paths: Vec<PathBuf> = imports
            .into_iter()
            .filter_map(|import| {
                let import_path = Path::new(&import);

                let full_import_path = if import_path.is_absolute() {
                    PathBuf::from(import_path)
                } else if import_path.starts_with("./") || import_path.starts_with("../") {
                    // Relative import
                    file_dir.join(import_path)
                } else {
                    // Module import
                    self.base_dir.join(import_path)
                };

                // Convert to relative path
                if let Ok(rel_path) = full_import_path.strip_prefix(&self.base_dir) {
                    Some(PathBuf::from(rel_path))
                } else {
                    None
                }
            })
            .collect();

        if import_paths.is_empty() {
            Ok(None)
        } else {
            Ok(Some(import_paths))
        }
    }

    fn detect_imports_with_regex(&self, content: &str, regex: &Regex) -> Vec<String> {
        regex
            .captures_iter(content)
            .filter_map(|caps| {
                caps.get(1).or_else(|| caps.get(2)).map(|m| m.as_str().to_string())
            })
            .collect()
    }

    fn detect_file_type(&self, path: &Path) -> FileType {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => FileType::Rust,
            Some("js") => FileType::JavaScript,
            Some("jsx") => FileType::JSX,
            Some("ts") => FileType::TypeScript,
            Some("tsx") => FileType::TSX,
            Some("rb") => FileType::Ruby,
            Some("py") => FileType::Python,
            _ => FileType::Unknown,
        }
    }

    pub fn generate_report(&self, result: &FileStructureAnalysisResult) -> Result<String> {
        // Generate a markdown report
        let mut report = String::new();

        // Header
        report.push_str("# File Structure Analysis Report\n\n");
        report.push_str(&format!("_Generated on: {}_\n\n", chrono::Local::now().format("%Y-%m-%d")));

        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Files**: {}\n", result.file_metadata.len()));
        report.push_str(&format!("- **Total Directories**: {}\n", result.directory_metadata.len()));
        report.push_str(&format!("- **Files with Dependencies**: {}\n", result.file_dependency_graph.len()));

        // Directory Purposes
        report.push_str("\n## Directory Purposes\n\n");
        report.push_str("| Directory | Purpose |\n");
        report.push_str("|-----------|--------|\n");

        let mut sorted_dirs: Vec<_> = result.directory_metadata.iter().collect();
        sorted_dirs.sort_by(|a, b| a.0.cmp(b.0));

        for (path, metadata) in sorted_dirs {
            report.push_str(&format!("| `{}` | {:?} |\n", path.display(), metadata.purpose));
        }

        // Files with Most Dependencies
        report.push_str("\n## Files with Most Dependencies\n\n");
        report.push_str("| File | Dependencies |\n");
        report.push_str("|------|--------------|n");

        let mut files_by_deps: Vec<_> = result.file_dependency_graph.iter().collect();
        files_by_deps.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (path, deps) in files_by_deps.iter().take(20) {
            report.push_str(&format!("| `{}` | {} |\n", path.display(), deps.len()));
        }

        // Most Referenced Files
        report.push_str("\n## Most Referenced Files\n\n");
        report.push_str("| File | References |\n");
        report.push_str("|------|------------|n");

        // Count references to each file
        let mut reference_counts: HashMap<PathBuf, usize> = HashMap::new();
        for (_, deps) in &result.file_dependency_graph {
            for dep in deps {
                *reference_counts.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        let mut files_by_refs: Vec<_> = reference_counts.iter().collect();
        files_by_refs.sort_by(|a, b| b.1.cmp(a.1));

        for (path, count) in files_by_refs.iter().take(20) {
            report.push_str(&format!("| `{}` | {} |\n", path.display(), count));
        }

        Ok(report)
    }

    pub fn export_to_json(&self, result: &FileStructureAnalysisResult) -> Result<String> {
        let json = serde_json::to_string_pretty(result)
            .context("Failed to serialize file structure analysis result to JSON")?;

        Ok(json)
    }
}

impl IncrementalAnalyzer<FileStructureAnalysisResult> for IncrementalFileStructureAnalyzer {
    fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    fn cache_path(&self) -> Option<&Path> {
        self.cache_path.as_deref()
    }

    fn use_incremental(&self) -> bool {
        self.use_incremental
    }

    fn config_hash(&self) -> String {
        use crate::utils::incremental_analyzer::calculate_hash;

        // Create a simple configuration object for hashing
        let config = (
            &self.exclude_dirs,
            &self.include_extensions,
        );

        calculate_hash(&config)
    }

    fn should_exclude_file(&self, file_path: &Path) -> bool {
        // Check if the file is in an excluded directory
        for dir in &self.exclude_dirs {
            if file_path.to_string_lossy().contains(dir) {
                return true;
            }
        }

        // If it's a directory, don't exclude it
        if file_path.is_dir() {
            return false;
        }

        // Check if the file has an included extension
        if let Some(ext) = file_path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return !self.include_extensions.contains(&ext_str.to_string());
            }
        }

        true // Exclude by default if no extension
    }

    fn analyze_file(&self, file_path: &Path) -> Result<FileStructureAnalysisResult> {
        self.analyze_file(file_path)
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

    fn setup_test_directory() -> (TempDir, IncrementalFileStructureAnalyzer) {
        let dir = tempdir().unwrap();
        let analyzer = IncrementalFileStructureAnalyzer::new(dir.path().to_path_buf());
        (dir, analyzer)
    }

    #[test]
    fn test_analyze_rust_file() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        // Create a Rust file with imports
        let rust_content = r#"
        use std::path::Path;
        use serde::{Serialize, Deserialize};

        mod utils;

        fn main() {
            println!("Hello, world!");
        }
        "#;

        let file_path = create_test_file(dir.path(), "main.rs", rust_content);

        let result = analyzer.analyze_file(&file_path)?;

        assert_eq!(result.file_metadata.len(), 1);

        let file_metadata = result.file_metadata.values().next().unwrap();
        assert_eq!(file_metadata.name, "main.rs");

        // Should detect imports
        assert!(result.file_dependency_graph.contains_key(&PathBuf::from("main.rs")));

        Ok(())
    }

    #[test]
    fn test_incremental_analysis() -> Result<()> {
        let (dir, analyzer) = setup_test_directory();

        // Create a models directory
        std::fs::create_dir_all(dir.path().join("models"))?;

        // Create a Rust file with imports
        let rust_content = r#"
        use std::path::Path;

        fn main() {
            println!("Hello, world!");
        }
        "#;

        let file_path = create_test_file(dir.path(), "main.rs", rust_content);

        // First analysis
        let result1 = analyzer.analyze()?;

        // Check that file was analyzed
        assert!(result1.file_metadata.contains_key(&PathBuf::from("main.rs")));

        // Check that directory was categorized
        assert!(result1.directory_metadata.contains_key(&PathBuf::from("models")));

        // Check that the cache file was created
        let cache_path = dir.path().join(".file_structure_analyzer_cache.json");
        assert!(cache_path.exists());

        // Create a new analyzer with the same cache path
        let analyzer2 = IncrementalFileStructureAnalyzer::new(dir.path().to_path_buf());

        // Second analysis - should use the cache
        let result2 = analyzer2.analyze()?;

        // Results should be the same
        assert_eq!(result1.file_metadata.len(), result2.file_metadata.len());
        assert_eq!(result1.directory_metadata.len(), result2.directory_metadata.len());

        // Modify the Rust file
        let new_rust_content = r#"
        use std::path::Path;
        use serde::{Serialize, Deserialize};

        fn main() {
            println!("Hello, world!");
        }
        "#;

        let _ = create_test_file(dir.path(), "main.rs", new_rust_content);

        // Third analysis - should detect the new import
        let result3 = analyzer2.analyze()?;

        // File metadata should be updated
        let deps = result3.file_dependency_graph.get(&PathBuf::from("main.rs")).unwrap();
        assert!(deps.iter().any(|p| p.to_string_lossy().contains("serde")));

        Ok(())
    }
}
