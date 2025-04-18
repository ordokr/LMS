use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactProp {
    pub name: String,
    pub prop_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactState {
    pub name: String,
    pub state_type: String,
    pub initial_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactEffect {
    pub dependencies: Vec<String>,
    pub description: Option<String>,
    pub code_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactHandler {
    pub name: String,
    pub parameters: Vec<String>,
    pub code_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactComponent {
    pub name: String,
    pub file_path: String,
    pub component_type: String, // "functional", "class", "memo", "forwardRef"
    pub props: Vec<ReactProp>,
    pub state: Vec<ReactState>,
    pub effects: Vec<ReactEffect>,
    pub handlers: Vec<ReactHandler>,
    pub child_components: Vec<String>,
    pub imports: HashMap<String, String>,
    pub jsx_structure: Option<String>,
    pub hooks_used: Vec<String>,
    pub context_used: Vec<String>,
    pub raw_content: String,
}

#[derive(Debug, Default)]
pub struct EnhancedReactAnalyzer {
    pub components: HashMap<String, ReactComponent>,
}

impl EnhancedReactAnalyzer {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing React components in directory: {:?}", directory);
        
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Check for React component files
                    if extension == "jsx" || extension == "tsx" || 
                       (extension == "js" && !path.to_string_lossy().contains(".test.")) ||
                       (extension == "ts" && !path.to_string_lossy().contains(".test.")) {
                        self.analyze_component_file(path)?;
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn analyze_component_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing React component file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        
        // Check if file contains React component
        if !self.is_react_component(&content) {
            return Ok(());
        }
        
        // Extract component name from file name or export
        let component_name = self.extract_component_name(file_path, &content);
        
        let mut component = ReactComponent {
            name: component_name,
            file_path: file_path.to_string_lossy().to_string(),
            raw_content: content.clone(),
            ..Default::default()
        };
        
        // Determine component type
        component.component_type = self.determine_component_type(&content);
        
        // Extract imports
        self.extract_imports(&content, &mut component);
        
        // Extract props
        self.extract_props(&content, &mut component);
        
        // Extract state
        self.extract_state(&content, &mut component);
        
        // Extract effects
        self.extract_effects(&content, &mut component);
        
        // Extract event handlers
        self.extract_handlers(&content, &mut component);
        
        // Extract child components
        self.extract_child_components(&content, &mut component);
        
        // Extract JSX structure
        self.extract_jsx_structure(&content, &mut component);
        
        // Extract hooks used
        self.extract_hooks(&content, &mut component);
        
        // Extract context used
        self.extract_context(&content, &mut component);
        
        // Add component to the collection
        self.components.insert(component.file_path.clone(), component);
        
        Ok(())
    }

    fn is_react_component(&self, content: &str) -> bool {
        // Check for React import
        if content.contains("import React") || content.contains("from 'react'") || content.contains("from \"react\"") {
            return true;
        }
        
        // Check for JSX syntax
        if content.contains("return (") && (content.contains("<div") || content.contains("<span") || content.contains("<React.Fragment")) {
            return true;
        }
        
        // Check for component definition
        if content.contains("class ") && content.contains(" extends React.Component") {
            return true;
        }
        
        if content.contains("function ") && content.contains("return (") && content.contains("props") {
            return true;
        }
        
        false
    }

    fn extract_component_name(&self, file_path: &Path, content: &str) -> String {
        // Try to extract from export statement
        lazy_static! {
            static ref EXPORT_REGEX: Regex = 
                Regex::new(r"export\s+(default\s+)?(function|class|const)\s+([A-Za-z0-9_]+)").unwrap();
        }
        
        if let Some(captures) = EXPORT_REGEX.captures(content) {
            return captures.get(3).unwrap().as_str().to_string();
        }
        
        // Try to extract from named export
        lazy_static! {
            static ref NAMED_EXPORT_REGEX: Regex = 
                Regex::new(r"export\s+const\s+([A-Za-z0-9_]+)\s+=\s+(React\.)?memo\(").unwrap();
        }
        
        if let Some(captures) = NAMED_EXPORT_REGEX.captures(content) {
            return captures.get(1).unwrap().as_str().to_string();
        }
        
        // Fall back to file name
        if let Some(file_name) = file_path.file_stem() {
            let file_name_str = file_name.to_string_lossy();
            // Convert kebab-case to PascalCase
            if file_name_str.contains('-') {
                return file_name_str
                    .split('-')
                    .map(|part| {
                        let mut chars = part.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("");
            }
            
            // Convert from camelCase or snake_case to PascalCase
            let mut result = String::new();
            let mut capitalize_next = true;
            
            for c in file_name_str.chars() {
                if c == '_' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_uppercase().next().unwrap());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }
            
            return result;
        }
        
        "UnknownComponent".to_string()
    }

    fn determine_component_type(&self, content: &str) -> String {
        if content.contains("class ") && content.contains(" extends React.Component") {
            return "class".to_string();
        }
        
        if content.contains("React.memo(") || content.contains("memo(") {
            return "memo".to_string();
        }
        
        if content.contains("React.forwardRef(") || content.contains("forwardRef(") {
            return "forwardRef".to_string();
        }
        
        "functional".to_string()
    }

    fn extract_imports(&self, content: &str, component: &mut ReactComponent) {
        lazy_static! {
            static ref IMPORT_REGEX: Regex = 
                Regex::new(r"import\s+(?:(?:(\{[^}]+\})|([A-Za-z0-9_]+))\s+from\s+)?['\"]([^'\"]+)['\"]").unwrap();
        }
        
        for captures in IMPORT_REGEX.captures_iter(content) {
            let imported_module = captures.get(3).unwrap().as_str().to_string();
            
            if let Some(named_imports) = captures.get(1) {
                // Handle named imports like: import { X, Y } from 'module'
                let imports_str = named_imports.as_str().trim_matches(|c| c == '{' || c == '}');
                for import in imports_str.split(',') {
                    let import_name = import.trim().split(' ').next().unwrap().to_string();
                    component.imports.insert(import_name, imported_module.clone());
                }
            } else if let Some(default_import) = captures.get(2) {
                // Handle default import like: import X from 'module'
                let import_name = default_import.as_str().to_string();
                component.imports.insert(import_name, imported_module);
            } else {
                // Handle side-effect import like: import 'module'
                component.imports.insert("".to_string(), imported_module);
            }
        }
    }

    fn extract_props(&self, content: &str, component: &mut ReactComponent) {
        if component.component_type == "class" {
            // Extract props from class components
            lazy_static! {
                static ref CLASS_PROPS_REGEX: Regex = 
                    Regex::new(r"static\s+propTypes\s+=\s+\{\s*([\s\S]*?)\s*\}").unwrap();
            }
            
            if let Some(captures) = CLASS_PROPS_REGEX.captures(content) {
                let props_str = captures.get(1).unwrap().as_str();
                self.parse_prop_types(props_str, component);
            }
        } else {
            // Extract props from functional components
            lazy_static! {
                static ref FUNC_PROPS_REGEX: Regex = 
                    Regex::new(r"function\s+[A-Za-z0-9_]+\s*\(\s*(?:\{([^}]*)\}|([A-Za-z0-9_]+))\s*\)").unwrap();
            }
            
            if let Some(captures) = FUNC_PROPS_REGEX.captures(content) {
                if let Some(destructured_props) = captures.get(1) {
                    // Handle destructured props like: function Component({ prop1, prop2 })
                    let props_str = destructured_props.as_str();
                    for prop in props_str.split(',') {
                        let prop_name = prop.trim().split(':').next().unwrap().trim().to_string();
                        if !prop_name.is_empty() {
                            component.props.push(ReactProp {
                                name: prop_name,
                                prop_type: "any".to_string(),
                                required: false,
                                ..Default::default()
                            });
                        }
                    }
                } else if let Some(props_param) = captures.get(2) {
                    // Handle props object like: function Component(props)
                    // Try to find prop usages like props.X
                    lazy_static! {
                        static ref PROPS_USAGE_REGEX: Regex = 
                            Regex::new(&format!(r"{}\.([A-Za-z0-9_]+)", props_param.as_str())).unwrap();
                    }
                    
                    let mut prop_names = std::collections::HashSet::new();
                    for captures in PROPS_USAGE_REGEX.captures_iter(content) {
                        let prop_name = captures.get(1).unwrap().as_str().to_string();
                        prop_names.insert(prop_name);
                    }
                    
                    for prop_name in prop_names {
                        component.props.push(ReactProp {
                            name: prop_name,
                            prop_type: "any".to_string(),
                            required: false,
                            ..Default::default()
                        });
                    }
                }
            }
            
            // Look for PropTypes definition for functional components
            lazy_static! {
                static ref FUNC_PROPTYPES_REGEX: Regex = 
                    Regex::new(&format!(r"{}\.propTypes\s+=\s+\{{\s*([\s\S]*?)\s*\}}", component.name)).unwrap();
            }
            
            if let Some(captures) = FUNC_PROPTYPES_REGEX.captures(content) {
                let props_str = captures.get(1).unwrap().as_str();
                self.parse_prop_types(props_str, component);
            }
            
            // Look for TypeScript props interface
            lazy_static! {
                static ref TS_PROPS_INTERFACE_REGEX: Regex = 
                    Regex::new(r"interface\s+([A-Za-z0-9_]+Props)\s*\{\s*([\s\S]*?)\s*\}").unwrap();
            }
            
            if let Some(captures) = TS_PROPS_INTERFACE_REGEX.captures(content) {
                let props_str = captures.get(2).unwrap().as_str();
                self.parse_typescript_props(props_str, component);
            }
        }
    }

    fn parse_prop_types(&self, props_str: &str, component: &mut ReactComponent) {
        lazy_static! {
            static ref PROP_REGEX: Regex = 
                Regex::new(r"([A-Za-z0-9_]+)\s*:\s*PropTypes\.([A-Za-z0-9_]+)(?:\.([A-Za-z0-9_]+))?").unwrap();
        }
        
        for captures in PROP_REGEX.captures_iter(props_str) {
            let prop_name = captures.get(1).unwrap().as_str().to_string();
            let prop_type = captures.get(2).unwrap().as_str().to_string();
            let required = captures.get(3).map_or(false, |m| m.as_str() == "isRequired");
            
            component.props.push(ReactProp {
                name: prop_name,
                prop_type,
                required,
                ..Default::default()
            });
        }
    }

    fn parse_typescript_props(&self, props_str: &str, component: &mut ReactComponent) {
        lazy_static! {
            static ref TS_PROP_REGEX: Regex = 
                Regex::new(r"([A-Za-z0-9_]+)\s*(\??):\s*([A-Za-z0-9_<>|&\[\]]+)").unwrap();
        }
        
        for captures in TS_PROP_REGEX.captures_iter(props_str) {
            let prop_name = captures.get(1).unwrap().as_str().to_string();
            let optional = captures.get(2).map_or(false, |m| m.as_str() == "?");
            let prop_type = captures.get(3).unwrap().as_str().to_string();
            
            component.props.push(ReactProp {
                name: prop_name,
                prop_type,
                required: !optional,
                ..Default::default()
            });
        }
    }

    fn extract_state(&self, content: &str, component: &mut ReactComponent) {
        if component.component_type == "class" {
            // Extract state from class components
            lazy_static! {
                static ref CLASS_STATE_REGEX: Regex = 
                    Regex::new(r"this\.state\s+=\s+\{\s*([\s\S]*?)\s*\}").unwrap();
            }
            
            if let Some(captures) = CLASS_STATE_REGEX.captures(content) {
                let state_str = captures.get(1).unwrap().as_str();
                self.parse_class_state(state_str, component);
            }
        } else {
            // Extract state from useState hooks
            lazy_static! {
                static ref USE_STATE_REGEX: Regex = 
                    Regex::new(r"const\s+\[\s*([A-Za-z0-9_]+)\s*,\s*set([A-Za-z0-9_]+)\s*\]\s*=\s*useState\s*\(\s*(.*?)\s*\)").unwrap();
            }
            
            for captures in USE_STATE_REGEX.captures_iter(content) {
                let state_name = captures.get(1).unwrap().as_str().to_string();
                let initial_value = captures.get(3).map(|m| m.as_str().to_string());
                
                // Try to determine state type from initial value
                let state_type = if let Some(ref value) = initial_value {
                    if value == "[]" {
                        "array".to_string()
                    } else if value == "{}" {
                        "object".to_string()
                    } else if value == "true" || value == "false" {
                        "boolean".to_string()
                    } else if value.starts_with('"') || value.starts_with('\'') {
                        "string".to_string()
                    } else if value.parse::<f64>().is_ok() {
                        "number".to_string()
                    } else {
                        "any".to_string()
                    }
                } else {
                    "any".to_string()
                };
                
                component.state.push(ReactState {
                    name: state_name,
                    state_type,
                    initial_value,
                });
            }
        }
    }

    fn parse_class_state(&self, state_str: &str, component: &mut ReactComponent) {
        lazy_static! {
            static ref STATE_PROP_REGEX: Regex = 
                Regex::new(r"([A-Za-z0-9_]+)\s*:\s*([^,]+)").unwrap();
        }
        
        for captures in STATE_PROP_REGEX.captures_iter(state_str) {
            let state_name = captures.get(1).unwrap().as_str().to_string();
            let initial_value = captures.get(2).map(|m| m.as_str().trim().to_string());
            
            // Try to determine state type from initial value
            let state_type = if let Some(ref value) = initial_value {
                if value == "[]" {
                    "array".to_string()
                } else if value == "{}" {
                    "object".to_string()
                } else if value == "true" || value == "false" {
                    "boolean".to_string()
                } else if value.starts_with('"') || value.starts_with('\'') {
                    "string".to_string()
                } else if value.parse::<f64>().is_ok() {
                    "number".to_string()
                } else {
                    "any".to_string()
                }
            } else {
                "any".to_string()
            };
            
            component.state.push(ReactState {
                name: state_name,
                state_type,
                initial_value,
            });
        }
    }

    fn extract_effects(&self, content: &str, component: &mut ReactComponent) {
        if component.component_type == "class" {
            // Extract lifecycle methods from class components
            lazy_static! {
                static ref COMPONENT_DID_MOUNT_REGEX: Regex = 
                    Regex::new(r"componentDidMount\s*\(\s*\)\s*\{([\s\S]*?)\}").unwrap();
            }
            
            if let Some(captures) = COMPONENT_DID_MOUNT_REGEX.captures(content) {
                let effect_code = captures.get(1).unwrap().as_str().trim().to_string();
                component.effects.push(ReactEffect {
                    dependencies: vec![],
                    description: Some("componentDidMount lifecycle method".to_string()),
                    code_snippet: effect_code,
                });
            }
            
            lazy_static! {
                static ref COMPONENT_DID_UPDATE_REGEX: Regex = 
                    Regex::new(r"componentDidUpdate\s*\(\s*prevProps,\s*prevState\s*\)\s*\{([\s\S]*?)\}").unwrap();
            }
            
            if let Some(captures) = COMPONENT_DID_UPDATE_REGEX.captures(content) {
                let effect_code = captures.get(1).unwrap().as_str().trim().to_string();
                component.effects.push(ReactEffect {
                    dependencies: vec![],
                    description: Some("componentDidUpdate lifecycle method".to_string()),
                    code_snippet: effect_code,
                });
            }
        } else {
            // Extract useEffect hooks
            lazy_static! {
                static ref USE_EFFECT_REGEX: Regex = 
                    Regex::new(r"useEffect\s*\(\s*\(\s*\)\s*=>\s*\{([\s\S]*?)\}\s*,\s*\[(.*?)\]\s*\)").unwrap();
            }
            
            for captures in USE_EFFECT_REGEX.captures_iter(content) {
                let effect_code = captures.get(1).unwrap().as_str().trim().to_string();
                let dependencies_str = captures.get(2).map_or("", |m| m.as_str().trim());
                
                let dependencies = if dependencies_str.is_empty() {
                    vec![]
                } else {
                    dependencies_str.split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };
                
                component.effects.push(ReactEffect {
                    dependencies,
                    description: None,
                    code_snippet: effect_code,
                });
            }
        }
    }

    fn extract_handlers(&self, content: &str, component: &mut ReactComponent) {
        if component.component_type == "class" {
            // Extract handlers from class components
            lazy_static! {
                static ref CLASS_HANDLER_REGEX: Regex = 
                    Regex::new(r"(?:handle|on)([A-Z][A-Za-z0-9_]*)\s*=\s*\(\s*(.*?)\s*\)\s*=>\s*\{([\s\S]*?)\}").unwrap();
            }
            
            for captures in CLASS_HANDLER_REGEX.captures_iter(content) {
                let handler_name = format!("handle{}", captures.get(1).unwrap().as_str());
                let params_str = captures.get(2).map_or("", |m| m.as_str());
                let handler_code = captures.get(3).unwrap().as_str().trim().to_string();
                
                let parameters = if params_str.is_empty() {
                    vec![]
                } else {
                    params_str.split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };
                
                component.handlers.push(ReactHandler {
                    name: handler_name,
                    parameters,
                    code_snippet: handler_code,
                });
            }
        } else {
            // Extract handlers from functional components
            lazy_static! {
                static ref FUNC_HANDLER_REGEX: Regex = 
                    Regex::new(r"const\s+(?:handle|on)([A-Z][A-Za-z0-9_]*)\s*=\s*(?:useCallback\s*\()?\(\s*(.*?)\s*\)\s*=>\s*\{([\s\S]*?)\}").unwrap();
            }
            
            for captures in FUNC_HANDLER_REGEX.captures_iter(content) {
                let handler_name = format!("handle{}", captures.get(1).unwrap().as_str());
                let params_str = captures.get(2).map_or("", |m| m.as_str());
                let handler_code = captures.get(3).unwrap().as_str().trim().to_string();
                
                let parameters = if params_str.is_empty() {
                    vec![]
                } else {
                    params_str.split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };
                
                component.handlers.push(ReactHandler {
                    name: handler_name,
                    parameters,
                    code_snippet: handler_code,
                });
            }
        }
    }

    fn extract_child_components(&self, content: &str, component: &mut ReactComponent) {
        // Extract child components from JSX
        lazy_static! {
            static ref JSX_COMPONENT_REGEX: Regex = 
                Regex::new(r"<([A-Z][A-Za-z0-9_]*)[^>]*>").unwrap();
        }
        
        let mut child_components = std::collections::HashSet::new();
        
        for captures in JSX_COMPONENT_REGEX.captures_iter(content) {
            let component_name = captures.get(1).unwrap().as_str().to_string();
            // Skip HTML elements and React fragments
            if !component_name.starts_with("React") && component_name != "Fragment" {
                child_components.insert(component_name);
            }
        }
        
        component.child_components = child_components.into_iter().collect();
    }

    fn extract_jsx_structure(&self, content: &str, component: &mut ReactComponent) {
        // Extract JSX structure from return statement
        if component.component_type == "class" {
            lazy_static! {
                static ref CLASS_RENDER_REGEX: Regex = 
                    Regex::new(r"render\s*\(\s*\)\s*\{\s*return\s*\(([\s\S]*?)\);\s*\}").unwrap();
            }
            
            if let Some(captures) = CLASS_RENDER_REGEX.captures(content) {
                component.jsx_structure = Some(captures.get(1).unwrap().as_str().trim().to_string());
            }
        } else {
            lazy_static! {
                static ref FUNC_RETURN_REGEX: Regex = 
                    Regex::new(r"return\s*\(([\s\S]*?)\);").unwrap();
            }
            
            if let Some(captures) = FUNC_RETURN_REGEX.captures(content) {
                component.jsx_structure = Some(captures.get(1).unwrap().as_str().trim().to_string());
            }
        }
    }

    fn extract_hooks(&self, content: &str, component: &mut ReactComponent) {
        // Extract React hooks
        lazy_static! {
            static ref HOOKS_REGEX: Regex = 
                Regex::new(r"use([A-Z][A-Za-z0-9_]*)\s*\(").unwrap();
        }
        
        let mut hooks = std::collections::HashSet::new();
        
        for captures in HOOKS_REGEX.captures_iter(content) {
            let hook_name = format!("use{}", captures.get(1).unwrap().as_str());
            hooks.insert(hook_name);
        }
        
        component.hooks_used = hooks.into_iter().collect();
    }

    fn extract_context(&self, content: &str, component: &mut ReactComponent) {
        // Extract React context usage
        lazy_static! {
            static ref USE_CONTEXT_REGEX: Regex = 
                Regex::new(r"useContext\s*\(\s*([A-Za-z0-9_]+)Context\s*\)").unwrap();
        }
        
        let mut contexts = std::collections::HashSet::new();
        
        for captures in USE_CONTEXT_REGEX.captures_iter(content) {
            let context_name = format!("{}Context", captures.get(1).unwrap().as_str());
            contexts.insert(context_name);
        }
        
        component.context_used = contexts.into_iter().collect();
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.components)
    }
}
