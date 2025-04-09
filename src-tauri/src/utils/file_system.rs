use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use regex::Regex;

// In file_system.rs, update the import to:
use crate::analyzers::project_structure::ProjectStructure;

/// Utility struct for file system operations related to project analysis
pub struct FileSystemUtils {
    base_dir: PathBuf,
    exclude_patterns: Vec<Regex>,
    all_files: Vec<PathBuf>,
    file_contents: HashMap<PathBuf, String>,
    // Project structure will be implemented in a separate struct
    project_structure: ProjectStructure,
    keyword_index: HashMap<String, HashSet<PathBuf>>,
}

impl FileSystemUtils {
    pub fn new(base_dir: PathBuf, exclude_patterns: Vec<&str>) -> Self {
        let compiled_patterns = exclude_patterns
            .iter()
            .filter_map(|p| Regex::new(p).ok())
            .collect();
            
        Self {
            base_dir,
            exclude_patterns: compiled_patterns,
            all_files: Vec::new(),
            file_contents: HashMap::new(),
            project_structure: ProjectStructure::new(),
            keyword_index: HashMap::new(),
        }
    }
    
    /// Discovers all relevant files in the base directory, excluding specified patterns.
    /// Populates project structure information.
    pub fn discover_files(&mut self) -> &Vec<PathBuf> {
        println!("Discovering files...");
        self.all_files = self.walk_dir(&self.base_dir);
        println!("Found {} files", self.all_files.len());
        &self.all_files
    }
    
    fn walk_dir(&mut self, dir: &Path) -> Vec<PathBuf> {
        let mut file_list = Vec::new();
        
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries.filter_map(Result::ok) {
                    let file_path = entry.path();
                    let relative_path = file_path.strip_prefix(&self.base_dir).unwrap_or(&file_path);
                    let relative_path_str = relative_path.to_string_lossy();
                    
                    // Skip if matches any exclude pattern
                    if self.exclude_patterns.iter().any(|pattern| pattern.is_match(&relative_path_str)) {
                        continue;
                    }
                    
                    if file_path.is_dir() {
                        // Track directory for project structure analysis
                        if !relative_path_str.starts_with('.') {
                            self.project_structure.add_directory(relative_path);
                            self.categorize_directory(relative_path);
                        }
                        let mut sub_files = self.walk_dir(&file_path);
                        file_list.append(&mut sub_files);
                    } else {
                        // Process file
                        let ext = file_path.extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                            
                        // Track by file type and directory
                        self.project_structure.add_file(relative_path, &ext);
                        
                        file_list.push(file_path);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading directory {:?}: {}", dir, err);
            }
        }
        
        file_list
    }
    
    fn categorize_directory(&mut self, relative_path: &Path) {
        let path_str = relative_path.to_string_lossy().to_lowercase();
        let categories = &mut self.project_structure.dir_categories;
        
        if path_str.contains("api") || path_str.contains("routes") {
            categories.api.insert(path_str.to_string());
        } else if path_str.contains("model") || path_str.contains("entity") {
            categories.models.insert(path_str.to_string());
        } else if path_str.contains("component") || path_str.contains("ui") || 
                  path_str.contains("pages") || path_str.contains("features") {
            categories.ui.insert(path_str.to_string());
        } else if path_str.contains("test") || path_str.contains("spec") {
            categories.tests.insert(path_str.to_string());
        } else if path_str.contains("service") || path_str.contains("util") || 
                  path_str.contains("core") || path_str.contains("shared") {
            categories.services.insert(path_str.to_string());
        }
    }
    
    /// Reads content of discovered text files, skipping binaries and large files
    pub fn read_file_contents(&mut self) -> &HashMap<PathBuf, String> {
        println!("Reading file contents with advanced processing...");
        
        // Define binary signatures
        let binary_signatures: Vec<Vec<u8>> = vec![
            vec![0xFF, 0xD8],                // JPEG
            vec![0x89, 0x50, 0x4E, 0x47],    // PNG
            vec![0x47, 0x49, 0x46],          // GIF
            vec![0x50, 0x4B, 0x03, 0x04],    // ZIP/JAR/DOCX
            vec![0x25, 0x50, 0x44, 0x46],    // PDF
        ];
        
        // Create skip and text extension sets
        let skip_extensions: HashSet<&str> = [
            "jpg", "jpeg", "png", "gif", "bmp", "ico", "webp",
            "mp3", "mp4", "avi", "mov", "wav", "flac",
            "zip", "tar", "gz", "rar", "7z",
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
            "sqlite", "db", "jar", "class", "pdb", "lock",
            "icns",
        ].iter().cloned().collect();
        
        let text_extensions: HashSet<&str> = [
            "rs", "ts", "tsx", "js", "jsx", "vue", "svelte",
            "html", "css", "scss", "sass", "less",
            "json", "toml", "yaml", "yml",
            "md", "markdown", "txt", "gitignore", "taurignore",
            "sh", "bash", "zsh", "fish", "bat", "ps1",
            "c", "cpp", "h", "hpp", "cs", "go", "py", "rb",
            "sql", "code-workspace", "svg",
        ].iter().cloned().collect();
        
        let mut stats = FileStats::default();
        
        for file_path in &self.all_files {
            match self.process_file(file_path, &binary_signatures, &skip_extensions, &text_extensions, &mut stats) {
                Ok(Some((path, content))) => {
                    self.file_contents.insert(path, content.clone());
                    self.index_file_keywords(&path, &content);
                    stats.read += 1;
                },
                Ok(None) => {}, // File was skipped
                Err(e) => {
                    stats.error += 1;
                    eprintln!("Error reading {:?}: {}", file_path, e);
                }
            }
        }
        
        println!("Read {} files, {} skipped/error", stats.read, 
                stats.skipped + stats.binary + stats.too_large + stats.empty_file + stats.error);
        println!("  Skipped: {} by extension, {} binary, {} too large, {} empty",
                stats.skipped, stats.binary, stats.too_large, stats.empty_file);
        println!("  Errors: {}", stats.error);
                
        &self.file_contents
    }
    
    fn process_file(
        &self,
        file_path: &PathBuf,
        binary_signatures: &[Vec<u8>],
        skip_extensions: &HashSet<&str>,
        text_extensions: &HashSet<&str>,
        stats: &mut FileStats,
    ) -> io::Result<Option<(PathBuf, String)>> {
        let ext = file_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Handle files with no extension
        if ext.is_empty() && !self.is_likely_binary(file_path, binary_signatures)? {
            // Continue processing
        } else if skip_extensions.contains(ext.as_str()) {
            stats.skipped += 1;
            return Ok(None);
        }
        
        let metadata = fs::metadata(file_path)?;
        if metadata.len() == 0 {
            stats.empty_file += 1;
            return Ok(None);
        }
        
        let is_known_text = text_extensions.contains(ext.as_str()) || ext.is_empty();
        let size_limit = if is_known_text { 5 * 1024 * 1024 } else { 1024 * 1024 };
        if metadata.len() > size_limit as u64 {
            stats.too_large += 1;
            return Ok(None);
        }
        
        if !is_known_text && self.is_likely_binary(file_path, binary_signatures)? {
            stats.binary += 1;
            return Ok(None);
        }
        
        // Read the file content
        let mut content = String::new();
        File::open(file_path)?.read_to_string(&mut content)?;
        
        Ok(Some((file_path.clone(), content)))
    }
    
    fn is_likely_binary(&self, file_path: &Path, signatures: &[Vec<u8>]) -> io::Result<bool> {
        let mut buffer = [0u8; 8];
        let mut file = File::open(file_path)?;
        let bytes_read = file.read(&mut buffer)?;
        
        // Check signatures
        for signature in signatures {
            if signature.len() <= bytes_read {
                let mut matches = true;
                for (i, &byte) in signature.iter().enumerate() {
                    if buffer[i] != byte {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    return Ok(true);
                }
            }
        }
        
        // Check for null bytes (common in binary files)
        for &byte in &buffer[..bytes_read] {
            if byte == 0 {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    fn index_file_keywords(&mut self, file_path: &PathBuf, content: &str) {
        // Important keywords to index
        let keywords = [
            // Models
            "struct", "enum", "trait", "impl", "class", "interface", "type", "model", "entity",
            // API
            "fn", "function", "route", "get", "post", "put", "delete", "api", "endpoint", "handler", "router", "controller",
            // UI
            "component", "function", "render", "return", "useState", "useEffect", "props", "view", "page", "layout",
            // Tests
            "test", "describe", "it", "expect", "assert", "mock",
            // General
            "use", "mod", "import", "require", "export", "async", "await", "pub", "static", "const"
        ];
        
        for &keyword in &keywords {
            if content.contains(keyword) {
                self.keyword_index
                    .entry(keyword.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(file_path.clone());
            }
        }
    }
    
    // Public getter methods
    pub fn get_all_files(&self) -> &Vec<PathBuf> {
        &self.all_files
    }
    
    pub fn get_file_contents(&self) -> &HashMap<PathBuf, String> {
        &self.file_contents
    }
    
    pub fn get_project_structure(&self) -> &ProjectStructure {
        &self.project_structure
    }
    
    pub fn get_keyword_index(&self) -> &HashMap<String, HashSet<PathBuf>> {
        &self.keyword_index
    }
    
    // Additional utility methods
    pub fn find_files_by_patterns(&self, patterns: &[Regex]) -> Vec<PathBuf> {
        self.all_files.iter()
            .filter(|file_path| {
                let relative = file_path.strip_prefix(&self.base_dir)
                    .unwrap_or(file_path)
                    .to_string_lossy();
                    
                patterns.iter().any(|pattern| pattern.is_match(&relative))
            })
            .cloned()
            .collect()
    }
    
    pub fn get_directory_stats(&self, dir_path: &Path) -> DirectoryStats {
        let full_path = self.base_dir.join(dir_path);
        let mut stats = DirectoryStats::default();
        
        self.walk_dir_stats(&full_path, &mut stats);
        stats
    }
    
    fn walk_dir_stats(&self, dir: &Path, stats: &mut DirectoryStats) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                let relative = path.strip_prefix(&self.base_dir).unwrap_or(&path);
                
                if self.exclude_patterns.iter().any(|p| p.is_match(&relative.to_string_lossy())) {
                    continue;
                }
                
                if path.is_dir() {
                    self.walk_dir_stats(&path, stats);
                } else if let Ok(metadata) = fs::metadata(&path) {
                    stats.file_count += 1;
                    stats.total_size += metadata.len();
                }
            }
        }
    }
    
    pub fn get_dir_category(&self, dir_path: &Path) -> DirCategory {
        let path_str = dir_path.to_string_lossy().replace('\\', "/").to_lowercase();
        
        if path_str.contains("/models") || path_str.contains("/entities") {
            DirCategory::Models
        } else if path_str.contains("/api") || path_str.contains("/routes") {
            DirCategory::Api
        } else if path_str.contains("/components") || path_str.contains("/views") || path_str.contains("/pages") {
            DirCategory::Ui
        } else if path_str.contains("/tests") || path_str.contains("/__tests__") || path_str.contains("/test") {
            DirCategory::Tests
        } else if path_str.contains("/services") || path_str.contains("/lib") {
            DirCategory::Service
        } else if path_str.contains("/utils") || path_str.contains("/helpers") {
            DirCategory::Utility
        } else if path_str.contains("/docs") || path_str.contains("/documentation") {
            DirCategory::Documentation
        } else if path_str.contains("/config") {
            DirCategory::Configuration
        } else {
            DirCategory::Other
        }
    }
    
    pub fn get_file_stats(&self) -> FileTypeStats {
        let js_files = self.all_files.iter()
            .filter(|p| {
                let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx"
            })
            .count();
            
        let rust_files = self.all_files.iter()
            .filter(|p| {
                let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                ext == "rs"
            })
            .count();
            
        let other_files = self.all_files.len() - js_files - rust_files;
        
        FileTypeStats {
            total: self.all_files.len(),
            js: js_files,
            rust: rust_files,
            other: other_files,
        }
    }
}

// Define supporting structs
#[derive(Default)]
struct FileStats {
    read: usize,
    skipped: usize, 
    binary: usize,
    too_large: usize,
    empty_file: usize,
    error: usize,
}

#[derive(Default)]
pub struct DirectoryStats {
    pub file_count: usize,
    pub total_size: u64,
}

pub struct FileTypeStats {
    pub total: usize,
    pub js: usize,
    pub rust: usize,
    pub other: usize,
}

pub enum DirCategory {
    Models,
    Api,
    Ui,
    Tests,
    Service,
    Utility,
    Documentation,
    Configuration,
    Other,
}

// At the bottom of file_system.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_discover_files() {
        let temp = tempdir().unwrap();
        
        // Create test directory structure
        let test_rs = temp.path().join("test.rs");
        let mut file = File::create(&test_rs).unwrap();
        writeln!(file, "fn main() {{}}").unwrap();
        
        let sub_dir = temp.path().join("src");
        std::fs::create_dir(&sub_dir).unwrap();
        let sub_file = sub_dir.join("mod.rs");
        let mut file = File::create(&sub_file).unwrap();
        writeln!(file, "pub mod test;").unwrap();
        
        // Initialize and test
        let mut fs_utils = FileSystemUtils::new(temp.path().to_path_buf(), vec![]);
        let files = fs_utils.discover_files();
        
        assert_eq!(files.len(), 2); // Should find both test files
    }
    
    #[test]
    fn test_binary_detection() {
        let temp = tempdir().unwrap();
        
        // Create a binary file
        let bin_file = temp.path().join("test.bin");
        let mut file = File::create(&bin_file).unwrap();
        let binary_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46]; // JPEG signature
        file.write_all(&binary_data).unwrap();
        
        // Create a text file
        let text_file = temp.path().join("test.txt");
        let mut file = File::create(&text_file).unwrap();
        writeln!(file, "This is a text file").unwrap();
        
        // Test binary detection
        let fs_utils = FileSystemUtils::new(temp.path().to_path_buf(), vec![]);
        assert!(fs_utils.is_likely_binary(&bin_file).unwrap());
        assert!(!fs_utils.is_likely_binary(&text_file).unwrap());
    }
    
    // Add more tests as needed
}