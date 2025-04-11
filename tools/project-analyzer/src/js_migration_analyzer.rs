use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

/// Status of JavaScript to Rust migration for a file
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationStatus {
    Completed,
    InProgress,
    NotStarted,
    NotNeeded,
}

/// Information about a JavaScript file for migration
#[derive(Debug, Clone)]
pub struct JsFile {
    pub path: String,
    pub relative_path: String,
    pub migration_status: MigrationStatus,
    pub rust_path: Option<String>,
    pub priority_score: u8,
}

/// Analysis of JavaScript to Rust migration
#[derive(Debug, Clone)]
pub struct JsMigrationAnalysis {
    pub total_js_files: usize,
    pub completed_migrations: usize,
    pub in_progress_migrations: usize,
    pub not_started_migrations: usize,
    pub not_needed_migrations: usize,
    pub js_files: Vec<JsFile>,
    pub high_priority_files: Vec<String>,
}

pub struct JsMigrationAnalyzer {
    base_dir: String,
    js_files: Vec<PathBuf>,
    tracking_path: Option<String>,
}

impl JsMigrationAnalyzer {
    pub fn new(base_dir: String) -> Self {
        JsMigrationAnalyzer {
            base_dir,
            js_files: Vec::new(),
            tracking_path: None,
        }
    }
    
    pub fn with_tracking(mut self, tracking_path: String) -> Self {
        self.tracking_path = Some(tracking_path);
        self
    }
    
    pub fn discover_js_files(&mut self) -> Vec<PathBuf> {
        println!("Discovering JavaScript files...");
        
        self.js_files = WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                if let Some(ext) = e.path().extension() {
                    if ext == "js" {
                        let path_str = e.path().to_string_lossy();
                        // Exclude certain directories
                        !path_str.contains("node_modules") &&
                        !path_str.contains("coverage") &&
                        !path_str.contains("build-output") &&
                        !path_str.contains("target")
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .map(|e| e.path().to_path_buf())
            .collect();
        
        println!("Found {} JavaScript files", self.js_files.len());
        self.js_files.clone()
    }
    
    pub fn analyze_js_files(&self) -> JsMigrationAnalysis {
        println!("Analyzing JavaScript files...");
        
        let mut js_files_info = Vec::new();
        let mut high_priority_files = Vec::new();
        
        for path in &self.js_files {
            let relative_path = path.strip_prefix(&self.base_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .replace("\\", "/");
                
            let (status, rust_path) = self.get_migration_status(&relative_path);
            
            // Calculate priority score (1-10)
            let priority_score = self.calculate_priority_score(&relative_path);
            
            if priority_score >= 8 {
                high_priority_files.push(relative_path.to_string());
            }
            
            js_files_info.push(JsFile {
                path: path.to_string_lossy().to_string(),
                relative_path: relative_path.to_string(),
                migration_status: status,
                rust_path,
                priority_score,
            });
        }
        
        // Count migrations by status
        let completed_migrations = js_files_info.iter()
            .filter(|f| f.migration_status == MigrationStatus::Completed)
            .count();
        
        let in_progress_migrations = js_files_info.iter()
            .filter(|f| f.migration_status == MigrationStatus::InProgress)
            .count();
        
        let not_started_migrations = js_files_info.iter()
            .filter(|f| f.migration_status == MigrationStatus::NotStarted)
            .count();
        
        let not_needed_migrations = js_files_info.iter()
            .filter(|f| f.migration_status == MigrationStatus::NotNeeded)
            .count();
        
        JsMigrationAnalysis {
            total_js_files: js_files_info.len(),
            completed_migrations,
            in_progress_migrations,
            not_started_migrations,
            not_needed_migrations,
            js_files: js_files_info,
            high_priority_files,
        }
    }
    
    fn get_migration_status(&self, js_file: &str) -> (MigrationStatus, Option<String>) {
        // Check if the JS file matches patterns for files that don't need migration
        let not_needed_patterns = [
            "webpack.config.js",
            "babel.config.js",
            "jest.config.js",
            ".eslintrc.js",
            "postcss.config.js",
        ];
        
        for pattern in &not_needed_patterns {
            if js_file.ends_with(pattern) {
                return (MigrationStatus::NotNeeded, None);
            }
        }
        
        // Check if it's a test file that doesn't need migration
        if js_file.contains("__tests__") || js_file.contains("test") || 
           js_file.ends_with(".test.js") || js_file.ends_with(".spec.js") {
            return (MigrationStatus::NotNeeded, None);
        }
        
        // Check tracking file if available
        if let Some(tracking_path) = &self.tracking_path {
            if let Ok(content) = fs::read_to_string(tracking_path) {
                // Look for completed migrations
                let completed_regex = Regex::new(r"\[\s*x\s*\]\s*(.*?)\s*\|\s*(.*?)\s*(\||$)").unwrap();
                for cap in completed_regex.captures_iter(&content) {
                    let tracked_js = cap[1].trim();
                    let tracked_rust = cap[2].trim();
                    
                    if tracked_js == js_file {
                        return (MigrationStatus::Completed, Some(tracked_rust.to_string()));
                    }
                }
                
                // Look for in-progress migrations
                let in_progress_regex = Regex::new(r"\[\s*\s*\]\s*(.*?)\s*\|\s*(.*?)\s*(\||$)").unwrap();
                for cap in in_progress_regex.captures_iter(&content) {
                    let tracked_js = cap[1].trim();
                    let tracked_rust = cap[2].trim();
                    
                    if tracked_js == js_file {
                        return (MigrationStatus::InProgress, Some(tracked_rust.to_string()));
                    }
                }
            }
        }
        
        // If not found in tracking, it's not started
        let rust_path = self.suggest_rust_path(js_file);
        (MigrationStatus::NotStarted, Some(rust_path))
    }
    
    fn suggest_rust_path(&self, js_file: &str) -> String {
        // Convert JS file path to a suitable Rust file path
        // e.g., src/utils/stringUtils.js -> src/utils/string_utils.rs
        let mut rust_path = js_file.replace(".js", ".rs");
        
        // Convert camelCase to snake_case in file name
        if let Some(last_slash) = rust_path.rfind('/') {
            let file_name = &rust_path[last_slash+1..];
            let mut snake_case = String::new();
            
            for (i, c) in file_name.chars().enumerate() {
                if i > 0 && c.is_uppercase() {
                    snake_case.push('_');
                }
                snake_case.push(c.to_lowercase().next().unwrap());
            }
            
            rust_path = format!("{}/{}", &rust_path[..last_slash], snake_case);
        }
        
        rust_path
    }
    
    fn calculate_priority_score(&self, js_file: &str) -> u8 {
        let mut score = 5; // Default priority
        
        // Increase score for core services and APIs
        if js_file.contains("/services/") || js_file.contains("/api/") {
            score += 3;
        }
        
        // Increase for data models
        if js_file.contains("/models/") {
            score += 2;
        }
        
        // Increase for utilities
        if js_file.contains("/utils/") || js_file.contains("/helpers/") {
            score += 1;
        }
        
        // Lower priority for tests
        if js_file.contains("/tests/") || js_file.contains(".test.js") {
            score -= 3;
        }
        
        // Ensure score is between 1 and 10
        score.clamp(1, 10)
    }
    
    pub fn generate_migration_plan(&self) -> Result<String, Box<dyn std::error::Error>> {
        let analysis = self.analyze_js_files();
        
        let mut plan = String::from("# JavaScript to Rust Migration Plan\n\n");
        
        plan.push_str(&format!("Total JavaScript files: {}\n", analysis.total_js_files));
        plan.push_str(&format!("Completed migrations: {}\n", analysis.completed_migrations));
        plan.push_str(&format!("In-progress migrations: {}\n", analysis.in_progress_migrations));
        plan.push_str(&format!("Not started migrations: {}\n", analysis.not_started_migrations));
        plan.push_str(&format!("Not needed migrations: {}\n\n", analysis.not_needed_migrations));
        
        // Add high priority files
        plan.push_str("## High Priority Files\n\n");
        if analysis.high_priority_files.is_empty() {
            plan.push_str("No high priority files identified.\n\n");
        } else {
            for file in &analysis.high_priority_files {
                plan.push_str(&format!("- {}\n", file));
            }
            plan.push_str("\n");
        }
        
        // Add in-progress migrations
        plan.push_str("## In Progress Migrations\n\n");
        let in_progress = analysis.js_files.iter()
            .filter(|f| f.migration_status == MigrationStatus::InProgress)
            .collect::<Vec<_>>();
            
        if in_progress.is_empty() {
            plan.push_str("No migrations currently in progress.\n\n");
        } else {
            plan.push_str("| JavaScript File | Planned Rust Equivalent |\n");
            plan.push_str("|----------------|-------------------------|\n");
            
            for file in in_progress {
                if let Some(rust_path) = &file.rust_path {
                    plan.push_str(&format!("| {} | {} |\n", file.relative_path, rust_path));
                }
            }
            plan.push_str("\n");
        }
        
        // Add not started migrations
        plan.push_str("## Not Started Migrations\n\n");
        let not_started = analysis.js_files.iter()
            .filter(|f| f.migration_status == MigrationStatus::NotStarted)
            .collect::<Vec<_>>();
            
        if not_started.is_empty() {
            plan.push_str("All files have been migrated or are in progress!\n\n");
        } else {
            plan.push_str("| JavaScript File | Suggested Rust Equivalent |\n");
            plan.push_str("|----------------|---------------------------|\n");
            
            for file in not_started {
                if let Some(rust_path) = &file.rust_path {
                    plan.push_str(&format!("| {} | {} |\n", file.relative_path, rust_path));
                }
            }
            plan.push_str("\n");
        }
        
        Ok(plan)
    }
}
