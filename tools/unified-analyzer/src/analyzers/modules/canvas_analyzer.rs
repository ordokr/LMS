use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use regex::Regex;

// Canvas model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasModel {
    pub name: String,
    pub file_path: String,
    pub attributes: Vec<String>,
    pub associations: Vec<String>,
}

// Canvas controller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasController {
    pub name: String,
    pub file_name: String,
    pub file_path: String,
    pub actions: Vec<String>,
}

// Canvas API endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasApiEndpoint {
    pub controller: String,
    pub action: String,
    pub route: String,
    pub method: String,
    pub file_path: String,
}

// Canvas module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasModule {
    pub name: String,
    pub file_count: usize,
    pub files: Vec<String>,
}

// Ruby gem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubyGem {
    pub name: String,
    pub description: String,
}

// JavaScript library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsLibrary {
    pub name: String,
    pub version: String,
}

// File statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub total: usize,
    pub ruby: usize,
    pub javascript: usize,
    pub css: usize,
    pub html: usize,
    pub other: usize,
}

// Canvas analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasAnalysisResult {
    pub models: Vec<CanvasModel>,
    pub controllers: Vec<CanvasController>,
    pub api_endpoints: Vec<CanvasApiEndpoint>,
    pub modules: Vec<CanvasModule>,
    pub ruby_gems: Vec<RubyGem>,
    pub js_libraries: Vec<JsLibrary>,
    pub file_stats: FileStats,
}

impl Default for CanvasAnalysisResult {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            controllers: Vec::new(),
            api_endpoints: Vec::new(),
            modules: Vec::new(),
            ruby_gems: Vec::new(),
            js_libraries: Vec::new(),
            file_stats: FileStats {
                total: 0,
                ruby: 0,
                javascript: 0,
                css: 0,
                html: 0,
                other: 0,
            },
        }
    }
}

// Canvas analyzer
pub struct CanvasAnalyzer {
    pub base_dir: PathBuf,
    pub result: CanvasAnalysisResult,
    pub all_files: Vec<PathBuf>,
}

impl CanvasAnalyzer {
    // Create a new Canvas analyzer
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            result: CanvasAnalysisResult::default(),
            all_files: Vec::new(),
        }
    }

    // Run the analysis
    pub fn analyze(&mut self) -> Result<CanvasAnalysisResult, Box<dyn std::error::Error + Send + Sync>> {
        println!("Starting Canvas LMS analysis...");

        // Discover all files
        self.discover_files()?;

        // Analyze file statistics
        self.analyze_file_stats()?;

        // Analyze models
        self.analyze_models()?;

        // Analyze controllers
        self.analyze_controllers()?;

        // Analyze API endpoints
        self.analyze_api_endpoints()?;

        // Analyze dependencies
        self.analyze_dependencies()?;

        // Analyze modules
        self.analyze_modules()?;

        // Print summary
        self.print_summary();

        Ok(self.result.clone())
    }

    // Discover all files in the project
    fn discover_files(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Discovering files...");

        self.all_files = Vec::new();

        // Walk the directory tree
        for entry in walkdir::WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                self.all_files.push(entry.path().to_path_buf());
            }
        }

        println!("Found {} files", self.all_files.len());
        Ok(())
    }

    // Analyze file statistics
    fn analyze_file_stats(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing file statistics...");

        let mut total = 0;
        let mut ruby = 0;
        let mut javascript = 0;
        let mut css = 0;
        let mut html = 0;
        let mut other = 0;

        for file in &self.all_files {
            total += 1;

            if let Some(ext) = file.extension() {
                match ext.to_string_lossy().as_ref() {
                    "rb" => ruby += 1,
                    "js" | "jsx" | "ts" | "tsx" => javascript += 1,
                    "css" | "scss" | "sass" => css += 1,
                    "html" | "erb" | "haml" => html += 1,
                    _ => other += 1,
                }
            } else {
                other += 1;
            }
        }

        self.result.file_stats.total = total;
        self.result.file_stats.ruby = ruby;
        self.result.file_stats.javascript = javascript;
        self.result.file_stats.css = css;
        self.result.file_stats.html = html;
        self.result.file_stats.other = other;

        println!("File statistics: {} total, {} Ruby, {} JavaScript, {} CSS, {} HTML, {} other",
                 total, ruby, javascript, css, html, other);
        Ok(())
    }

    // Analyze models
    fn analyze_models(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing Canvas models...");

        // Find all model files
        let model_files = self.all_files.iter()
            .filter(|path| {
                path.to_string_lossy().contains("/app/models/") &&
                path.extension().map_or(false, |ext| ext == "rb")
            })
            .collect::<Vec<_>>();

        for model_file in model_files {
            if let Ok(content) = fs::read_to_string(model_file) {
                // Extract model name from file path
                if let Some(file_name) = model_file.file_name() {
                    let file_name = file_name.to_string_lossy();
                    if let Some(model_name) = file_name.strip_suffix(".rb") {
                        // Convert snake_case to CamelCase
                        let model_name = model_name.split('_')
                            .map(|s| {
                                let mut c = s.chars();
                                match c.next() {
                                    None => String::new(),
                                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                                }
                            })
                            .collect::<String>();

                        // Extract attributes
                        let mut attributes = Vec::new();
                        let attr_regex = Regex::new(r"(?m)^\s*(?:attr_accessor|attr_reader|attr_writer)\s+:([\w_]+)").unwrap();
                        for captures in attr_regex.captures_iter(&content) {
                            if let Some(attr) = captures.get(1) {
                                attributes.push(attr.as_str().to_string());
                            }
                        }

                        // Extract associations
                        let mut associations = Vec::new();
                        let assoc_regex = Regex::new(r"(?m)^\s*(?:belongs_to|has_many|has_one|has_and_belongs_to_many)\s+:([\w_]+)").unwrap();
                        for captures in assoc_regex.captures_iter(&content) {
                            if let Some(assoc) = captures.get(1) {
                                associations.push(assoc.as_str().to_string());
                            }
                        }

                        // Add to models
                        self.result.models.push(CanvasModel {
                            name: model_name,
                            file_path: model_file.to_string_lossy().to_string(),
                            attributes,
                            associations,
                        });
                    }
                }
            }
        }

        println!("Found {} models", self.result.models.len());
        Ok(())
    }

    // Analyze controllers
    fn analyze_controllers(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing Canvas controllers...");

        // Find all controller files
        let controller_files = self.all_files.iter()
            .filter(|path| {
                path.to_string_lossy().contains("/app/controllers/") &&
                path.extension().map_or(false, |ext| ext == "rb")
            })
            .collect::<Vec<_>>();

        for controller_file in controller_files {
            if let Ok(content) = fs::read_to_string(controller_file) {
                // Extract controller name from file path
                if let Some(file_name) = controller_file.file_name() {
                    let file_name = file_name.to_string_lossy();
                    if let Some(controller_name) = file_name.strip_suffix("_controller.rb") {
                        // Convert snake_case to CamelCase
                        let controller_name = controller_name.split('_')
                            .map(|s| {
                                let mut c = s.chars();
                                match c.next() {
                                    None => String::new(),
                                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                                }
                            })
                            .collect::<String>() + "Controller";

                        // Extract actions
                        let mut actions = Vec::new();
                        let action_regex = Regex::new(r"(?m)^\s*def\s+([\w_]+)").unwrap();
                        for captures in action_regex.captures_iter(&content) {
                            if let Some(action) = captures.get(1) {
                                let action_name = action.as_str();
                                if !action_name.starts_with('_') {
                                    actions.push(action_name.to_string());
                                }
                            }
                        }

                        // Add to controllers
                        self.result.controllers.push(CanvasController {
                            name: controller_name.clone(),
                            file_name: controller_name,
                            file_path: controller_file.to_string_lossy().to_string(),
                            actions,
                        });
                    }
                }
            }
        }

        println!("Found {} controllers with {} actions", 
                 self.result.controllers.len(),
                 self.result.controllers.iter().map(|c| c.actions.len()).sum::<usize>());
        Ok(())
    }

    // Analyze API endpoints
    fn analyze_api_endpoints(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing Canvas API endpoints...");

        // Find routes file
        let routes_path = self.base_dir.join("config/routes.rb");
        let routes_content = if routes_path.exists() {
            fs::read_to_string(&routes_path).unwrap_or_default()
        } else {
            String::new()
        };

        // Process each controller and its actions
        for controller in &self.result.controllers {
            for action in &controller.actions {
                let mut route = String::new();
                let mut method = String::new();

                // Try to find the route in the routes file
                if !routes_content.is_empty() {
                    let route_pattern = format!("{}#{}", controller.file_name, action);
                    if routes_content.contains(&route_pattern) {
                        // Try to extract the route path and method
                        // Simplified regex to avoid syntax errors
                        let route_regex = Regex::new(r#"(get|post|put|delete|patch)\s+["']([^"']+)["']"#).unwrap();
                        if let Some(captures) = route_regex.captures(&routes_content) {
                            if let (Some(m), Some(r)) = (captures.get(1), captures.get(2)) {
                                method = m.as_str().to_uppercase();
                                route = r.as_str().to_string();
                            }
                        }
                    }
                }

                // Add to metrics
                self.result.api_endpoints.push(CanvasApiEndpoint {
                    controller: controller.name.clone(),
                    action: action.clone(),
                    route,
                    method,
                    file_path: controller.file_path.clone(),
                });
            }
        }

        println!("Found {} API endpoints", self.result.api_endpoints.len());
        Ok(())
    }

    // Analyze dependencies
    fn analyze_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing Canvas dependencies...");

        // Find Gemfile
        let gemfile_path = self.base_dir.join("Gemfile");
        if gemfile_path.exists() {
            if let Ok(content) = fs::read_to_string(&gemfile_path) {
                let gem_regex = Regex::new(r#"^\s*gem\s+["'](\w+)["']"#).unwrap();
                
                for captures in gem_regex.captures_iter(&content) {
                    if let Some(gem_name) = captures.get(1) {
                        self.result.ruby_gems.push(RubyGem {
                            name: gem_name.as_str().to_string(),
                            description: self.get_gem_description(gem_name.as_str(), &content),
                        });
                    }
                }
            }
        }

        // Find JavaScript dependencies
        let package_json_path = self.base_dir.join("package.json");
        if package_json_path.exists() {
            if let Ok(content) = fs::read_to_string(&package_json_path) {
                if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let dependencies = self.extract_dependencies(&package_json, "dependencies");
                    let dev_dependencies = self.extract_dependencies(&package_json, "devDependencies");

                    for (name, version) in dependencies.into_iter().chain(dev_dependencies) {
                        self.result.js_libraries.push(JsLibrary {
                            name,
                            version,
                        });
                    }
                }
            }
        }

        println!("Found {} Ruby gems and {} JS libraries", 
                 self.result.ruby_gems.len(), 
                 self.result.js_libraries.len());
        Ok(())
    }

    // Extract dependencies from package.json
    fn extract_dependencies(&self, package_json: &serde_json::Value, key: &str) -> Vec<(String, String)> {
        let mut result = Vec::new();

        if let Some(deps) = package_json.get(key).and_then(|v| v.as_object()) {
            for (name, version) in deps {
                if let Some(version_str) = version.as_str() {
                    result.push((name.clone(), version_str.to_string()));
                }
            }
        }

        result
    }

    // Get gem description from Gemfile
    fn get_gem_description(&self, gem_name: &str, gemfile_content: &str) -> String {
        let desc_regex = Regex::new(&format!(r#"gem\s+["']{}["']"#, gem_name)).unwrap();
        
        if let Some(captures) = desc_regex.captures(gemfile_content) {
            if let Some(desc) = captures.get(0) {
                return desc.as_str().to_string();
            }
        }
        
        "N/A".to_string()
    }

    // Analyze modules
    fn analyze_modules(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Analyzing Canvas modules and components...");

        // Canvas has certain key modules/components we want to identify
        let module_patterns = vec![
            ("Authentication", vec!["app/models/authentication_provider"]),
            ("Authorization", vec!["app/models/role", "app/models/permission"]),
            ("Gradebook", vec!["app/controllers/gradebooks"]),
            ("Calendar", vec!["app/controllers/calendars"]),
            ("Assignments", vec!["app/models/assignment", "app/controllers/assignments"]),
            ("Courses", vec!["app/models/course", "app/controllers/courses"]),
            ("Users", vec!["app/models/user", "app/controllers/users"]),
            ("Files", vec!["app/models/attachment", "app/controllers/files"]),
            ("Discussions", vec!["app/models/discussion"]),
            ("Quiz", vec!["app/models/quiz", "app/controllers/quizzes"]),
            ("Notifications", vec!["app/models/notification"]),
            ("API", vec!["app/controllers/api"]),
        ];

        for (module_name, dir_patterns) in module_patterns {
            let mut module_files = Vec::new();

            for dir_pattern in dir_patterns {
                let dir_path = self.base_dir.join(dir_pattern);
                
                for file in &self.all_files {
                    if file.starts_with(&dir_path) {
                        if let Ok(relative_path) = file.strip_prefix(&self.base_dir) {
                            module_files.push(relative_path.to_string_lossy().to_string());
                        }
                    }
                }
            }

            if !module_files.is_empty() {
                self.result.modules.push(CanvasModule {
                    name: module_name.to_string(),
                    file_count: module_files.len(),
                    files: module_files,
                });
            }
        }

        println!("Identified {} Canvas modules/components", self.result.modules.len());
        Ok(())
    }

    // Print summary
    fn print_summary(&self) {
        println!("\n=== Canvas LMS Analysis Summary ===\n");
        println!("Models: {}", self.result.models.len());
        println!("Controllers: {}", self.result.controllers.len());
        println!("API Endpoints: {}", self.result.api_endpoints.len());
        println!("Ruby Gems: {}", self.result.ruby_gems.len());
        println!("JS Libraries: {}", self.result.js_libraries.len());
        println!("Total Files: {}", self.result.file_stats.total);
    }
}
