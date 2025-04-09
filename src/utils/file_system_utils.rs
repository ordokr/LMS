use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::{Context, Result};
use log::{error, info, warn};
use regex::Regex;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Project structure to track files and directories.
#[derive(Debug, Clone)]
pub struct ProjectStructure {
    pub directories: HashSet<String>,
    pub files_by_type: HashMap<String, Vec<String>>,
    pub files_by_dir: HashMap<String, Vec<String>>,
    pub dir_categories: DirCategories,
}

/// Directory categories for better organization.
#[derive(Debug, Clone)]
pub struct DirCategories {
    pub api: HashSet<String>,
    pub models: HashSet<String>,
    pub ui: HashSet<String>,
    pub tests: HashSet<String>,
    pub services: HashSet<String>,
}

impl DirCategories {
    /// Creates a new empty set of directory categories.
    pub fn new() -> Self {
        Self {
            api: HashSet::new(),
            models: HashSet::new(),
            ui: HashSet::new(),
            tests: HashSet::new(),
            services: HashSet::new(),
        }
    }
}

impl ProjectStructure {
    /// Creates a new empty project structure.
    pub fn new() -> Self {
        Self {
            directories: HashSet::new(),
            files_by_type: HashMap::new(),
            files_by_dir: HashMap::new(),
            dir_categories: DirCategories::new(),
        }
    }
}

/// Utility struct for file system operations related to project analysis.
pub struct FileSystemUtils {
    base_dir: PathBuf,
    exclude_patterns: Vec<Regex>,
    all_files: Vec<PathBuf>,
    file_contents: HashMap<PathBuf, String>,
    project_structure: ProjectStructure,
    keyword_index: HashMap<String, HashSet<PathBuf>>,
}

impl FileSystemUtils {
    /// Creates a new FileSystemUtils instance.
    ///
    /// # Arguments
    /// * `base_dir` - The base directory to analyze
    /// * `exclude_patterns` - Regex patterns for paths to exclude
    ///
    /// # Returns
    /// A new FileSystemUtils instance
    pub fn new<P: AsRef<Path>>(base_dir: P, exclude_patterns: Vec<Regex>) -> Self {
        let base_dir = base_dir.as_ref().to_path_buf();
        Self {
            base_dir,
            exclude_patterns,
            all_files: Vec::new(),
            file_contents: HashMap::new(),
            project_structure: ProjectStructure::new(),
            keyword_index: HashMap::new(),
        }
    }

    /// Discovers all relevant files in the base directory, excluding specified patterns.
    /// Populates project structure information.
    ///
    /// # Returns
    /// A vector of discovered file paths
    pub fn discover_files(&mut self) -> Result<&[PathBuf]> {
        info!("Discovering files...");
        self.all_files = self.walk_dir(&self.base_dir)?;
        info!("Found {} files", self.all_files.len());
        Ok(&self.all_files)
    }

    /// Recursively walks a directory to find all files.
    ///
    /// # Arguments
    /// * `dir` - The directory to walk
    ///
    /// # Returns
    /// A vector of file paths found
    fn walk_dir(&mut self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut file_list = Vec::new();
        
        if !dir.is_dir() {
            return Ok(file_list);
        }
        
        let entries = fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
            
        for entry_result in entries {
            let entry = entry_result.with_context(|| format!("Failed to read entry in {}", dir.display()))?;
            let path = entry.path();
            let relative_path = path.strip_prefix(&self.base_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            
            // Skip if matches any exclude pattern
            if self.exclude_patterns.iter().any(|pattern| pattern.is_match(&relative_path)) {
                continue;
            }
            
            if path.is_dir() {
                // Track directory for project structure analysis
                if !relative_path.starts_with(".") { // Avoid hidden dirs like .git
                    self.project_structure.directories.insert(relative_path.clone());
                    self.categorize_directory(&relative_path);
                }
                
                // Recursively walk subdirectories
                let sub_files = self.walk_dir(&path)?;
                file_list.extend(sub_files);
            } else {
                // Process file
                if let Some(ext) = path.extension() {
                    let ext = format!(".{}", ext.to_string_lossy().to_lowercase());
                    
                    // Track by file type
                    self.project_structure.files_by_type
                        .entry(ext)
                        .or_insert_with(Vec::new)
                        .push(relative_path.clone());
                }
                
                // Track by directory
                let dir_name = path.parent()
                    .and_then(|p| p.strip_prefix(&self.base_dir).ok())
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                self.project_structure.files_by_dir
                    .entry(dir_name)
                    .or_insert_with(Vec::new)
                    .push(relative_path);
                
                file_list.push(path.clone());
            }
        }
        
        Ok(file_list)
    }

    /// Categorizes a directory based on its path.
    ///
    /// # Arguments
    /// * `relative_path` - The relative path to categorize
    fn categorize_directory(&mut self, relative_path: &str) {
        let categories = &mut self.project_structure.dir_categories;
        
        if relative_path.contains("api") || relative_path.contains("routes") {
            categories.api.insert(relative_path.to_string());
        } else if relative_path.contains("model") || relative_path.contains("entity") {
            categories.models.insert(relative_path.to_string());
        } else if relative_path.contains("component") || relative_path.contains("ui") || 
                  relative_path.contains("pages") || relative_path.contains("features") {
            categories.ui.insert(relative_path.to_string());
        } else if relative_path.contains("test") || relative_path.contains("spec") {
            categories.tests.insert(relative_path.to_string());
        } else if relative_path.contains("service") || relative_path.contains("util") || 
                  relative_path.contains("core") || relative_path.contains("shared") {
            categories.services.insert(relative_path.to_string());
        }
    }

    /// Determines if a file is likely binary based on its content.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file to check
    ///
    /// # Returns
    /// `true` if the file is likely binary, `false` otherwise
    fn is_likely_binary(&self, file_path: &Path) -> bool {
        // Binary file signatures/magic numbers (hex)
        let binary_signatures: Vec<&[u8]> = vec![
            &[0xFF, 0xD8], // JPEG
            &[0x89, 0x50, 0x4E, 0x47], // PNG
            &[0x47, 0x49, 0x46], // GIF
            &[0x50, 0x4B, 0x03, 0x04], // ZIP/JAR/DOCX
            &[0x25, 0x50, 0x44, 0x46], // PDF
        ];
        
        let file = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(err) => {
                error!("Error opening file {}: {}", file_path.display(), err);
                return true; // Assume binary on error
            }
        };
        
        let mut buffer = [0u8; 8]; // Read first 8 bytes
        if let Err(err) = io::Read::read_exact(&mut file.take(8), &mut buffer) {
            error!("Error reading file {}: {}", file_path.display(), err);
            return true; // Assume binary on error
        }
        
        // Check if the file starts with any of the binary signatures
        let is_binary_sig = binary_signatures.iter().any(|signature| {
            if buffer.len() < signature.len() {
                return false;
            }
            
            for i in 0..signature.len() {
                if buffer[i] != signature[i] {
                    return false;
                }
            }
            true
        });
        
        // Check if the file contains null bytes
        let has_null_byte = buffer.contains(&0);
        
        is_binary_sig || has_null_byte
    }

    /// Reads the content of discovered text files, skipping binaries and large files.
    /// Populates the file_contents map and keyword index.
    ///
    /// # Returns
    /// A reference to the populated file contents map
    pub fn read_file_contents(&mut self) -> Result<&HashMap<PathBuf, String>> {
        info!("Reading file contents with advanced processing...");
        
        let skip_extensions: HashSet<&str> = [
            ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".ico", ".webp",
            ".mp3", ".mp4", ".avi", ".mov", ".wav", ".flac",
            ".zip", ".tar", ".gz", ".rar", ".7z",
            ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx",
            ".sqlite", ".db", ".jar", ".class", ".pdb", ".lock",
            ".icns",
        ].iter().cloned().collect();
        
        let text_extensions: HashSet<&str> = [
            ".rs", ".ts", ".tsx", ".js", ".jsx", ".vue", ".svelte",
            ".html", ".css", ".scss", ".sass", ".less",
            ".json", ".toml", ".yaml", ".yml",
            ".md", ".markdown", ".txt", ".gitignore", ".taurignore",
            ".sh", ".bash", ".zsh", ".fish", ".bat", ".ps1",
            ".c", ".cpp", ".h", ".hpp", ".cs", ".go", ".py", ".rb",
            ".sql", ".code-workspace", ".svg",
        ].iter().cloned().collect();
        
        let mut file_stats = FileStats::default();
        
        for file_path in &self.all_files {
            let ext = file_path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| format!(".{}", e.to_lowercase()))
                .unwrap_or_default();
                
            // Handle files with no extension - treat as text unless binary detected
            if ext.is_empty() && !self.is_likely_binary(file_path) {
                // Allow reading files with no extension if they seem like text
            } else if skip_extensions.contains(ext.as_str()) {
                file_stats.skipped += 1;
                continue;
            }
            
            let metadata = match fs::metadata(file_path) {
                Ok(metadata) => metadata,
                Err(err) => {
                    error!("Error getting metadata for {}: {}", file_path.display(), err);
                    file_stats.error += 1;
                    continue;
                }
            };
            
            if metadata.len() == 0 {
                file_stats.empty_file += 1;
                continue;
            }
            
            let is_known_text = text_extensions.contains(ext.as_str()) || ext.is_empty();
            let size_limit = if is_known_text { 5 * 1024 * 1024 } else { 1024 * 1024 }; // 5MB or 1MB
            
            if metadata.len() > size_limit as u64 {
                file_stats.too_large += 1;
                continue;
            }
            
            // Check binary status only if not explicitly known text or empty extension
            if !is_known_text && self.is_likely_binary(file_path) {
                file_stats.binary += 1;
                continue;
            }
            
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    self.file_contents.insert(file_path.clone(), content.clone());
                    self.index_file_keywords(file_path, &content);
                    file_stats.read += 1;
                }
                Err(err) => {
                    // Handle potential permission errors more gracefully
                    if err.kind() == io::ErrorKind::PermissionDenied {
                        warn!("Permission denied reading {}", file_path.display());
                    } else {
                        error!("Error reading {}: {}", file_path.display(), err);
                    }
                    file_stats.error += 1;
                }
            }
        }
        
        info!(
            "Read {} files, {} skipped/error",
            file_stats.read,
            file_stats.skipped + file_stats.binary + file_stats.too_large + file_stats.empty_file + file_stats.error
        );
        info!(
            "  Skipped: {} by extension, {} binary, {} too large, {} empty",
            file_stats.skipped, file_stats.binary, file_stats.too_large, file_stats.empty_file
        );
        info!("  Errors: {}", file_stats.error);
        
        Ok(&self.file_contents)
    }

    /// Index file keywords for faster searching.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file being indexed
    /// * `content` - The content of the file to index
    fn index_file_keywords(&mut self, file_path: &Path, content: &str) {
        // List of important keywords to index
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
            "use", "mod", "import", "require", "export", "async", "await", "pub", "static", "const",
        ];
        
        // Check for each keyword
        for keyword in keywords.iter() {
            if content.contains(keyword) {
                self.keyword_index
                    .entry(keyword.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(file_path.to_path_buf());
            }
        }
    }

    /// Finds files matching a list of regex patterns.
    ///
    /// # Arguments
    /// * `patterns` - The regex patterns to match against file paths
    ///
    /// # Returns
    /// A vector of matching file paths
    pub fn find_files_by_patterns(&self, patterns: &[Regex]) -> Vec<PathBuf> {
        self.all_files
            .iter()
            .filter(|file_path| {
                let relative_path = file_path
                    .strip_prefix(&self.base_dir)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| file_path.to_string_lossy().to_string());
                
                patterns.iter().any(|pattern| pattern.is_match(&relative_path))
            })
            .cloned()
            .collect()
    }

    /// Gets statistics about a directory (file count, total size).
    ///
    /// # Arguments
    /// * `dir_path` - The path to the directory to analyze
    ///
    /// # Returns
    /// Statistics for the directory including file count and total size
    pub fn get_directory_stats(&self, dir_path: &str) -> Result<DirectoryStats> {
        let mut file_count = 0;
        let mut total_size = 0;
        let full_path = self.base_dir.join(dir_path);
        
        self.walk_dir_stats(&full_path, &mut file_count, &mut total_size)?;
        
        Ok(DirectoryStats {
            file_count,
            total_size,
        })
    }

    /// Helper function to walk a directory and calculate stats.
    ///
    /// # Arguments
    /// * `dir` - The directory to walk
    /// * `file_count` - Mutable reference to the file count
    /// * `total_size` - Mutable reference to the total size
    ///
    /// # Returns
    /// Result indicating success or failure
    fn walk_dir_stats(&self, dir: &Path, file_count: &mut usize, total_size: &mut u64) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        let entries = fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
            
        for entry_result in entries {
            let entry = entry_result.with_context(|| format!("Failed to read entry in {}", dir.display()))?;
            let path = entry.path();
            let relative_path = path.strip_prefix(&self.base_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            
            // Skip if matches any exclude pattern
            if self.exclude_patterns.iter().any(|pattern| pattern.is_match(&relative_path)) {
                continue;
            }
            
            if path.is_dir() {
                self.walk_dir_stats(&path, file_count, total_size)?;
            } else {
                *file_count += 1;
                if let Ok(metadata) = fs::metadata(&path) {
                    *total_size += metadata.len();
                }
            }
        }
        
        Ok(())
    }

    /// Determines the category of a directory based on its path.
    ///
    /// # Arguments
    /// * `dir_path` - The directory path to categorize
    ///
    /// # Returns
    /// The category name (models, api, ui, tests, etc.)
    pub fn get_dir_category(&self, dir_path: &str) -> &'static str {
        // Normalize path for consistent pattern matching
        let normalized_path = dir_path.replace('\\', "/").to_lowercase();
        
        if normalized_path.contains("/models") || normalized_path.contains("/entities") {
            "models"
        } else if normalized_path.contains("/api") || normalized_path.contains("/routes") {
            "api"
        } else if normalized_path.contains("/components") || normalized_path.contains("/views") || normalized_path.contains("/pages") {
            "ui"
        } else if normalized_path.contains("/tests") || normalized_path.contains("/__tests__") || normalized_path.contains("/test") {
            "tests"
        } else if normalized_path.contains("/services") || normalized_path.contains("/lib") {
            "service"
        } else if normalized_path.contains("/utils") || normalized_path.contains("/helpers") {
            "utility"
        } else if normalized_path.contains("/docs") || normalized_path.contains("/documentation") {
            "documentation"
        } else if normalized_path.contains("/config") {
            "configuration"
        } else {
            "other"
        }
    }

    /// Filter files by pattern
    ///
    /// # Arguments
    /// * `pattern` - The regex pattern to match file paths
    ///
    /// # Returns
    /// Filtered file paths
    pub fn filter_files(&self, pattern: &Regex) -> Vec<&PathBuf> {
        if self.all_files.is_empty() {
            return Vec::new();
        }
        
        self.all_files
            .iter()
            .filter(|file| {
                let path_str = file.to_string_lossy();
                pattern.is_match(&path_str)
            })
            .collect()
    }

    /// Get file statistics
    ///
    /// # Returns
    /// File statistics with counts by file type
    pub fn get_file_stats(&self) -> FileTypeStats {
        let all_files = &self.all_files;
        
        // Count files by type
        let js = all_files.iter()
            .filter(|file| {
                let path_str = file.to_string_lossy();
                path_str.ends_with(".js") || path_str.ends_with(".jsx") || 
                path_str.ends_with(".ts") || path_str.ends_with(".tsx")
            })
            .count();
            
        let rust = all_files.iter()
            .filter(|file| file.to_string_lossy().ends_with(".rs"))
            .count();
            
        let other = all_files.len() - js - rust;
        
        FileTypeStats {
            total: all_files.len(),
            js,
            rust,
            other,
        }
    }

    // --- Getters for accessed properties ---
    
    /// Gets all discovered files.
    pub fn get_all_files(&self) -> &[PathBuf] {
        &self.all_files
    }
    
    /// Gets the map of file contents.
    pub fn get_file_contents_map(&self) -> &HashMap<PathBuf, String> {
        &self.file_contents
    }
    
    /// Gets the project structure.
    pub fn get_project_structure(&self) -> &ProjectStructure {
        &self.project_structure
    }
    
    /// Gets the keyword index.
    pub fn get_keyword_index(&self) -> &HashMap<String, HashSet<PathBuf>> {
        &self.keyword_index
    }
}

/// Statistics for a directory.
#[derive(Debug, Clone, Copy)]
pub struct DirectoryStats {
    pub file_count: usize,
    pub total_size: u64,
}

/// Statistics for file types in the project.
#[derive(Debug, Clone, Copy)]
pub struct FileTypeStats {
    pub total: usize,
    pub js: usize,
    pub rust: usize,
    pub other: usize,
}

/// Statistics for file processing.
#[derive(Debug, Clone, Copy, Default)]
struct FileStats {
    read: usize,
    skipped: usize,
    binary: usize,
    too_large: usize,
    error: usize,
    empty_file: usize,
}

// WebAssembly bindings for JavaScript interop
#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use js_sys::{Array, Object, Reflect, RegExp};
    use wasm_bindgen::prelude::*;
    use serde::{Serialize, Deserialize};
    use std::str::FromStr;

    #[wasm_bindgen]
    pub struct WasmFileSystemUtils {
        inner: Arc<Mutex<FileSystemUtils>>,
    }

    /// Serializable project structure for JS interop
    #[derive(Serialize, Deserialize)]
    pub struct JsProjectStructure {
        pub directories: Vec<String>,
        pub files_by_type: HashMap<String, Vec<String>>,
        pub files_by_dir: HashMap<String, Vec<String>>,
        pub dir_categories: JsDirCategories,
    }

    /// Serializable directory categories for JS interop
    #[derive(Serialize, Deserialize)]
    pub struct JsDirCategories {
        pub api: Vec<String>,
        pub models: Vec<String>,
        pub ui: Vec<String>,
        pub tests: Vec<String>,
        pub services: Vec<String>,
    }

    /// Serializable file stats for JS interop
    #[derive(Serialize, Deserialize)]
    pub struct JsFileTypeStats {
        pub total: usize,
        pub js: usize,
        pub rust: usize,
        pub other: usize,
    }

    #[wasm_bindgen]
    impl WasmFileSystemUtils {
        /// Create a new WasmFileSystemUtils instance
        #[wasm_bindgen(constructor)]
        pub fn new(base_dir: &str, exclude_patterns_js: &JsValue) -> Result<WasmFileSystemUtils, JsValue> {
            console_error_panic_hook::set_once(); // Better error handling for WebAssembly

            // Convert JS array of regexes to Rust Vec<Regex>
            let exclude_patterns = if exclude_patterns_js.is_array() {
                let array = Array::from(exclude_patterns_js);
                let mut patterns = Vec::new();

                for i in 0..array.length() {
                    let pattern = array.get(i);
                    if pattern.is_instance_of::<RegExp>() {
                        let regexp = RegExp::from(pattern);
                        let pattern_str = regexp.source();
                        match Regex::new(&pattern_str) {
                            Ok(regex) => patterns.push(regex),
                            Err(err) => return Err(JsValue::from_str(&format!("Invalid regex pattern: {}", err))),
                        }
                    } else if let Some(pattern) = pattern.as_string() {
                        match Regex::new(&pattern) {
                            Ok(regex) => patterns.push(regex),
                            Err(err) => return Err(JsValue::from_str(&format!("Invalid regex pattern: {}", err))),
                        }
                    }
                }
                patterns
            } else {
                Vec::new()
            };

            let fs_utils = FileSystemUtils::new(base_dir, exclude_patterns);
            Ok(WasmFileSystemUtils {
                inner: Arc::new(Mutex::new(fs_utils)),
            })
        }

        /// Discover all relevant files in the base directory
        #[wasm_bindgen]
        pub fn discover_files(&self) -> Result<usize, JsValue> {
            let mut fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(JsValue::from_str("Failed to acquire lock")),
            };

            match fs_utils.discover_files() {
                Ok(files) => Ok(files.len()),
                Err(err) => Err(JsValue::from_str(&format!("Failed to discover files: {}", err))),
            }
        }

        /// Read the content of discovered text files
        #[wasm_bindgen]
        pub fn read_file_contents(&self) -> Result<usize, JsValue> {
            let mut fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(JsValue::from_str("Failed to acquire lock")),
            };

            match fs_utils.read_file_contents() {
                Ok(contents) => Ok(contents.len()),
                Err(err) => Err(JsValue::from_str(&format!("Failed to read file contents: {}", err))),
            }
        }

        /// Get all discovered files as a JS array
        #[wasm_bindgen]
        pub fn get_all_files(&self) -> Result<JsValue, JsValue> {
            let fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(JsValue::from_str("Failed to acquire lock")),
            };

            let files = fs_utils.get_all_files();
            let js_array = Array::new_with_length(files.len() as u32);

            for (i, file) in files.iter().enumerate() {
                js_array.set(i as u32, JsValue::from_str(&file.to_string_lossy()));
            }

            Ok(js_array.into())
        }

        /// Get the project structure as a JS object
        #[wasm_bindgen]
        pub fn get_project_structure(&self) -> Result<JsValue, JsValue> {
            let fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(JsValue::from_str("Failed to acquire lock")),
            };

            let project_structure = fs_utils.get_project_structure();
            
            // Convert to JS-friendly structure
            let js_structure = JsProjectStructure {
                directories: project_structure.directories.iter().cloned().collect(),
                files_by_type: project_structure.files_by_type.clone(),
                files_by_dir: project_structure.files_by_dir.clone(),
                dir_categories: JsDirCategories {
                    api: project_structure.dir_categories.api.iter().cloned().collect(),
                    models: project_structure.dir_categories.models.iter().cloned().collect(),
                    ui: project_structure.dir_categories.ui.iter().cloned().collect(),
                    tests: project_structure.dir_categories.tests.iter().cloned().collect(),
                    services: project_structure.dir_categories.services.iter().cloned().collect(),
                },
            };

            match serde_wasm_bindgen::to_value(&js_structure) {
                Ok(value) => Ok(value),
                Err(err) => Err(JsValue::from_str(&format!("Failed to serialize project structure: {}", err))),
            }
        }

        /// Get file content by path
        #[wasm_bindgen]
        pub fn get_file_content(&self, file_path: &str) -> Result<JsValue, JsValue> {
            let fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(JsValue::from_str("Failed to acquire lock")),
            };

            let path = PathBuf::from(file_path);
            match fs_utils.get_file_contents_map().get(&path) {
                Some(content) => Ok(JsValue::from_str(content)),
                None => Ok(JsValue::null()),
            }
        }

        /// Get file statistics
        #[wasm_bindgen]
        pub fn get_file_stats(&self) -> Result<JsValue, JsValue> {
            let fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(JsValue::from_str("Failed to acquire lock")),
            };

            let stats = fs_utils.get_file_stats();
            let js_stats = JsFileTypeStats {
                total: stats.total,
                js: stats.js,
                rust: stats.rust,
                other: stats.other,
            };

            match serde_wasm_bindgen::to_value(&js_stats) {
                Ok(value) => Ok(value),
                Err(err) => Err(JsValue::from_str(&format!("Failed to serialize file stats: {}", err))),
            }
        }

        /// Get category of a directory
        #[wasm_bindgen]
        pub fn get_dir_category(&self, dir_path: &str) -> String {
            let fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return "error".to_string(),
            };

            fs_utils.get_dir_category(dir_path).to_string()
        }

        /// Filter files by pattern
        #[wasm_bindgen]
        pub fn filter_files(&self, pattern_str: &str) -> Result<JsValue, JsValue> {
            let fs_utils = match self.inner.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(JsValue::from_str("Failed to acquire lock")),
            };

            let pattern = match Regex::new(pattern_str) {
                Ok(re) => re,
                Err(err) => return Err(JsValue::from_str(&format!("Invalid regex pattern: {}", err))),
            };

            let filtered_files = fs_utils.filter_files(&pattern);
            let js_array = Array::new_with_length(filtered_files.len() as u32);

            for (i, file) in filtered_files.iter().enumerate() {
                js_array.set(i as u32, JsValue::from_str(&file.to_string_lossy()));
            }

            Ok(js_array.into())
        }
    }
}
