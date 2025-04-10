use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use regex::Regex;
use serde::{Serialize, Deserialize};

/// Module for analyzing JavaScript files and suggesting Rust implementations
pub struct JsMigrationAnalyzer {
    base_dir: PathBuf,
    js_files: Vec<PathBuf>,
    patterns: HashMap<&'static str, Regex>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsFileAnalysis {
    pub path: PathBuf,
    pub relative_path: String,
    pub line_count: usize,
    pub dependencies: Vec<String>,
    pub exported_functions: Vec<String>,
    pub exports_class: bool,
    pub port_status: PortStatus,
    pub equivalent_rust_module: Option<String>,
    pub suggested_rust_path: Option<String>,
    pub port_priority: u8, // 0-10 with 10 being highest priority
    pub analysis_comments: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PortStatus {
    NotStarted,
    InProgress,
    Completed,
    Obsolete,
    CannotPort,
}

impl JsMigrationAnalyzer {
    /// Create a new JS migration analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        let mut patterns = HashMap::new();
        
        // Common patterns to identify in JS files
        patterns.insert("require", Regex::new(r"(?m)require\s*\(\s*['\"]([^'\"]+)['\"]").unwrap());
        patterns.insert("export_func", Regex::new(r"(?m)(?:exports|module\.exports)\.(\w+)\s*=").unwrap());
        patterns.insert("export_class", Regex::new(r"(?m)class\s+(\w+).*?\n.*?module\.exports\s*=\s*\1").unwrap());
        patterns.insert("module_exports", Regex::new(r"(?m)module\.exports\s*=").unwrap());
        
        Self {
            base_dir,
            js_files: Vec::new(),
            patterns,
        }
    }
    
    /// Find all JavaScript files in the workspace
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
    
    /// Analyze a single JavaScript file
    pub fn analyze_js_file(&self, path: &Path) -> Result<JsFileAnalysis, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
        
        let line_count = content.lines().count();
        let relative_path = path.strip_prefix(&self.base_dir)
            .map_err(|_| "Failed to create relative path".to_string())?
            .to_string_lossy()
            .to_string();
        
        // Extract dependencies
        let mut dependencies = Vec::new();
        if let Some(require_pattern) = self.patterns.get("require") {
            for cap in require_pattern.captures_iter(&content) {
                if let Some(dep) = cap.get(1) {
                    dependencies.push(dep.as_str().to_string());
                }
            }
        }
        
        // Check if file exports a class
        let exports_class = if let Some(export_class_pattern) = self.patterns.get("export_class") {
            export_class_pattern.is_match(&content)
        } else {
            false
        };
        
        // Extract exported functions
        let mut exported_functions = Vec::new();
        if let Some(export_func_pattern) = self.patterns.get("export_func") {
            for cap in export_func_pattern.captures_iter(&content) {
                if let Some(func) = cap.get(1) {
                    exported_functions.push(func.as_str().to_string());
                }
            }
        }
        
        // Determine port status
        let port_status = self.determine_port_status(path, &content);
        
        // Determine equivalent Rust module
        let (equivalent_rust_module, suggested_rust_path) = self.suggest_rust_equivalent(path, &content);
        
        // Determine port priority
        let port_priority = self.calculate_port_priority(path, &content, &dependencies);
        
        // Generate analysis comments
        let analysis_comments = self.generate_analysis_comments(path, &content, &dependencies, exports_class);
        
        Ok(JsFileAnalysis {
            path: path.to_path_buf(),
            relative_path,
            line_count,
            dependencies,
            exported_functions,
            exports_class,
            port_status,
            equivalent_rust_module,
            suggested_rust_path,
            port_priority,
            analysis_comments,
        })
    }
    
    /// Determine the port status of a JavaScript file
    fn determine_port_status(&self, path: &Path, content: &str) -> PortStatus {
        // Check if there's a comment indicating status
        if content.contains("// PORT STATUS: COMPLETED") {
            return PortStatus::Completed;
        } else if content.contains("// PORT STATUS: IN_PROGRESS") {
            return PortStatus::InProgress;
        } else if content.contains("// PORT STATUS: OBSOLETE") {
            return PortStatus::Obsolete;
        } else if content.contains("// PORT STATUS: CANNOT_PORT") {
            return PortStatus::CannotPort;
        }
        
        // Check if there's an equivalent Rust file
        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        
        // Generate possible Rust paths
        let possible_rust_paths = vec![
            parent.join(format!("{}.rs", file_stem)),
            parent.join(format!("{}_rs.rs", file_stem)),
            parent.join(format!("{}_rust.rs", file_stem)),
            self.base_dir.join("src-tauri").join("src").join("analyzers").join(format!("{}.rs", file_stem)),
            self.base_dir.join("src-tauri").join("src").join("utils").join(format!("{}.rs", file_stem)),
        ];
        
        for rust_path in possible_rust_paths {
            if rust_path.exists() {
                return PortStatus::Completed;
            }
        }
        
        // Check file content for indicators
        if content.contains("DO NOT PORT") || content.contains("@deprecated") {
            return PortStatus::Obsolete;
        }
        
        PortStatus::NotStarted
    }
    
    /// Suggest an equivalent Rust module and path
    fn suggest_rust_equivalent(&self, path: &Path, content: &str) -> (Option<String>, Option<String>) {
        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let parent = path.parent().unwrap_or_else(|| Path::new("")).to_string_lossy();
        
        // Determine module type from content or path
        let module_name = if path.to_string_lossy().contains("analyzer") || content.contains("Analyzer") {
            format!("analyzers::{}", Self::to_snake_case(&file_stem))
        } else if path.to_string_lossy().contains("utils") || content.contains("Utils") {
            format!("utils::{}", Self::to_snake_case(&file_stem))
        } else if path.to_string_lossy().contains("service") || content.contains("Service") {
            format!("services::{}", Self::to_snake_case(&file_stem))
        } else if path.to_string_lossy().contains("model") || content.contains("Model") {
            format!("models::{}", Self::to_snake_case(&file_stem))
        } else {
            format!("{}", Self::to_snake_case(&file_stem))
        };
        
        // Determine suggested path
        let suggested_path = if module_name.starts_with("analyzers") {
            Some(format!("src-tauri/src/analyzers/{}.rs", Self::to_snake_case(&file_stem)))
        } else if module_name.starts_with("utils") {
            Some(format!("src-tauri/src/utils/{}.rs", Self::to_snake_case(&file_stem)))
        } else if module_name.starts_with("services") {
            Some(format!("src-tauri/src/services/{}.rs", Self::to_snake_case(&file_stem)))
        } else if module_name.starts_with("models") {
            Some(format!("src-tauri/src/models/{}.rs", Self::to_snake_case(&file_stem)))
        } else {
            // Try to infer from parent directory
            if parent.contains("analyzers") {
                Some(format!("src-tauri/src/analyzers/{}.rs", Self::to_snake_case(&file_stem)))
            } else if parent.contains("utils") {
                Some(format!("src-tauri/src/utils/{}.rs", Self::to_snake_case(&file_stem)))
            } else if parent.contains("services") {
                Some(format!("src-tauri/src/services/{}.rs", Self::to_snake_case(&file_stem)))
            } else if parent.contains("models") {
                Some(format!("src-tauri/src/models/{}.rs", Self::to_snake_case(&file_stem)))
            } else {
                None
            }
        };
        
        (Some(module_name), suggested_path)
    }
    
    /// Calculate port priority for a JavaScript file
    fn calculate_port_priority(&self, path: &Path, content: &str, dependencies: &[String]) -> u8 {
        let mut priority = 5; // Default mid-priority
        
        // Increase priority for analyzer files
        if path.to_string_lossy().contains("analyzer") || content.contains("Analyzer") {
            priority += 3;
        }
        
        // Increase priority for core utilities
        if path.to_string_lossy().contains("utils") || content.contains("Utils") {
            priority += 2;
        }
        
        // Increase priority for files with few dependencies
        if dependencies.len() <= 2 {
            priority += 1;
        }
        
        // Decrease priority for large files
        let line_count = content.lines().count();
        if line_count > 1000 {
            priority = priority.saturating_sub(2);
        } else if line_count > 500 {
            priority = priority.saturating_sub(1);
        }
        
        // Cap priority at 10
        priority.min(10)
    }
    
    /// Generate analysis comments for a JavaScript file
    fn generate_analysis_comments(&self, path: &Path, content: &str, dependencies: &[String], exports_class: bool) -> Vec<String> {
        let mut comments = Vec::new();
        
        // Check for node-specific dependencies
        let node_specific_deps = dependencies.iter()
            .filter(|d| {
                d.starts_with("fs") || 
                d.starts_with("path") || 
                d.starts_with("child_process") ||
                d.starts_with("os") ||
                d.starts_with("http") ||
                d.starts_with("https")
            })
            .collect::<Vec<_>>();
        
        if !node_specific_deps.is_empty() {
            comments.push(format!(
                "Uses Node.js specific modules: {}. Replace with Rust std or other crates.",
                node_specific_deps.iter().map(|d| format!("`{}`", d)).collect::<Vec<_>>().join(", ")
            ));
        }
        
        // Check for class-based structure
        if exports_class {
            comments.push("Exports a class. Consider implementing as a Rust struct with methods.".to_string());
        }
        
        // Check for async/await usage
        if content.contains("async ") && content.contains("await ") {
            comments.push("Uses async/await. Implement with Rust's async/await and tokio.".to_string());
        }
        
        // Check for promise usage
        if content.contains("Promise") {
            comments.push("Uses Promises. Implement with Rust's Result and Future types.".to_string());
        }
        
        // Check for regular expressions
        if content.contains("Regex") || content.contains("RegExp") || content.contains("/\\w+/") {
            comments.push("Uses regular expressions. Implement with the regex crate.".to_string());
        }
        
        // Check for file operations
        if content.contains("fs.") || content.contains("readFile") || content.contains("writeFile") {
            comments.push("Performs file operations. Use Rust's std::fs module.".to_string());
        }
        
        // Check for JSON handling
        if content.contains("JSON.parse") || content.contains("JSON.stringify") {
            comments.push("Handles JSON data. Use serde_json for serialization/deserialization.".to_string());
        }
        
        // Check file size
        let line_count = content.lines().count();
        if line_count > 1000 {
            comments.push("Large file (>1000 lines). Consider splitting into multiple modules.".to_string());
        }
        
        comments
    }
    
    /// Run a complete analysis of all JS files
    pub fn analyze_all_js_files(&mut self) -> HashMap<String, JsFileAnalysis> {
        println!("Analyzing all JavaScript files...");
        
        if self.js_files.is_empty() {
            self.discover_js_files();
        }
        
        let mut results = HashMap::new();
        
        for js_file in &self.js_files {
            match self.analyze_js_file(js_file) {
                Ok(analysis) => {
                    let rel_path = analysis.relative_path.clone();
                    results.insert(rel_path, analysis);
                },
                Err(e) => println!("Error analyzing {}: {}", js_file.display(), e),
            }
        }
        
        println!("Completed analysis of {} JavaScript files", results.len());
        results
    }
    
    /// Generate migration plan documentation
    pub fn generate_migration_plan(&mut self) -> Result<String, String> {
        let analyses = self.analyze_all_js_files();
        
        let mut content = String::new();
        
        // Header
        content.push_str("# JavaScript to Rust Migration Plan\n\n");
        content.push_str(&format!("_Generated on: {}_\n\n", chrono::Local::now().format("%Y-%m-%d")));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str(&format!("This plan outlines the migration of {} JavaScript files to Rust implementations.\n\n", analyses.len()));
        
        // Migration summary
        let not_started = analyses.values().filter(|a| matches!(a.port_status, PortStatus::NotStarted)).count();
        let in_progress = analyses.values().filter(|a| matches!(a.port_status, PortStatus::InProgress)).count();
        let completed = analyses.values().filter(|a| matches!(a.port_status, PortStatus::Completed)).count();
        let obsolete = analyses.values().filter(|a| matches!(a.port_status, PortStatus::Obsolete)).count();
        let cannot_port = analyses.values().filter(|a| matches!(a.port_status, PortStatus::CannotPort)).count();
        
        content.push_str("## Migration Status\n\n");
        content.push_str("| Status | Count | Percentage |\n");
        content.push_str("|--------|-------|------------|\n");
        content.push_str(&format!("| Not Started | {} | {:.1}% |\n", not_started, (not_started as f32 / analyses.len() as f32) * 100.0));
        content.push_str(&format!("| In Progress | {} | {:.1}% |\n", in_progress, (in_progress as f32 / analyses.len() as f32) * 100.0));
        content.push_str(&format!("| Completed | {} | {:.1}% |\n", completed, (completed as f32 / analyses.len() as f32) * 100.0));
        content.push_str(&format!("| Obsolete | {} | {:.1}% |\n", obsolete, (obsolete as f32 / analyses.len() as f32) * 100.0));
        content.push_str(&format!("| Cannot Port | {} | {:.1}% |\n", cannot_port, (cannot_port as f32 / analyses.len() as f32) * 100.0));
        content.push_str("\n");
        
        // High priority migrations
        content.push_str("## High Priority Migrations\n\n");
        content.push_str("These files should be migrated first due to their importance or simplicity:\n\n");
        
        let mut high_priority: Vec<&JsFileAnalysis> = analyses.values()
            .filter(|a| a.port_priority >= 8 && matches!(a.port_status, PortStatus::NotStarted))
            .collect();
        
        high_priority.sort_by(|a, b| b.port_priority.cmp(&a.port_priority));
        
        if high_priority.is_empty() {
            content.push_str("No high priority files found. All critical files have been migrated!\n\n");
        } else {
            content.push_str("| File | Priority | Suggested Rust Path | Notes |\n");
            content.push_str("|------|----------|---------------------|-------|\n");
            
            for analysis in high_priority.iter().take(10) {
                let notes = if analysis.analysis_comments.is_empty() {
                    "N/A".to_string()
                } else {
                    analysis.analysis_comments[0].clone()
                };
                
                content.push_str(&format!("| {} | {} | {} | {} |\n", 
                    analysis.relative_path,
                    analysis.port_priority,
                    analysis.suggested_rust_path.as_ref().unwrap_or(&"N/A".to_string()),
                    notes
                ));
            }
            content.push_str("\n");
        }
        
        // Migration by category
        let categories = [
            ("Analyzers", "analyzer"),
            ("Utilities", "util"),
            ("Services", "service"),
            ("Models", "model"),
        ];
        
        for (category_name, pattern) in categories {
            content.push_str(&format!("## {} Migration\n\n", category_name));
            
            let category_files: Vec<&JsFileAnalysis> = analyses.values()
                .filter(|a| {
                    a.relative_path.contains(pattern) || 
                    a.equivalent_rust_module.as_ref().map_or(false, |m| m.contains(pattern))
                })
                .collect();
            
            if category_files.is_empty() {
                content.push_str(&format!("No {} files found.\n\n", category_name.to_lowercase()));
                continue;
            }
            
            content.push_str("| File | Status | Rust Module | Priority |\n");
            content.push_str("|------|--------|-------------|----------|\n");
            
            for analysis in category_files {
                let status = match analysis.port_status {
                    PortStatus::NotStarted => "Not Started",
                    PortStatus::InProgress => "In Progress",
                    PortStatus::Completed => "Completed",
                    PortStatus::Obsolete => "Obsolete",
                    PortStatus::CannotPort => "Cannot Port",
                };
                
                content.push_str(&format!("| {} | {} | {} | {} |\n", 
                    analysis.relative_path,
                    status,
                    analysis.equivalent_rust_module.as_ref().unwrap_or(&"N/A".to_string()),
                    analysis.port_priority
                ));
            }
            content.push_str("\n");
        }
        
        // Complex migrations
        let complex_migrations: Vec<&JsFileAnalysis> = analyses.values()
            .filter(|a| 
                a.line_count > 500 && 
                matches!(a.port_status, PortStatus::NotStarted) && 
                !matches!(a.port_status, PortStatus::Obsolete | PortStatus::CannotPort)
            )
            .collect();
        
        if !complex_migrations.is_empty() {
            content.push_str("## Complex Migrations\n\n");
            content.push_str("These files are complex and may require significant effort to port:\n\n");
            
            content.push_str("| File | Line Count | Notes |\n");
            content.push_str("|------|------------|-------|\n");
            
            for analysis in complex_migrations {
                let notes = analysis.analysis_comments.join("; ");
                
                content.push_str(&format!("| {} | {} | {} |\n", 
                    analysis.relative_path,
                    analysis.line_count,
                    if notes.is_empty() { "Complex file, may need to be split" } else { &notes }
                ));
            }
            content.push_str("\n");
        }
        
        // Next steps
        content.push_str("## Next Steps\n\n");
        
        if !high_priority.is_empty() {
            content.push_str("1. Migrate high priority files first\n");
            for (i, analysis) in high_priority.iter().take(3).enumerate() {
                content.push_str(&format!("   {}. Port `{}` to `{}`\n", 
                    i+1, 
                    analysis.relative_path,
                    analysis.suggested_rust_path.as_ref().unwrap_or(&"appropriate Rust module".to_string())
                ));
            }
        }
        
        content.push_str("2. Update existing migrations in progress\n");
        content.push_str("3. Create unit tests for all migrated modules\n");
        content.push_str("4. Ensure documentation is updated to reflect new Rust implementations\n");
        
        Ok(content)
    }
    
    /// Generate template for a Rust implementation of a JavaScript file
    pub fn generate_rust_template(&self, js_path: &Path) -> Result<String, String> {
        let analysis = self.analyze_js_file(js_path)?;
        
        // Read JS file content
        let js_content = fs::read_to_string(js_path)
            .map_err(|e| format!("Failed to read JS file: {}", e))?;
        
        let mut content = String::new();
        
        // Add file header comment
        content.push_str(&format!("// Ported from JavaScript: {}\n", analysis.relative_path));
        content.push_str("// This file was automatically generated by the JS Migration Analyzer\n\n");
        
        // Add common imports
        content.push_str("use std::path::{Path, PathBuf};\n");
        
        if js_content.contains("fs.") || js_content.contains("readFile") || js_content.contains("writeFile") {
            content.push_str("use std::fs;\n");
        }
        
        if js_content.contains("JSON.parse") || js_content.contains("JSON.stringify") {
            content.push_str("use serde::{Serialize, Deserialize};\n");
            content.push_str("use serde_json;\n");
        }
        
        if js_content.contains("Regex") || js_content.contains("RegExp") || js_content.contains("/\\w+/") {
            content.push_str("use regex::Regex;\n");
        }
        
        if js_content.contains("async ") && js_content.contains("await ") {
            content.push_str("use tokio;\n");
        }
        
        content.push_str("\n");
        
        // Generate struct for class
        if analysis.exports_class {
            let class_name = Self::extract_class_name(&js_content)
                .unwrap_or_else(|| Self::to_pascal_case(&js_path.file_stem().unwrap_or_default().to_string_lossy()));
            
            content.push_str(&format!("/// Rust implementation of JavaScript {}\n", class_name));
            content.push_str(&format!("pub struct {} {{\n", class_name));
            
            // Add fields based on constructor or properties
            let fields = Self::extract_class_fields(&js_content);
            for (name, field_type) in fields {
                content.push_str(&format!("    pub {}: {},\n", name, field_type));
            }
            
            content.push_str("}\n\n");
            
            // Implementation block
            content.push_str(&format!("impl {} {{\n", class_name));
            
            // Constructor
            content.push_str("    /// Create a new instance\n");
            content.push_str("    pub fn new(");
            
            // Constructor parameters based on fields
            let mut first = true;
            for (name, field_type) in &fields {
                if !first {
                    content.push_str(", ");
                }
                content.push_str(&format!("{}: {}", name, field_type));
                first = false;
            }
            
            content.push_str(") -> Self {\n");
            content.push_str("        Self {\n");
            
            for (name, _) in &fields {
                content.push_str(&format!("            {},\n", name));
            }
            
            content.push_str("        }\n");
            content.push_str("    }\n\n");
            
            // Add methods based on exported functions
            for func in &analysis.exported_functions {
                content.push_str(&format!("    /// TODO: Implement {}\n", func));
                content.push_str(&format!("    pub fn {}(&self) -> Result<(), String> {{\n", Self::to_snake_case(func)));
                content.push_str("        // TODO: Implement this method\n");
                content.push_str("        unimplemented!()\n");
                content.push_str("    }\n\n");
            }
            
            content.push_str("}\n");
        } else {
            // Generate individual functions
            for func in &analysis.exported_functions {
                content.push_str(&format!("/// TODO: Implement {}\n", func));
                content.push_str(&format!("pub fn {}() -> Result<(), String> {{\n", Self::to_snake_case(func)));
                content.push_str("    // TODO: Implement this function\n");
                content.push_str("    unimplemented!()\n");
                content.push_str("}\n\n");
            }
        }
        
        // Add TODOs based on analysis comments
        if !analysis.analysis_comments.is_empty() {
            content.push_str("// TODO Items:\n");
            for comment in &analysis.analysis_comments {
                content.push_str(&format!("// - {}\n", comment));
            }
            content.push_str("\n");
        }
        
        Ok(content)
    }
    
    /// Extract class name from JavaScript content
    fn extract_class_name(content: &str) -> Option<String> {
        let re = Regex::new(r"(?m)class\s+(\w+)").ok()?;
        re.captures(content)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }
    
    /// Extract class fields from JavaScript content
    fn extract_class_fields(content: &str) -> Vec<(String, String)> {
        let mut fields = Vec::new();
        
        // Try to extract constructor parameters
        if let Ok(re) = Regex::new(r"(?m)constructor\s*\(([^)]*)\)") {
            if let Some(cap) = re.captures(content) {
                if let Some(params) = cap.get(1) {
                    for param in params.as_str().split(',') {
                        let param = param.trim();
                        if !param.is_empty() {
                            // Extract param name and guess type
                            let param_name = param.split('=').next().unwrap_or(param).trim();
                            let param_type = if param_name.contains("path") || param_name.contains("dir") || param_name.contains("file") {
                                "PathBuf"
                            } else if param_name.contains("options") || param_name.contains("config") {
                                "Option<Config>"
                            } else if param_name.contains("callback") {
                                "Box<dyn Fn() -> Result<(), String>>"
                            } else {
                                "String" // Default to String
                            };
                            
                            fields.push((param_name.to_string(), param_type.to_string()));
                        }
                    }
                }
            }
        }
        
        // Also try to find instance fields (this.field = ...)
        if let Ok(re) = Regex::new(r"(?m)this\.(\w+)\s*=") {
            for cap in re.captures_iter(content) {
                if let Some(field) = cap.get(1) {
                    let field_name = field.as_str();
                    if !fields.iter().any(|(name, _)| name == field_name) {
                        let field_type = if field_name.contains("path") || field_name.contains("dir") || field_name.contains("file") {
                            "PathBuf"
                        } else if field_name.contains("options") || field_name.contains("config") {
                            "Option<Config>"
                        } else if field_name.contains("callback") {
                            "Box<dyn Fn() -> Result<(), String>>"
                        } else if field_name.contains("list") || field_name.contains("array") || field_name.contains("items") {
                            "Vec<String>"
                        } else if field_name.contains("map") || field_name.contains("dict") {
                            "HashMap<String, String>"
                        } else {
                            "String" // Default to String
                        };
                        
                        fields.push((field_name.to_string(), field_type.to_string()));
                    }
                }
            }
        }
        
        fields
    }
    
    /// Convert CamelCase to snake_case
    fn to_snake_case(name: &str) -> String {
        let mut result = String::new();
        let mut prev_char_was_upper = false;
        
        for (i, c) in name.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && !prev_char_was_upper {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
                prev_char_was_upper = true;
            } else {
                result.push(c);
                prev_char_was_upper = false;
            }
        }
        
        result
    }
    
    /// Convert snake_case to PascalCase
    fn to_pascal_case(name: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        
        for c in name.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }
}
