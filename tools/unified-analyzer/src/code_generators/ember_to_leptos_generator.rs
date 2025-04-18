use crate::analyzers::modules::enhanced_ember_analyzer::{EmberComponent, EmberProperty, EmberAction, EmberComputed, EmberService, EmberLifecycleHook};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use lazy_static::lazy_static;

pub struct EmberToLeptosGenerator {
    pub output_dir: PathBuf,
    pub components_dir: PathBuf,
}

impl EmberToLeptosGenerator {
    pub fn new(output_dir: &Path) -> Self {
        let components_dir = output_dir.join("components");
        
        Self {
            output_dir: output_dir.to_path_buf(),
            components_dir,
        }
    }
    
    pub fn generate_component(&self, component: &EmberComponent) -> Result<(), Box<dyn std::error::Error>> {
        // Create components directory if it doesn't exist
        fs::create_dir_all(&self.components_dir)?;
        
        // Generate file name (snake_case)
        let file_name = to_snake_case(&component.name);
        let file_path = self.components_dir.join(format!("{}.rs", file_name));
        
        // Generate Leptos component
        let leptos_code = self.generate_leptos_code(component)?;
        
        // Write to file
        fs::write(file_path, leptos_code)?;
        
        Ok(())
    }
    
    fn generate_leptos_code(&self, component: &EmberComponent) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        // Add imports
        code.push_str("use leptos::*;\n");
        code.push_str("use leptos_dom::*;\n");
        
        // Add any additional imports based on component dependencies
        let mut additional_imports = HashMap::new();
        
        // Add imports for child components
        for child in &component.child_components {
            // Convert Ember component names to PascalCase
            let child_pascal = if child.contains('-') {
                // Convert kebab-case to PascalCase
                child.split('-')
                    .map(|part| {
                        let mut chars = part.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            } else if child.chars().next().unwrap().is_uppercase() {
                // Already PascalCase
                child.clone()
            } else {
                // Convert from other formats to PascalCase
                to_pascal_case(child)
            };
            
            let child_module = to_snake_case(&child_pascal);
            additional_imports.insert(child_pascal.clone(), format!("use crate::components::{}::{};\n", child_module, child_pascal));
        }
        
        // Add the additional imports
        for (_, import) in additional_imports.iter() {
            code.push_str(import);
        }
        
        // Add imports for services
        for service in &component.services {
            let service_pascal = to_pascal_case(&service.service_type);
            let service_module = to_snake_case(&service.service_type);
            code.push_str(&format!("use crate::services::{}::{};\n", service_module, service_pascal));
        }
        
        code.push_str("\n");
        
        // Add component props struct
        let has_args = !component.properties.iter().filter(|p| p.property_type == "argument").collect::<Vec<_>>().is_empty();
        
        if has_args {
            code.push_str("#[derive(Props, Clone)]\n");
            code.push_str("pub struct ");
            code.push_str(&to_pascal_case(&component.name));
            code.push_str("Props {\n");
            
            for prop in component.properties.iter().filter(|p| p.property_type == "argument") {
                let prop_type = self.map_property_type(&prop.property_type);
                
                if prop.default_value.is_none() {
                    code.push_str(&format!("    pub {}: {},\n", prop.name, prop_type));
                } else {
                    code.push_str(&format!("    #[prop(optional)]\n"));
                    code.push_str(&format!("    pub {}: Option<{}>,\n", prop.name, prop_type));
                }
            }
            
            // Add children prop if component likely accepts yield
            if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("{{yield}}")) {
                code.push_str("    #[prop(optional)]\n");
                code.push_str("    pub children: Option<Children>,\n");
            }
            
            code.push_str("}\n\n");
        }
        
        // Add component function
        code.push_str("#[component]\n");
        code.push_str("pub fn ");
        code.push_str(&to_pascal_case(&component.name));
        
        if has_args {
            code.push_str("(cx: Scope, props: ");
            code.push_str(&to_pascal_case(&component.name));
            code.push_str("Props");
        } else {
            code.push_str("(cx: Scope");
        }
        
        code.push_str(") -> impl IntoView {\n");
        
        // Add services
        for service in &component.services {
            let service_pascal = to_pascal_case(&service.service_type);
            code.push_str(&format!("    let {} = use_context::<{}>(cx).expect(\"No {} provided\");\n", 
                service.name, service_pascal, service_pascal));
        }
        
        // Add state signals for properties
        for prop in component.properties.iter().filter(|p| p.property_type != "argument" && p.property_type != "computed") {
            let prop_type = self.map_property_type(&prop.property_type);
            let initial_value = prop.default_value.as_ref().map_or("Default::default()", |v| v.as_str());
            
            code.push_str(&format!("    let ({}, set_{}) = create_signal(cx, {});\n", 
                prop.name, prop.name, initial_value));
        }
        
        // Add computed properties as memos
        for computed in &component.computed_properties {
            code.push_str(&format!("    let {} = create_memo(cx, move |_| {{\n", computed.name));
            
            // Add dependencies
            for dep in &computed.dependencies {
                if component.properties.iter().any(|p| &p.name == dep) {
                    code.push_str(&format!("        let {} = {}();\n", dep, dep));
                }
            }
            
            // Add computed code as comment
            for line in computed.code_snippet.lines() {
                code.push_str(&format!("        // {}\n", line.trim()));
            }
            
            // Add placeholder implementation
            code.push_str("        // TODO: Implement computed property\n");
            code.push_str("        String::new() // Placeholder return value\n");
            
            code.push_str("    });\n\n");
        }
        
        // Add lifecycle hooks
        if component.lifecycle_hooks.iter().any(|h| h.name == "init" || h.name == "constructor") {
            code.push_str("    // Initialize component\n");
            code.push_str("    let _ = use_effect(cx, (), |_| {\n");
            
            // Add init code as comment
            let init_hook = component.lifecycle_hooks.iter().find(|h| h.name == "init" || h.name == "constructor");
            if let Some(hook) = init_hook {
                for line in hook.code_snippet.lines() {
                    code.push_str(&format!("        // {}\n", line.trim()));
                }
            }
            
            code.push_str("        // TODO: Implement initialization\n");
            
            // Add cleanup for willDestroy
            let destroy_hook = component.lifecycle_hooks.iter().find(|h| h.name == "willDestroy");
            if let Some(hook) = destroy_hook {
                code.push_str("        || {\n");
                for line in hook.code_snippet.lines() {
                    code.push_str(&format!("            // {}\n", line.trim()));
                }
                code.push_str("            // TODO: Implement cleanup\n");
                code.push_str("        }\n");
            } else {
                code.push_str("        || {}\n");
            }
            
            code.push_str("    });\n\n");
        }
        
        // Add actions as event handlers
        for action in &component.actions {
            code.push_str(&format!("    let {} = move |", action.name));
            
            // Add parameters
            if !action.parameters.is_empty() {
                code.push_str(&action.parameters.join(", "));
            }
            
            code.push_str("| {\n");
            
            // Add action code as comment
            for line in action.code_snippet.lines() {
                code.push_str(&format!("        // {}\n", line.trim()));
            }
            
            // Add placeholder implementation
            code.push_str("        // TODO: Implement action\n");
            
            code.push_str("    };\n\n");
        }
        
        // Add view
        code.push_str("    view! { cx,\n");
        
        // Convert template to Leptos view
        if let Some(template) = &component.template_content {
            let leptos_view = self.convert_template_to_leptos(template, component);
            code.push_str(&leptos_view);
        } else {
            code.push_str("        <div>\n");
            code.push_str("            // TODO: Implement view\n");
            code.push_str("        </div>\n");
        }
        
        code.push_str("    }\n");
        code.push_str("}\n");
        
        Ok(code)
    }
    
    fn map_property_type(&self, ember_type: &str) -> String {
        match ember_type {
            "string" => "String".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => "Vec<String>".to_string(), // Default to Vec<String>, should be refined based on actual usage
            "object" => "serde_json::Value".to_string(),
            "argument" => "String".to_string(), // Default to String for arguments, should be refined based on actual usage
            _ => "String".to_string(), // Default to String for unknown types
        }
    }
    
    fn convert_template_to_leptos(&self, template: &str, component: &EmberComponent) -> String {
        let mut leptos_view = String::new();
        
        // Simple HBS to Leptos conversion
        // This is a basic implementation and would need to be enhanced for complex templates
        
        // Process each line
        for line in template.lines() {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                leptos_view.push_str("        \n");
                continue;
            }
            
            // Convert {{yield}} to {children}
            if trimmed.contains("{{yield}}") {
                let converted = trimmed.replace("{{yield}}", "{children}");
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }
            
            // Convert {{#if condition}}...{{/if}} to <Show when=move || condition>...</Show>
            lazy_static! {
                static ref IF_REGEX: Regex = Regex::new(r"\{\{#if\s+([^}]+)\}\}").unwrap();
            }
            
            if IF_REGEX.is_match(trimmed) {
                let captures = IF_REGEX.captures(trimmed).unwrap();
                let condition = captures.get(1).unwrap().as_str();
                
                let converted = IF_REGEX.replace(trimmed, |_: &regex::Captures| {
                    format!("<Show when=move || {}>", self.convert_ember_expression(condition, component))
                });
                
                let converted = converted.replace("{{/if}}", "</Show>");
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }
            
            // Convert {{#each items as |item|}}...{{/each}} to <For each=move || items key=|item| item.id let:item>...</For>
            lazy_static! {
                static ref EACH_REGEX: Regex = Regex::new(r"\{\{#each\s+([^}]+)\s+as\s+\|([^|]+)\|\}\}").unwrap();
            }
            
            if EACH_REGEX.is_match(trimmed) {
                let captures = EACH_REGEX.captures(trimmed).unwrap();
                let items = captures.get(1).unwrap().as_str();
                let item = captures.get(2).unwrap().as_str();
                
                let converted = EACH_REGEX.replace(trimmed, |_: &regex::Captures| {
                    format!("<For each=move || {} key=|{}| {}.id let:{}>", 
                        self.convert_ember_expression(items, component),
                        item.trim(),
                        item.trim(),
                        item.trim())
                });
                
                let converted = converted.replace("{{/each}}", "</For>");
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }
            
            // Convert {{property}} to {property}
            lazy_static! {
                static ref PROPERTY_REGEX: Regex = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
            }
            
            if PROPERTY_REGEX.is_match(trimmed) {
                let converted = PROPERTY_REGEX.replace_all(trimmed, |caps: &regex::Captures| {
                    let expr = caps.get(1).unwrap().as_str();
                    format!("{{{}}}", self.convert_ember_expression(expr, component))
                });
                
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }
            
            // Convert component invocations
            // {{component-name param1=value1}} to <ComponentName param1=value1 />
            lazy_static! {
                static ref COMPONENT_REGEX: Regex = Regex::new(r"\{\{([a-z0-9-]+)([^}]*)\}\}").unwrap();
            }
            
            if COMPONENT_REGEX.is_match(trimmed) {
                let captures = COMPONENT_REGEX.captures(trimmed).unwrap();
                let component_name = captures.get(1).unwrap().as_str();
                let params = captures.get(2).map_or("", |m| m.as_str());
                
                // Skip helpers and built-ins
                if !component_name.contains("-") && 
                   vec!["if", "each", "unless", "yield", "outlet", "link-to", "input", "textarea", "log", "debugger"]
                       .contains(&component_name) {
                    leptos_view.push_str(&format!("        // TODO: Convert Ember helper: {}\n", trimmed));
                    continue;
                }
                
                // Convert kebab-case to PascalCase
                let pascal_name = component_name.split('-')
                    .map(|part| {
                        let mut chars = part.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("");
                
                // Convert params
                let converted_params = self.convert_ember_params(params, component);
                
                leptos_view.push_str(&format!("        <{}{} />\n", pascal_name, converted_params));
                continue;
            }
            
            // Convert HTML elements with actions
            // <button {{action "doSomething"}}> to <button on:click=doSomething>
            lazy_static! {
                static ref ACTION_REGEX: Regex = Regex::new(r"\{\{action\s+['\"]([^'\"]+)['\"](?:\s+([^}]+))?\}\}").unwrap();
            }
            
            if ACTION_REGEX.is_match(trimmed) {
                let converted = ACTION_REGEX.replace_all(trimmed, |caps: &regex::Captures| {
                    let action_name = caps.get(1).unwrap().as_str();
                    format!("on:click={}", action_name)
                });
                
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }
            
            // Pass through other HTML
            leptos_view.push_str(&format!("        {}\n", trimmed));
        }
        
        leptos_view
    }
    
    fn convert_ember_expression(&self, expr: &str, component: &EmberComponent) -> String {
        let expr = expr.trim();
        
        // Convert this.property to property()
        if expr.starts_with("this.") {
            let property = &expr[5..];
            
            // Check if it's a computed property
            if component.computed_properties.iter().any(|cp| cp.name == property) {
                return format!("{}()", property);
            }
            
            // Check if it's a regular property
            if component.properties.iter().any(|p| p.name == property) {
                return format!("{}()", property);
            }
            
            // Otherwise, just return as is
            return property.to_string();
        }
        
        // Convert @property to props.property
        if expr.starts_with('@') {
            let property = &expr[1..];
            return format!("props.{}", property);
        }
        
        // Handle simple property references
        if component.properties.iter().any(|p| p.name == expr) {
            return format!("{}()", expr);
        }
        
        // Handle computed properties
        if component.computed_properties.iter().any(|cp| cp.name == expr) {
            return format!("{}()", expr);
        }
        
        // Pass through other expressions
        expr.to_string()
    }
    
    fn convert_ember_params(&self, params: &str, component: &EmberComponent) -> String {
        let mut result = String::new();
        
        // Extract param=value pairs
        lazy_static! {
            static ref PARAM_REGEX: Regex = Regex::new(r"([a-zA-Z0-9_@]+)=([^=\s]+|\{[^}]+\}|\"[^\"]+\"|'[^']+')").unwrap();
        }
        
        for captures in PARAM_REGEX.captures_iter(params) {
            let param_name = captures.get(1).unwrap().as_str();
            let param_value = captures.get(2).unwrap().as_str();
            
            // Convert @param to param
            let param_name = if param_name.starts_with('@') {
                &param_name[1..]
            } else {
                param_name
            };
            
            // Convert values
            if param_value.starts_with('{') && param_value.ends_with('}') {
                // Handle expressions
                let expr = &param_value[1..param_value.len()-1];
                let converted_expr = self.convert_ember_expression(expr, component);
                result.push_str(&format!(" {}={{{}}}", param_name, converted_expr));
            } else if param_value.starts_with('"') || param_value.starts_with('\'') {
                // Handle string literals
                result.push_str(&format!(" {}={}", param_name, param_value));
            } else {
                // Handle other values
                result.push_str(&format!(" {}=\"{}\"", param_name, param_value));
            }
        }
        
        result
    }
}

// Helper function to convert PascalCase to snake_case
fn to_snake_case(pascal_case: &str) -> String {
    let mut snake_case = String::new();
    
    for (i, c) in pascal_case.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }
        snake_case.push(c.to_lowercase().next().unwrap());
    }
    
    snake_case
}

// Helper function to convert snake_case or kebab-case to PascalCase
fn to_pascal_case(name: &str) -> String {
    let mut pascal_case = String::new();
    let mut capitalize_next = true;
    
    for c in name.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            pascal_case.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            pascal_case.push(c);
        }
    }
    
    pascal_case
}
