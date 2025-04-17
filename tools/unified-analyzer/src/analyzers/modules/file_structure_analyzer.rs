use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, path::{Path, PathBuf}};
use walkdir::WalkDir;
use regex::Regex;
use lazy_static::lazy_static;

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
    pub modified_time: String,
    pub content: Cow<'static, str>,
    pub dependencies: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DirectoryPurpose {
    Model,
    Controller,
    View,
    Route,
    Helper,
    Mailer,
    Job,
    Serializer,
    Migration,
    Config,
    Lib,
    Service,
    Component,
    Util,
    Style,
    Test,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryMetadata {
    pub purpose: DirectoryPurpose,
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
            modified_time: String::new(),
            content: Cow::Borrowed(""),
            dependencies: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct FileStructureAnalyzer {
    pub directory_metadata: HashMap<PathBuf, DirectoryMetadata>,
    pub file_dependency_graph: HashMap<PathBuf, Vec<PathBuf>>,
}

impl Default for FileStructureAnalyzer {
    fn default() -> Self {
        FileStructureAnalyzer {
            directory_metadata: HashMap::new(),
            file_dependency_graph: HashMap::new(),
        }
    }
}

impl FileStructureAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn analyze(&self, root_path: &str) -> Result<String, FileStructureError> {
        let mut analyzer = FileStructureAnalyzer::default();
        let root_path_buf = PathBuf::from(root_path);

        // First pass: Collect file metadata and categorize directories
        for entry in WalkDir::new(root_path).into_iter().filter_map(Result::ok) {
            let relative_path = entry.path().strip_prefix(&root_path_buf).map_err(|e| FileStructureError::PathError(e.to_string()))?.to_path_buf();
            let absolute_path = entry.path().to_path_buf();

            if entry.path().is_dir() {
                let purpose = analyzer.categorize_directory(&relative_path);
                analyzer.directory_metadata.insert(absolute_path, DirectoryMetadata { purpose });
            }
        }

        // Second pass: Analyze files and build dependency graph
        let mut file_metadata_list = Vec::new();
        for entry in WalkDir::new(root_path).into_iter().filter_map(Result::ok) {
            if entry.path().is_file() {
                let relative_path = entry.path().strip_prefix(&root_path_buf).map_err(|e| FileStructureError::PathError(e.to_string()))?.to_path_buf();
                let file_metadata = analyzer.analyze_file_metadata(&root_path_buf, &relative_path)?;
                file_metadata_list.push(file_metadata.clone());

                if !file_metadata.dependencies.is_empty() {
                    analyzer.file_dependency_graph.insert(
                        file_metadata.relative_path,
                        file_metadata.dependencies,
                    );
                }
            }
        }

        serde_json::to_string_pretty(&analyzer.file_dependency_graph).map_err(FileStructureError::JsonError)
    }

    /// Analyzes a single file to extract its metadata.
    ///
    /// This function takes the root path of the project and the relative path of a file,
    /// then retrieves metadata (size, modified time, etc.) and optionally reads the file
    /// content if it is a code file.
    ///
    /// # Arguments
    /// * `root_path` - The root directory of the project as a `PathBuf`.
    /// * `relative_path` - The path of the file relative to the root directory as a `PathBuf`.
    ///
    /// # Returns
    /// A `Result` containing the `FileMetadata` on success, or a `FileStructureError` on failure.
    fn analyze_file_metadata(
        &mut self,
        root_path: &PathBuf,
        relative_path: &PathBuf,
    ) -> Result<FileMetadata, FileStructureError> {
        let absolute_path = root_path.join(relative_path);
        let metadata = std::fs::metadata(&absolute_path)?;

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
            .modified()?
            .elapsed()
            .map_err(FileStructureError::FileSystemError)?
            .as_secs()
            .to_string();
        let content = if file_type == "file" && detect_file_type(&absolute_path).is_code() {
            self.get_file_content(&absolute_path)?
        } else {
            String::new()
        };

        let parent_directory = relative_path.parent().map(PathBuf::from);

        let mut file_metadata = FileMetadata {
            name: relative_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            parent_directory: parent_directory.clone(),
            relative_path: relative_path.clone(),
            absolute_path,
            file_type,
            size,
            modified_time,
            content: Cow::Owned(content),
            ..Default::default()
        };

        if file_metadata.file_type == "file" {
            if let Some(dependencies) = self.detect_imports(
                root_path,
                &file_metadata,
                parent_directory
                    .as_ref()
                    .and_then(|parent| {
                        self.directory_metadata
                            .get(parent)
                            .map(|dir_meta| dir_meta.purpose.clone())
                    })
                    .unwrap_or(DirectoryPurpose::Unknown),
            ) {
                file_metadata.dependencies = dependencies;
            }
        }

        // Update file dependency graph
        self.file_dependency_graph.insert(
            file_metadata.relative_path.clone(),
            file_metadata.dependencies.clone(),
        );

        Ok(file_metadata)
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

    /// Detects imports within a given file.
    ///
    /// This function reads the content of a file, determines its type, and then uses
    /// appropriate regex patterns to identify import statements. The detected imports
    /// are adjusted to be relative to the project's root path.
    ///
    /// # Arguments
    /// * `root_path` - The root directory of the project as a `PathBuf`.
    /// * `file_metadata` - Metadata of the file being analyzed, including its content.
    ///
    /// # Returns
    /// An `Option` containing a vector of import paths (relative to the project root) on success,
    /// or `None` if the file is not a code file or no imports were found.
    fn detect_imports(
        &self,
        root_path: &PathBuf,
        file_metadata: &FileMetadata,
        directory_purpose: DirectoryPurpose,
    ) -> Option<Vec<PathBuf>> {
        if file_metadata.file_type != "file" || directory_purpose == DirectoryPurpose::Migration {
            return None;
        }

        let file_type = detect_file_type(&file_metadata.absolute_path);
        if !file_type.is_code() {
            return None;
        }

        let detect_imports_fn: Box<dyn Fn(&str) -> Vec<String>> = match file_type {
            FileType::Rust => Box::new(|content| self.detect_imports_with_regex(content, &RUST_IMPORT_REGEX)),
            FileType::JavaScript | FileType::JSX => Box::new(|content| self.detect_imports_with_regex(content, &JAVASCRIPT_IMPORT_REQUIRE_REGEX)),
            FileType::TypeScript | FileType::TSX => Box::new(|content| self.detect_imports_with_regex(content, &TYPESCRIPT_IMPORT_REGEX)),
            FileType::Ruby => Box::new(|content| self.detect_imports_with_regex(content, &RUBY_REQUIRE_REGEX)),
            FileType::Python => Box::new(|content| self.detect_imports_with_regex(content, &PYTHON_IMPORT_REGEX)),
            _ => return Some(Vec::new()),
        };

        let imports = detect_imports_fn(&file_metadata.content);

        if imports.is_empty() {
            return None;
        }

        let file_dir = file_metadata.absolute_path.parent()?;
        let import_paths: Vec<PathBuf> = imports
            .into_iter()
            .map(|import| {
                let import_path = Path::new(&import);

                let mut full_import_path = if import_path.starts_with(".") {
                    file_dir.join(import_path)
                } else if let Ok(relative_path) = file_dir.strip_prefix(root_path) {
                    // Handling imports relative to the project root
                    // Convert components to strings to avoid borrowing issues
                    let rel_path_str = relative_path.to_string_lossy().to_string();
                    let import_path_str = import_path.to_string_lossy().to_string();

                    if import_path.components().count() > 1 {
                        // Combine the paths
                        let combined_path = if rel_path_str.is_empty() {
                            import_path_str
                        } else {
                            format!("{}/{}", rel_path_str, import_path_str)
                        };

                        let path = root_path.join(combined_path);
                        if path.extension().is_some() {
                            path
                        } else {
                            path.with_extension(match detect_file_type(&path) {
                                FileType::Rust => "rs",
                                FileType::JavaScript | FileType::JSX => "js",
                                FileType::TypeScript | FileType::TSX => "ts",
                                FileType::Ruby => "rb",
                                FileType::Python => "py",
                                _ => return None,
                            })
                        }
                    } else {
                        let path = root_path.join(import_path);
                        if path.extension().is_some() {
                            path
                        } else {
                            path.with_extension(match detect_file_type(&path) {
                                FileType::Rust => "rs",
                                FileType::JavaScript | FileType::JSX => "js",
                                FileType::TypeScript | FileType::TSX => "ts",
                                FileType::Ruby => "rb",
                                FileType::Python => "py",
                                _ => return None,
                            })
                        }
                    }
                } else {
                    let path = root_path.join(import_path);
                    if path.extension().is_some() {
                        path
                    } else {
                        path.with_extension(match detect_file_type(&path) {
                            FileType::Rust => "rs",
                            FileType::JavaScript | FileType::JSX => "js",
                            FileType::TypeScript | FileType::TSX => "ts",
                            FileType::Ruby => "rb",
                            FileType::Python => "py",
                            _ => return None,
                        })
                    }
                };

                let result_path = if full_import_path.is_dir() {
                    full_import_path.push("mod");
                    match detect_file_type(&full_import_path) {
                        FileType::Rust => full_import_path.with_extension("rs"),
                        _ => full_import_path,
                    }
                } else {
                    if full_import_path.extension().is_some() {
                        full_import_path
                    } else {
                        match detect_file_type(&full_import_path) {
                            FileType::Rust => full_import_path.with_extension("rs"),
                            FileType::JavaScript | FileType::JSX => full_import_path.with_extension("js"),
                            FileType::TypeScript | FileType::TSX => full_import_path.with_extension("ts"),
                            FileType::Ruby => full_import_path.with_extension("rb"),
                            FileType::Python => full_import_path.with_extension("py"),
                            _ => return None,
                        }
                    }
                };
                Some(result_path)
            })
            .filter_map(|path| {
                if let Some(path_buf) = path {
                    if let Ok(rel_path) = path_buf.strip_prefix(root_path) {
                        Some(PathBuf::from(rel_path))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        if import_paths.is_empty() {
            None
        } else {
            Some(import_paths)
        }
    }

    /// Generic method to detect imports using a regex pattern.
    ///
    /// This function takes a regex pattern and searches for matches within the file content.
    /// It extracts the import paths from the regex capture groups.
    ///
    /// # Arguments
    /// * `content` - The content of the file as a string slice.
    /// * `regex` - A compiled regex pattern to match import statements.
    ///
    /// # Returns
    /// A vector of strings, each representing an imported path.
    fn detect_imports_with_regex(&self, content: &str, regex: &Regex) -> Vec<String> {
        regex
            .captures_iter(content)
            .filter_map(|caps| {
                caps.get(1).or_else(|| caps.get(2)).map(|m| m.as_str().to_string())
            })
            .collect()
    }

    /// Compile a regex pattern and return a Regex object
    #[allow(dead_code)]
    fn compile_regex(&self, pattern: &str) -> Result<Regex, FileStructureError> {
        Regex::new(pattern).map_err(|e| {
            FileStructureError::RegexError(format!("Failed to compile regex: {}", e))
        })
    }

    fn get_file_content(&self, file_path: &Path) -> Result<String, FileStructureError> {
        match std::fs::read_to_string(file_path) {
            Ok(content) => Ok(content),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Err(FileStructureError::IoError(e))
                } else if e.kind() == std::io::ErrorKind::PermissionDenied {
                    Err(FileStructureError::IoError(e))
                } else {
                    // Use the Other variant for unexpected errors
                    Err(FileStructureError::Other(format!("Unexpected error reading file: {}", e)))
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum FileStructureError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Walkdir error: {0}")]
    WalkdirError(#[from] walkdir::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Error: {0}")]
    Other(String),

    #[error("Regex error: {0}")]
    RegexError(String),
    #[error("File system error: {0}")]
    FileSystemError(#[from] std::time::SystemTimeError),
    #[error("Path error: {0}")]
    PathError(String),
}

// These are now handled by the #[from] attribute in the thiserror derive

#[derive(Debug, PartialEq)]
pub enum FileType {
    Rust,
    JavaScript,
    JSX,
    TypeScript,
    TSX,
    Ruby,
    Python,
    Unknown,
}

impl FileType {
    fn is_code(&self) -> bool {
        !matches!(self, FileType::Unknown)
    }
}

fn detect_file_type(path: &Path) -> FileType {
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
