rust
use chrono::{DateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::SystemTime};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
struct FileMetadata {
    file_name: String,
    file_type: String,
    file_size: u64,
    modified_date: String,
    file_path: String,
}
/*
/// The absolute path of the file.
absolute_path: PathBuf,

/// The type of the file (e.g., "file", "directory").
    file_type: String,


    /// The size of the file in bytes, if it is a file. Otherwise, 0.
    size: u64,    
    /// The last modified time of the file.
    modified_time: String,

    /// The content of the file, if it's a code file. Otherwise, an empty string.
    content: Cow<'static, str>,
}*/


#[derive(Debug)]
pub struct FileStructureAnalyzer {
    // We might need to store configuration or other data here later
    // For now, it's empty
}
/*#[derive(Debug, Clone)]
pub enum FileMetadataResult {
    Ok(FileMetadata),
    Err(FileStructureError),
}

/// Represents the type of a file based on its extension.
#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
    Rust,
    JSX,
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

fn detect_file_type(file_path: &Path) -> FileType {
        Some("rs") => FileType::Rust,
        Some("js") => FileType::JavaScript,
        Some("jsx") => FileType::JSX,
        Some("ts") => FileType::TypeScript,
        Some("tsx") => FileType::TSX,
        Some("rb") => FileType::Ruby,
        Some("py") => FileType::Python,
        _ => FileType::Unknown,
}

#[derive(Debug, ThisError)]
pub enum FileStructureError {
    /// Represents errors related to file system operations.
    #[error("File system error: {0}")]    
    FileSystemError(#[source] std::io::Error),

    /// Represents errors that occur during regex operations.
    #[error("Regex error: {0}")]    
    RegexError(#[source] regex::Error),

    /// Represents errors that occur when stripping a prefix from a path.
    #[error("Path stripping error: {0}")]    
    PathStripError(#[source] std::path::StripPrefixError),

    /// Represents errors that occur when reading a file.
    #[error("Error reading file: {file_path:?}: {source}")]    
    ReadFileError { file_path: PathBuf, source: String },
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
        &self,
        root_path: &PathBuf,
        relative_path: &PathBuf,
    ) -> Result<FileMetadata, FileStructureError> {
        let absolute_path = root_path.join(relative_path);
        let metadata = std::fs::metadata(&absolute_path).map_err(|err| {
            FileStructureError::ReadFileError {
                file_path: absolute_path.clone(),                
                source: err.to_string(),
            }
        })?;

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

        Ok(FileMetadata {
            relative_path: relative_path.clone(),
            absolute_path,
            file_type,
            size,            
            modified_time,
            content: Cow::Owned(content),
        })
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
    fn detect_imports(&self, root_path: &PathBuf, file_metadata: &FileMetadata) -> Option<Vec<PathBuf>> {
        if file_metadata.file_type != "file" {
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
        let import_paths = imports
            .into_iter()
            .map(|import| {
                let import_path = Path::new(&import);

                let mut full_import_path = if import_path.starts_with(".") {
                    file_dir.join(import_path)
                } else {
                    // Handling imports relative to the project root
                    if let Ok(relative_path) = file_dir.strip_prefix(root_path) {
                        let mut components = relative_path.components().map(|c| c.as_ref()).collect::<Vec<_>>();
                        if import_path.components().count() > 1{
                            components.extend(import_path.components().skip(1).map(|c| c.as_ref()));
                            let mut path = root_path.join(components.iter().collect::<PathBuf>());
                            if let Some(ext) = path.extension(){
                                path
                            }else{
                                path.with_extension(match detect_file_type(&path){
                                    FileType::Rust => "rs",
                                    FileType::JavaScript | FileType::JSX => "js",
                                    FileType::TypeScript | FileType::TSX => "ts",
                                    FileType::Ruby => "rb",
                                    FileType::Python => "py",
                                    _ => return None,
                                })
                            }
                        }else{
                            let mut path = root_path.join(import_path);
                            if let Some(ext) = path.extension(){
                                path
                            }else{
                                path.with_extension(match detect_file_type(&path){
                                    FileType::Rust => "rs",
                                    FileType::JavaScript | FileType::JSX => "js",
                                    FileType::TypeScript | FileType::TSX => "ts",
                                    FileType::Ruby => "rb",
                                    FileType::Python => "py",
                                    _ => return None,
                                })
                            }
                        }
                    }else{
                        root_path.join(import_path)
                    }
                }else{
                    let mut path = root_path.join(import_path);
                    if let Some(ext) = path.extension(){
                        path
                    }else{
                        path.with_extension(match detect_file_type(&path){
                            FileType::Rust => "rs",
                            FileType::JavaScript | FileType::JSX => "js",
                            FileType::TypeScript | FileType::TSX => "ts",
                            FileType::Ruby => "rb",
                            FileType::Python => "py",
                            _ => return None,
                        })
                    }
                };

                if full_import_path.is_dir(){
                    full_import_path.push("mod");
                    match detect_file_type(&full_import_path){
                        FileType::Rust => full_import_path.with_extension("rs"),
                        _ => full_import_path,
                    }
                }else{
                    if let Some(ext) = full_import_path.extension(){
                        full_import_path
                    }else{
                        match detect_file_type(&full_import_path){
                            FileType::Rust => full_import_path.with_extension("rs"),
                            FileType::JavaScript | FileType::JSX => full_import_path.with_extension("js"),
                            FileType::TypeScript | FileType::TSX => full_import_path.with_extension("ts"),
                            FileType::Ruby => full_import_path.with_extension("rb"),
                            FileType::Python => full_import_path.with_extension("py"),
                            _ => return None,
                        }
                    }
                }
            })
            .filter_map(|path| path.strip_prefix(root_path).ok().map(PathBuf::from))
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

    fn get_file_content(&self, file_path: &Path) -> Result<String, FileStructureError> {        
        std::fs::read_to_string(file_path).map_err(|e| FileStructureError::ReadFileError {
            file_path: file_path.to_path_buf(),
            source: e.to_string(),
        })
    }

    pub fn analyze(&self, root_path: &str) -> Result<String, FileStructureError> {
        let mut file_metadata_list: Vec<FileMetadata> = Vec::new();

        for entry in WalkDir::new(root_path) {
            let entry = entry.map_err(FileStructureError::IoError)?;
            let path = entry.path();
            let metadata = fs::metadata(path).map_err(FileStructureError::IoError)?;
            let file_type = if metadata.is_file() {
                "file".to_string()
            } else if metadata.is_dir() {
                "directory".to_string()
            } else {
                "unknown".to_string()
            };

            let modified_date: DateTime<Utc> = metadata
                .modified()
                .map_err(FileStructureError::IoError)?
                .into();

            let file_metadata_item = FileMetadata {
                file_name: entry.file_name().to_string_lossy().to_string(),
                file_type,
                file_size: metadata.len(),
                modified_date: modified_date.to_rfc3339(),
                file_path: path.to_string_lossy().to_string(),
            };

            file_metadata_list.push(file_metadata_item);
        }

        let json_output = serde_json::to_string_pretty(&file_metadata_list)
            .map_err(FileStructureError::JsonError)?;

        debug!("{}", json_output);

        Ok(json_output)
    }

    pub fn new() -> Self {
        FileStructureAnalyzer {}
    }
    pub fn default() -> Self {
        Self::new()
    }

    /*
}
*/
impl Default for FileStructureAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

    }
}

#[derive(Debug)]
pub enum FileStructureError {
    /// Represents errors related to file system operations.
    IoError(io::Error),
    /// Represents errors during JSON serialization.
    JsonError(serde_json::Error),
}

impl From<io::Error> for FileStructureError {
    fn from(error: io::Error) -> Self {
        FileStructureError::IoError(error)
    }
}
impl From<serde_json::Error> for FileStructureError {
    fn from(error: serde_json::Error) -> Self {
        FileStructureError::JsonError(error)
    }
}
const JAVASCRIPT_EXTENSION: &str = "js";
const TYPESCRIPT_EXTENSION: &str = "ts";
const JSX_EXTENSION: &str = "jsx";
const TSX_EXTENSION: &str = "tsx";
const RUBY_EXTENSION: &str = "rb";
const PYTHON_EXTENSION: &str = "py";

lazy_static::lazy_static! {
    /// Regex patterns for detecting imports in different file types.
    static ref RUST_IMPORT_REGEX: Regex = Regex::new(r#"(?:use|mod)\s+([\w::{}]*);?"#)
        .expect(RUST_IMPORT_REGEX_ERROR_MESSAGE);

    static ref JAVASCRIPT_IMPORT_REQUIRE_REGEX: Regex = Regex::new(
        r#"(?:import|require)(?:\s+(?:[\w\s{},*]+)\s+from)?\s+["']([^"']+)["']|require\(['"]([^'"]+)['"]\)"#
    )
    .expect(JAVASCRIPT_IMPORT_REQUIRE_REGEX_ERROR_MESSAGE);

    static ref TYPESCRIPT_IMPORT_REGEX: Regex = Regex::new(
        r#"(?:import|require)(?:\s+(?:[\w\s{},*]+)\s+from)?\s+["']([^"']+)["']"#
    )
    .expect(TYPESCRIPT_IMPORT_REGEX_ERROR_MESSAGE);

    static ref RUBY_REQUIRE_REGEX: Regex = Regex::new(r#"^\s*require\s+(['"])(.*?)\1"#)
        .expect(RUBY_REQUIRE_REGEX_ERROR_MESSAGE);

    static ref PYTHON_IMPORT_REGEX: Regex = Regex::new(r#"(?:^|\n)import (?:(?:([\w\d._]+)|([\w\d._]+) as [\w\d]+)|(?:((.*)))|(*))|(?:^|\n)from (?:([\w\d._]+) )?import (?:([\w\d._]+)|([\w\d._]+) as [\w\d]+|(?:((.*))))"#)
        .expect(PYTHON_IMPORT_REGEX_ERROR_MESSAGE);
}

/// Regex patterns error messages.
const RUST_IMPORT_REGEX_ERROR_MESSAGE: &str = "Unable to create rust import regex";
const JAVASCRIPT_IMPORT_REQUIRE_REGEX_ERROR_MESSAGE: &str = "Unable to create javascript import/require regex";
const TYPESCRIPT_IMPORT_REGEX_ERROR_MESSAGE: &str = "Unable to create typescript import regex";
const RUBY_REQUIRE_REGEX_ERROR_MESSAGE: &str = "Unable to create ruby require regex";
const PYTHON_IMPORT_REGEX_ERROR_MESSAGE: &str = "Unable to create python import regex";
