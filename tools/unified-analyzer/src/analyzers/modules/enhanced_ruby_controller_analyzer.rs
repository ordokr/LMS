use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ControllerAction {
    pub name: String,
    pub http_method: Option<String>,
    pub path: Option<String>,
    pub parameters: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ControllerFilter {
    pub filter_type: String,
    pub methods: Vec<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ControllerHelper {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnhancedRubyController {
    pub name: String,
    pub file_path: String,
    pub parent_class: String,
    pub actions: Vec<ControllerAction>,
    pub filters: Vec<ControllerFilter>,
    pub helpers: Vec<ControllerHelper>,
    pub concerns: Vec<String>,
    pub layout: Option<String>,
    pub respond_to_formats: Vec<String>,
}

#[derive(Debug, Default)]
pub struct EnhancedRubyControllerAnalyzer {
    pub controllers: HashMap<String, EnhancedRubyController>,
    pub routes: HashMap<String, Vec<(String, String, String)>>, // controller -> [(action, http_method, path)]
}

impl EnhancedRubyControllerAnalyzer {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
            routes: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Ruby controllers in directory: {:?}", directory);
        
        // First, try to find and parse routes.rb to extract routing information
        self.extract_routes(directory)?;
        
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "rb" {
                        // Check if it's likely a controller file
                        if let Some(file_name) = path.file_name() {
                            let file_name_str = file_name.to_string_lossy();
                            if file_name_str.ends_with("_controller.rb") {
                                self.analyze_controller_file(path)?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    fn extract_routes(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Try to find routes.rb file
        let routes_path = find_routes_file(directory);
        
        if let Some(routes_path) = routes_path {
            println!("Found routes file: {:?}", routes_path);
            let content = fs::read_to_string(routes_path)?;
            
            // Extract resource routes
            lazy_static! {
                static ref RESOURCES_REGEX: Regex = 
                    Regex::new(r"resources\s+:([a-z_]+)(?:,\s*(.+?))?(?:\n|\r|$)").unwrap();
            }
            
            for captures in RESOURCES_REGEX.captures_iter(&content) {
                let resource_name = captures.get(1).unwrap().as_str();
                let controller_name = format!("{}Controller", to_pascal_case(resource_name));
                
                // Standard RESTful actions
                let actions = vec![
                    ("index", "GET", format!("/{}", resource_name)),
                    ("show", "GET", format!("/{}/{{id}}", resource_name)),
                    ("new", "GET", format!("/{}/new", resource_name)),
                    ("create", "POST", format!("/{}", resource_name)),
                    ("edit", "GET", format!("/{}/{{id}}/edit", resource_name)),
                    ("update", "PUT/PATCH", format!("/{}/{{id}}", resource_name)),
                    ("destroy", "DELETE", format!("/{}/{{id}}", resource_name)),
                ];
                
                let mut controller_routes = Vec::new();
                for (action, method, path) in actions {
                    controller_routes.push((action.to_string(), method.to_string(), path));
                }
                
                self.routes.insert(controller_name, controller_routes);
            }
            
            // Extract custom routes
            lazy_static! {
                static ref CUSTOM_ROUTE_REGEX: Regex = 
                    Regex::new(r"(get|post|put|patch|delete)\s+['\"]([^'\"]+)['\"](?:,\s*to:\s*['\"]([^:]+):([^'\"]+)['\"])?").unwrap();
            }
            
            for captures in CUSTOM_ROUTE_REGEX.captures_iter(&content) {
                let http_method = captures.get(1).unwrap().as_str().to_uppercase();
                let path = captures.get(2).unwrap().as_str();
                
                if let (Some(controller_match), Some(action_match)) = (captures.get(3), captures.get(4)) {
                    let controller = controller_match.as_str();
                    let action = action_match.as_str();
                    
                    let controller_name = format!("{}Controller", to_pascal_case(controller));
                    
                    let controller_routes = self.routes.entry(controller_name).or_insert_with(Vec::new);
                    controller_routes.push((action.to_string(), http_method, path.to_string()));
                }
            }
        }
        
        Ok(())
    }

    pub fn analyze_controller_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing controller file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        
        // Extract controller name and parent class
        lazy_static! {
            static ref CONTROLLER_CLASS_REGEX: Regex = 
                Regex::new(r"class\s+([A-Za-z0-9:]+Controller)\s*<\s*([A-Za-z0-9:]+)").unwrap();
        }
        
        if let Some(captures) = CONTROLLER_CLASS_REGEX.captures(&content) {
            let controller_name = captures.get(1).unwrap().as_str().to_string();
            let parent_class = captures.get(2).unwrap().as_str().to_string();
            
            let mut controller = EnhancedRubyController {
                name: controller_name.clone(),
                file_path: file_path.to_string_lossy().to_string(),
                parent_class,
                ..Default::default()
            };
            
            // Extract actions
            self.extract_actions(&content, &mut controller);
            
            // Extract filters
            self.extract_filters(&content, &mut controller);
            
            // Extract helpers
            self.extract_helpers(&content, &mut controller);
            
            // Extract concerns
            self.extract_concerns(&content, &mut controller);
            
            // Extract layout
            self.extract_layout(&content, &mut controller);
            
            // Extract respond_to formats
            self.extract_respond_to(&content, &mut controller);
            
            // Add controller to the collection
            self.controllers.insert(controller_name, controller);
        }
        
        Ok(())
    }

    fn extract_actions(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract actions (public methods)
        lazy_static! {
            static ref ACTION_REGEX: Regex = 
                Regex::new(r"def\s+([a-z0-9_?!]+)(\([^)]*\))?(.*?)end").unwrap();
        }
        
        for captures in ACTION_REGEX.captures_iter(content) {
            let action_name = captures.get(1).unwrap().as_str().to_string();
            
            // Skip if it's a helper method (usually private)
            if action_name.starts_with("_") {
                continue;
            }
            
            let parameters_str = captures.get(2).map_or("".to_string(), |m| m.as_str().to_string());
            let body = captures.get(3).map_or("".to_string(), |m| m.as_str().trim().to_string());
            
            // Extract parameters
            let mut parameters = Vec::new();
            if !parameters_str.is_empty() {
                let params = parameters_str.trim_start_matches('(').trim_end_matches(')');
                for param in params.split(',') {
                    let param_name = param.trim().to_string();
                    if !param_name.is_empty() {
                        parameters.push(param_name);
                    }
                }
            }
            
            // Try to find HTTP method and path from routes
            let mut http_method = None;
            let mut path = None;
            
            if let Some(routes) = self.routes.get(&controller.name) {
                for (route_action, route_method, route_path) in routes {
                    if route_action == &action_name {
                        http_method = Some(route_method.clone());
                        path = Some(route_path.clone());
                        break;
                    }
                }
            }
            
            controller.actions.push(ControllerAction {
                name: action_name,
                http_method,
                path,
                parameters,
                body,
            });
        }
    }

    fn extract_filters(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract filters (before_action, after_action, etc.)
        lazy_static! {
            static ref FILTER_REGEX: Regex = 
                Regex::new(r"(before_action|after_action|around_action|skip_before_action|skip_after_action|skip_around_action)\s+:([a-z0-9_]+)(?:,\s*(.+?))?(?:\n|\r|$)").unwrap();
        }
        
        for captures in FILTER_REGEX.captures_iter(content) {
            let filter_type = captures.get(1).unwrap().as_str().to_string();
            let method_name = captures.get(2).unwrap().as_str().to_string();
            
            let mut filter = ControllerFilter {
                filter_type,
                methods: vec![method_name],
                options: HashMap::new(),
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Extract only: option
                if let Some(only_match) = Regex::new(r"only:\s*\[?:([a-z0-9_,\s]+)\]?").unwrap().captures(options) {
                    let only_str = only_match.get(1).unwrap().as_str();
                    filter.options.insert("only".to_string(), only_str.to_string());
                }
                
                // Extract except: option
                if let Some(except_match) = Regex::new(r"except:\s*\[?:([a-z0-9_,\s]+)\]?").unwrap().captures(options) {
                    let except_str = except_match.get(1).unwrap().as_str();
                    filter.options.insert("except".to_string(), except_str.to_string());
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|\[([^\]]+)\]|(\d+)|true|false)").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .or_else(|| option_match.get(5))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    filter.options.insert(key, value);
                }
            }
            
            controller.filters.push(filter);
        }
    }

    fn extract_helpers(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract helper methods (private methods)
        lazy_static! {
            static ref PRIVATE_SECTION_REGEX: Regex = 
                Regex::new(r"private(?:\s*\n|\s*\r\n)(.*?)(?:protected|public|\Z)").unwrap();
        }
        
        if let Some(captures) = PRIVATE_SECTION_REGEX.captures(content) {
            let private_section = captures.get(1).unwrap().as_str();
            
            lazy_static! {
                static ref HELPER_REGEX: Regex = 
                    Regex::new(r"def\s+([a-z0-9_?!]+)(\([^)]*\))?(.*?)end").unwrap();
            }
            
            for captures in HELPER_REGEX.captures_iter(private_section) {
                let helper_name = captures.get(1).unwrap().as_str().to_string();
                let parameters_str = captures.get(2).map_or("".to_string(), |m| m.as_str().to_string());
                let body = captures.get(3).map_or("".to_string(), |m| m.as_str().trim().to_string());
                
                // Extract parameters
                let mut parameters = Vec::new();
                if !parameters_str.is_empty() {
                    let params = parameters_str.trim_start_matches('(').trim_end_matches(')');
                    for param in params.split(',') {
                        let param_name = param.trim().to_string();
                        if !param_name.is_empty() {
                            parameters.push(param_name);
                        }
                    }
                }
                
                controller.helpers.push(ControllerHelper {
                    name: helper_name,
                    parameters,
                    body,
                });
            }
        }
    }

    fn extract_concerns(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract concerns
        lazy_static! {
            static ref CONCERN_REGEX: Regex = 
                Regex::new(r"include\s+([A-Za-z0-9:]+)").unwrap();
        }
        
        for captures in CONCERN_REGEX.captures_iter(content) {
            let concern = captures.get(1).unwrap().as_str().to_string();
            controller.concerns.push(concern);
        }
    }

    fn extract_layout(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract layout
        lazy_static! {
            static ref LAYOUT_REGEX: Regex = 
                Regex::new(r"layout\s+['\"]([^'\"]+)['\"]").unwrap();
        }
        
        if let Some(captures) = LAYOUT_REGEX.captures(content) {
            controller.layout = Some(captures.get(1).unwrap().as_str().to_string());
        }
    }

    fn extract_respond_to(&self, content: &str, controller: &mut EnhancedRubyController) {
        // Extract respond_to formats
        lazy_static! {
            static ref RESPOND_TO_REGEX: Regex = 
                Regex::new(r"respond_to\s+:([a-z,\s]+)").unwrap();
        }
        
        for captures in RESPOND_TO_REGEX.captures_iter(content) {
            let formats_str = captures.get(1).unwrap().as_str();
            for format in formats_str.split(',') {
                let format_name = format.trim().to_string();
                if !format_name.is_empty() {
                    controller.respond_to_formats.push(format_name);
                }
            }
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.controllers)
    }
}

// Helper function to find routes.rb file
fn find_routes_file(directory: &Path) -> Option<PathBuf> {
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if file_name == "routes.rb" {
                    return Some(path.to_path_buf());
                }
            }
        }
    }
    
    None
}

// Helper function to convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}
