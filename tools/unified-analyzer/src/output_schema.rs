rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
pub struct UnifiedOutput {
    pub files: Vec<FileMetadata>,
    // Add more fields here
    pub file_dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileMetadata {
    pub path: String,
    pub file_type: String,
    pub size: u64,
    pub modified_time: String,
}