use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct EnhancedEmberProperty {
    pub name: String,
    pub property_type: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedEmberAction {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct EnhancedEmberService {
    pub name: String,
    pub service_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedEmberComputed {
    pub name: String,
    pub dependencies: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct EnhancedEmberObserver {
    pub name: String,
    pub observed_property: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct EnhancedEmberImport {
    pub module: String,
    pub items: Vec<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct EnhancedEmberComponent {
    pub name: String,
    pub file_path: String,
    pub is_glimmer: bool,
    pub properties: Vec<EnhancedEmberProperty>,
    pub actions: Vec<EnhancedEmberAction>,
    pub services: Vec<EnhancedEmberService>,
    pub computed: Vec<EnhancedEmberComputed>,
    pub observers: Vec<EnhancedEmberObserver>,
    pub imports: Vec<EnhancedEmberImport>,
    pub template: Option<String>,
}

pub struct EnhancedEmberAnalyzer {
    pub components: HashMap<String, EnhancedEmberComponent>,
}

impl EnhancedEmberAnalyzer {
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
                if extension == "js" || extension == "ts" {
                    self.analyze_file(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        
        // Check if this is an Ember component file
        if !self.is_ember_component(&content) {
            return Ok(());
        }
        
        // Extract component name from file name or class/function name
        let component_name = self.extract_component_name(file_path, &content);
        
        // Determine if this is a Glimmer component
        let is_glimmer = self.is_glimmer_component(&content);
        
        // Create component
        let mut component = EnhancedEmberComponent {
            name: component_name,
            file_path: file_path.to_string_lossy().to_string(),
            is_glimmer,
            properties: Vec::new(),
            actions: Vec::new(),
            services: Vec::new(),
            computed: Vec::new(),
            observers: Vec::new(),
            imports: Vec::new(),
            template: None,
        };
        
        // Extract imports
        self.extract_imports(&content, &mut component);
        
        // Extract properties
        self.extract_properties(&content, &mut component);
        
        // Extract actions
        self.extract_actions(&content, &mut component);
        
        // Extract services
        self.extract_services(&content, &mut component);
        
        // Extract computed properties
        self.extract_computed(&content, &mut component);
        
        // Extract observers
        self.extract_observers(&content, &mut component);
        
        // Extract template
        self.extract_template(&content, &mut component);
        
        self.components.insert(component.file_path.clone(), component);
        
        Ok(())
    }
    
    fn is_ember_component(&self, content: &str) -> bool {
        // Check if the file imports Ember
        if content.contains("import Ember") || content.contains("from 'ember'") || content.contains("from \"ember\"") {
            return true;
        }
        
        // Check if the file extends Ember.Component or Component
        if content.contains("extends Ember.Component") || content.contains("extends Component") {
            return true;
        }
        
        // Check if the file is a Glimmer component
        if content.contains("import Component from '@glimmer/component'") || content.contains("import Component from \"@glimmer/component\"") {
            return true;
        }
        
        // Check if the file contains Ember-specific features
        if content.contains("@action") || content.contains("@service") || content.contains("@computed") {
            return true;
        }
        
        false
    }
    
    fn extract_component_name(&self, file_path: &Path, content: &str) -> String {
        // Try to extract from class definition
        lazy_static! {
            static ref CLASS_REGEX: Regex =
                Regex::new(r#"class\s+([A-Za-z0-9_]+)\s+extends\s+(Ember\.Component|Component)"#).unwrap();
        }
        
        if let Some(captures) = CLASS_REGEX.captures(content) {
            return captures.get(1).unwrap().as_str().to_string();
        }
        
        // Fall back to file name
        file_path.file_stem().unwrap().to_string_lossy().to_string()
    }
    
    fn is_glimmer_component(&self, content: &str) -> bool {
        // Check if the file imports from @glimmer/component
        if content.contains("import Component from '@glimmer/component'") || content.contains("import Component from \"@glimmer/component\"") {
            return true;
        }
        
        // Check if the file uses @tracked
        if content.contains("@tracked") {
            return true;
        }
        
        false
    }
    
    fn extract_imports(&self, content: &str, component: &mut EnhancedEmberComponent) {
        // Extract import statements
        lazy_static! {
            static ref IMPORT_REGEX: Regex =
                Regex::new(r#"import\s+(?:(?:(\{[^}]+\})|([A-Za-z0-9_]+))\s+from\s+)?['"]([^'"]+)['"]"#).unwrap();
        }
        
        for captures in IMPORT_REGEX.captures_iter(content) {
            let module = captures.get(3).unwrap().as_str().to_string();
            
            let mut import = EnhancedEmberImport {
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
    
    fn extract_properties(&self, content: &str, component: &mut EnhancedEmberComponent) {
        if component.is_glimmer {
            // Extract properties from Glimmer component
            lazy_static! {
                static ref TRACKED_PROPERTY_REGEX: Regex =
                    Regex::new(r#"@tracked\s+([a-zA-Z0-9_]+)(?:\s*=\s*([^;]+))?"#).unwrap();
            }
            
            for captures in TRACKED_PROPERTY_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let default_value = captures.get(2).map(|m| m.as_str().to_string());
                
                let property = EnhancedEmberProperty {
                    name,
                    property_type: None,
                    default_value,
                };
                
                component.properties.push(property);
            }
            
            // Extract constructor arguments
            lazy_static! {
                static ref CONSTRUCTOR_ARGS_REGEX: Regex =
                    Regex::new(r#"constructor\s*\(\s*(?:args|([a-zA-Z0-9_]+))\s*(?:,\s*([^)]+))?\s*\)"#).unwrap();
            }
            
            if let Some(captures) = CONSTRUCTOR_ARGS_REGEX.captures(content) {
                if let Some(args_param) = captures.get(1) {
                    let property = EnhancedEmberProperty {
                        name: args_param.as_str().to_string(),
                        property_type: None,
                        default_value: None,
                    };
                    
                    component.properties.push(property);
                }
            }
        } else {
            // Extract properties from classic Ember component
            lazy_static! {
                static ref PROPERTY_REGEX: Regex =
                    Regex::new(r#"([a-zA-Z0-9_]+):\s*([^,]+)(?:,|$)"#).unwrap();
            }
            
            for captures in PROPERTY_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let value = captures.get(2).unwrap().as_str().trim().to_string();
                
                // Skip functions, computed properties, and observers
                if value.starts_with("function") || value.starts_with("Ember.computed") || value.starts_with("computed") || value.starts_with("observer") {
                    continue;
                }
                
                let property = EnhancedEmberProperty {
                    name,
                    property_type: None,
                    default_value: Some(value),
                };
                
                component.properties.push(property);
            }
        }
    }
    
    fn extract_actions(&self, content: &str, component: &mut EnhancedEmberComponent) {
        if component.is_glimmer {
            // Extract actions from Glimmer component
            lazy_static! {
                static ref GLIMMER_ACTION_REGEX: Regex =
                    Regex::new(r#"@action\s+([a-zA-Z0-9_]+)\s*\(([^)]*)\)\s*\{([^}]*)\}"#).unwrap();
            }
            
            for captures in GLIMMER_ACTION_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let params_str = captures.get(2).unwrap().as_str();
                let body = captures.get(3).unwrap().as_str().to_string();
                
                let mut parameters = Vec::new();
                for param in params_str.split(',') {
                    let param = param.trim();
                    if !param.is_empty() {
                        parameters.push(param.to_string());
                    }
                }
                
                let action = EnhancedEmberAction {
                    name,
                    parameters,
                    body,
                };
                
                component.actions.push(action);
            }
        } else {
            // Extract actions from classic Ember component
            lazy_static! {
                static ref CLASSIC_ACTIONS_REGEX: Regex =
                    Regex::new(r#"actions:\s*\{([^}]*)\}"#).unwrap();
            }
            
            if let Some(captures) = CLASSIC_ACTIONS_REGEX.captures(content) {
                let actions_block = captures.get(1).unwrap().as_str();
                
                // Extract individual actions
                lazy_static! {
                    static ref ACTION_REGEX: Regex =
                        Regex::new(r#"([a-zA-Z0-9_]+)\s*\(([^)]*)\)\s*\{([^}]*)\}"#).unwrap();
                }
                
                for action_match in ACTION_REGEX.captures_iter(actions_block) {
                    let name = action_match.get(1).unwrap().as_str().to_string();
                    let params_str = action_match.get(2).unwrap().as_str();
                    let body = action_match.get(3).unwrap().as_str().to_string();
                    
                    let mut parameters = Vec::new();
                    for param in params_str.split(',') {
                        let param = param.trim();
                        if !param.is_empty() {
                            parameters.push(param.to_string());
                        }
                    }
                    
                    let action = EnhancedEmberAction {
                        name,
                        parameters,
                        body,
                    };
                    
                    component.actions.push(action);
                }
            }
        }
    }
    
    fn extract_services(&self, content: &str, component: &mut EnhancedEmberComponent) {
        if component.is_glimmer {
            // Extract services from Glimmer component
            lazy_static! {
                static ref GLIMMER_SERVICE_REGEX: Regex =
                    Regex::new(r#"@service(?:\s*\(\s*(?:'([^']+)'|"([^"]+)")\s*\))?\s+([a-zA-Z0-9_]+)"#).unwrap();
            }
            
            for captures in GLIMMER_SERVICE_REGEX.captures_iter(content) {
                let service_name = captures.get(1).or_else(|| captures.get(2)).map(|m| m.as_str().to_string());
                let name = captures.get(3).unwrap().as_str().to_string();
                
                let service = EnhancedEmberService {
                    name,
                    service_name,
                };
                
                component.services.push(service);
            }
        } else {
            // Extract services from classic Ember component
            lazy_static! {
                static ref CLASSIC_SERVICE_REGEX: Regex =
                    Regex::new(r#"([a-zA-Z0-9_]+):\s*service\s*\(\s*(?:'([^']+)'|"([^"]+)")?\s*\)"#).unwrap();
            }
            
            for captures in CLASSIC_SERVICE_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let service_name = captures.get(2).or_else(|| captures.get(3)).map(|m| m.as_str().to_string());
                
                let service = EnhancedEmberService {
                    name,
                    service_name,
                };
                
                component.services.push(service);
            }
        }
    }
    
    fn extract_computed(&self, content: &str, component: &mut EnhancedEmberComponent) {
        if component.is_glimmer {
            // Glimmer components don't have computed properties in the same way
            // They use getters instead
            lazy_static! {
                static ref GETTER_REGEX: Regex =
                    Regex::new(r#"get\s+([a-zA-Z0-9_]+)\s*\(\)\s*\{([^}]*)\}"#).unwrap();
            }
            
            for captures in GETTER_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let body = captures.get(2).unwrap().as_str().to_string();
                
                // Try to extract dependencies from the body
                let mut dependencies = Vec::new();
                if body.contains("this.args.") {
                    lazy_static! {
                        static ref ARGS_REGEX: Regex =
                            Regex::new(r#"this\.args\.([a-zA-Z0-9_]+)"#).unwrap();
                    }
                    
                    for arg_match in ARGS_REGEX.captures_iter(&body) {
                        let arg = arg_match.get(1).unwrap().as_str().to_string();
                        if !dependencies.contains(&arg) {
                            dependencies.push(arg);
                        }
                    }
                }
                
                let computed = EnhancedEmberComputed {
                    name,
                    dependencies,
                    body,
                };
                
                component.computed.push(computed);
            }
        } else {
            // Extract computed properties from classic Ember component
            lazy_static! {
                static ref COMPUTED_REGEX: Regex =
                    Regex::new(r#"([a-zA-Z0-9_]+):\s*(?:Ember\.)?computed\s*\(\s*(?:'([^']*)'|"([^"]*)")(?:\s*,\s*(?:'([^']*)'|"([^"]*)"))*\s*,\s*function\s*\(\)\s*\{([^}]*)\}\s*\)"#).unwrap();
            }
            
            for captures in COMPUTED_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let body = captures.get(6).unwrap().as_str().to_string();
                
                // Extract dependencies
                let mut dependencies = Vec::new();
                if let Some(dep) = captures.get(2).or_else(|| captures.get(3)) {
                    dependencies.push(dep.as_str().to_string());
                }
                
                if let Some(dep) = captures.get(4).or_else(|| captures.get(5)) {
                    dependencies.push(dep.as_str().to_string());
                }
                
                let computed = EnhancedEmberComputed {
                    name,
                    dependencies,
                    body,
                };
                
                component.computed.push(computed);
            }
        }
    }
    
    fn extract_observers(&self, content: &str, component: &mut EnhancedEmberComponent) {
        if !component.is_glimmer {
            // Only classic Ember components have observers
            lazy_static! {
                static ref OBSERVER_REGEX: Regex =
                    Regex::new(r#"([a-zA-Z0-9_]+):\s*(?:Ember\.)?observer\s*\(\s*(?:'([^']*)'|"([^"]*)")(?:\s*,\s*(?:'([^']*)'|"([^"]*)"))*\s*,\s*function\s*\(\)\s*\{([^}]*)\}\s*\)"#).unwrap();
            }
            
            for captures in OBSERVER_REGEX.captures_iter(content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let observed_property = captures.get(2).or_else(|| captures.get(3)).unwrap().as_str().to_string();
                let body = captures.get(6).unwrap().as_str().to_string();
                
                let observer = EnhancedEmberObserver {
                    name,
                    observed_property,
                    body,
                };
                
                component.observers.push(observer);
            }
        }
    }
    
    fn extract_template(&self, content: &str, component: &mut EnhancedEmberComponent) {
        // Look for template string in the file
        lazy_static! {
            static ref TEMPLATE_REGEX: Regex =
                Regex::new(r#"(?:layout|template)(?::\s*hbs)?(?:\s*`|=\s*hbs`)([^`]*)`"#).unwrap();
        }
        
        if let Some(captures) = TEMPLATE_REGEX.captures(content) {
            component.template = Some(captures.get(1).unwrap().as_str().to_string());
        }
    }
}
