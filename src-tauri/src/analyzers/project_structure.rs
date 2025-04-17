use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStructure {
    pub root: String,
    pub files: Vec<FileInfo>,
    pub directories: Vec<DirectoryInfo>,
    pub file_count: usize,
    pub directory_count: usize,
    pub language_stats: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryInfo {
    pub path: String,
    pub name: String,
    pub file_count: usize,
    pub subdirectory_count: usize,
}

impl ProjectStructure {
    pub fn analyze<P: AsRef<Path>>(root_path: P) -> Result<Self, String> {
        let root = root_path.as_ref().to_string_lossy().to_string();
        let mut files = Vec::new();
        let mut directories = Vec::new();
        let mut language_stats = HashMap::new();
        
        // Walk the directory tree
        for entry in WalkDir::new(&root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let relative_path = path.strip_prefix(&root_path)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
                
            if path.is_file() {
                let name = path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                    
                let extension = path.extension()
                    .map(|ext| ext.to_string_lossy().to_string());
                    
                let size = fs::metadata(path)
                    .map(|meta| meta.len())
                    .unwrap_or(0);
                    
                let language = extension.as_ref()
                    .and_then(|ext| detect_language(ext));
                    
                if let Some(lang) = &language {
                    *language_stats.entry(lang.clone()).or_insert(0) += 1;
                }
                
                files.push(FileInfo {
                    path: relative_path,
                    name,
                    extension,
                    size,
                    language,
                });
            } else if path.is_dir() {
                let name = path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                    
                let file_count = fs::read_dir(path)
                    .map(|entries| entries.filter(|e| e.is_ok() && e.as_ref().unwrap().path().is_file()).count())
                    .unwrap_or(0);
                    
                let subdirectory_count = fs::read_dir(path)
                    .map(|entries| entries.filter(|e| e.is_ok() && e.as_ref().unwrap().path().is_dir()).count())
                    .unwrap_or(0);
                    
                directories.push(DirectoryInfo {
                    path: relative_path,
                    name,
                    file_count,
                    subdirectory_count,
                });
            }
        }
        
        Ok(ProjectStructure {
            root,
            file_count: files.len(),
            directory_count: directories.len(),
            files,
            directories,
            language_stats,
        })
    }
}

fn detect_language(extension: &str) -> Option<String> {
    match extension.to_lowercase().as_str() {
        "rs" => Some("Rust".to_string()),
        "js" => Some("JavaScript".to_string()),
        "ts" => Some("TypeScript".to_string()),
        "jsx" => Some("React".to_string()),
        "tsx" => Some("React TypeScript".to_string()),
        "html" => Some("HTML".to_string()),
        "css" => Some("CSS".to_string()),
        "scss" => Some("SCSS".to_string()),
        "json" => Some("JSON".to_string()),
        "md" => Some("Markdown".to_string()),
        "py" => Some("Python".to_string()),
        "rb" => Some("Ruby".to_string()),
        "go" => Some("Go".to_string()),
        "java" => Some("Java".to_string()),
        "c" => Some("C".to_string()),
        "cpp" => Some("C++".to_string()),
        "h" => Some("C/C++ Header".to_string()),
        "hpp" => Some("C++ Header".to_string()),
        "cs" => Some("C#".to_string()),
        "php" => Some("PHP".to_string()),
        "swift" => Some("Swift".to_string()),
        "kt" => Some("Kotlin".to_string()),
        "sql" => Some("SQL".to_string()),
        "sh" => Some("Shell".to_string()),
        "bat" => Some("Batch".to_string()),
        "ps1" => Some("PowerShell".to_string()),
        "toml" => Some("TOML".to_string()),
        "yaml" | "yml" => Some("YAML".to_string()),
        _ => None,
    }
}
