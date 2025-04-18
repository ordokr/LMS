use crate::analyzers::modules::enhanced_ruby_view_analyzer::{EnhancedRubyView, ViewForm, ViewFormField, ViewLink, ViewPartial};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct RubyToLeptosViewGenerator {
    pub output_dir: PathBuf,
    pub components_dir: PathBuf,
}

impl RubyToLeptosViewGenerator {
    pub fn new(output_dir: &Path) -> Self {
        let components_dir = output_dir.join("components");
        
        Self {
            output_dir: output_dir.to_path_buf(),
            components_dir,
        }
    }
    
    pub fn generate_view(&self, view: &EnhancedRubyView) -> Result<(), Box<dyn std::error::Error>> {
        // Create components directory if it doesn't exist
        fs::create_dir_all(&self.components_dir)?;
        
        // Generate component name (PascalCase)
        let component_name = if let (Some(controller), Some(action)) = (&view.controller, &view.action) {
            format!("{}{}", to_pascal_case(controller), to_pascal_case(action))
        } else {
            to_pascal_case(&view.name)
        };
        
        // Generate file name (snake_case)
        let file_name = to_snake_case(&component_name);
        let file_path = self.components_dir.join(format!("{}.rs", file_name));
        
        // Generate Leptos component
        let leptos_code = self.generate_leptos_code(view, &component_name)?;
        
        // Write to file
        fs::write(file_path, leptos_code)?;
        
        Ok(())
    }
    
    fn generate_leptos_code(&self, view: &EnhancedRubyView, component_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        // Add imports
        code.push_str("use leptos::*;\n");
        code.push_str("use leptos_router::*;\n");
        
        // Add model imports based on instance variables
        if !view.instance_variables.is_empty() {
            for var in &view.instance_variables {
                let model_name = to_pascal_case(var);
                code.push_str(&format!("use crate::models::{}::{};\n", to_snake_case(var), model_name));
            }
        }
        
        // Add imports for partials
        for partial in &view.partials {
            let partial_name = partial.name.split('/').last().unwrap_or(&partial.name);
            let component_name = to_pascal_case(partial_name);
            code.push_str(&format!("use crate::components::{}::{};\n", to_snake_case(partial_name), component_name));
        }
        
        code.push_str("\n");
        
        // Add component function
        code.push_str("#[component]\n");
        code.push_str(&format!("pub fn {}(cx: Scope", component_name));
        
        // Add props based on instance variables
        for var in &view.instance_variables {
            let model_name = to_pascal_case(var);
            code.push_str(&format!(", {}: {}", var, model_name));
        }
        
        code.push_str(") -> impl IntoView {\n");
        
        // Generate view content
        code.push_str("    view! { cx,\n");
        
        // Generate div with class based on controller and action
        if let (Some(controller), Some(action)) = (&view.controller, &view.action) {
            code.push_str(&format!("        <div class=\"{}-{}\">\n", controller, action));
        } else {
            code.push_str("        <div>\n");
        }
        
        // Generate content based on view elements
        
        // Generate partials
        for partial in &view.partials {
            self.generate_partial_code(&mut code, partial);
        }
        
        // Generate links
        for link in &view.links {
            self.generate_link_code(&mut code, link);
        }
        
        // Generate forms
        for form in &view.forms {
            self.generate_form_code(&mut code, form);
        }
        
        code.push_str("        </div>\n");
        code.push_str("    }\n");
        code.push_str("}\n");
        
        Ok(code)
    }
    
    fn generate_partial_code(&self, code: &mut String, partial: &ViewPartial) {
        let partial_name = partial.name.split('/').last().unwrap_or(&partial.name);
        let component_name = to_pascal_case(partial_name);
        
        code.push_str(&format!("            <{}", component_name));
        
        // Add props based on locals
        for local in &partial.locals {
            code.push_str(&format!(" {}={{{}}}",  local, local));
        }
        
        code.push_str(" />\n");
    }
    
    fn generate_link_code(&self, code: &mut String, link: &ViewLink) {
        code.push_str("            <A");
        
        // Add href
        if link.url.contains("_path") || link.url.contains("_url") {
            // Convert Rails path helper to Leptos route
            let route = link.url.replace("_path", "").replace("_url", "");
            code.push_str(&format!(" href=\"/{}\"", route));
        } else {
            code.push_str(&format!(" href=\"{}\"", link.url));
        }
        
        // Add class if present
        if let Some(class) = link.options.get("class") {
            code.push_str(&format!(" class=\"{}\"", class));
        }
        
        code.push_str(">");
        
        // Add link text
        if let Some(text) = &link.text {
            code.push_str(text);
        } else {
            code.push_str("Link");
        }
        
        code.push_str("</A>\n");
    }
    
    fn generate_form_code(&self, code: &mut String, form: &ViewForm) {
        code.push_str("            <form");
        
        // Add action
        if let Some(action) = &form.action {
            code.push_str(&format!(" action=\"{}\"", action));
        }
        
        // Add method
        if let Some(method) = &form.method {
            code.push_str(&format!(" method=\"{}\"", method));
        }
        
        // Add class if present
        if let Some(class) = form.options.get("class") {
            code.push_str(&format!(" class=\"{}\"", class));
        }
        
        code.push_str(">\n");
        
        // Add form fields
        for field in &form.fields {
            self.generate_form_field_code(code, field);
        }
        
        // Add submit button
        code.push_str("                <button type=\"submit\">Submit</button>\n");
        
        code.push_str("            </form>\n");
    }
    
    fn generate_form_field_code(&self, code: &mut String, field: &ViewFormField) {
        code.push_str("                <div class=\"form-group\">\n");
        
        // Add label
        if let Some(label) = &field.label {
            code.push_str(&format!("                    <label for=\"{}\">{}</label>\n", field.name, label));
        } else {
            code.push_str(&format!("                    <label for=\"{}\">{}</label>\n", field.name, to_title_case(&field.name)));
        }
        
        // Add input based on field type
        match field.field_type.as_str() {
            "text_field" => {
                code.push_str(&format!("                    <input type=\"text\" id=\"{}\" name=\"{}\"", field.name, field.name));
                
                if field.required {
                    code.push_str(" required");
                }
                
                // Add class if present
                if let Some(class) = field.options.get("class") {
                    code.push_str(&format!(" class=\"{}\"", class));
                }
                
                code.push_str(" />\n");
            },
            "password_field" => {
                code.push_str(&format!("                    <input type=\"password\" id=\"{}\" name=\"{}\"", field.name, field.name));
                
                if field.required {
                    code.push_str(" required");
                }
                
                // Add class if present
                if let Some(class) = field.options.get("class") {
                    code.push_str(&format!(" class=\"{}\"", class));
                }
                
                code.push_str(" />\n");
            },
            "text_area" => {
                code.push_str(&format!("                    <textarea id=\"{}\" name=\"{}\"", field.name, field.name));
                
                if field.required {
                    code.push_str(" required");
                }
                
                // Add class if present
                if let Some(class) = field.options.get("class") {
                    code.push_str(&format!(" class=\"{}\"", class));
                }
                
                code.push_str("></textarea>\n");
            },
            "select" => {
                code.push_str(&format!("                    <select id=\"{}\" name=\"{}\"", field.name, field.name));
                
                if field.required {
                    code.push_str(" required");
                }
                
                // Add class if present
                if let Some(class) = field.options.get("class") {
                    code.push_str(&format!(" class=\"{}\"", class));
                }
                
                code.push_str(">\n");
                code.push_str("                        <option value=\"\">Select an option</option>\n");
                code.push_str("                        <!-- TODO: Add options -->\n");
                code.push_str("                    </select>\n");
            },
            "check_box" => {
                code.push_str(&format!("                    <input type=\"checkbox\" id=\"{}\" name=\"{}\"", field.name, field.name));
                
                // Add class if present
                if let Some(class) = field.options.get("class") {
                    code.push_str(&format!(" class=\"{}\"", class));
                }
                
                code.push_str(" />\n");
            },
            _ => {
                code.push_str(&format!("                    <input type=\"text\" id=\"{}\" name=\"{}\"", field.name, field.name));
                
                if field.required {
                    code.push_str(" required");
                }
                
                // Add class if present
                if let Some(class) = field.options.get("class") {
                    code.push_str(&format!(" class=\"{}\"", class));
                }
                
                code.push_str(" />\n");
            }
        }
        
        code.push_str("                </div>\n");
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

// Helper function to convert snake_case to Title Case
fn to_title_case(snake_case: &str) -> String {
    let pascal_case = to_pascal_case(snake_case);
    
    let mut title_case = String::new();
    
    for (i, c) in pascal_case.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            title_case.push(' ');
        }
        title_case.push(c);
    }
    
    title_case
}
