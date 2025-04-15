use std::path::{Path, PathBuf};
use std::collections::HashMap;

pub struct CodeMetrics {
    pub file_complexity: HashMap<PathBuf, u32>,
    pub components: Vec<ComponentInfo>,
    pub total_complexity: u32,
    pub average_complexity: f32,
    pub file_count: usize,
}

impl Default for CodeMetrics {
    fn default() -> Self {
        Self {
            file_complexity: HashMap::new(),
            components: Vec::new(),
            total_complexity: 0,
            average_complexity: 0.0,
            file_count: 0,
        }
    }
}

impl CodeMetrics {
    pub fn add_file_complexity(&mut self, path: PathBuf, complexity: u32) {
        self.file_complexity.insert(path, complexity);
    }
}

pub struct ComponentInfo {
    pub name: String,
    pub file_path: PathBuf,
    pub complexity: u32,
    pub type_name: String,  // "React", "Vue", "Rust", etc.
    pub props: Vec<PropInfo>,
    pub lines_of_code: usize,
}

pub struct PropInfo {
    pub name: String,
    pub prop_type: String,
    pub is_required: bool,
}

pub struct AstAnalyzer;

impl AstAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn analyze_project_code(&self, _base_dir: &Path) -> CodeMetrics {
        // This is a simplified implementation
        CodeMetrics::default()
    }
}
