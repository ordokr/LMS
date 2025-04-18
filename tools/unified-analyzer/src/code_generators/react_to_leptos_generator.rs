use crate::analyzers::modules::enhanced_react_analyzer::{ReactComponent, ReactProp, ReactState, ReactEffect, ReactHandler};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ReactToLeptosGenerator {
    pub output_dir: PathBuf,
    pub components_dir: PathBuf,
}

impl ReactToLeptosGenerator {
    pub fn new(output_dir: &Path) -> Self {
        let components_dir = output_dir.join("components");
        
        Self {
            output_dir: output_dir.to_path_buf(),
            components_dir,
        }
    }
    
    pub fn generate_component(&self, component: &ReactComponent) -> Result<(), Box<dyn std::error::Error>> {
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
    
    fn generate_leptos_code(&self, component: &ReactComponent) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        // Add imports
        code.push_str("use leptos::*;\n");
        code.push_str("use leptos_dom::*;\n");
        
        // Add any additional imports based on component dependencies
        let mut additional_imports = HashMap::new();
        
        // Add imports for child components
        for child in &component.child_components {
            let child_module = to_snake_case(child);
            let child_component = child.clone();
            additional_imports.insert(child_component, format!("use crate::components::{}::{};\n", child_module, child));
        }
        
        // Add the additional imports
        for (_, import) in additional_imports.iter() {
            code.push_str(import);
        }
        
        code.push_str("\n");
        
        // Add component props struct
        if !component.props.is_empty() {
            code.push_str("#[derive(Props, Clone)]\n");
            code.push_str("pub struct ");
            code.push_str(&component.name);
            code.push_str("Props {\n");
            
            for prop in &component.props {
                let prop_type = self.map_prop_type(&prop.prop_type);
                
                if prop.required {
                    code.push_str(&format!("    pub {}: {},\n", prop.name, prop_type));
                } else {
                    code.push_str(&format!("    #[prop(optional)]\n"));
                    code.push_str(&format!("    pub {}: Option<{}>,\n", prop.name, prop_type));
                }
            }
            
            // Add children prop if component likely accepts children
            if component.jsx_structure.as_ref().map_or(false, |jsx| jsx.contains("{children}") || jsx.contains("{props.children}")) {
                code.push_str("    #[prop(optional)]\n");
                code.push_str("    pub children: Option<Children>,\n");
            }
            
            code.push_str("}\n\n");
        }
        
        // Add component function
        code.push_str("#[component]\n");
        code.push_str("pub fn ");
        code.push_str(&component.name);
        
        if !component.props.is_empty() {
            code.push_str("(cx: Scope, props: ");
            code.push_str(&component.name);
            code.push_str("Props");
        } else {
            code.push_str("(cx: Scope");
        }
        
        code.push_str(") -> impl IntoView {\n");
        
        // Add state signals
        for state in &component.state {
            let state_type = self.map_state_type(&state.state_type);
            let initial_value = state.initial_value.as_ref().map_or("Default::default()", |v| v.as_str());
            
            code.push_str(&format!("    let ({}, set_{}) = create_signal(cx, {});\n", 
                state.name, state.name, initial_value));
        }
        
        // Add effects
        for (i, effect) in component.effects.iter().enumerate() {
            code.push_str(&format!("    // Effect {}\n", i + 1));
            code.push_str("    create_effect(cx, move |_| {\n");
            
            // Add effect dependencies as watch signals
            for dep in &effect.dependencies {
                if component.state.iter().any(|s| &s.name == dep) {
                    code.push_str(&format!("        let _ = {}();\n", dep));
                } else if component.props.iter().any(|p| &p.name == dep) {
                    code.push_str(&format!("        let _ = props.{};\n", dep));
                }
            }
            
            // Add effect code as comment
            for line in effect.code_snippet.lines() {
                code.push_str(&format!("        // {}\n", line.trim()));
            }
            
            // Add placeholder implementation
            code.push_str("        // TODO: Implement effect\n");
            
            code.push_str("    });\n\n");
        }
        
        // Add event handlers
        for handler in &component.handlers {
            code.push_str(&format!("    let {} = move |", handler.name));
            
            // Add parameters
            if !handler.parameters.is_empty() {
                code.push_str(&handler.parameters.join(", "));
            }
            
            code.push_str("| {\n");
            
            // Add handler code as comment
            for line in handler.code_snippet.lines() {
                code.push_str(&format!("        // {}\n", line.trim()));
            }
            
            // Add placeholder implementation
            code.push_str("        // TODO: Implement handler\n");
            
            // For state updates, add signal updates
            for state in &component.state {
                if handler.code_snippet.contains(&format!("set{}", to_pascal_case(&state.name))) || 
                   handler.code_snippet.contains(&format!("this.setState({{ {}", state.name)) {
                    code.push_str(&format!("        // set_{}(new_value);\n", state.name));
                }
            }
            
            code.push_str("    };\n\n");
        }
        
        // Add view
        code.push_str("    view! { cx,\n");
        
        // Convert JSX to Leptos view
        if let Some(jsx) = &component.jsx_structure {
            let leptos_view = self.convert_jsx_to_leptos(jsx, component);
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
    
    fn map_prop_type(&self, react_type: &str) -> String {
        match react_type {
            "string" => "String".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => "Vec<String>".to_string(), // Default to Vec<String>, should be refined based on actual usage
            "object" => "serde_json::Value".to_string(),
            "func" => "Callback<()>".to_string(), // Default to no-arg callback, should be refined based on actual usage
            "node" => "Children".to_string(),
            "element" => "Children".to_string(),
            _ => "String".to_string(), // Default to String for unknown types
        }
    }
    
    fn map_state_type(&self, react_type: &str) -> String {
        match react_type {
            "string" => "String".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => "Vec<String>".to_string(), // Default to Vec<String>, should be refined based on actual usage
            "object" => "serde_json::Value".to_string(),
            _ => "String".to_string(), // Default to String for unknown types
        }
    }
    
    fn convert_jsx_to_leptos(&self, jsx: &str, component: &ReactComponent) -> String {
        let mut leptos_view = String::new();
        
        // Simple JSX to Leptos conversion
        // This is a basic implementation and would need to be enhanced for complex JSX
        
        // Process each line
        for line in jsx.lines() {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                leptos_view.push_str("        \n");
                continue;
            }
            
            // Convert JSX opening tags
            if trimmed.starts_with('<') && !trimmed.starts_with("</") {
                let mut tag_line = trimmed.to_string();
                
                // Convert React component references
                for child in &component.child_components {
                    let jsx_tag = format!("<{}", child);
                    if tag_line.contains(&jsx_tag) {
                        tag_line = tag_line.replace(&jsx_tag, &format!("<{}", child));
                    }
                }
                
                // Convert className to class
                tag_line = tag_line.replace("className=", "class=");
                
                // Convert onClick to on:click
                tag_line = tag_line.replace("onClick=", "on:click=");
                tag_line = tag_line.replace("onMouseEnter=", "on:mouseenter=");
                tag_line = tag_line.replace("onMouseLeave=", "on:mouseleave=");
                
                // Convert JSX expressions
                tag_line = self.convert_jsx_expressions(&tag_line, component);
                
                leptos_view.push_str(&format!("        {}\n", tag_line));
            }
            // Convert JSX closing tags
            else if trimmed.starts_with("</") {
                leptos_view.push_str(&format!("        {}\n", trimmed));
            }
            // Convert JSX expressions
            else {
                let converted_line = self.convert_jsx_expressions(trimmed, component);
                leptos_view.push_str(&format!("        {}\n", converted_line));
            }
        }
        
        leptos_view
    }
    
    fn convert_jsx_expressions(&self, line: &str, component: &ReactComponent) -> String {
        let mut result = line.to_string();
        
        // Convert {props.X} to {props.X}
        for prop in &component.props {
            let jsx_expr = format!("{{props.{}}}", prop.name);
            if result.contains(&jsx_expr) {
                result = result.replace(&jsx_expr, &format!("{{props.{}}}", prop.name));
            }
        }
        
        // Convert {this.state.X} to {X()}
        for state in &component.state {
            let jsx_expr = format!("{{this.state.{}}}", state.name);
            if result.contains(&jsx_expr) {
                result = result.replace(&jsx_expr, &format!("{{{}}}", state.name));
            }
            
            // Also check for {stateVar} in functional components
            let jsx_expr2 = format!("{{{}}}", state.name);
            if result.contains(&jsx_expr2) {
                result = result.replace(&jsx_expr2, &format!("{{{}}}", state.name));
            }
        }
        
        // Convert {this.X} to {X} for computed properties
        for computed in &component.computed_properties {
            let jsx_expr = format!("{{this.{}}}", computed.name);
            if result.contains(&jsx_expr) {
                result = result.replace(&jsx_expr, &format!("{{{}}}", computed.name));
            }
        }
        
        // Convert {this.handleX} to {handleX}
        for handler in &component.handlers {
            let jsx_expr = format!("{{this.{}}}", handler.name);
            if result.contains(&jsx_expr) {
                result = result.replace(&jsx_expr, &format!("{{{}}}", handler.name));
            }
            
            // Also check for {handleX} in functional components
            let jsx_expr2 = format!("{{{}}}", handler.name);
            if result.contains(&jsx_expr2) {
                result = result.replace(&jsx_expr2, &format!("{{{}}}", handler.name));
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

// Helper function to convert snake_case to PascalCase
fn to_pascal_case(snake_case: &str) -> String {
    let mut pascal_case = String::new();
    let mut capitalize_next = true;
    
    for c in snake_case.chars() {
        if c == '_' {
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
