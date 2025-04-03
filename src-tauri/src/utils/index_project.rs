use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FileEntry {
    path: String,
    is_directory: bool,
    size: u64,
    children: Vec<FileEntry>,
}

impl FileEntry {
    fn new(path: &Path, base_path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        let is_directory = metadata.is_dir();
        let size = if is_directory { 0 } else { metadata.len() };
        
        let rel_path = path.strip_prefix(base_path).unwrap_or(path);
        let path_str = rel_path.to_string_lossy().to_string();
        
        let mut children = Vec::new();
        
        if is_directory {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let child_path = entry.path();
                if let Ok(child_entry) = FileEntry::new(&child_path, base_path) {
                    children.push(child_entry);
                }
            }
        }
        
        Ok(FileEntry {
            path: path_str,
            is_directory,
            size,
            children,
        })
    }
}

pub fn create_project_index(output_path: &Path) -> io::Result<()> {
    let project_dir = PathBuf::from(".");
    let entry = FileEntry::new(&project_dir, &project_dir)?;
    
    let json = serde_json::to_string_pretty(&entry)?;
    
    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;
    
    println!("Project index created at: {}", output_path.display());
    Ok(())
}

pub fn create_missing_components_list(output_path: &Path) -> io::Result<()> {
    let mut missing = Vec::new();
    
    // Backend components to check
    let backend_components = [
        "src-tauri/src/core/auth.rs",
        "src-tauri/src/api/courses.rs",
        "src-tauri/src/api/forum.rs",
        "src-tauri/src/database/repositories/forum.rs",
        "src-tauri/src/database/repositories/course.rs",
    ];
    
    // Frontend components to check
    let frontend_components = [
        "src/components/auth/login.rs",
        "src/components/auth/register.rs",
        "src/components/forum/category_list.rs",
        "src/components/forum/topic_list.rs",
        "src/services/api.rs",
        "src/app.rs",
    ];
    
    for path in backend_components.iter().chain(frontend_components.iter()) {
        if !Path::new(path).exists() {
            missing.push(path);
        }
    }
    
    let mut file = File::create(output_path)?;
    writeln!(file, "# Missing Components")?;
    writeln!(file, "\nThe following components are missing and should be created:\n")?;
    
    for path in missing {
        writeln!(file, "- {}", path)?;
    }
    
    println!("Missing components list created at: {}", output_path.display());
    Ok(())
}

pub fn list_rust_files() -> io::Result<Vec<String>> {
    let mut rust_files = Vec::new();
    
    for entry in WalkDir::new(".") {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "rs") {
            rust_files.push(path.to_string_lossy().to_string());
        }
    }
    
    Ok(rust_files)
}

#[tauri::command]
pub fn generate_project_index() -> Result<String, String> {
    let output_path = PathBuf::from("project_index.json");
    create_project_index(&output_path).map_err(|e| e.to_string())?;
    
    let missing_components_path = PathBuf::from("missing_components.txt");
    create_missing_components_list(&missing_components_path).map_err(|e| e.to_string())?;
    
    Ok("Project indexed successfully".to_string())
}

// New functions to help with your port
pub fn analyze_port_conflicts() -> io::Result<Vec<String>> {
    let mut conflicts = Vec::new();
    
    // Get paths from the reference port folder
    let port_path = PathBuf::from(r"C:\Users\Tim\Desktop\port");
    
    // Check for file conflicts
    if port_path.exists() {
        analyze_directory_conflicts(&port_path, &PathBuf::from("."), &mut conflicts)?;
    } else {
        conflicts.push(format!("Reference port directory not found at: {}", port_path.display()));
    }
    
    Ok(conflicts)
}

fn analyze_directory_conflicts(port_dir: &Path, current_dir: &Path, conflicts: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(port_dir)? {
        let entry = entry?;
        let port_path = entry.path();
        let rel_path = port_path.strip_prefix(port_dir).unwrap_or(&port_path);
        let target_path = current_dir.join(rel_path);
        
        if port_path.is_dir() {
            if !target_path.exists() {
                // Directory doesn't exist in target - this is fine
            } else {
                analyze_directory_conflicts(&port_path, &target_path, conflicts)?;
            }
        } else {
            if target_path.exists() {
                conflicts.push(format!("File conflict: {}", target_path.display()));
            }
        }
    }
    
    Ok(())
}

#[tauri::command]
pub fn check_port_conflicts() -> Result<Vec<String>, String> {
    analyze_port_conflicts().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn check_missing_imports() -> Result<Vec<String>, String> {
    // Check for imports related to the components that were mentioned in the error
    let components_to_check = [
        "CategoriesList", "CategoryDetail", "CategoryForm", 
        "TopicsList", "TopicForm", "TopicDetail",
        "AuthProvider", "Login", "Register", "Layout"
    ];
    
    let mut missing_imports = Vec::new();
    
    // Check in Rust files for the component definitions
    let rust_files = list_rust_files().map_err(|e| e.to_string())?;
    
    for component in &components_to_check {
        let mut found = false;
        
        for file_path in &rust_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains(&format!("struct {}", component)) || 
                   content.contains(&format!("fn {}(", component)) {
                    found = true;
                    break;
                }
            }
        }
        
        if !found {
            missing_imports.push(component.to_string());
        }
    }
    
    Ok(missing_imports)
}