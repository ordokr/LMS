use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmberProperty {
    pub name: String,
    pub property_type: String,
    pub default_value: Option<String>,
    pub is_tracked: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmberAction {
    pub name: String,
    pub parameters: Vec<String>,
    pub code_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmberComputed {
    pub name: String,
    pub dependencies: Vec<String>,
    pub code_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmberService {
    pub name: String,
    pub service_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmberLifecycleHook {
    pub name: String, // init, didInsertElement, willDestroy, etc.
    pub code_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmberComponent {
    pub name: String,
    pub file_path: String,
    pub template_path: Option<String>,
    pub component_type: String, // "classic", "glimmer", "modifier"
    pub properties: Vec<EmberProperty>,
    pub actions: Vec<EmberAction>,
    pub computed_properties: Vec<EmberComputed>,
    pub services: Vec<EmberService>,
    pub lifecycle_hooks: Vec<EmberLifecycleHook>,
    pub child_components: Vec<String>,
    pub imports: HashMap<String, String>,
    pub template_content: Option<String>,
    pub raw_content: String,
}

#[derive(Debug, Default)]
pub struct EnhancedEmberAnalyzer {
    pub components: HashMap<String, EmberComponent>,
}

impl EnhancedEmberAnalyzer {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Ember components in directory: {:?}", directory);
        
        // First, find all component JS files
        let mut component_files = Vec::new();
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Check for Ember component files
                    if (extension == "js" || extension == "ts") && 
                       (path.to_string_lossy().contains("/components/") || 
                        path.to_string_lossy().contains("/modifiers/")) {
                        component_files.push(path.to_path_buf());
                    }
                }
            }
        }
        
        // Then, analyze each component file
        for file_path in component_files {
            self.analyze_component_file(&file_path)?;
        }
        
        Ok(())
    }

    pub fn analyze_component_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Ember component file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        
        // Check if file contains Ember component
        if !self.is_ember_component(&content) {
            return Ok(());
        }
        
        // Extract component name from file path
        let component_name = self.extract_component_name(file_path);
        
        let mut component = EmberComponent {
            name: component_name,
            file_path: file_path.to_string_lossy().to_string(),
            raw_content: content.clone(),
            ..Default::default()
        };
        
        // Determine component type
        component.component_type = self.determine_component_type(&content);
        
        // Find template path
        component.template_path = self.find_template_path(file_path);
        
        // Load template content if available
        if let Some(template_path) = &component.template_path {
            if let Ok(template_content) = fs::read_to_string(template_path) {
                component.template_content = Some(template_content);
            }
        }
        
        // Extract imports
        self.extract_imports(&content, &mut component);
        
        // Extract properties
        self.extract_properties(&content, &mut component);
        
        // Extract actions
        self.extract_actions(&content, &mut component);
        
        // Extract computed properties
        self.extract_computed_properties(&content, &mut component);
        
        // Extract services
        self.extract_services(&content, &mut component);
        
        // Extract lifecycle hooks
        self.extract_lifecycle_hooks(&content, &mut component);
        
        // Extract child components from template
        if let Some(template_content) = &component.template_content {
            self.extract_child_components(template_content, &mut component);
        }
        
        // Add component to the collection
        self.components.insert(component.file_path.clone(), component);
        
        Ok(())
    }

    fn is_ember_component(&self, content: &str) -> bool {
        // Check for Ember imports
        if content.contains("import Component from '@ember/component'") || 
           content.contains("import Component from '@glimmer/component'") ||
           content.contains("import Modifier from 'ember-modifier'") {
            return true;
        }
        
        // Check for Ember component definition
        if content.contains("export default Component.extend(") || 
           content.contains("export default class") && content.contains("extends Component") {
            return true;
        }
        
        false
    }

    fn extract_component_name(&self, file_path: &Path) -> String {
        // Extract component name from file path
        // Example: app/components/ui/button.js -> ui/button
        let path_str = file_path.to_string_lossy();
        
        if let Some(components_pos) = path_str.find("/components/") {
            let component_path = &path_str[components_pos + 12..];
            if let Some(dot_pos) = component_path.rfind('.') {
                return component_path[..dot_pos].to_string();
            }
            return component_path.to_string();
        } else if let Some(modifiers_pos) = path_str.find("/modifiers/") {
            let modifier_path = &path_str[modifiers_pos + 11..];
            if let Some(dot_pos) = modifier_path.rfind('.') {
                return modifier_path[..dot_pos].to_string();
            }
            return modifier_path.to_string();
        }
        
        // Fall back to file name
        if let Some(file_name) = file_path.file_stem() {
            return file_name.to_string_lossy().to_string();
        }
        
        "UnknownComponent".to_string()
    }

    fn determine_component_type(&self, content: &str) -> String {
        if content.contains("import Component from '@glimmer/component'") {
            return "glimmer".to_string();
        }
        
        if content.contains("import Modifier from 'ember-modifier'") {
            return "modifier".to_string();
        }
        
        "classic".to_string()
    }

    fn find_template_path(&self, js_file_path: &Path) -> Option<String> {
        // For classic components, look for a template with the same name
        // Example: app/components/ui/button.js -> app/templates/components/ui/button.hbs
        let path_str = js_file_path.to_string_lossy();
        
        if let Some(components_pos) = path_str.find("/components/") {
            let component_path = &path_str[components_pos + 12..];
            if let Some(dot_pos) = component_path.rfind('.') {
                let component_name = &component_path[..dot_pos];
                
                // Try to find template in app/templates/components
                let template_path = path_str[..components_pos].to_string() + "/templates/components/" + component_name + ".hbs";
                let template_path_buf = PathBuf::from(&template_path);
                if template_path_buf.exists() {
                    return Some(template_path);
                }
                
                // Try to find co-located template (for Ember Octane)
                let colocated_template_path = path_str[..path_str.rfind('.').unwrap()].to_string() + ".hbs";
                let colocated_template_path_buf = PathBuf::from(&colocated_template_path);
                if colocated_template_path_buf.exists() {
                    return Some(colocated_template_path);
                }
            }
        }
        
        None
    }

    fn extract_imports(&self, content: &str, component: &mut EmberComponent) {
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

    fn extract_properties(&self, content: &str, component: &mut EmberComponent) {
        if component.component_type == "classic" {
            // Extract properties from classic components
            lazy_static! {
                static ref CLASSIC_PROPS_REGEX: Regex = 
                    Regex::new(r"([a-zA-Z0-9_]+):\s*([^,]+)").unwrap();
            }
            
            for captures in CLASSIC_PROPS_REGEX.captures_iter(content) {
                let prop_name = captures.get(1).unwrap().as_str().to_string();
                let default_value = Some(captures.get(2).unwrap().as_str().trim().to_string());
                
                // Skip actions, computed properties, and lifecycle hooks
                if prop_name == "actions" || 
                   prop_name == "init" || 
                   prop_name == "didInsertElement" || 
                   prop_name == "willDestroy" {
                    continue;
                }
                
                // Try to determine property type from default value
                let property_type = if let Some(ref value) = default_value {
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
                    } else if value.contains("computed(") {
                        "computed".to_string()
                    } else {
                        "any".to_string()
                    }
                } else {
                    "any".to_string()
                };
                
                // Skip computed properties
                if property_type == "computed" {
                    continue;
                }
                
                component.properties.push(EmberProperty {
                    name: prop_name,
                    property_type,
                    default_value,
                    is_tracked: false,
                    description: None,
                });
            }
        } else {
            // Extract properties from Glimmer components
            lazy_static! {
                static ref GLIMMER_PROPS_REGEX: Regex = 
                    Regex::new(r"@tracked\s+([a-zA-Z0-9_]+)(?:\s*=\s*([^;]+))?").unwrap();
            }
            
            for captures in GLIMMER_PROPS_REGEX.captures_iter(content) {
                let prop_name = captures.get(1).unwrap().as_str().to_string();
                let default_value = captures.get(2).map(|m| m.as_str().trim().to_string());
                
                // Try to determine property type from default value
                let property_type = if let Some(ref value) = default_value {
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
                
                component.properties.push(EmberProperty {
                    name: prop_name,
                    property_type,
                    default_value,
                    is_tracked: true,
                    description: None,
                });
            }
            
            // Also look for arguments (args) in Glimmer components
            lazy_static! {
                static ref GLIMMER_ARGS_REGEX: Regex = 
                    Regex::new(r"get\s+([a-zA-Z0-9_]+)\(\)\s*\{\s*return\s+this\.args\.([a-zA-Z0-9_]+)").unwrap();
            }
            
            for captures in GLIMMER_ARGS_REGEX.captures_iter(content) {
                let prop_name = captures.get(1).unwrap().as_str().to_string();
                let arg_name = captures.get(2).unwrap().as_str().to_string();
                
                component.properties.push(EmberProperty {
                    name: prop_name,
                    property_type: "argument".to_string(),
                    default_value: None,
                    is_tracked: false,
                    description: Some(format!("Getter for @{} argument", arg_name)),
                });
            }
        }
    }

    fn extract_actions(&self, content: &str, component: &mut EmberComponent) {
        if component.component_type == "classic" {
            // Extract actions from classic components
            lazy_static! {
                static ref CLASSIC_ACTIONS_REGEX: Regex = 
                    Regex::new(r"actions:\s*\{([\s\S]*?)\}").unwrap();
            }
            
            if let Some(captures) = CLASSIC_ACTIONS_REGEX.captures(content) {
                let actions_block = captures.get(1).unwrap().as_str();
                
                lazy_static! {
                    static ref ACTION_REGEX: Regex = 
                        Regex::new(r"([a-zA-Z0-9_]+)\s*\(\s*(.*?)\s*\)\s*\{([\s\S]*?)(?:\},|\}$)").unwrap();
                }
                
                for captures in ACTION_REGEX.captures_iter(actions_block) {
                    let action_name = captures.get(1).unwrap().as_str().to_string();
                    let params_str = captures.get(2).map_or("", |m| m.as_str());
                    let action_code = captures.get(3).unwrap().as_str().trim().to_string();
                    
                    let parameters = if params_str.is_empty() {
                        vec![]
                    } else {
                        params_str.split(',')
                            .map(|s| s.trim().to_string())
                            .collect()
                    };
                    
                    component.actions.push(EmberAction {
                        name: action_name,
                        parameters,
                        code_snippet: action_code,
                    });
                }
            }
        } else {
            // Extract actions from Glimmer components
            lazy_static! {
                static ref GLIMMER_ACTIONS_REGEX: Regex = 
                    Regex::new(r"@action\s+([a-zA-Z0-9_]+)\s*\(\s*(.*?)\s*\)\s*\{([\s\S]*?)(?:\}\n|\}$)").unwrap();
            }
            
            for captures in GLIMMER_ACTIONS_REGEX.captures_iter(content) {
                let action_name = captures.get(1).unwrap().as_str().to_string();
                let params_str = captures.get(2).map_or("", |m| m.as_str());
                let action_code = captures.get(3).unwrap().as_str().trim().to_string();
                
                let parameters = if params_str.is_empty() {
                    vec![]
                } else {
                    params_str.split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };
                
                component.actions.push(EmberAction {
                    name: action_name,
                    parameters,
                    code_snippet: action_code,
                });
            }
        }
    }

    fn extract_computed_properties(&self, content: &str, component: &mut EmberComponent) {
        if component.component_type == "classic" {
            // Extract computed properties from classic components
            lazy_static! {
                static ref CLASSIC_COMPUTED_REGEX: Regex = 
                    Regex::new(r"([a-zA-Z0-9_]+):\s*computed\s*\(\s*'(.*?)'\s*,\s*function\s*\(\)\s*\{([\s\S]*?)(?:\},|\}$)").unwrap();
            }
            
            for captures in CLASSIC_COMPUTED_REGEX.captures_iter(content) {
                let prop_name = captures.get(1).unwrap().as_str().to_string();
                let deps_str = captures.get(2).unwrap().as_str();
                let computed_code = captures.get(3).unwrap().as_str().trim().to_string();
                
                let dependencies = deps_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                
                component.computed_properties.push(EmberComputed {
                    name: prop_name,
                    dependencies,
                    code_snippet: computed_code,
                });
            }
        } else {
            // Extract computed properties from Glimmer components
            lazy_static! {
                static ref GLIMMER_COMPUTED_REGEX: Regex = 
                    Regex::new(r"get\s+([a-zA-Z0-9_]+)\s*\(\)\s*\{([\s\S]*?)(?:\}\n|\}$)").unwrap();
            }
            
            for captures in GLIMMER_COMPUTED_REGEX.captures_iter(content) {
                let prop_name = captures.get(1).unwrap().as_str().to_string();
                let computed_code = captures.get(2).unwrap().as_str().trim().to_string();
                
                // Try to extract dependencies by looking for this.X references
                lazy_static! {
                    static ref DEPS_REGEX: Regex = 
                        Regex::new(r"this\.([a-zA-Z0-9_]+)").unwrap();
                }
                
                let mut dependencies = Vec::new();
                for dep_captures in DEPS_REGEX.captures_iter(&computed_code) {
                    let dep_name = dep_captures.get(1).unwrap().as_str().to_string();
                    if !dependencies.contains(&dep_name) && dep_name != prop_name {
                        dependencies.push(dep_name);
                    }
                }
                
                component.computed_properties.push(EmberComputed {
                    name: prop_name,
                    dependencies,
                    code_snippet: computed_code,
                });
            }
        }
    }

    fn extract_services(&self, content: &str, component: &mut EmberComponent) {
        // Extract services from both classic and Glimmer components
        lazy_static! {
            static ref CLASSIC_SERVICE_REGEX: Regex = 
                Regex::new(r"([a-zA-Z0-9_]+):\s*service\s*\(\s*(?:'([^']+)'|\"([^\"]+)\")?\s*\)").unwrap();
        }
        
        for captures in CLASSIC_SERVICE_REGEX.captures_iter(content) {
            let service_name = captures.get(1).unwrap().as_str().to_string();
            let service_type = captures.get(2)
                .or_else(|| captures.get(3))
                .map_or(service_name.clone(), |m| m.as_str().to_string());
            
            component.services.push(EmberService {
                name: service_name,
                service_type,
            });
        }
        
        // For Glimmer components, also look for @service decorators
        lazy_static! {
            static ref GLIMMER_SERVICE_REGEX: Regex = 
                Regex::new(r"@service(?:\s*\(\s*(?:'([^']+)'|\"([^\"]+)\")\s*\))?\s+([a-zA-Z0-9_]+)").unwrap();
        }
        
        for captures in GLIMMER_SERVICE_REGEX.captures_iter(content) {
            let service_name = captures.get(3).unwrap().as_str().to_string();
            let service_type = captures.get(1)
                .or_else(|| captures.get(2))
                .map_or(service_name.clone(), |m| m.as_str().to_string());
            
            component.services.push(EmberService {
                name: service_name,
                service_type,
            });
        }
    }

    fn extract_lifecycle_hooks(&self, content: &str, component: &mut EmberComponent) {
        if component.component_type == "classic" {
            // Extract lifecycle hooks from classic components
            let hooks = vec!["init", "didReceiveAttrs", "didInsertElement", "didRender", "didUpdateAttrs", "willUpdate", "willRender", "willDestroyElement", "willDestroy"];
            
            for hook in hooks {
                lazy_static! {
                    static ref HOOK_REGEX: Regex = 
                        Regex::new(&format!(r"{}:\s*function\s*\(\)\s*\{{([\s\S]*?)(?:\}},|\}}$)", hook)).unwrap();
                }
                
                if let Some(captures) = HOOK_REGEX.captures(content) {
                    let hook_code = captures.get(1).unwrap().as_str().trim().to_string();
                    
                    component.lifecycle_hooks.push(EmberLifecycleHook {
                        name: hook.to_string(),
                        code_snippet: hook_code,
                    });
                }
            }
        } else {
            // Extract lifecycle hooks from Glimmer components
            let hooks = vec!["constructor", "willDestroy"];
            
            for hook in hooks {
                lazy_static! {
                    static ref HOOK_REGEX: Regex = 
                        Regex::new(&format!(r"{}(?:\s*\([^)]*\))?\s*\{{([\s\S]*?)(?:\}}\n|\}}$)", hook)).unwrap();
                }
                
                if let Some(captures) = HOOK_REGEX.captures(content) {
                    let hook_code = captures.get(1).unwrap().as_str().trim().to_string();
                    
                    component.lifecycle_hooks.push(EmberLifecycleHook {
                        name: hook.to_string(),
                        code_snippet: hook_code,
                    });
                }
            }
            
            // Also look for modifier lifecycle hooks
            if component.component_type == "modifier" {
                let modifier_hooks = vec!["didInstall", "didReceiveArguments", "didUpdateArguments", "willRemove"];
                
                for hook in modifier_hooks {
                    lazy_static! {
                        static ref MODIFIER_HOOK_REGEX: Regex = 
                            Regex::new(&format!(r"{}(?:\s*\([^)]*\))?\s*\{{([\s\S]*?)(?:\}}\n|\}}$)", hook)).unwrap();
                    }
                    
                    if let Some(captures) = MODIFIER_HOOK_REGEX.captures(content) {
                        let hook_code = captures.get(1).unwrap().as_str().trim().to_string();
                        
                        component.lifecycle_hooks.push(EmberLifecycleHook {
                            name: hook.to_string(),
                            code_snippet: hook_code,
                        });
                    }
                }
            }
        }
    }

    fn extract_child_components(&self, template_content: &str, component: &mut EmberComponent) {
        // Extract child components from template
        // Look for component invocations like {{component-name}} or <ComponentName>
        lazy_static! {
            static ref CURLY_COMPONENT_REGEX: Regex = 
                Regex::new(r"\{\{([a-z0-9/-]+)(?:\s|/|\}\})").unwrap();
        }
        
        let mut child_components = std::collections::HashSet::new();
        
        for captures in CURLY_COMPONENT_REGEX.captures_iter(template_content) {
            let component_name = captures.get(1).unwrap().as_str().to_string();
            // Skip helpers and built-ins
            if !component_name.contains("/") && 
               !component_name.contains("-") && 
               !vec!["if", "each", "unless", "yield", "outlet", "link-to", "input", "textarea", "log", "debugger"].contains(&component_name.as_str()) {
                continue;
            }
            child_components.insert(component_name);
        }
        
        // Look for angle bracket components
        lazy_static! {
            static ref ANGLE_COMPONENT_REGEX: Regex = 
                Regex::new(r"<([A-Z][A-Za-z0-9]+|[a-z][a-z0-9]*-[a-z0-9-]+)(?:\s|/|>)").unwrap();
        }
        
        for captures in ANGLE_COMPONENT_REGEX.captures_iter(template_content) {
            let component_name = captures.get(1).unwrap().as_str().to_string();
            child_components.insert(component_name);
        }
        
        component.child_components = child_components.into_iter().collect();
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.components)
    }
}
