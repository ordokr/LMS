use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{self},
    path::PathBuf,
};
use walkdir::WalkDir;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub modules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub course_id: String,
    pub due_date: Option<String>,
    pub points_possible: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub course_id: String,
    pub items: Vec<ModuleItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: String,
    pub title: String,
    pub item_type: String,
    pub content_id: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CanvasAnalyzer {
    pub courses: HashMap<String, Course>,
    pub assignments: HashMap<String, Assignment>,
    pub modules: HashMap<String, Module>,
}

impl CanvasAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, project_path: &str) -> Result<String, CanvasError> {
        let mut result = CanvasAnalyzer::default();
        let canvas_dir = PathBuf::from(project_path);

        println!("Looking for Canvas directory at: {}", canvas_dir.display());

        if !canvas_dir.exists() {
            println!("Canvas directory not found at: {}", canvas_dir.display());
            return Ok("Canvas directory not found".to_string());
        }

        println!("Found Canvas directory at: {}", canvas_dir.display());

        // Analyze courses
        result.analyze_courses(&canvas_dir)?;

        // Analyze assignments
        result.analyze_assignments(&canvas_dir)?;

        // Analyze modules
        result.analyze_modules(&canvas_dir)?;

        Ok(serde_json::to_string_pretty(&result)?)
    }

    fn analyze_courses(&mut self, canvas_dir: &PathBuf) -> Result<(), CanvasError> {
        let courses_dir = canvas_dir.join("app").join("views").join("courses");
        if !courses_dir.exists() {
            return Ok(());
        }

        println!("Analyzing courses directory: {}", courses_dir.display());

        for entry in WalkDir::new(&courses_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "erb" || ext == "html") {
                println!("Found course file: {}", path.display());
                if let Ok(content) = fs::read_to_string(path) {
                    println!("File content length: {} bytes", content.len());

                    // Extract course information - more flexible regex that doesn't require attributes
                    let course_regex = Regex::new(r#"<h1[^>]*>(.*?)</h1>"#).unwrap();

                    if let Some(captures) = course_regex.captures(&content) {
                        let name = captures.get(1).map_or("", |m| m.as_str()).to_string();
                        println!("Found course name: {}", name);

                        // Generate a simple ID based on the file name
                        let id = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        println!("Course ID: {}", id);

                        // Extract description if available - more flexible regex
                        let description_regex = Regex::new(r#"<div class="course-description"[^>]*>(.*?)</div>"#).unwrap();
                        let description = description_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

                        if let Some(desc) = &description {
                            println!("Found course description: {}", desc);
                        }

                        // Extract modules if available - more flexible regex
                        let modules_regex = Regex::new(r#"<div class="module"[^>]*id="([^"]+)"[^>]*>"#).unwrap();
                        let modules: Vec<String> = modules_regex
                            .captures_iter(&content)
                            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                            .collect();

                        println!("Found {} modules", modules.len());
                        for module in &modules {
                            println!("  Module ID: {}", module);
                        }

                        self.courses.insert(
                            id.clone(),
                            Course {
                                id,
                                name,
                                description,
                                modules,
                            },
                        );
                    } else {
                        println!("No course information found in the file");
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_assignments(&mut self, canvas_dir: &PathBuf) -> Result<(), CanvasError> {
        let assignments_dir = canvas_dir.join("app").join("views").join("assignments");
        if !assignments_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(&assignments_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "erb" || ext == "html") {
                if let Ok(content) = fs::read_to_string(path) {
                    // Extract assignment information
                    let assignment_regex = Regex::new(r#"<h1.*?>(.*?)</h1>"#).unwrap();

                    if let Some(captures) = assignment_regex.captures(&content) {
                        let name = captures.get(1).map_or("", |m| m.as_str()).to_string();

                        // Generate a simple ID based on the file name
                        let id = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

                        // Extract description if available
                        let description_regex = Regex::new(r#"<div class="description">(.*?)</div>"#).unwrap();
                        let description = description_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

                        // Try to extract course ID from the file path
                        let course_id = path
                            .parent()
                            .and_then(|p| p.file_name())
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        // Extract due date if available
                        let due_date_regex = Regex::new(r#"<span class="due-date">(.*?)</span>"#).unwrap();
                        let due_date = due_date_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

                        // Extract points possible if available
                        let points_regex = Regex::new(r#"<span class="points">(.*?)</span>"#).unwrap();
                        let points_possible = points_regex
                            .captures(&content)
                            .and_then(|cap| cap.get(1).map(|m| m.as_str()))
                            .and_then(|s| s.parse::<f64>().ok());

                        self.assignments.insert(
                            id.clone(),
                            Assignment {
                                id,
                                name,
                                description,
                                course_id,
                                due_date,
                                points_possible,
                            },
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_modules(&mut self, canvas_dir: &PathBuf) -> Result<(), CanvasError> {
        // First check the context_modules directory
        let modules_dir = canvas_dir.join("app").join("views").join("context_modules");
        let mut found_modules = false;

        if modules_dir.exists() {
            found_modules = self.process_modules_directory(&modules_dir)?;
        }

        // If no modules found, also check for modules within course files
        if !found_modules {
            println!("No modules found in context_modules directory, checking course files...");
            let courses_dir = canvas_dir.join("app").join("views").join("courses");
            if courses_dir.exists() {
                self.extract_modules_from_courses(&courses_dir)?;
            }
        }

        Ok(())
    }

    fn process_modules_directory(&mut self, modules_dir: &PathBuf) -> Result<bool, CanvasError> {
        println!("Analyzing modules directory: {}", modules_dir.display());
        let mut found_modules = false;

        for entry in WalkDir::new(modules_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "erb" || ext == "html") {
                println!("Found module file: {}", path.display());
                if let Ok(content) = fs::read_to_string(path) {
                    // Extract module information
                    let module_regex = Regex::new(r#"<div class="module-header"[^>]*>(.*?)</div>"#).unwrap();

                    if let Some(captures) = module_regex.captures(&content) {
                        found_modules = true;
                        let name = captures.get(1).map_or("", |m| m.as_str()).to_string();
                        println!("Found module name: {}", name);

                        // Generate a simple ID based on the file name
                        let id = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        println!("Module ID: {}", id);

                        // Try to extract course ID from the file path
                        let course_id = path
                            .parent()
                            .and_then(|p| p.file_name())
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        println!("Course ID: {}", course_id);

                        // Extract module items
                        let item_regex = Regex::new(r#"<li class="module-item"[^>]*id="([^"]+)"[^>]*>(.*?)</li>"#).unwrap();
                        let title_regex = Regex::new(r#"<span class="item-title"[^>]*>(.*?)</span>"#).unwrap();
                        let type_regex = Regex::new(r#"<span class="item-type"[^>]*>(.*?)</span>"#).unwrap();

                        let mut items = Vec::new();
                        for item_capture in item_regex.captures_iter(&content) {
                            if let Some(item_id) = item_capture.get(1) {
                                let item_content = item_capture.get(2).map_or("", |m| m.as_str());

                                let title = title_regex
                                    .captures(item_content)
                                    .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                                    .unwrap_or_else(|| "Untitled".to_string());

                                let item_type = type_regex
                                    .captures(item_content)
                                    .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                                    .unwrap_or_else(|| "Unknown".to_string());

                                // Try to extract content ID if available
                                let content_id_regex = Regex::new(r#"data-content-id="([^"]+)""#).unwrap();
                                let content_id = content_id_regex
                                    .captures(item_content)
                                    .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

                                println!("  Found module item: {} ({})", title, item_type);
                                if let Some(content_id) = &content_id {
                                    println!("    Content ID: {}", content_id);
                                }

                                items.push(ModuleItem {
                                    id: item_id.as_str().to_string(),
                                    title,
                                    item_type,
                                    content_id,
                                });
                            }
                        }

                        self.modules.insert(
                            id.clone(),
                            Module {
                                id,
                                name,
                                course_id,
                                items,
                            },
                        );
                    }
                }
            }
        }

        Ok(found_modules)
    }

    fn extract_modules_from_courses(&mut self, courses_dir: &PathBuf) -> Result<(), CanvasError> {
        println!("Extracting modules from course files in: {}", courses_dir.display());

        for entry in WalkDir::new(courses_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "erb" || ext == "html") {
                println!("Checking course file for modules: {}", path.display());
                if let Ok(content) = fs::read_to_string(path) {
                    // Generate course ID based on the file name
                    let course_id_str = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

                    // Extract modules
                    let module_regex = Regex::new(r#"<div class="module"[^>]*id="([^"]+)"[^>]*>(?:[\s\S]*?)<div class="module-header"[^>]*>(.*?)</div>"#).unwrap();

                    for module_capture in module_regex.captures_iter(&content) {
                        let module_id = module_capture.get(1).map_or("", |m| m.as_str()).to_string();
                        let module_name = module_capture.get(2).map_or("", |m| m.as_str()).to_string();

                        println!("Found module in course file: {} ({})", module_name, module_id);

                        // Extract the full module content
                        let module_content_regex = Regex::new(&format!(r#"<div class="module"[^>]*id="{}"[^>]*>([\s\S]*?)</div>\s*(?:<div class="module"|$)"#, regex::escape(&module_id))).unwrap();

                        if let Some(module_content_capture) = module_content_regex.captures(&content) {
                            let module_content = module_content_capture.get(1).map_or("", |m| m.as_str());

                            // Extract module items
                            let item_regex = Regex::new(r#"<li class="module-item"[^>]*id="([^"]+)"[^>]*>(.*?)</li>"#).unwrap();
                            let title_regex = Regex::new(r#"<span class="item-title"[^>]*>(.*?)</span>"#).unwrap();
                            let type_regex = Regex::new(r#"<span class="item-type"[^>]*>(.*?)</span>"#).unwrap();

                            let mut items = Vec::new();
                            for item_capture in item_regex.captures_iter(module_content) {
                                if let Some(item_id) = item_capture.get(1) {
                                    let item_content = item_capture.get(2).map_or("", |m| m.as_str());

                                    let title = title_regex
                                        .captures(item_content)
                                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                                        .unwrap_or_else(|| "Untitled".to_string());

                                    let item_type = type_regex
                                        .captures(item_content)
                                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                                        .unwrap_or_else(|| "Unknown".to_string());

                                    // Try to extract content ID if available
                                    let content_id_regex = Regex::new(r#"data-content-id="([^"]+)""#).unwrap();
                                    let content_id = content_id_regex
                                        .captures(item_content)
                                        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

                                    println!("  Found module item: {} ({})", title, item_type);
                                    if let Some(content_id) = &content_id {
                                        println!("    Content ID: {}", content_id);
                                    }

                                    items.push(ModuleItem {
                                        id: item_id.as_str().to_string(),
                                        title,
                                        item_type,
                                        content_id,
                                    });
                                }
                            }

                            self.modules.insert(
                                module_id.clone(),
                                Module {
                                    id: module_id,
                                    name: module_name,
                                    course_id: course_id_str.clone(),
                                    items,
                                },
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CanvasError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
