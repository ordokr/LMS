use std::fmt;
use std::error::Error;

/// Custom error type for generator operations
#[derive(Debug)]
pub enum GeneratorError {
    /// Error when creating a directory
    DirectoryCreation(String),
    /// Error when writing to a file
    FileWrite(String),
    /// Error when reading from a file
    FileRead(String),
    /// Error when parsing data
    DataParsing(String),
    /// Error when generating content
    ContentGeneration(String),
    /// Other errors
    Other(String),
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneratorError::DirectoryCreation(msg) => write!(f, "Directory creation error: {}", msg),
            GeneratorError::FileWrite(msg) => write!(f, "File write error: {}", msg),
            GeneratorError::FileRead(msg) => write!(f, "File read error: {}", msg),
            GeneratorError::DataParsing(msg) => write!(f, "Data parsing error: {}", msg),
            GeneratorError::ContentGeneration(msg) => write!(f, "Content generation error: {}", msg),
            GeneratorError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for GeneratorError {}

/// Convert std::io::Error to GeneratorError
impl From<std::io::Error> for GeneratorError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => GeneratorError::FileRead(format!("File not found: {}", error)),
            std::io::ErrorKind::PermissionDenied => GeneratorError::FileWrite(format!("Permission denied: {}", error)),
            _ => GeneratorError::Other(format!("IO error: {}", error)),
        }
    }
}

/// Result type for generator operations
pub type GeneratorResult<T> = Result<T, GeneratorError>;
