use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct EnhancedRubyRoute {
    pub http_method: String,
    pub path: String,
    pub controller: Option<String>,
    pub action: Option<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyFilter {
    pub filter_type: String,
    pub methods: Vec<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyControllerMethod {
    pub name: String,
    pub parameters: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyController {
    pub name: String,
    pub parent_class: Option<String>,
    pub file_path: String,
    pub routes: Vec<EnhancedRubyRoute>,
    pub filters: Vec<EnhancedRubyFilter>,
    pub methods: Vec<EnhancedRubyControllerMethod>,
    pub layout: Option<String>,
}

pub struct EnhancedRubyControllerAnalyzer {
    pub controllers: HashMap<String, EnhancedRubyController>,
    pub routes: Vec<EnhancedRubyRoute>,
}

impl EnhancedRubyControllerAnalyzer {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
            routes: Vec::new(),
        }
    }
    
    pub fn analyze_directory(&mut self, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(format!("Directory does not exist: {:?}", dir_path).into());
        }
        
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.analyze_directory(&path)?;
            } else if let Some(extension) = path.extension() {
                if extension == "rb" {
                    self.analyze_file(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        
        // Check if this is a controller file
        if self.is_controller_file(&content) {
            let controller = self.parse_controller(&content, file_path)?;
            self.controllers.insert(controller.name.clone(), controller);
        }
        
        // Check if this is a routes file
        if self.is_routes_file(file_path) {
            self.parse_routes(&content)?;
        }
        
        Ok(())
    }
    
    fn is_controller_file(&self, content: &str) -> bool {
        // Check if the file contains a class that inherits from ApplicationController or ActionController::Base
        lazy_static! {
            static ref CLASS_REGEX: Regex =
                Regex::new(r#"class\s+([A-Za-z0-9:]+)\s*<\s*([A-Za-z0-9:]+)"#).unwrap();
        }
        
        if let Some(captures) = CLASS_REGEX.captures(content) {
            if let Some(parent) = captures.get(2) {
                let parent_class = parent.as_str();
                return parent_class == "ApplicationController" || parent_class == "ActionController::Base";
            }
        }
        
        false
    }
    
    fn is_routes_file(&self, file_path: &Path) -> bool {
        // Check if the file is named routes.rb or contains routes in its path
        if let Some(file_name) = file_path.file_name() {
            if file_name == "routes.rb" {
                return true;
            }
        }
        
        if let Some(path_str) = file_path.to_str() {
            return path_str.contains("config/routes");
        }
        
        false
    }
    
    fn parse_controller(&self, content: &str, file_path: &Path) -> Result<EnhancedRubyController, Box<dyn std::error::Error>> {
        // Extract class name and parent class
        lazy_static! {
            static ref CLASS_REGEX: Regex =
                Regex::new(r#"class\s+([A-Za-z0-9:]+)\s*<\s*([A-Za-z0-9:]+)"#).unwrap();
        }
        
        let (name, parent_class) = if let Some(captures) = CLASS_REGEX.captures(content) {
            (
                captures.get(1).unwrap().as_str().to_string(),
                Some(captures.get(2).unwrap().as_str().to_string()),
            )
        } else {
            return Err("Could not extract class name and parent class".into());
        };
        
        // Create controller
        let mut controller = EnhancedRubyController {
            name,
            parent_class,
            file_path: file_path.to_string_lossy().to_string(),
            routes: Vec::new(),
            filters: Vec::new(),
            methods: Vec::new(),
            layout: None,
        };
        
        // Extract routes
        self.extract_routes(content, &mut controller);
        
        // Extract filters
        self.extract_filters(content, &mut controller);
        
        // Extract methods
        self.extract_methods(content, &mut controller);
        
        // Extract layout
        self.extract_layout(content, &mut controller);
        
        Ok(controller)
    }
    
    fn extract_routes(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract routes from routes.rb file
        lazy_static! {
            static ref ROUTE_REGEX: Regex =
                Regex::new(r#"(get|post|put|patch|delete)\s+['"]([^'"]+)['"](?:,\s*to:\s*['"]([^:]+):([^'"]+)['"])?"#).unwrap();
        }
        
        for captures in ROUTE_REGEX.captures_iter(content) {
            let http_method = captures.get(1).unwrap().as_str().to_string();
            let path = captures.get(2).unwrap().as_str().to_string();
            
            let mut route = EnhancedRubyRoute {
                http_method,
                path,
                controller: None,
                action: None,
                options: HashMap::new(),
            };
            
            // Extract controller and action
            if let Some(controller_match) = captures.get(3) {
                route.controller = Some(controller_match.as_str().to_string());
            }
            
            if let Some(action_match) = captures.get(4) {
                route.action = Some(action_match.as_str().to_string());
            }
            
            controller.routes.push(route);
        }
    }
    
    fn parse_routes(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Extract routes from routes.rb file
        lazy_static! {
            static ref ROUTE_REGEX: Regex =
                Regex::new(r#"(get|post|put|patch|delete)\s+['"]([^'"]+)['"](?:,\s*to:\s*['"]([^:]+):([^'"]+)['"])?"#).unwrap();
        }
        
        for captures in ROUTE_REGEX.captures_iter(content) {
            let http_method = captures.get(1).unwrap().as_str().to_string();
            let path = captures.get(2).unwrap().as_str().to_string();
            
            let mut route = EnhancedRubyRoute {
                http_method,
                path,
                controller: None,
                action: None,
                options: HashMap::new(),
            };
            
            // Extract controller and action
            if let Some(controller_match) = captures.get(3) {
                route.controller = Some(controller_match.as_str().to_string());
            }
            
            if let Some(action_match) = captures.get(4) {
                route.action = Some(action_match.as_str().to_string());
            }
            
            self.routes.push(route);
        }
        
        Ok(())
    }
    
    fn extract_filters(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract filters (before_action, after_action, around_action)
        lazy_static! {
            static ref FILTER_REGEX: Regex =
                Regex::new(r#"(before_action|after_action|around_action)\s+:([a-z0-9_]+)(?:,\s*(.+?))?(?:\n|\r|$)"#).unwrap();
        }
        
        for captures in FILTER_REGEX.captures_iter(content) {
            let filter_type = captures.get(1).unwrap().as_str().to_string();
            let method = captures.get(2).unwrap().as_str().to_string();
            
            let mut filter = EnhancedRubyFilter {
                filter_type,
                methods: vec![method],
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Store all options in the options map
                lazy_static! {
                    static ref OPTION_REGEX: Regex =
                        Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|\[([^\]]+)\]|(\d+)|true|false)"#).unwrap();
                }
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .or_else(|| option_match.get(5))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
                    filter.options.insert(key, value);
                }
            }
            
            controller.filters.push(filter);
        }
    }
    
    fn extract_methods(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract methods
        lazy_static! {
            static ref METHOD_REGEX: Regex =
                Regex::new(r#"def\s+([a-z0-9_?!]+)(\([^)]*\))?(.*?)end"#).unwrap();
        }
        
        for captures in METHOD_REGEX.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let parameters = captures.get(2).map(|m| m.as_str().to_string());
            let body = captures.get(3).map(|m| m.as_str().to_string());
            
            let method = EnhancedRubyControllerMethod {
                name,
                parameters,
                body,
            };
            
            controller.methods.push(method);
        }
    }
    
    fn extract_layout(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract layout
        lazy_static! {
            static ref LAYOUT_REGEX: Regex =
                Regex::new(r#"layout\s+['"]([^'"]+)['"]"#).unwrap();
        }
        
        if let Some(captures) = LAYOUT_REGEX.captures(content) {
            controller.layout = Some(captures.get(1).unwrap().as_str().to_string());
        }
    }
}
