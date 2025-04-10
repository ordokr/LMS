use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

// Structure to represent a JavaScript file that needs migration
#[derive(Debug, Serialize, Deserialize)]
pub struct JsFile {
    pub path: String,
    pub relative_path: String,
    pub rust_equivalent_path: Option<String>,
    pub migration_status: MigrationStatus,
    pub priority: u8,  // 1-10 scale, 10 being highest priority
    pub complexity: u8, // 1-10 scale, 10 being most complex
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MigrationStatus {
    NotStarted,
    InProgress,
    Completed,
    NotNeeded,  // For JS files that don't need migration (e.g., test configs)
}

// Structure to hold the JS migration analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct JsMigrationAnalysis {
    pub total_js_files: usize,
    pub migrated_count: usize,
    pub not_started_count: usize,
    pub in_progress_count: usize,
    pub not_needed_count: usize,
    pub js_files: Vec<JsFile>,
    pub high_priority_files: Vec<String>,
    pub completion_percentage: f32,
}

impl JsMigrationAnalysis {
    pub fn new() -> Self {
        JsMigrationAnalysis {
            total_js_files: 0,
            migrated_count: 0,
            not_started_count: 0,
            in_progress_count: 0,
            not_needed_count: 0,
            js_files: Vec::new(),
            high_priority_files: Vec::new(),
            completion_percentage: 0.0,
        }
    }
}

pub fn analyze_js_migration(root_dir: &Path) -> Result<JsMigrationAnalysis, Box<dyn Error>> {
    let mut analysis = JsMigrationAnalysis::new();
    let mut js_files = Vec::new();
    
    // Read the migration tracking file to get the status of known files
    let migration_tracking = read_migration_tracking()?;
    
    // Find all JS files
    for entry in WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "js"))
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(root_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        
        // Skip node_modules and similar
        if relative_path.contains("node_modules") || 
           relative_path.contains("coverage") ||
           relative_path.contains("dist") {
            continue;
        }
        
        // Determine migration status from the tracking file
        let (status, rust_path) = determine_migration_status(&relative_path, &migration_tracking);
        
        // Calculate priority based on file path
        let priority = calculate_priority(&relative_path);
        
        // Calculate complexity based on file size and content
        let complexity = calculate_complexity(path)?;
        
        // Find dependencies
        let dependencies = find_dependencies(path)?;
        
        let js_file = JsFile {
            path: path.to_string_lossy().to_string(),
            relative_path,
            rust_equivalent_path: rust_path,
            migration_status: status.clone(),
            priority,
            complexity,
            dependencies,
        };
        
        // Update counters
        match status {
            MigrationStatus::Completed => analysis.migrated_count += 1,
            MigrationStatus::NotStarted => analysis.not_started_count += 1,
            MigrationStatus::InProgress => analysis.in_progress_count += 1,
            MigrationStatus::NotNeeded => analysis.not_needed_count += 1,
        }
        
        // Add to high priority list if priority is high
        if priority >= 8 {
            analysis.high_priority_files.push(js_file.relative_path.clone());
        }
        
        js_files.push(js_file);
    }
    
    analysis.total_js_files = js_files.len();
    analysis.js_files = js_files;
    
    // Calculate completion percentage
    if analysis.total_js_files > 0 {
        analysis.completion_percentage = (analysis.migrated_count as f32 + analysis.not_needed_count as f32) / 
                                         (analysis.total_js_files as f32) * 100.0;
    }
    
    Ok(analysis)
}

// Read the migration tracking markdown file to determine status of known files
fn read_migration_tracking() -> Result<HashMap<String, (MigrationStatus, Option<String>)>, Box<dyn Error>> {
    let mut result = HashMap::new();
    
    // Try to read the migration tracking file
    let tracking_path = "JavaScript to Rust Migration Tracking.md";
    if let Ok(content) = fs::read_to_string(tracking_path) {
        // Parse the markdown to extract completed migrations
        for line in content.lines() {
            if line.contains("→") && (line.contains("[x]") || line.contains("- [x]")) {
                // This is a completed migration
                if let Some(parts) = line.split("→").collect::<Vec<_>>().get(0..2) {
                    let js_path = parts[0].trim().replace("- [x] ", "").replace("[x] ", "");
                    let rust_path = parts[1].trim().replace(")", "");
                    result.insert(js_path, (MigrationStatus::Completed, Some(rust_path)));
                }
            } else if line.contains("→") && (line.contains("[ ]") || line.contains("- [ ]")) {
                // This is a planned but not started migration
                if let Some(parts) = line.split("→").collect::<Vec<_>>().get(0..2) {
                    let js_path = parts[0].trim().replace("- [ ] ", "").replace("[ ] ", "");
                    let rust_path = parts[1].trim().replace(")", "");
                    result.insert(js_path, (MigrationStatus::NotStarted, Some(rust_path)));
                }
            }
        }
    }
    
    Ok(result)
}

// Determine the migration status of a JS file
fn determine_migration_status(
    relative_path: &str, 
    tracking: &HashMap<String, (MigrationStatus, Option<String>)>
) -> (MigrationStatus, Option<String>) {
    // Check if the file is in the tracking list
    for (js_path, (status, rust_path)) in tracking.iter() {
        if relative_path.contains(js_path) {
            return (status.clone(), rust_path.clone());
        }
    }
    
    // If not found, determine based on path
    if relative_path.contains("test") || 
       relative_path.contains("config") ||
       relative_path.contains("jest") ||
       relative_path.ends_with(".config.js") {
        (MigrationStatus::NotNeeded, None)
    } else {
        (MigrationStatus::NotStarted, None)
    }
}

// Calculate priority for migration based on file path
fn calculate_priority(path: &str) -> u8 {
    if path.contains("src/services") || path.contains("src/api") {
        10 // Highest priority for core services and APIs
    } else if path.contains("src/models") {
        9 // High priority for data models
    } else if path.contains("src/utils") {
        8 // High priority for utilities
    } else if path.contains("src/controllers") || path.contains("src/routes") {
        7 // Medium-high priority for controllers and routes
    } else if path.contains("src") {
        6 // Medium priority for other source files
    } else if path.contains("tools") || path.contains("scripts") {
        5 // Medium-low priority for tools and scripts
    } else if path.contains("test") {
        3 // Low priority for tests
    } else {
        4 // Default medium-low priority
    }
}

// Calculate complexity based on file size and content
fn calculate_complexity(path: &Path) -> Result<u8, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    
    // Simple complexity heuristic based on file size and certain indicators
    let size_factor = content.len() as f32 / 1000.0; // Size in KB
    let import_count = content.matches("import").count() as f32;
    let function_count = content.matches("function").count() as f32;
    let class_count = content.matches("class").count() as f32;
    let async_count = content.matches("async").count() as f32;
    
    let complexity_score = size_factor * 0.3 + 
                           import_count * 0.1 + 
                           function_count * 0.2 + 
                           class_count * 0.3 + 
                           async_count * 0.1;
    
    // Convert to 1-10 scale
    let complexity = (complexity_score.min(10.0).max(1.0)).round() as u8;
    
    Ok(complexity)
}

// Find dependencies of a JS file
fn find_dependencies(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let mut dependencies = Vec::new();
    
    // Simple regex-like approach to find imports
    for line in content.lines() {
        if line.contains("import") && line.contains("from") {
            if let Some(from_part) = line.split("from").nth(1) {
                let module = from_part.trim()
                    .trim_matches(|c| c == '\'' || c == '"' || c == ';')
                    .to_string();
                
                // Only include local dependencies, not npm packages
                if module.starts_with(".") {
                    dependencies.push(module);
                }
            }
        }
    }
    
    Ok(dependencies)
}

// Generate the markdown report for JS migration status
pub fn generate_js_migration_report(analysis: &JsMigrationAnalysis) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    
    report.push_str("# JavaScript to Rust Migration Analysis\n\n");
    
    // Add summary statistics
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total JavaScript files: {}\n", analysis.total_js_files));
    report.push_str(&format!("- Migration completed: {} files ({}%)\n", 
                            analysis.migrated_count,
                            analysis.completion_percentage.round()));
    report.push_str(&format!("- Migration not started: {} files\n", analysis.not_started_count));
    report.push_str(&format!("- Migration in progress: {} files\n", analysis.in_progress_count));
    report.push_str(&format!("- Migration not needed: {} files\n\n", analysis.not_needed_count));
    
    // Add high priority files section
    report.push_str("## High Priority Files for Migration\n\n");
    if analysis.high_priority_files.is_empty() {
        report.push_str("*All high priority files have been migrated!*\n\n");
    } else {
        for file in &analysis.high_priority_files {
            report.push_str(&format!("- `{}`\n", file));
        }
        report.push_str("\n");
    }
    
    // Add files by status
    report.push_str("## Files by Migration Status\n\n");
    
    // Completed migrations
    report.push_str("### Completed Migrations\n\n");
    for file in &analysis.js_files {
        if file.migration_status == MigrationStatus::Completed {
            let rust_path = file.rust_equivalent_path.as_deref().unwrap_or("Unknown Rust path");
            report.push_str(&format!("- [x] {} → {}\n", file.relative_path, rust_path));
        }
    }
    report.push_str("\n");
    
    // In progress migrations
    report.push_str("### In Progress Migrations\n\n");
    for file in &analysis.js_files {
        if file.migration_status == MigrationStatus::InProgress {
            let rust_path = file.rust_equivalent_path.as_deref().unwrap_or("Planned Rust path");
            report.push_str(&format!("- [ ] {} → {}\n", file.relative_path, rust_path));
        }
    }
    report.push_str("\n");
    
    // Not started but needed migrations (limit to top 10 by priority)
    report.push_str("### Not Started Migrations (Top 10 by Priority)\n\n");
    let mut not_started: Vec<_> = analysis.js_files.iter()
        .filter(|f| f.migration_status == MigrationStatus::NotStarted)
        .collect();
    
    not_started.sort_by(|a, b| b.priority.cmp(&a.priority));
    
    for file in not_started.iter().take(10) {
        let suggested_path = suggest_rust_path(&file.relative_path);
        report.push_str(&format!("- [ ] {} → {}\n", file.relative_path, suggested_path));
    }
    
    Ok(report)
}

// Suggest a Rust equivalent path for a JavaScript file
fn suggest_rust_path(js_path: &str) -> String {
    // Convert path format from JavaScript to Rust style
    let path = js_path.replace(".js", ".rs")
                     .replace("Service", "_service")
                     .replace("Controller", "_controller")
                     .replace("Middleware", "_middleware");
    
    // Apply Rust naming conventions (snake_case)
    let mut rust_path = String::new();
    let parts: Vec<&str> = path.split('/').collect();
    
    for (i, part) in parts.iter().enumerate() {
        // Keep directory structure, transform only file names
        if i < parts.len() - 1 {
            rust_path.push_str(part);
            rust_path.push('/');
        } else {
            // Convert camelCase or PascalCase to snake_case for the file name
            let file_name = to_snake_case(part);
            rust_path.push_str(&file_name);
        }
    }
    
    // Move to appropriate Rust structure based on file type
    if js_path.contains("src/services") {
        rust_path = rust_path.replace("src/services", "src-tauri/src/services");
    } else if js_path.contains("src/api") {
        rust_path = rust_path.replace("src/api", "src-tauri/src/api");
    } else if js_path.contains("src/models") {
        rust_path = rust_path.replace("src/models", "src-tauri/src/models");
    } else if js_path.contains("src/utils") {
        rust_path = rust_path.replace("src/utils", "src-tauri/src/utils");
    } else if js_path.contains("src/controllers") || js_path.contains("src/routes") {
        rust_path = rust_path.replace("src/controllers", "src-tauri/src/commands")
                           .replace("src/routes", "src-tauri/src/commands");
    } else if js_path.starts_with("src/") {
        rust_path = rust_path.replace("src/", "src-tauri/src/");
    }
    
    rust_path
}

// Helper function to convert camelCase or PascalCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in s.char_indices() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    
    result
}
