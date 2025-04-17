use std::path::{Path, PathBuf};
use std::collections::HashMap;
use walkdir::WalkDir;

pub struct ProjectStructure {
    pub base_dir: PathBuf,
    pub directories: Vec<PathBuf>,
    pub files: HashMap<String, Vec<PathBuf>>,
}

impl ProjectStructure {
    pub fn new(base_dir: &Path) -> Self {
        let mut structure = Self {
            base_dir: base_dir.to_path_buf(),
            directories: Vec::new(),
            files: HashMap::new(),
        };

        structure.analyze();
        structure
    }

    fn analyze(&mut self) {
        for entry in WalkDir::new(&self.base_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path().to_path_buf();

            if path.is_dir() {
                self.directories.push(path);
            } else if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_string();
                self.files.entry(ext).or_insert_with(Vec::new).push(path);
            }
        }
    }

    pub fn get_file_count(&self) -> usize {
        self.files.values().map(|files| files.len()).sum()
    }

    pub fn get_directory_count(&self) -> usize {
        self.directories.len()
    }

    pub fn get_files_by_extension(&self, extension: &str) -> Vec<PathBuf> {
        self.files.get(extension).cloned().unwrap_or_default()
    }

    pub fn get_directory_structure(&self) -> Vec<String> {
        self.directories.iter()
            .map(|dir| dir.to_string_lossy().to_string())
            .collect()
    }

    pub fn get_file_paths(&self) -> Vec<PathBuf> {
        self.files.values()
            .flat_map(|files| files.iter().cloned())
            .collect()
    }
}
