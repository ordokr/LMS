use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

pub struct FileSystemUtils;

impl FileSystemUtils {
    pub fn new() -> Self {
        Self
    }
    
    pub fn find_files(&self, base_dir: &Path, extension: &str) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(base_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
                files.push(path.to_path_buf());
            }
        }
        
        files
    }
    
    pub fn find_files_by_pattern(&self, base_dir: &Path, pattern: &str) -> Vec<PathBuf> {
        let regex = Regex::new(pattern).unwrap_or_else(|_| Regex::new(r".*").unwrap());
        let mut files = Vec::new();
        
        for entry in WalkDir::new(base_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_file() && regex.is_match(&path.to_string_lossy()) {
                files.push(path.to_path_buf());
            }
        }
        
        files
    }
    
    pub fn read_file(&self, path: &Path) -> Option<String> {
        fs::read_to_string(path).ok()
    }
    
    pub fn write_file(&self, path: &Path, content: &str) -> std::io::Result<()> {
        // Ensure the directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(path, content)
    }
}
