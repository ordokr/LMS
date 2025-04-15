#!/usr/bin/env python
"""
Creates the utility module structure for the unified analyzer.
"""

import os
import sys
from pathlib import Path

def create_file_system_utils(src_dir):
    """Create the file_system_utils.rs file"""
    utils_dir = os.path.join(src_dir, "utils")
    os.makedirs(utils_dir, exist_ok=True)
    
    filepath = os.path.join(utils_dir, "file_system.rs")
    content = """use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use walkdir::WalkDir;

// File system utilities
pub struct FileSystemUtils {
    // Add any configuration or state as needed
}

impl FileSystemUtils {
    pub fn new() -> Self {
        Self {}
    }
    
    // List all files in a directory, recursively
    pub fn list_files(&self, dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let dir = dir.as_ref();
        
        if !dir.exists() || !dir.is_dir() {
            return Ok(files);
        }
        
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path().to_path_buf();
            if path.is_file() {
                files.push(path);
            }
        }
        
        Ok(files)
    }
    
    // Read file content
    pub fn read_file(&self, path: impl AsRef<Path>) -> io::Result<String> {
        fs::read_to_string(path)
    }
    
    // Write content to file
    pub fn write_file(&self, path: impl AsRef<Path>, content: &str) -> io::Result<()> {
        // Ensure directory exists
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(path, content)
    }
    
    // Check if path exists
    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }
    
    // Get file extension
    pub fn get_extension(&self, path: impl AsRef<Path>) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_string())
    }
}
"""
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    # Create the mod.rs file
    mod_path = os.path.join(utils_dir, "mod.rs")
    mod_content = """// File system utilities
pub mod file_system;

// Reexport for easier access
pub use file_system::FileSystemUtils;
"""
    
    with open(mod_path, 'w', encoding='utf-8') as f:
        f.write(mod_content)
    
    return filepath

def create_generators_module(src_dir):
    """Create the generators module structure"""
    generators_dir = os.path.join(src_dir, "generators")
    os.makedirs(generators_dir, exist_ok=True)
    
    # Create basic generator files
    generators = [
        ("summary_report_generator.rs", create_summary_report_generator),
        ("api_doc_generator.rs", create_api_doc_generator),
        ("database_architecture_generator.rs", create_db_architecture_generator),
        ("sync_architecture_generator.rs", create_sync_architecture_generator)
    ]
    
    for filename, generator_func in generators:
        filepath = os.path.join(generators_dir, filename)
        content = generator_func()
        
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
    
    # Create the mod.rs file
    mod_path = os.path.join(generators_dir, "mod.rs")
    mod_content = """// Documentation generators
pub mod summary_report_generator;
pub mod api_doc_generator;
pub mod database_architecture_generator;
pub mod sync_architecture_generator;

// Reexport for easier access
pub use summary_report_generator::SummaryReportGenerator;
pub use api_doc_generator::ApiDocGenerator;
pub use database_architecture_generator::DatabaseArchitectureGenerator;
pub use sync_architecture_generator::SyncArchitectureGenerator;
"""
    
    with open(mod_path, 'w', encoding='utf-8') as f:
        f.write(mod_content)
    
    return generators_dir

def create_summary_report_generator():
    """Create the summary report generator"""
    return """use std::path::Path;
use std::fs;
use crate::analyzers::AnalysisResult;

// Summary report generator
pub struct SummaryReportGenerator {
    // Add any configuration or state as needed
}

impl SummaryReportGenerator {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn generate(&self, result: &AnalysisResult, output_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output_path = output_dir.as_ref().join("summary_report.md");
        
        // Ensure the directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Generate the report content
        let content = self.generate_report_content(result);
        
        // Write the report to file
        fs::write(&output_path, content)?;
        
        println!("Generated summary report at {:?}", output_path);
        
        Ok(())
    }
    
    fn generate_report_content(&self, result: &AnalysisResult) -> String {
        // Implementation details for generating the report content
        let mut content = String::new();
        
        content.push_str("# LMS Migration Analysis Summary Report\\n\\n");
        content.push_str(&format!("Generated on: {}\\n\\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        // Add more report content
        
        content
    }
}
"""

def create_api_doc_generator():
    """Create the API documentation generator"""
    return """use std::path::Path;
use std::fs;
use crate::analyzers::AnalysisResult;

// API documentation generator
pub struct ApiDocGenerator {
    // Add any configuration or state as needed
}

impl ApiDocGenerator {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn generate(&self, result: &AnalysisResult, output_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output_path = output_dir.as_ref().join("api_documentation.md");
        
        // Ensure the directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Generate the documentation content
        let content = self.generate_doc_content(result);
        
        // Write the documentation to file
        fs::write(&output_path, content)?;
        
        println!("Generated API documentation at {:?}", output_path);
        
        Ok(())
    }
    
    fn generate_doc_content(&self, result: &AnalysisResult) -> String {
        // Implementation details for generating the API documentation content
        let mut content = String::new();
        
        content.push_str("# LMS API Documentation\\n\\n");
        content.push_str(&format!("Generated on: {}\\n\\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        // Add more documentation content
        
        content
    }
}
"""

def create_db_architecture_generator():
    """Create the database architecture generator"""
    return """use std::path::Path;
use std::fs;
use crate::analyzers::AnalysisResult;

// Database architecture generator
pub struct DatabaseArchitectureGenerator {
    // Add any configuration or state as needed
}

impl DatabaseArchitectureGenerator {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn generate(&self, result: &AnalysisResult, output_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output_path = output_dir.as_ref().join("database_architecture.md");
        
        // Ensure the directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Generate the documentation content
        let content = self.generate_doc_content(result);
        
        // Write the documentation to file
        fs::write(&output_path, content)?;
        
        println!("Generated database architecture documentation at {:?}", output_path);
        
        Ok(())
    }
    
    fn generate_doc_content(&self, result: &AnalysisResult) -> String {
        // Implementation details for generating the database architecture documentation content
        let mut content = String::new();
        
        content.push_str("# LMS Database Architecture\\n\\n");
        content.push_str(&format!("Generated on: {}\\n\\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        // Add more documentation content
        
        content
    }
}
"""

def create_sync_architecture_generator():
    """Create the sync architecture generator"""
    return """use std::path::Path;
use std::fs;
use crate::analyzers::AnalysisResult;

// Sync architecture generator
pub struct SyncArchitectureGenerator {
    // Add any configuration or state as needed
}

impl SyncArchitectureGenerator {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn generate(&self, result: &AnalysisResult, output_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output_path = output_dir.as_ref().join("sync_architecture.md");
        
        // Ensure the directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Generate the documentation content
        let content = self.generate_doc_content(result);
        
        // Write the documentation to file
        fs::write(&output_path, content)?;
        
        println!("Generated sync architecture documentation at {:?}", output_path);
        
        Ok(())
    }
    
    fn generate_doc_content(&self, result: &AnalysisResult) -> String {
        // Implementation details for generating the sync architecture documentation content
        let mut content = String::new();
        
        content.push_str("# LMS Sync Architecture\\n\\n");
        content.push_str(&format!("Generated on: {}\\n\\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        // Add more documentation content
        
        content
    }
}
"""

def create_unified_analyzer(src_dir):
    """Create a basic unified analyzer implementation"""
    analyzers_dir = os.path.join(src_dir, "analyzers")
    os.makedirs(analyzers_dir, exist_ok=True)
    
    filepath = os.path.join(analyzers_dir, "unified_analyzer.rs")
    content = """use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::utils::FileSystemUtils;

// Project statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub files_by_extension: HashMap<String, usize>,
    pub lines_by_extension: HashMap<String, usize>,
}

// Project analysis result
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisResult {
    pub project_name: String,
    pub stats: ProjectStats,
    pub entities: Vec<String>,
    pub components: Vec<String>,
}

// Unified project analyzer
pub struct UnifiedProjectAnalyzer {
    pub base_dir: PathBuf,
    pub fs_utils: Arc<FileSystemUtils>,
}

impl UnifiedProjectAnalyzer {
    pub fn new(base_dir: impl Into<PathBuf>, fs_utils: Arc<FileSystemUtils>) -> Self {
        Self {
            base_dir: base_dir.into(),
            fs_utils,
        }
    }
    
    // Main analysis function
    pub async fn analyze(&self) -> Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing project at {:?}...", self.base_dir);
        
        let mut result = AnalysisResult {
            project_name: self.base_dir.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ..Default::default()
        };
        
        // Collect basic file statistics
        self.collect_file_statistics(&mut result).await?;
        
        println!("Analysis complete!");
        println!("Found {} files with {} lines of code", 
            result.stats.total_files, 
            result.stats.total_lines);
        
        Ok(result)
    }
    
    // Collect file statistics
    async fn collect_file_statistics(&self, result: &mut AnalysisResult) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let files = self.fs_utils.list_files(&self.base_dir)?;
        
        result.stats.total_files = files.len();
        
        for file in files {
            if let Ok(content) = self.fs_utils.read_file(&file) {
                let line_count = content.lines().count();
                result.stats.total_lines += line_count;
                
                if let Some(ext) = self.fs_utils.get_extension(&file) {
                    let ext = ext.to_lowercase();
                    
                    // Update files by extension
                    *result.stats.files_by_extension
                        .entry(ext.clone())
                        .or_insert(0) += 1;
                    
                    // Update lines by extension
                    *result.stats.lines_by_extension
                        .entry(ext)
                        .or_insert(0) += line_count;
                }
            }
        }
        
        Ok(())
    }
}
"""
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return filepath

def main():
    # Get the location of the script
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Path to the src directory in the unified-analyzer
    src_dir = os.path.join(script_dir, "unified-analyzer", "src")
    os.makedirs(src_dir, exist_ok=True)
    
    # Create the file system utilities
    fs_utils_path = create_file_system_utils(src_dir)
    print(f"Created file system utilities at {fs_utils_path}")
    
    # Create the generators module
    generators_dir = create_generators_module(src_dir)
    print(f"Created generators module at {generators_dir}")
    
    # Create the unified analyzer
    unified_analyzer_path = create_unified_analyzer(src_dir)
    print(f"Created unified analyzer at {unified_analyzer_path}")
    
    print("\nCreated all necessary utility modules for the unified analyzer.")
    print("These modules are required for the integrated_migration_analyzer.rs to compile.")

if __name__ == "__main__":
    main()