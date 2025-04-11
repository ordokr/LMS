use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use regex::Regex;
use serde::{Serialize, Deserialize};

/// Project structure information
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectStructure {
    pub directories: HashSet<String>,
    pub files_by_type: HashMap<String, Vec<String>>,
    pub files_by_dir: HashMap<String, Vec<String>>,
    pub dir_categories: DirCategories,
}

/// Directory categories for classification
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DirCategories {
    pub api: HashSet<String>,
    pub models: HashSet<String>,
    pub ui: HashSet<String>,
    pub tests: HashSet<String>,
    pub services: HashSet<String>,
}

/// File system utilities for project analysis
pub struct FileSystemUtils {
    base_dir: PathBuf,
    exclude_patterns: Vec<Regex>,
    all_files: Vec<PathBuf>,
    file_contents: HashMap<PathBuf, String>,
    project_structure: ProjectStructure,
    keyword_index: HashMap<String, Vec<PathBuf>>,
}

impl FileSystemUtils {
    /// Create a new FileSystemUtils instance
    pub fn new<P: AsRef<Path>>(base_dir: P, exclude_patterns: Vec<&str>) -> Self {
        let compiled_patterns = exclude_patterns
            .iter()
            .filter_map(|pattern| Regex::new(pattern).ok())
            .collect();

        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            exclude_patterns: compiled_patterns,
            all_files: Vec::new(),
            file_contents: HashMap::new(),
            project_structure: ProjectStructure::default(),
            keyword_index: HashMap::new(),
        }
    }

    /// Discover all relevant files in the base directory
    pub fn discover_files(&mut self) -> &Vec<PathBuf> {
        println!("Discovering files...");
        self.all_files = self.walk_dir(&self.base_dir);
        println!("Found {} files", self.all_files.len());
        &self.all_files
    }

    /// Walk directory and collect files
    fn walk_dir(&mut self, dir: &Path) -> Vec<PathBuf> {
        let mut file_list = Vec::new();

        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path().to_path_buf();
            let relative_path = path
                .strip_prefix(&self.base_dir)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            // Skip if matches any exclude pattern
            if self
                .exclude_patterns
                .iter()
                .any(|pattern| pattern.is_match(&relative_path))
            {
                continue;
            }

            if entry.file_type().is_dir() {
                // Track directory for project structure analysis
                if !relative_path.starts_with('.') {
                    // Avoid hidden dirs like .git
                    self.project_structure
                        .directories
                        .insert(relative_path.clone());
                    self.categorize_directory(&relative_path);
                }
            } else {
                // Process file
                let ext = path
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                // Track by file type
                self.project_structure
                    .files_by_type
                    .entry(ext.clone())
                    .or_default()
                    .push(relative_path.clone());

                // Track by directory
                let dir_name = path
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .strip_prefix(&self.base_dir)
                    .unwrap_or_else(|_| Path::new(""))
                    .to_string_lossy()
                    .to_string();

                self.project_structure
                    .files_by_dir
                    .entry(dir_name)
                    .or_default()
                    .push(relative_path);

                file_list.push(path);
            }
        }

        file_list
    }

    /// Categorize directory based on its name
    fn categorize_directory(&mut self, relative_path: &str) {
        let categories = &mut self.project_structure.dir_categories;

        if relative_path.contains("api") || relative_path.contains("routes") {
            categories.api.insert(relative_path.to_string());
        } else if relative_path.contains("model") || relative_path.contains("entity") {
            categories.models.insert(relative_path.to_string());
        } else if relative_path.contains("component")
            || relative_path.contains("ui")
            || relative_path.contains("pages")
            || relative_path.contains("features")
        {
            categories.ui.insert(relative_path.to_string());
        } else if relative_path.contains("test") || relative_path.contains("spec") {
            categories.tests.insert(relative_path.to_string());
        } else if relative_path.contains("service")
            || relative_path.contains("util")
            || relative_path.contains("core")
            || relative_path.contains("shared")
        {
            categories.services.insert(relative_path.to_string());
        }
    }

    /// Read file contents, skipping binaries and large files
    pub fn read_file_contents(&mut self) -> &HashMap<PathBuf, String> {
        println!("Reading file contents with advanced processing...");

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

        // Stats for tracking processed files
        let mut read = 0;
        let mut skipped = 0;
        let mut binary = 0;
        let mut too_large = 0;
        let mut empty_file = 0;
        let mut error = 0;

        for file_path in &self.all_files {
            if let Some(ext) = file_path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if skip_extensions.contains(ext_str.as_str()) {
                    skipped += 1;
                    continue;
                }

                match fs::metadata(file_path) {
                    Ok(stats) => {
                        if stats.len() == 0 {
                            empty_file += 1;
                            continue;
                        }

                        let is_known_text = text_extensions.contains(ext_str.as_str());
                        let size_limit = if is_known_text { 5 * 1024 * 1024 } else { 1024 * 1024 };
                        if stats.len() > size_limit {
                            too_large += 1;
                            continue;
                        }

                        // Skip binary check for known text files
                        if !is_known_text && self.is_likely_binary(file_path) {
                            binary += 1;
                            continue;
                        }

                        match fs::read_to_string(file_path) {
                            Ok(content) => {
                                self.file_contents.insert(file_path.clone(), content.clone());
                                self.index_file_keywords(file_path, &content);
                                read += 1;
                            }
                            Err(_) => {
                                error += 1;
                            }
                        }
                    }
                    Err(_) => {
                        error += 1;
                    }
                }
            }
        }

        println!(
            "Read {} files, {} skipped",
            read,
            skipped + binary + too_large + empty_file
        );
        println!(
            "  Skipped: {} by extension, {} binary, {} too large, {} empty",
            skipped, binary, too_large, empty_file
        );

        &self.file_contents
    }

    /// Check if a file is likely binary
    fn is_likely_binary(&self, file_path: &Path) -> bool {
        // Common binary file signatures
        let binary_signatures: Vec<&[u8]> = vec![
            &[0xFF, 0xD8],                 // JPEG
            &[0x89, 0x50, 0x4E, 0x47],     // PNG
            &[0x47, 0x49, 0x46],           // GIF
            &[0x50, 0x4B, 0x03, 0x04],     // ZIP/JAR/DOCX
            &[0x25, 0x50, 0x44, 0x46],     // PDF
        ];

        if let Ok(content) = fs::read(file_path) {
            if content.len() < 8 {
                return false;
            }

            // Check for binary signatures
            for signature in binary_signatures {
                if content.starts_with(signature) {
                    return true;
                }
            }

            // Check for high percentage of non-ASCII characters
            let non_ascii_count = content
                .iter()
                .take(1024)
                .filter(|&&b| b > 127 || (b < 32 && b != 9 && b != 10 && b != 13))
                .count();

            if content.len() > 0 {
                let non_ascii_ratio = non_ascii_count as f64 / content.len().min(1024) as f64;
                if non_ascii_ratio > 0.3 {
                    return true;
                }
            }
        }

        false
    }

    /// Index keywords in a file for faster searching
    fn index_file_keywords(&mut self, file_path: &Path, content: &str) {
        // List of important keywords to index
        let keywords = [
            // Models
            "struct", "enum", "trait", "impl", "class", "interface", "type", "model", "entity",
            // API
            "fn", "function", "route", "get", "post", "put", "delete", "api", "endpoint", "handler",
            "router", "controller",
            // UI
            "component", "function", "render", "return", "useState", "useEffect", "props", "view",
            "page", "layout",
            // Tests
            "test", "describe", "it", "expect", "assert", "mock",
            // General
            "import", "export", "module", "require", "use", "let", "const", "var", "pub", "async",
            "await",
        ];

        for keyword in keywords.iter() {
            if content.contains(keyword) {
                self.keyword_index
                    .entry(keyword.to_string())
                    .or_default()
                    .push(file_path.to_path_buf());
            }
        }
    }

    /// Find files matching a list of regex patterns
    pub fn find_files_by_patterns(&self, patterns: &[Regex]) -> Vec<PathBuf> {
        let mut matching_files = Vec::new();

        for file_path in &self.all_files {
            let path_str = file_path.to_string_lossy();
            if patterns.iter().any(|pattern| pattern.is_match(&path_str)) {
                matching_files.push(file_path.clone());
            }
        }

        matching_files
    }

    /// Filter files by pattern
    pub fn filter_files(&self, pattern: &Regex) -> Vec<PathBuf> {
        self.all_files
            .iter()
            .filter(|file| pattern.is_match(&file.to_string_lossy()))
            .cloned()
            .collect()
    }

    /// Get directory category (models, api, ui, etc.)
    pub fn get_dir_category(&self, dir_path: &str) -> String {
        let categories = &self.project_structure.dir_categories;
        
        if categories.models.contains(dir_path) {
            return "models".to_string();
        } else if categories.api.contains(dir_path) {
            return "api".to_string();
        } else if categories.ui.contains(dir_path) {
            return "ui".to_string();
        } else if categories.tests.contains(dir_path) {
            return "tests".to_string();
        } else if categories.services.contains(dir_path) {
            return "services".to_string();
        }
        
        "other".to_string()
    }

    /// Get file statistics
    pub fn get_file_stats(&self) -> FileStats {
        let all_files = &self.all_files;
        
        // Count files by type
        let js = all_files
            .iter()
            .filter(|file| {
                let ext = file.extension().unwrap_or_default().to_string_lossy();
                ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx"
            })
            .count();
            
        let rust = all_files
            .iter()
            .filter(|file| file.extension().unwrap_or_default() == "rs")
            .count();
            
        let other = all_files.len() - js - rust;
        
        FileStats {
            total: all_files.len(),
            js,
            rust,
            other,
        }
    }

    // --- Getters for accessed properties ---
    
    /// Get all discovered files
    pub fn get_all_files(&self) -> &Vec<PathBuf> {
        &self.all_files
    }

    /// Get map of file contents
    pub fn get_file_contents_map(&self) -> &HashMap<PathBuf, String> {
        &self.file_contents
    }

    /// Get project structure information
    pub fn get_project_structure(&self) -> &ProjectStructure {
        &self.project_structure
    }

    /// Get keyword index
    pub fn get_keyword_index(&self) -> &HashMap<String, Vec<PathBuf>> {
        &self.keyword_index
    }
}

/// File statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStats {
    pub total: usize,
    pub js: usize,
    pub rust: usize,
    pub other: usize,
}
