use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct EnhancedReactProp {
    pub name: String,
    pub prop_type: Option<String>,
    pub default_value: Option<String>,
    pub is_required: bool,
}

#[derive(Debug, Clone)]
pub struct EnhancedReactState {
    pub name: String,
    pub state_type: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedReactMethod {
    pub name: String,
    pub is_lifecycle: bool,
    pub parameters: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct EnhancedReactImport {
    pub module: String,
    pub items: Vec<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct EnhancedReactComponent {
    pub name: String,
    pub file_path: String,
    pub is_functional: bool,
    pub props: Vec<EnhancedReactProp>,
    pub state: Vec<EnhancedReactState>,
    pub methods: Vec<EnhancedReactMethod>,
    pub imports: Vec<EnhancedReactImport>,
    pub jsx: Option<String>,
}

pub struct EnhancedReactAnalyzer {
    pub components: HashMap<String, EnhancedReactComponent>,
}

impl EnhancedReactAnalyzer {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
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
                if extension == "jsx" || extension == "tsx" || extension == "js" || extension == "ts" {
                    self.analyze_file(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        
        // Check if this is a React component file
        if !self.is_react_component(&content) {
            return Ok(());
        }
        
        // Extract component name from file name or class/function name
        let component_name = self.extract_component_name(file_path, &content);
        
        // Determine if this is a functional component
        let is_functional = self.is_functional_component(&content);
        
        // Create component
        let mut component = EnhancedReactComponent {
            name: component_name,
            file_path: file_path.to_string_lossy().to_string(),
            is_functional,
            props: Vec::new(),
            state: Vec::new(),
            methods: Vec::new(),
            imports: Vec::new(),
            jsx: None,
        };
        
        // Extract imports
        self.extract_imports(&content, &mut component);
        
        // Extract props
        self.extract_props(&content, &mut component);
        
        // Extract state
        if !is_functional {
            self.extract_state(&content, &mut component);
        }
        
        // Extract methods
        self.extract_methods(&content, &mut component);
        
        // Extract JSX
        self.extract_jsx(&content, &mut component);
        
        self.components.insert(component.file_path.clone(), component);
        
        Ok(())
    }
    
    fn is_react_component(&self, content: &str) -> bool {
        // Check if the file imports React
        if content.contains("import React") || content.contains("from 'react'") || content.contains("from \"react\"") {
            return true;
        }
        
        // Check if the file extends React.Component
        if content.contains("extends React.Component") || content.contains("extends Component") {
            return true;
        }
        
        // Check if the file contains JSX
        if content.contains("return (") && (content.contains("<div") || content.contains("<span") || content.contains("<React.Fragment")) {
            return true;
        }
        
        false
    }
    
    fn extract_component_name(&self, file_path: &Path, content: &str) -> String {
        // Try to extract from class definition
        lazy_static! {
            static ref CLASS_REGEX: Regex =
                Regex::new(r#"class\s+([A-Za-z0-9_]+)\s+extends\s+(React\.Component|Component)"#).unwrap();
        }
        
        if let Some(captures) = CLASS_REGEX.captures(content) {
            return captures.get(1).unwrap().as_str().to_string();
        }
        
        // Try to extract from function definition
        lazy_static! {
            static ref FUNCTION_REGEX: Regex =
                Regex::new(r#"(?:function|const)\s+([A-Za-z0-9_]+)\s*(?:=\s*(?:\([^)]*\)|[A-Za-z0-9_]+)\s*=>|\([^)]*\)\s*{)"#).unwrap();
        }
        
        if let Some(captures) = FUNCTION_REGEX.captures(content) {
            return captures.get(1).unwrap().as_str().to_string();
        }
        
        // Fall back to file name
        file_path.file_stem().unwrap().to_string_lossy().to_string()
    }
    
    fn is_functional_component(&self, content: &str) -> bool {
        // Check if the file contains a class component
        if content.contains("extends React.Component") || content.contains("extends Component") {
            return false;
        }
        
        // Check if the file contains a functional component
        if content.contains("function ") && content.contains("return (") {
            return true;
        }
        
        if content.contains("const ") && content.contains(" = (") && content.contains("return (") {
            return true;
        }
        
        if content.contains("const ") && content.contains(" = props") && content.contains("return (") {
            return true;
        }
        
        // Default to functional if we can't determine
        true
    }
    
    fn extract_imports(&self, content: &str, component: &mut EnhancedReactComponent) {
        // Extract import statements
        lazy_static! {
            static ref IMPORT_REGEX: Regex =
                Regex::new(r#"import\s+(?:(?:(\{[^}]+\})|([A-Za-z0-9_]+))\s+from\s+)?['"]([^'"]+)['"]"#).unwrap();
        }
        
        for captures in IMPORT_REGEX.captures_iter(content) {
            let module = captures.get(3).unwrap().as_str().to_string();
            
            let mut import = EnhancedReactImport {
                module,
                items: Vec::new(),
                is_default: false,
            };
            
            // Check if this is a default import
            if let Some(default_import) = captures.get(2) {
                import.items.push(default_import.as_str().to_string());
                import.is_default = true;
            }
            
            // Check if this is a named import
            if let Some(named_imports) = captures.get(1) {
                let named_imports_str = named_imports.as_str();
                let items = named_imports_str.trim_start_matches('{').trim_end_matches('}');
                
                for item in items.split(',') {
                    let item = item.trim();
                    if !item.is_empty() {
                        import.items.push(item.to_string());
                    }
                }
            }
            
            component.imports.push(import);
        }
    }
    
    fn extract_props(&self, content: &str, component: &mut EnhancedReactComponent) {
        if component.is_functional {
            // Extract props from functional component
            lazy_static! {
                static ref PROPS_REGEX: Regex =
                    Regex::new(r#"(?:function|const)\s+[A-Za-z0-9_]+\s*(?:=\s*)?\(\s*(?:\{([^}]*)\}|([A-Za-z0-9_]+))\s*\)"#).unwrap();
            }
            
            if let Some(captures) = PROPS_REGEX.captures(content) {
                if let Some(props_list) = captures.get(1) {
                    // Destructured props
                    let props_str = props_list.as_str();
                    for prop in props_str.split(',') {
                        let prop = prop.trim();
                        if !prop.is_empty() {
                            let parts: Vec<&str> = prop.split('=').collect();
                            let name = parts[0].trim().to_string();
                            
                            let default_value = if parts.len() > 1 {
                                Some(parts[1].trim().to_string())
                            } else {
                                None
                            };
                            
                            let prop = EnhancedReactProp {
                                name,
                                prop_type: None,
                                default_value,
                                is_required: false,
                            };
                            
                            component.props.push(prop);
                        }
                    }
                } else if let Some(props_param) = captures.get(2) {
                    // Single props parameter
                    let prop = EnhancedReactProp {
                        name: props_param.as_str().to_string(),
                        prop_type: None,
                        default_value: None,
                        is_required: false,
                    };
                    
                    component.props.push(prop);
                }
            }
        } else {
            // Extract props from class component
            lazy_static! {
                static ref PROP_TYPES_REGEX: Regex =
                    Regex::new(r#"static\s+propTypes\s*=\s*\{([^}]*)\}"#).unwrap();
            }
            
            if let Some(captures) = PROP_TYPES_REGEX.captures(content) {
                let prop_types_str = captures.get(1).unwrap().as_str();
                
                for prop_line in prop_types_str.split(',') {
                    let prop_line = prop_line.trim();
                    if !prop_line.is_empty() {
                        let parts: Vec<&str> = prop_line.split(':').collect();
                        if parts.len() >= 2 {
                            let name = parts[0].trim().to_string();
                            let prop_type_str = parts[1].trim();
                            
                            let prop_type = Some(prop_type_str.to_string());
                            let is_required = prop_type_str.contains(".isRequired");
                            
                            let prop = EnhancedReactProp {
                                name,
                                prop_type,
                                default_value: None,
                                is_required,
                            };
                            
                            component.props.push(prop);
                        }
                    }
                }
            }
            
            // Extract default props
            lazy_static! {
                static ref DEFAULT_PROPS_REGEX: Regex =
                    Regex::new(r#"static\s+defaultProps\s*=\s*\{([^}]*)\}"#).unwrap();
            }
            
            if let Some(captures) = DEFAULT_PROPS_REGEX.captures(content) {
                let default_props_str = captures.get(1).unwrap().as_str();
                
                for prop_line in default_props_str.split(',') {
                    let prop_line = prop_line.trim();
                    if !prop_line.is_empty() {
                        let parts: Vec<&str> = prop_line.split(':').collect();
                        if parts.len() >= 2 {
                            let name = parts[0].trim().to_string();
                            let default_value = parts[1].trim().to_string();
                            
                            // Update existing prop or add new one
                            let mut found = false;
                            for prop in &mut component.props {
                                if prop.name == name {
                                    prop.default_value = Some(default_value.clone());
                                    found = true;
                                    break;
                                }
                            }
                            
                            if !found {
                                let prop = EnhancedReactProp {
                                    name,
                                    prop_type: None,
                                    default_value: Some(default_value),
                                    is_required: false,
                                };
                                
                                component.props.push(prop);
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn extract_state(&self, content: &str, component: &mut EnhancedReactComponent) {
        // Extract state from constructor
        lazy_static! {
            static ref CONSTRUCTOR_REGEX: Regex =
                Regex::new(r#"constructor\s*\([^)]*\)\s*\{[^{]*this\.state\s*=\s*\{([^}]*)\}"#).unwrap();
        }
        
        if let Some(captures) = CONSTRUCTOR_REGEX.captures(content) {
            let state_str = captures.get(1).unwrap().as_str();
            
            for state_line in state_str.split(',') {
                let state_line = state_line.trim();
                if !state_line.is_empty() {
                    let parts: Vec<&str> = state_line.split(':').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim().to_string();
                        let default_value = parts[1].trim().to_string();
                        
                        let state = EnhancedReactState {
                            name,
                            state_type: None,
                            default_value: Some(default_value),
                        };
                        
                        component.state.push(state);
                    }
                }
            }
        }
        
        // Extract state from class field
        lazy_static! {
            static ref STATE_FIELD_REGEX: Regex =
                Regex::new(r#"state\s*=\s*\{([^}]*)\}"#).unwrap();
        }
        
        if let Some(captures) = STATE_FIELD_REGEX.captures(content) {
            let state_str = captures.get(1).unwrap().as_str();
            
            for state_line in state_str.split(',') {
                let state_line = state_line.trim();
                if !state_line.is_empty() {
                    let parts: Vec<&str> = state_line.split(':').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim().to_string();
                        let default_value = parts[1].trim().to_string();
                        
                        // Check if this state already exists
                        let mut found = false;
                        for state in &component.state {
                            if state.name == name {
                                found = true;
                                break;
                            }
                        }
                        
                        if !found {
                            let state = EnhancedReactState {
                                name,
                                state_type: None,
                                default_value: Some(default_value),
                            };
                            
                            component.state.push(state);
                        }
                    }
                }
            }
        }
    }
    
    fn extract_methods(&self, content: &str, component: &mut EnhancedReactComponent) {
        if component.is_functional {
            // Extract hooks and helper functions
            lazy_static! {
                static ref FUNCTION_REGEX: Regex =
                    Regex::new(r#"const\s+([A-Za-z0-9_]+)\s*=\s*(?:useCallback|useMemo|useEffect)?\s*\(\s*(?:\([^)]*\)|)\s*=>\s*\{([^}]*(?:\{[^}]*\})*[^}]*)\}"#).unwrap();
            }
            
            for captures in FUNCTION_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let body = captures.get(2).unwrap().as_str().to_string();
                
                // Skip the component itself
                if name == component.name {
                    continue;
                }
                
                let method = EnhancedReactMethod {
                    name,
                    is_lifecycle: false,
                    parameters: Vec::new(),
                    body,
                };
                
                component.methods.push(method);
            }
        } else {
            // Extract class methods
            lazy_static! {
                static ref METHOD_REGEX: Regex =
                    Regex::new(r#"(?:async\s+)?([a-zA-Z0-9_]+)\s*=?\s*(?:\([^)]*\)|)\s*(?:=>)?\s*\{([^}]*(?:\{[^}]*\})*[^}]*)\}"#).unwrap();
            }
            
            for captures in METHOD_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let body = captures.get(2).unwrap().as_str().to_string();
                
                // Skip constructor and render
                if name == "constructor" || name == "render" {
                    continue;
                }
                
                // Check if this is a lifecycle method
                let is_lifecycle = name == "componentDidMount" || 
                                  name == "componentDidUpdate" || 
                                  name == "componentWillUnmount" || 
                                  name == "shouldComponentUpdate" || 
                                  name == "getSnapshotBeforeUpdate" || 
                                  name == "componentDidCatch";
                
                let method = EnhancedReactMethod {
                    name,
                    is_lifecycle,
                    parameters: Vec::new(),
                    body,
                };
                
                component.methods.push(method);
            }
        }
    }
    
    fn extract_jsx(&self, content: &str, component: &mut EnhancedReactComponent) {
        if component.is_functional {
            // Extract JSX from return statement
            lazy_static! {
                static ref JSX_REGEX: Regex =
                    Regex::new(r#"return\s*\(\s*(<.*?>.*?</.*?>|<.*?/>)\s*\)"#).unwrap();
            }
            
            if let Some(captures) = JSX_REGEX.captures(content) {
                component.jsx = Some(captures.get(1).unwrap().as_str().to_string());
            }
        } else {
            // Extract JSX from render method
            lazy_static! {
                static ref RENDER_REGEX: Regex =
                    Regex::new(r#"render\s*\(\s*\)\s*\{[^{]*return\s*\(\s*(<.*?>.*?</.*?>|<.*?/>)\s*\)"#).unwrap();
            }
            
            if let Some(captures) = RENDER_REGEX.captures(content) {
                component.jsx = Some(captures.get(1).unwrap().as_str().to_string());
            }
        }
    }
}
