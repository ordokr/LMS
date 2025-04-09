use wasm_bindgen::prelude::*;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use serde::{Serialize, Deserialize};

// Set up better panic messages in WebAssembly
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// File type statistics for a project
#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct FileTypeStats {
    pub total: usize,
    pub js: usize,
    pub rust: usize,
    pub other: usize,
}

/// Project structure for tracking files and directories
#[wasm_bindgen]
pub struct ProjectStructure {
    directories: js_sys::Array,
    files_by_type: js_sys::Map,
}

#[wasm_bindgen]
impl ProjectStructure {
    /// Creates a new empty project structure
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            directories: js_sys::Array::new(),
            files_by_type: js_sys::Map::new(),
        }
    }
    
    /// Add a directory to the project structure
    #[wasm_bindgen]
    pub fn add_directory(&mut self, dir_path: &str) {
        self.directories.push(&JsValue::from_str(dir_path));
    }
    
    /// Add a file to the project structure
    #[wasm_bindgen]
    pub fn add_file(&mut self, file_path: &str, file_type: &str) {
        let ext = JsValue::from_str(file_type);
        
        if !self.files_by_type.has(&ext) {
            self.files_by_type.set(&ext, &js_sys::Array::new());
        }
        
        let type_array = js_sys::Array::from(&self.files_by_type.get(&ext));
        type_array.push(&JsValue::from_str(file_path));
    }
    
    /// Get all directories
    #[wasm_bindgen]
    pub fn get_directories(&self) -> js_sys::Array {
        self.directories.clone()
    }
    
    /// Get all files of a specific type
    #[wasm_bindgen]
    pub fn get_files_by_type(&self, file_type: &str) -> js_sys::Array {
        let ext = JsValue::from_str(file_type);
        
        if self.files_by_type.has(&ext) {
            js_sys::Array::from(&self.files_by_type.get(&ext))
        } else {
            js_sys::Array::new()
        }
    }
}

/// WebAssembly-compatible file system utilities
#[wasm_bindgen]
pub struct FileSystemUtils {
    base_dir: String,
    files: Vec<String>,
    file_contents: HashMap<String, String>,
    project_structure: ProjectStructure,
}

#[wasm_bindgen]
impl FileSystemUtils {
    /// Create a new FileSystemUtils instance
    #[wasm_bindgen(constructor)]
    pub fn new(base_dir: &str) -> Self {
        console_error_panic_hook::set_once();
        
        Self {
            base_dir: base_dir.to_string(),
            files: Vec::new(),
            file_contents: HashMap::new(),
            project_structure: ProjectStructure::new(),
        }
    }
    
    /// Add a file to the tracked files list
    #[wasm_bindgen]
    pub fn add_file(&mut self, file_path: &str) {
        self.files.push(file_path.to_string());
        
        // Extract file type and add to project structure
        if let Some(dot_pos) = file_path.rfind('.') {
            let ext = &file_path[dot_pos..];
            self.project_structure.add_file(file_path, ext);
        } else {
            // No extension
            self.project_structure.add_file(file_path, "");
        }
    }
    
    /// Add a directory to the tracked directories
    #[wasm_bindgen]
    pub fn add_directory(&mut self, dir_path: &str) {
        self.project_structure.add_directory(dir_path);
    }
    
    /// Add file content to the content map
    #[wasm_bindgen]
    pub fn add_file_content(&mut self, file_path: &str, content: &str) {
        self.file_contents.insert(file_path.to_string(), content.to_string());
    }
    
    /// Get the count of tracked files
    #[wasm_bindgen]
    pub fn get_file_count(&self) -> usize {
        self.files.len()
    }
    
    /// Get all tracked files as a JavaScript array
    #[wasm_bindgen]
    pub fn get_all_files(&self) -> js_sys::Array {
        let result = js_sys::Array::new();
        
        for (i, file_path) in self.files.iter().enumerate() {
            result.set(i as u32, JsValue::from_str(file_path));
        }
        
        result
    }
    
    /// Get file content by path
    #[wasm_bindgen]
    pub fn get_file_content(&self, file_path: &str) -> JsValue {
        match self.file_contents.get(file_path) {
            Some(content) => JsValue::from_str(content),
            None => JsValue::null(),
        }
    }
    
    /// Get file statistics
    #[wasm_bindgen]
    pub fn get_file_stats(&self) -> FileTypeStats {
        let mut stats = FileTypeStats {
            total: self.files.len(),
            js: 0,
            rust: 0,
            other: 0,
        };
        
        for path in &self.files {
            if path.ends_with(".js") || path.ends_with(".jsx") || 
               path.ends_with(".ts") || path.ends_with(".tsx") {
                stats.js += 1;
            } else if path.ends_with(".rs") {
                stats.rust += 1;
            } else {
                stats.other += 1;
            }
        }
        
        stats
    }
    
    /// Filter files by regex pattern
    #[wasm_bindgen]
    pub fn filter_files(&self, pattern_str: &str) -> Result<js_sys::Array, JsValue> {
        let pattern = match Regex::new(pattern_str) {
            Ok(re) => re,
            Err(err) => return Err(JsValue::from_str(&format!("Invalid regex pattern: {}", err))),
        };
        
        let result = js_sys::Array::new();
        let mut index = 0;
        
        for path in &self.files {
            if pattern.is_match(path) {
                result.set(index, JsValue::from_str(path));
                index += 1;
            }
        }
        
        Ok(result)
    }
    
    /// Get the category of a directory based on its path
    #[wasm_bindgen]
    pub fn get_dir_category(&self, dir_path: &str) -> String {
        // Normalize path for consistent pattern matching
        let normalized_path = dir_path.replace('\\', "/").to_lowercase();
        
        if normalized_path.contains("/models") || normalized_path.contains("/entities") {
            "models".to_string()
        } else if normalized_path.contains("/api") || normalized_path.contains("/routes") {
            "api".to_string()
        } else if normalized_path.contains("/components") || normalized_path.contains("/views") || normalized_path.contains("/pages") {
            "ui".to_string()
        } else if normalized_path.contains("/tests") || normalized_path.contains("/__tests__") || normalized_path.contains("/test") {
            "tests".to_string()
        } else if normalized_path.contains("/services") || normalized_path.contains("/lib") {
            "service".to_string()
        } else if normalized_path.contains("/utils") || normalized_path.contains("/helpers") {
            "utility".to_string()
        } else if normalized_path.contains("/docs") || normalized_path.contains("/documentation") {
            "documentation".to_string()
        } else if normalized_path.contains("/config") {
            "configuration".to_string()
        } else {
            "other".to_string()
        }
    }
    
    /// Get the project structure
    #[wasm_bindgen]
    pub fn get_project_structure(&self) -> ProjectStructure {
        self.project_structure.clone()
    }
}
