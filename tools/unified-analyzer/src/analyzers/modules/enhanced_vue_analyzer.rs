use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VueProp {
    pub name: String,
    pub prop_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub validator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VueData {
    pub name: String,
    pub data_type: String,
    pub initial_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VueComputed {
    pub name: String,
    pub dependencies: Vec<String>,
    pub getter: String,
    pub setter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VueMethod {
    pub name: String,
    pub parameters: Vec<String>,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VueWatch {
    pub target: String,
    pub handler: String,
    pub immediate: bool,
    pub deep: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VueLifecycleHook {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VueComponent {
    pub name: String,
    pub file_path: String,
    pub component_type: String, // "options_api", "composition_api", "class_based"
    pub template: Option<String>,
    pub props: Vec<VueProp>,
    pub data: Vec<VueData>,
    pub computed: Vec<VueComputed>,
    pub methods: Vec<VueMethod>,
    pub watches: Vec<VueWatch>,
    pub lifecycle_hooks: Vec<VueLifecycleHook>,
    pub components: HashMap<String, String>,
    pub directives: Vec<String>,
    pub mixins: Vec<String>,
    pub imports: HashMap<String, String>,
    pub raw_content: String,
}

#[derive(Debug, Default)]
pub struct EnhancedVueAnalyzer {
    pub components: HashMap<String, VueComponent>,
}

impl EnhancedVueAnalyzer {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Vue components in directory: {:?}", directory);

        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Check for Vue component files
                    if extension == "vue" {
                        self.analyze_vue_file(path)?;
                    } else if (extension == "js" || extension == "ts") &&
                              !path.to_string_lossy().contains(".test.") {
                        // Check if it's a JS/TS file that might contain a Vue component
                        self.analyze_js_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn analyze_vue_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Vue file: {:?}", file_path);

        let content = fs::read_to_string(file_path)?;

        // Extract component name from file name
        let component_name = self.extract_component_name(file_path);

        let mut component = VueComponent {
            name: component_name,
            file_path: file_path.to_string_lossy().to_string(),
            raw_content: content.clone(),
            ..Default::default()
        };

        // Extract template
        self.extract_template(&content, &mut component);

        // Extract script content
        if let Some(script_content) = self.extract_script(&content) {
            // Determine component type
            component.component_type = self.determine_component_type(&script_content);

            // Extract imports
            self.extract_imports(&script_content, &mut component);

            // Extract component definition based on type
            match component.component_type.as_str() {
                "options_api" => self.extract_options_api(&script_content, &mut component),
                "composition_api" => self.extract_composition_api(&script_content, &mut component),
                "class_based" => self.extract_class_based(&script_content, &mut component),
                _ => {}
            }
        }

        // Add component to the collection
        self.components.insert(component.file_path.clone(), component);

        Ok(())
    }

    fn analyze_js_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing JS/TS file for Vue components: {:?}", file_path);

        let content = fs::read_to_string(file_path)?;

        // Check if file contains Vue component
        if !self.is_vue_component(&content) {
            return Ok(());
        }

        // Extract component name from file name or export
        let component_name = self.extract_component_name_from_js(file_path, &content);

        let mut component = VueComponent {
            name: component_name,
            file_path: file_path.to_string_lossy().to_string(),
            raw_content: content.clone(),
            ..Default::default()
        };

        // Determine component type
        component.component_type = self.determine_component_type(&content);

        // Extract imports
        self.extract_imports(&content, &mut component);

        // Extract component definition based on type
        match component.component_type.as_str() {
            "options_api" => self.extract_options_api(&content, &mut component),
            "composition_api" => self.extract_composition_api(&content, &mut component),
            "class_based" => self.extract_class_based(&content, &mut component),
            _ => {}
        }

        // Add component to the collection
        self.components.insert(component.file_path.clone(), component);

        Ok(())
    }

    fn extract_component_name(&self, file_path: &Path) -> String {
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

            // If already PascalCase, return as is
            if let Some(first_char) = file_name_str.chars().next() {
                if first_char.is_uppercase() {
                    return file_name_str.to_string();
                }
            }

            // Convert from camelCase to PascalCase
            if let Some(first_char) = file_name_str.chars().next() {
                return first_char.to_uppercase().to_string() + &file_name_str[1..];
            }

            return file_name_str.to_string();
        }

        "UnknownComponent".to_string()
    }

    fn extract_component_name_from_js(&self, file_path: &Path, content: &str) -> String {
        // Try to extract from export statement
        lazy_static! {
            static ref EXPORT_REGEX: Regex =
                Regex::new(r"export\s+(?:default\s+)?(?:const\s+)?([A-Za-z0-9_]+)").unwrap();
        }

        if let Some(captures) = EXPORT_REGEX.captures(content) {
            return captures.get(1).unwrap().as_str().to_string();
        }

        // Fall back to file name
        self.extract_component_name(file_path)
    }

    fn is_vue_component(&self, content: &str) -> bool {
        // Check for Vue import
        if content.contains("import Vue from 'vue'") ||
           content.contains("import { defineComponent } from 'vue'") {
            return true;
        }

        // Check for Vue.component or Vue.extend
        if content.contains("Vue.component(") || content.contains("Vue.extend(") {
            return true;
        }

        // Check for export default { ... } with Vue component structure
        if content.contains("export default {") &&
           (content.contains("data() {") ||
            content.contains("methods: {") ||
            content.contains("computed: {") ||
            content.contains("components: {")) {
            return true;
        }

        // Check for defineComponent
        if content.contains("defineComponent(") {
            return true;
        }

        // Check for class-based components
        if content.contains("@Component") && content.contains("class ") && content.contains(" extends Vue") {
            return true;
        }

        false
    }

    fn extract_template(&self, content: &str, component: &mut VueComponent) {
        // Extract template from .vue file
        lazy_static! {
            static ref TEMPLATE_REGEX: Regex =
                Regex::new(r"<template>([\s\S]*?)</template>").unwrap();
        }

        if let Some(captures) = TEMPLATE_REGEX.captures(content) {
            component.template = Some(captures.get(1).unwrap().as_str().trim().to_string());
        }
    }

    fn extract_script(&self, content: &str) -> Option<String> {
        // Extract script from .vue file
        lazy_static! {
            static ref SCRIPT_REGEX: Regex =
                Regex::new(r"<script>([\s\S]*?)</script>").unwrap();
        }

        if let Some(captures) = SCRIPT_REGEX.captures(content) {
            return Some(captures.get(1).unwrap().as_str().trim().to_string());
        }

        // Also check for script with lang attribute
        lazy_static! {
            static ref SCRIPT_LANG_REGEX: Regex =
                Regex::new(r"<script\s+lang=['"](?:ts|js|typescript|javascript)['"]>([\s\S]*?)</script>").unwrap();
        }

        if let Some(captures) = SCRIPT_LANG_REGEX.captures(content) {
            return Some(captures.get(1).unwrap().as_str().trim().to_string());
        }

        None
    }

    fn determine_component_type(&self, content: &str) -> String {
        // Check for composition API
        if content.contains("defineComponent(") ||
           content.contains("setup(") ||
           content.contains("ref(") ||
           content.contains("reactive(") {
            return "composition_api".to_string();
        }

        // Check for class-based components
        if content.contains("@Component") && content.contains("class ") && content.contains(" extends Vue") {
            return "class_based".to_string();
        }

        // Default to options API
        "options_api".to_string()
    }

    fn extract_imports(&self, content: &str, component: &mut VueComponent) {
        lazy_static! {
            static ref IMPORT_REGEX: Regex =
                Regex::new(r"import\s+(?:(?:(\{[^}]+\})|([A-Za-z0-9_]+))\s+from\s+)?['"]([^'"]+)['"];").unwrap();
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

    fn extract_options_api(&self, content: &str, component: &mut VueComponent) {
        // Extract component definition
        lazy_static! {
            static ref COMPONENT_DEF_REGEX: Regex =
                Regex::new(r"(?:export\s+default|Vue\.component\([^,]+,|Vue\.extend\()\s*\{([\s\S]*?)\}\s*(?:\)|;|$)").unwrap();
        }

        let component_def = if let Some(captures) = COMPONENT_DEF_REGEX.captures(content) {
            captures.get(1).unwrap().as_str()
        } else {
            return;
        };

        // Extract props
        self.extract_options_api_props(component_def, component);

        // Extract data
        self.extract_options_api_data(component_def, component);

        // Extract computed properties
        self.extract_options_api_computed(component_def, component);

        // Extract methods
        self.extract_options_api_methods(component_def, component);

        // Extract watches
        self.extract_options_api_watches(component_def, component);

        // Extract lifecycle hooks
        self.extract_options_api_lifecycle(component_def, component);

        // Extract components
        self.extract_options_api_components(component_def, component);
    }

    fn extract_options_api_props(&self, component_def: &str, component: &mut VueComponent) {
        // Extract props as array
        lazy_static! {
            static ref PROPS_ARRAY_REGEX: Regex =
                Regex::new(r"props:\s*\[([^\]]+)\]").unwrap();
        }

        if let Some(captures) = PROPS_ARRAY_REGEX.captures(component_def) {
            let props_str = captures.get(1).unwrap().as_str();
            for prop in props_str.split(',') {
                let prop_name = prop.trim().trim_matches('"').trim_matches('\'').to_string();
                if !prop_name.is_empty() {
                    component.props.push(VueProp {
                        name: prop_name,
                        prop_type: "any".to_string(),
                        required: false,
                        ..Default::default()
                    });
                }
            }
            return;
        }

        // Extract props as object
        lazy_static! {
            static ref PROPS_OBJECT_REGEX: Regex =
                Regex::new(r"props:\s*\{([\s\S]*?)\}(?:,|$)").unwrap();
        }

        if let Some(captures) = PROPS_OBJECT_REGEX.captures(component_def) {
            let props_obj = captures.get(1).unwrap().as_str();

            // Extract individual props
            lazy_static! {
                static ref PROP_REGEX: Regex =
                    Regex::new(r"([a-zA-Z0-9_]+):\s*(?:\{([\s\S]*?)\}|([a-zA-Z0-9_]+))(?:,|$)").unwrap();
            }

            for captures in PROP_REGEX.captures_iter(props_obj) {
                let prop_name = captures.get(1).unwrap().as_str().to_string();

                let mut prop = VueProp {
                    name: prop_name,
                    ..Default::default()
                };

                if let Some(prop_obj) = captures.get(2) {
                    // Handle object form: { type: String, required: true, ... }
                    let prop_obj_str = prop_obj.as_str();

                    // Extract type
                    if let Some(type_match) = Regex::new(r"type:\s*([a-zA-Z0-9_]+)").unwrap().captures(prop_obj_str) {
                        prop.prop_type = type_match.get(1).unwrap().as_str().to_string();
                    }

                    // Extract required
                    if let Some(_) = Regex::new(r"required:\s*true").unwrap().captures(prop_obj_str) {
                        prop.required = true;
                    }

                    // Extract default value
                    if let Some(default_match) = Regex::new(r"default:\s*(?:function\s*\(\)\s*\{\s*return\s*([^;]+);|([^,]+))").unwrap().captures(prop_obj_str) {
                        prop.default_value = default_match.get(1)
                            .or_else(|| default_match.get(2))
                            .map(|m| m.as_str().trim().to_string());
                    }

                    // Extract validator
                    if let Some(validator_match) = Regex::new(r"validator:\s*function\s*\([^)]*\)\s*\{([\s\S]*?)\}").unwrap().captures(prop_obj_str) {
                        prop.validator = Some(validator_match.get(1).unwrap().as_str().trim().to_string());
                    }
                } else if let Some(prop_type) = captures.get(3) {
                    // Handle shorthand form: propName: String
                    prop.prop_type = prop_type.as_str().to_string();
                }

                component.props.push(prop);
            }
        }
    }

    fn extract_options_api_data(&self, component_def: &str, component: &mut VueComponent) {
        // Extract data as function
        lazy_static! {
            static ref DATA_FUNC_REGEX: Regex =
                Regex::new(r"data\s*\(\)\s*\{\s*return\s*\{([\s\S]*?)\};").unwrap();
        }

        if let Some(captures) = DATA_FUNC_REGEX.captures(component_def) {
            let data_obj = captures.get(1).unwrap().as_str();

            // Extract individual data properties
            lazy_static! {
                static ref DATA_PROP_REGEX: Regex =
                    Regex::new(r"([a-zA-Z0-9_]+):\s*([^,]+)(?:,|$)").unwrap();
            }

            for captures in DATA_PROP_REGEX.captures_iter(data_obj) {
                let data_name = captures.get(1).unwrap().as_str().to_string();
                let initial_value = captures.get(2).map(|m| m.as_str().trim().to_string());

                // Try to determine data type from initial value
                let data_type = if let Some(ref value) = initial_value {
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

                component.data.push(VueData {
                    name: data_name,
                    data_type,
                    initial_value,
                });
            }
        }
    }

    fn extract_options_api_computed(&self, component_def: &str, component: &mut VueComponent) {
        // Extract computed properties
        lazy_static! {
            static ref COMPUTED_REGEX: Regex =
                Regex::new(r"computed:\s*\{([\s\S]*?)\}(?:,|$)").unwrap();
        }

        if let Some(captures) = COMPUTED_REGEX.captures(component_def) {
            let computed_obj = captures.get(1).unwrap().as_str();

            // Extract individual computed properties
            lazy_static! {
                static ref COMPUTED_PROP_REGEX: Regex =
                    Regex::new(r"([a-zA-Z0-9_]+)\s*:\s*(?:function\s*\(\)\s*\{([\s\S]*?)\}|\{([\s\S]*?)\})(?:,|$)").unwrap();
            }

            for captures in COMPUTED_PROP_REGEX.captures_iter(computed_obj) {
                let computed_name = captures.get(1).unwrap().as_str().to_string();

                let mut computed = VueComputed {
                    name: computed_name,
                    ..Default::default()
                };

                if let Some(func_body) = captures.get(2) {
                    // Function form
                    computed.getter = func_body.as_str().trim().to_string();

                    // Try to extract dependencies by looking for this.X references
                    lazy_static! {
                        static ref DEPS_REGEX: Regex =
                            Regex::new(r"this\.([a-zA-Z0-9_]+)").unwrap();
                    }

                    for dep_captures in DEPS_REGEX.captures_iter(&computed.getter) {
                        let dep_name = dep_captures.get(1).unwrap().as_str().to_string();
                        if !computed.dependencies.contains(&dep_name) {
                            computed.dependencies.push(dep_name);
                        }
                    }
                } else if let Some(obj_body) = captures.get(3) {
                    // Object form with get/set
                    let obj_str = obj_body.as_str();

                    // Extract getter
                    if let Some(getter_match) = Regex::new(r"get\s*\(\)\s*\{([\s\S]*?)\}").unwrap().captures(obj_str) {
                        computed.getter = getter_match.get(1).unwrap().as_str().trim().to_string();

                        // Extract dependencies
                        lazy_static! {
                            static ref DEPS_REGEX: Regex =
                                Regex::new(r"this\.([a-zA-Z0-9_]+)").unwrap();
                        }

                        for dep_captures in DEPS_REGEX.captures_iter(&computed.getter) {
                            let dep_name = dep_captures.get(1).unwrap().as_str().to_string();
                            if !computed.dependencies.contains(&dep_name) {
                                computed.dependencies.push(dep_name);
                            }
                        }
                    }

                    // Extract setter
                    if let Some(setter_match) = Regex::new(r"set\s*\([^)]*\)\s*\{([\s\S]*?)\}").unwrap().captures(obj_str) {
                        computed.setter = Some(setter_match.get(1).unwrap().as_str().trim().to_string());
                    }
                }

                component.computed.push(computed);
            }
        }
    }

    fn extract_options_api_methods(&self, component_def: &str, component: &mut VueComponent) {
        // Extract methods
        lazy_static! {
            static ref METHODS_REGEX: Regex =
                Regex::new(r"methods:\s*\{([\s\S]*?)\}(?:,|$)").unwrap();
        }

        if let Some(captures) = METHODS_REGEX.captures(component_def) {
            let methods_obj = captures.get(1).unwrap().as_str();

            // Extract individual methods
            lazy_static! {
                static ref METHOD_REGEX: Regex =
                    Regex::new(r"([a-zA-Z0-9_]+)\s*\(([^)]*)\)\s*\{([\s\S]*?)\}(?:,|$)").unwrap();
            }

            for captures in METHOD_REGEX.captures_iter(methods_obj) {
                let method_name = captures.get(1).unwrap().as_str().to_string();
                let params_str = captures.get(2).map_or("", |m| m.as_str());
                let method_body = captures.get(3).unwrap().as_str().trim().to_string();

                let parameters = if params_str.is_empty() {
                    Vec::new()
                } else {
                    params_str.split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };

                component.methods.push(VueMethod {
                    name: method_name,
                    parameters,
                    code: method_body,
                });
            }
        }
    }

    fn extract_options_api_watches(&self, component_def: &str, component: &mut VueComponent) {
        // Extract watches
        lazy_static! {
            static ref WATCH_REGEX: Regex =
                Regex::new(r"watch:\s*\{([\s\S]*?)\}(?:,|$)").unwrap();
        }

        if let Some(captures) = WATCH_REGEX.captures(component_def) {
            let watch_obj = captures.get(1).unwrap().as_str();

            // Extract individual watches
            lazy_static! {
                static ref WATCH_PROP_REGEX: Regex =
                    Regex::new(r"'?([a-zA-Z0-9_.]+)'?\s*:\s*(?:function\s*\([^)]*\)\s*\{([\s\S]*?)\}|\{([\s\S]*?)\})(?:,|$)").unwrap();
            }

            for captures in WATCH_PROP_REGEX.captures_iter(watch_obj) {
                let target = captures.get(1).unwrap().as_str().to_string();

                let mut watch = VueWatch {
                    target,
                    immediate: false,
                    deep: false,
                    ..Default::default()
                };

                if let Some(func_body) = captures.get(2) {
                    // Function form
                    watch.handler = func_body.as_str().trim().to_string();
                } else if let Some(obj_body) = captures.get(3) {
                    // Object form with handler, immediate, deep
                    let obj_str = obj_body.as_str();

                    // Extract handler
                    if let Some(handler_match) = Regex::new(r"handler\s*\([^)]*\)\s*\{([\s\S]*?)\}").unwrap().captures(obj_str) {
                        watch.handler = handler_match.get(1).unwrap().as_str().trim().to_string();
                    } else if let Some(handler_match) = Regex::new(r"handler:\s*function\s*\([^)]*\)\s*\{([\s\S]*?)\}").unwrap().captures(obj_str) {
                        watch.handler = handler_match.get(1).unwrap().as_str().trim().to_string();
                    }

                    // Check for immediate
                    if obj_str.contains("immediate: true") {
                        watch.immediate = true;
                    }

                    // Check for deep
                    if obj_str.contains("deep: true") {
                        watch.deep = true;
                    }
                }

                component.watches.push(watch);
            }
        }
    }

    fn extract_options_api_lifecycle(&self, component_def: &str, component: &mut VueComponent) {
        // Extract lifecycle hooks
        let lifecycle_hooks = vec![
            "beforeCreate", "created", "beforeMount", "mounted",
            "beforeUpdate", "updated", "activated", "deactivated",
            "beforeDestroy", "destroyed", "errorCaptured"
        ];

        for hook in lifecycle_hooks {
            lazy_static! {
                static ref HOOK_REGEX: Regex =
                    Regex::new(&format!(r"{}\s*\([^)]*\)\s*\{{([\s\S]*?)\}}(?:,|$)", hook)).unwrap();
            }

            if let Some(captures) = HOOK_REGEX.captures(component_def) {
                let hook_body = captures.get(1).unwrap().as_str().trim().to_string();

                component.lifecycle_hooks.push(VueLifecycleHook {
                    name: hook.to_string(),
                    code: hook_body,
                });
            }
        }
    }

    fn extract_options_api_components(&self, component_def: &str, component: &mut VueComponent) {
        // Extract components
        lazy_static! {
            static ref COMPONENTS_REGEX: Regex =
                Regex::new(r"components:\s*\{([\s\S]*?)\}(?:,|$)").unwrap();
        }

        if let Some(captures) = COMPONENTS_REGEX.captures(component_def) {
            let components_obj = captures.get(1).unwrap().as_str();

            // Extract individual components
            lazy_static! {
                static ref COMPONENT_REGEX: Regex =
                    Regex::new(r"'?([a-zA-Z0-9_-]+)'?\s*:\s*([a-zA-Z0-9_]+)(?:,|$)").unwrap();
            }

            for captures in COMPONENT_REGEX.captures_iter(components_obj) {
                let component_name = captures.get(1).unwrap().as_str().to_string();
                let component_ref = captures.get(2).unwrap().as_str().to_string();

                component.components.insert(component_name, component_ref);
            }
        }
    }

    fn extract_composition_api(&self, content: &str, component: &mut VueComponent) {
        // Extract setup function
        lazy_static! {
            static ref SETUP_REGEX: Regex =
                Regex::new(r"setup\s*\(([^)]*)\)\s*\{([\s\S]*?)(?:return\s*\{([\s\S]*?)\}|\})").unwrap();
        }

        if let Some(captures) = SETUP_REGEX.captures(content) {
            let props_str = captures.get(1).map_or("", |m| m.as_str());
            let setup_body = captures.get(2).unwrap().as_str();
            let return_obj = captures.get(3).map_or("", |m| m.as_str());

            // Extract props from setup function parameters
            if !props_str.is_empty() {
                self.extract_composition_api_props(props_str, component);
            }

            // Extract refs and reactive state
            self.extract_composition_api_state(setup_body, component);

            // Extract computed properties
            self.extract_composition_api_computed(setup_body, component);

            // Extract watches
            self.extract_composition_api_watches(setup_body, component);

            // Extract methods
            self.extract_composition_api_methods(setup_body, return_obj, component);

            // Extract lifecycle hooks
            self.extract_composition_api_lifecycle(setup_body, component);
        } else {
            // Try to extract from defineComponent
            lazy_static! {
                static ref DEFINE_COMPONENT_REGEX: Regex =
                    Regex::new(r"defineComponent\s*\(\s*\{([\s\S]*?)\}\s*\)").unwrap();
            }

            if let Some(captures) = DEFINE_COMPONENT_REGEX.captures(content) {
                let component_def = captures.get(1).unwrap().as_str();

                // Extract props
                self.extract_options_api_props(component_def, component);

                // Extract setup function
                lazy_static! {
                    static ref SETUP_IN_DEFINE_REGEX: Regex =
                        Regex::new(r"setup\s*\(([^)]*)\)\s*\{([\s\S]*?)(?:return\s*\{([\s\S]*?)\}|\})").unwrap();
                }

                if let Some(captures) = SETUP_IN_DEFINE_REGEX.captures(component_def) {
                    let props_str = captures.get(1).map_or("", |m| m.as_str());
                    let setup_body = captures.get(2).unwrap().as_str();
                    let return_obj = captures.get(3).map_or("", |m| m.as_str());

                    // Extract refs and reactive state
                    self.extract_composition_api_state(setup_body, component);

                    // Extract computed properties
                    self.extract_composition_api_computed(setup_body, component);

                    // Extract watches
                    self.extract_composition_api_watches(setup_body, component);

                    // Extract methods
                    self.extract_composition_api_methods(setup_body, return_obj, component);

                    // Extract lifecycle hooks
                    self.extract_composition_api_lifecycle(setup_body, component);
                }

                // Extract components
                self.extract_options_api_components(component_def, component);
            }
        }
    }

    fn extract_composition_api_props(&self, props_str: &str, component: &mut VueComponent) {
        // Extract props from setup function parameters
        // Example: setup(props, context) or setup({ prop1, prop2 })

        if props_str.contains('{') && props_str.contains('}') {
            // Destructured props
            lazy_static! {
                static ref DESTRUCTURED_PROPS_REGEX: Regex =
                    Regex::new(r"\{([^}]+)\}").unwrap();
            }

            if let Some(captures) = DESTRUCTURED_PROPS_REGEX.captures(props_str) {
                let props_list = captures.get(1).unwrap().as_str();

                for prop in props_list.split(',') {
                    let prop_name = prop.trim().to_string();
                    if !prop_name.is_empty() {
                        component.props.push(VueProp {
                            name: prop_name,
                            prop_type: "any".to_string(),
                            required: true, // Destructured props are typically required
                            ..Default::default()
                        });
                    }
                }
            }
        } else if props_str.contains("props") {
            // Named props parameter
            // We can't extract individual props from this form directly,
            // but we can look for props usage in the setup body
        }
    }

    fn extract_composition_api_state(&self, setup_body: &str, component: &mut VueComponent) {
        // Extract ref declarations
        lazy_static! {
            static ref REF_REGEX: Regex =
                Regex::new(r"(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*ref\(([^)]*)\)").unwrap();
        }

        for captures in REF_REGEX.captures_iter(setup_body) {
            let data_name = captures.get(1).unwrap().as_str().to_string();
            let initial_value = captures.get(2).map(|m| m.as_str().trim().to_string());

            // Try to determine data type from initial value
            let data_type = if let Some(ref value) = initial_value {
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

            component.data.push(VueData {
                name: data_name,
                data_type,
                initial_value,
            });
        }

        // Extract reactive declarations
        lazy_static! {
            static ref REACTIVE_REGEX: Regex =
                Regex::new(r"(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*reactive\(([^)]*)\)").unwrap();
        }

        for captures in REACTIVE_REGEX.captures_iter(setup_body) {
            let data_name = captures.get(1).unwrap().as_str().to_string();
            let initial_value = captures.get(2).map(|m| m.as_str().trim().to_string());

            component.data.push(VueData {
                name: data_name,
                data_type: "object".to_string(), // reactive is typically used for objects
                initial_value,
            });
        }
    }

    fn extract_composition_api_computed(&self, setup_body: &str, component: &mut VueComponent) {
        // Extract computed declarations
        lazy_static! {
            static ref COMPUTED_REGEX: Regex =
                Regex::new(r"(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*computed\(\(\)\s*=>\s*\{([\s\S]*?)\}\)").unwrap();
        }

        for captures in COMPUTED_REGEX.captures_iter(setup_body) {
            let computed_name = captures.get(1).unwrap().as_str().to_string();
            let computed_body = captures.get(2).unwrap().as_str().trim().to_string();

            let mut computed = VueComputed {
                name: computed_name,
                getter: computed_body,
                ..Default::default()
            };

            // Try to extract dependencies by looking for state.X or X.value references
            lazy_static! {
                static ref DEPS_REGEX: Regex =
                    Regex::new(r"([a-zA-Z0-9_]+)(?:\.value|\.[a-zA-Z0-9_]+)").unwrap();
            }

            for dep_captures in DEPS_REGEX.captures_iter(&computed.getter) {
                let dep_name = dep_captures.get(1).unwrap().as_str().to_string();
                if !computed.dependencies.contains(&dep_name) {
                    computed.dependencies.push(dep_name);
                }
            }

            component.computed.push(computed);
        }

        // Also check for the getter/setter form
        lazy_static! {
            static ref COMPUTED_GET_SET_REGEX: Regex =
                Regex::new(r"(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*computed\(\{\s*get\(\)\s*\{([\s\S]*?)\}(?:,\s*set\(([^)]*)\)\s*\{([\s\S]*?)\})?\s*\}\)").unwrap();
        }

        for captures in COMPUTED_GET_SET_REGEX.captures_iter(setup_body) {
            let computed_name = captures.get(1).unwrap().as_str().to_string();
            let getter_body = captures.get(2).unwrap().as_str().trim().to_string();
            let setter_body = captures.get(4).map(|m| m.as_str().trim().to_string());

            let mut computed = VueComputed {
                name: computed_name,
                getter: getter_body,
                setter: setter_body,
                ..Default::default()
            };

            // Extract dependencies
            lazy_static! {
                static ref DEPS_REGEX: Regex =
                    Regex::new(r"([a-zA-Z0-9_]+)(?:\.value|\.[a-zA-Z0-9_]+)").unwrap();
            }

            for dep_captures in DEPS_REGEX.captures_iter(&computed.getter) {
                let dep_name = dep_captures.get(1).unwrap().as_str().to_string();
                if !computed.dependencies.contains(&dep_name) {
                    computed.dependencies.push(dep_name);
                }
            }

            component.computed.push(computed);
        }
    }

    fn extract_composition_api_watches(&self, setup_body: &str, component: &mut VueComponent) {
        // Extract watch declarations
        lazy_static! {
            static ref WATCH_REGEX: Regex =
                Regex::new(r"watch\(\s*(?:([a-zA-Z0-9_]+)(?:\.value)?|\(\)\s*=>\s*([a-zA-Z0-9_]+)(?:\.value)?)\s*,\s*(?:function\s*\([^)]*\)|\([^)]*\)\s*=>)\s*\{([\s\S]*?)\}(?:,\s*\{([\s\S]*?)\})?\s*\)").unwrap();
        }

        for captures in WATCH_REGEX.captures_iter(setup_body) {
            let target = captures.get(1)
                .or_else(|| captures.get(2))
                .map_or("unknown".to_string(), |m| m.as_str().to_string());

            let handler_body = captures.get(3).unwrap().as_str().trim().to_string();
            let options_str = captures.get(4).map_or("", |m| m.as_str());

            let mut watch = VueWatch {
                target,
                handler: handler_body,
                immediate: false,
                deep: false,
            };

            // Check for options
            if options_str.contains("immediate: true") {
                watch.immediate = true;
            }

            if options_str.contains("deep: true") {
                watch.deep = true;
            }

            component.watches.push(watch);
        }

        // Also check for watchEffect
        lazy_static! {
            static ref WATCH_EFFECT_REGEX: Regex =
                Regex::new(r"watchEffect\(\s*(?:function\s*\(\)|\(\)\s*=>)\s*\{([\s\S]*?)\}\s*\)").unwrap();
        }

        for captures in WATCH_EFFECT_REGEX.captures_iter(setup_body) {
            let handler_body = captures.get(1).unwrap().as_str().trim().to_string();

            let watch = VueWatch {
                target: "effect".to_string(), // Special target for watchEffect
                handler: handler_body,
                immediate: true, // watchEffect is always immediate
                deep: false,
            };

            component.watches.push(watch);
        }
    }

    fn extract_composition_api_methods(&self, setup_body: &str, return_obj: &str, component: &mut VueComponent) {
        // Extract function declarations
        lazy_static! {
            static ref FUNCTION_REGEX: Regex =
                Regex::new(r"(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*(?:function\s*\(([^)]*)\)|\(([^)]*)\)\s*=>)\s*\{([\s\S]*?)\}").unwrap();
        }

        for captures in FUNCTION_REGEX.captures_iter(setup_body) {
            let method_name = captures.get(1).unwrap().as_str().to_string();

            // Skip if this is a computed property or watch handler
            if setup_body.contains(&format!("const {} = computed", method_name)) ||
               setup_body.contains(&format!("const {} = watch", method_name)) {
                continue;
            }

            let params_str = captures.get(2)
                .or_else(|| captures.get(3))
                .map_or("", |m| m.as_str());

            let method_body = captures.get(4).unwrap().as_str().trim().to_string();

            let parameters = if params_str.is_empty() {
                Vec::new()
            } else {
                params_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };

            // Only include methods that are returned from setup
            if return_obj.contains(&method_name) {
                component.methods.push(VueMethod {
                    name: method_name,
                    parameters,
                    code: method_body,
                });
            }
        }
    }

    fn extract_composition_api_lifecycle(&self, setup_body: &str, component: &mut VueComponent) {
        // Map of Vue 3 lifecycle hooks to their Vue 2 equivalents
        let lifecycle_hooks = vec![
            ("onBeforeMount", "beforeMount"),
            ("onMounted", "mounted"),
            ("onBeforeUpdate", "beforeUpdate"),
            ("onUpdated", "updated"),
            ("onBeforeUnmount", "beforeDestroy"),
            ("onUnmounted", "destroyed"),
            ("onErrorCaptured", "errorCaptured"),
            ("onActivated", "activated"),
            ("onDeactivated", "deactivated"),
        ];

        for (hook_name, equivalent) in lifecycle_hooks {
            lazy_static! {
                static ref HOOK_REGEX: Regex =
                    Regex::new(&format!(r"{}\(\s*(?:function\s*\(\)|\(\)\s*=>)\s*\{{([\s\S]*?)\}}\s*\)", hook_name)).unwrap();
            }

            for captures in HOOK_REGEX.captures_iter(setup_body) {
                let hook_body = captures.get(1).unwrap().as_str().trim().to_string();

                component.lifecycle_hooks.push(VueLifecycleHook {
                    name: equivalent.to_string(),
                    code: hook_body,
                });
            }
        }
    }

    fn extract_class_based(&self, content: &str, component: &mut VueComponent) {
        // Extract class definition
        lazy_static! {
            static ref CLASS_REGEX: Regex =
                Regex::new(r"@Component\(\{([\s\S]*?)\}\)\s*export\s+(?:default\s+)?class\s+([A-Za-z0-9_]+)\s+extends\s+Vue\s*\{([\s\S]*?)\}").unwrap();
        }

        if let Some(captures) = CLASS_REGEX.captures(content) {
            let options_str = captures.get(1).unwrap().as_str();
            let class_name = captures.get(2).unwrap().as_str().to_string();
            let class_body = captures.get(3).unwrap().as_str();

            // Update component name if it was extracted from the class
            if component.name == "UnknownComponent" {
                component.name = class_name;
            }

            // Extract props from @Prop decorators
            self.extract_class_props(class_body, component);

            // Extract data properties
            self.extract_class_data(class_body, component);

            // Extract computed properties
            self.extract_class_computed(class_body, component);

            // Extract methods
            self.extract_class_methods(class_body, component);

            // Extract watches from @Watch decorators
            self.extract_class_watches(class_body, component);

            // Extract lifecycle hooks
            self.extract_class_lifecycle(class_body, component);

            // Extract components from options
            self.extract_class_components(options_str, component);
        }
    }

    fn extract_class_props(&self, class_body: &str, component: &mut VueComponent) {
        // Extract props from @Prop decorators
        lazy_static! {
            static ref PROP_REGEX: Regex =
                Regex::new(r"@Prop\(\{([\s\S]*?)\}\)\s*([a-zA-Z0-9_]+)!?:\s*([a-zA-Z0-9_<>|]+)").unwrap();
        }

        for captures in PROP_REGEX.captures_iter(class_body) {
            let options_str = captures.get(1).unwrap().as_str();
            let prop_name = captures.get(2).unwrap().as_str().to_string();
            let prop_type = captures.get(3).unwrap().as_str().to_string();

            let mut prop = VueProp {
                name: prop_name,
                prop_type,
                required: false,
                ..Default::default()
            };

            // Extract options
            if options_str.contains("required: true") {
                prop.required = true;
            }

            // Extract default value
            if let Some(default_match) = Regex::new(r"default:\s*(?:function\s*\(\)\s*\{\s*return\s*([^;]+);|([^,]+))").unwrap().captures(options_str) {
                prop.default_value = default_match.get(1)
                    .or_else(|| default_match.get(2))
                    .map(|m| m.as_str().trim().to_string());
            }

            component.props.push(prop);
        }

        // Also check for simpler @Prop syntax
        lazy_static! {
            static ref SIMPLE_PROP_REGEX: Regex =
                Regex::new(r"@Prop\(\)\s*([a-zA-Z0-9_]+)!?:\s*([a-zA-Z0-9_<>|]+)").unwrap();
        }

        for captures in SIMPLE_PROP_REGEX.captures_iter(class_body) {
            let prop_name = captures.get(1).unwrap().as_str().to_string();
            let prop_type = captures.get(2).unwrap().as_str().to_string();

            let prop = VueProp {
                name: prop_name,
                prop_type,
                required: false,
                ..Default::default()
            };

            component.props.push(prop);
        }
    }

    fn extract_class_data(&self, class_body: &str, component: &mut VueComponent) {
        // Extract class properties as data
        lazy_static! {
            static ref DATA_REGEX: Regex =
                Regex::new(r"([a-zA-Z0-9_]+)\s*(?::\s*([a-zA-Z0-9_<>|]+))?\s*=\s*([^;]+);").unwrap();
        }

        for captures in DATA_REGEX.captures_iter(class_body) {
            let data_name = captures.get(1).unwrap().as_str().to_string();

            // Skip if this is a prop
            if component.props.iter().any(|p| p.name == data_name) {
                continue;
            }

            // Skip if this is a computed property or method
            if class_body.contains(&format!("get {}()", data_name)) ||
               class_body.contains(&format!("{} (", data_name)) {
                continue;
            }

            let data_type = captures.get(2).map_or("any".to_string(), |m| m.as_str().to_string());
            let initial_value = captures.get(3).map(|m| m.as_str().trim().to_string());

            component.data.push(VueData {
                name: data_name,
                data_type,
                initial_value,
            });
        }
    }

    fn extract_class_computed(&self, class_body: &str, component: &mut VueComponent) {
        // Extract computed properties (getters)
        lazy_static! {
            static ref COMPUTED_REGEX: Regex =
                Regex::new(r"get\s+([a-zA-Z0-9_]+)\s*\(\)\s*\{([\s\S]*?)\}").unwrap();
        }

        for captures in COMPUTED_REGEX.captures_iter(class_body) {
            let computed_name = captures.get(1).unwrap().as_str().to_string();
            let computed_body = captures.get(2).unwrap().as_str().trim().to_string();

            let mut computed = VueComputed {
                name: computed_name.clone(),
                getter: computed_body,
                ..Default::default()
            };

            // Try to extract dependencies by looking for this.X references
            lazy_static! {
                static ref DEPS_REGEX: Regex =
                    Regex::new(r"this\.([a-zA-Z0-9_]+)").unwrap();
            }

            for dep_captures in DEPS_REGEX.captures_iter(&computed.getter) {
                let dep_name = dep_captures.get(1).unwrap().as_str().to_string();
                if !computed.dependencies.contains(&dep_name) && dep_name != computed_name {
                    computed.dependencies.push(dep_name);
                }
            }

            // Check for setter
            lazy_static! {
                static ref SETTER_REGEX: Regex =
                    Regex::new(&format!(r"set\s+{}\s*\([^)]*\)\s*\{{([\s\S]*?)\}}", computed_name)).unwrap();
            }

            if let Some(setter_captures) = SETTER_REGEX.captures(class_body) {
                let setter_body = setter_captures.get(1).unwrap().as_str().trim().to_string();
                computed.setter = Some(setter_body);
            }

            component.computed.push(computed);
        }
    }

    fn extract_class_methods(&self, class_body: &str, component: &mut VueComponent) {
        // Extract methods
        lazy_static! {
            static ref METHOD_REGEX: Regex =
                Regex::new(r"([a-zA-Z0-9_]+)\s*\(([^)]*)\)\s*\{([\s\S]*?)\}").unwrap();
        }

        for captures in METHOD_REGEX.captures_iter(class_body) {
            let method_name = captures.get(1).unwrap().as_str().to_string();

            // Skip getters and setters
            if method_name.starts_with("get ") || method_name.starts_with("set ") {
                continue;
            }

            // Skip lifecycle hooks
            if ["beforeCreate", "created", "beforeMount", "mounted",
                "beforeUpdate", "updated", "activated", "deactivated",
                "beforeDestroy", "destroyed", "errorCaptured"].contains(&method_name.as_str()) {
                continue;
            }

            let params_str = captures.get(2).map_or("", |m| m.as_str());
            let method_body = captures.get(3).unwrap().as_str().trim().to_string();

            let parameters = if params_str.is_empty() {
                Vec::new()
            } else {
                params_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };

            component.methods.push(VueMethod {
                name: method_name,
                parameters,
                code: method_body,
            });
        }
    }

    fn extract_class_watches(&self, class_body: &str, component: &mut VueComponent) {
        // Extract watches from @Watch decorators
        lazy_static! {
            static ref WATCH_REGEX: Regex =
                Regex::new(r"@Watch\(['"]([^'"]+)['"](?:,\s*\{([\s\S]*?)\})?\)\s*([a-zA-Z0-9_]+)\s*\([^)]*\)\s*\{([\s\S]*?)\}").unwrap();
        }

        for captures in WATCH_REGEX.captures_iter(class_body) {
            let target = captures.get(1).unwrap().as_str().to_string();
            let options_str = captures.get(2).map_or("", |m| m.as_str());
            let handler_name = captures.get(3).unwrap().as_str().to_string();
            let handler_body = captures.get(4).unwrap().as_str().trim().to_string();

            let mut watch = VueWatch {
                target,
                handler: handler_body,
                immediate: false,
                deep: false,
            };

            // Check for options
            if options_str.contains("immediate: true") {
                watch.immediate = true;
            }

            if options_str.contains("deep: true") {
                watch.deep = true;
            }

            component.watches.push(watch);
        }
    }

    fn extract_class_lifecycle(&self, class_body: &str, component: &mut VueComponent) {
        // Extract lifecycle hooks
        let lifecycle_hooks = vec![
            "beforeCreate", "created", "beforeMount", "mounted",
            "beforeUpdate", "updated", "activated", "deactivated",
            "beforeDestroy", "destroyed", "errorCaptured"
        ];

        for hook in lifecycle_hooks {
            lazy_static! {
                static ref HOOK_REGEX: Regex =
                    Regex::new(&format!(r"{}\s*\([^)]*\)\s*\{{([\s\S]*?)\}}", hook)).unwrap();
            }

            if let Some(captures) = HOOK_REGEX.captures(class_body) {
                let hook_body = captures.get(1).unwrap().as_str().trim().to_string();

                component.lifecycle_hooks.push(VueLifecycleHook {
                    name: hook.to_string(),
                    code: hook_body,
                });
            }
        }
    }

    fn extract_class_components(&self, options_str: &str, component: &mut VueComponent) {
        // Extract components from @Component options
        lazy_static! {
            static ref COMPONENTS_REGEX: Regex =
                Regex::new(r"components:\s*\{([\s\S]*?)\}(?:,|$)").unwrap();
        }

        if let Some(captures) = COMPONENTS_REGEX.captures(options_str) {
            let components_obj = captures.get(1).unwrap().as_str();

            // Extract individual components
            lazy_static! {
                static ref COMPONENT_REGEX: Regex =
                    Regex::new(r"([a-zA-Z0-9_]+)(?::\s*([a-zA-Z0-9_]+))?(?:,|$)").unwrap();
            }

            for captures in COMPONENT_REGEX.captures_iter(components_obj) {
                let component_name = captures.get(1).unwrap().as_str().to_string();
                let component_ref = captures.get(2).map_or(component_name.clone(), |m| m.as_str().to_string());

                component.components.insert(component_name, component_ref);
            }
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.components)
    }
