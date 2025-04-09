use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct ProjectStructure {
    pub directories: HashSet<PathBuf>,
    pub files_by_type: HashMap<String, Vec<PathBuf>>,
    pub files_by_dir: HashMap<PathBuf, Vec<PathBuf>>,
    pub dir_categories: DirCategories,
}

pub struct DirCategories {
    pub api: HashSet<String>,
    pub models: HashSet<String>,
    pub ui: HashSet<String>,
    pub tests: HashSet<String>,
    pub services: HashSet<String>,
}

impl ProjectStructure {
    pub fn new() -> Self {
        Self {
            directories: HashSet::new(),
            files_by_type: HashMap::new(),
            files_by_dir: HashMap::new(),
            dir_categories: DirCategories {
                api: HashSet::new(),
                models: HashSet::new(),
                ui: HashSet::new(),
                tests: HashSet::new(),
                services: HashSet::new(),
            },
        }
    }
    
    pub fn add_directory(&mut self, path: &Path) {
        self.directories.insert(path.to_path_buf());
    }
    
    pub fn add_file(&mut self, relative_path: &Path, extension: &str) {
        // Track by file type
        self.files_by_type
            .entry(extension.to_string())
            .or_insert_with(Vec::new)
            .push(relative_path.to_path_buf());
        
        // Track by directory
        let dir = relative_path.parent().unwrap_or(Path::new("")).to_path_buf();
        self.files_by_dir
            .entry(dir)
            .or_insert_with(Vec::new)
            .push(relative_path.to_path_buf());
    }
    
    pub fn get_directories_by_category(&self, category: &str) -> HashSet<String> {
        match category {
            "api" => self.dir_categories.api.clone(),
            "models" => self.dir_categories.models.clone(),
            "ui" => self.dir_categories.ui.clone(),
            "tests" => self.dir_categories.tests.clone(),
            "services" => self.dir_categories.services.clone(),
            _ => HashSet::new(),
        }
    }
    
    pub fn get_files_by_extension(&self, extension: &str) -> Vec<PathBuf> {
        self.files_by_type.get(extension)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
}

impl Default for ProjectStructure {
    fn default() -> Self {
        Self::new()
    }
}