use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use walkdir::WalkDir;

/// Utility functions for file system operations
pub struct FileSystemUtils {
    /// Base directory
    base_dir: PathBuf,
    
    /// Patterns to exclude
    exclude_patterns: Vec<String>,
    
    /// Discovered files
    files: Vec<PathBuf>,
    
    /// File contents
    file_contents: HashMap<PathBuf, String>,
}

impl FileSystemUtils {
    /// Create a new FileSystemUtils instance
    pub fn new(base_dir: &Path, exclude_patterns: Vec<String>) -> Self {
        Self {
            base_dir: base_dir.to_path_buf(),
            exclude_patterns,
            files: Vec::new(),
            file_contents: HashMap::new(),
        }
    }
    
    /// Discover files in the base directory
    pub fn discover_files(&mut self) {
        self.files.clear();
        
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e.path()))
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            self.files.push(entry.path().to_path_buf());
        }
    }
    
    /// Read file contents
    pub fn read_file_contents(&mut self) {
        self.file_contents.clear();
        
        for file in &self.files {
            if let Ok(content) = fs::read_to_string(file) {
                self.file_contents.insert(file.clone(), content);
            }
        }
    }
    
    /// Get all discovered files
    pub fn get_all_files(&self) -> &[PathBuf] {
        &self.files
    }
    
    /// Get file content
    pub fn get_file_content(&self, path: &Path) -> Option<&String> {
        self.file_contents.get(path)
    }
    
    /// List files with a specific extension
    pub fn list_files_with_extension(&self, dir: &Path, extension: &str) -> Result<Vec<PathBuf>, String> {
        let mut files = Vec::new();
        
        if !dir.exists() {
            return Ok(files);
        }
        
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            
            if let Some(ext) = path.extension() {
                if ext == extension {
                    files.push(path.to_path_buf());
                }
            }
        }
        
        Ok(files)
    }
    
    /// Check if a path should be excluded
    fn is_excluded(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern in &self.exclude_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }
}
