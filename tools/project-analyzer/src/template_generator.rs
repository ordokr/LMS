//! Generator for Rust templates from JavaScript files
//!
//! This module provides functionality to generate Rust template files from JavaScript files
//! to facilitate the JavaScript to Rust migration process.

use std::fs;
use std::path::Path;
use std::error::Error;

/// Generate a Rust template file from a JavaScript file
pub fn generate_rust_template(js_file: &str, rust_file: &str) -> Result<(), Box<dyn Error>> {
    println!("Generating Rust template: {} -> {}", js_file, rust_file);
    
    // Ensure source file exists
    if !Path::new(js_file).exists() {
        return Err(format!("JavaScript file not found: {}", js_file).into());
    }
    
    // Create directories for the output file if needed
    if let Some(parent) = Path::new(rust_file).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Read the JavaScript file
    let js_content = fs::read_to_string(js_file)?;
    
    // Analyze the JavaScript file to determine its type
    let file_type = determine_file_type(js_file, &js_content);
    
    // Generate appropriate Rust template based on file type
    let rust_template = match file_type {
        FileType::Model => generate_model_template(js_file, &js_content),
        FileType::Controller => generate_controller_template(js_file, &js_content),
        FileType::Service => generate_service_template(js_file, &js_content),
        FileType::Utility => generate_utility_template(js_file, &js_content),
        FileType::Route => generate_route_template(js_file, &js_content),
        FileType::Middleware => generate_middleware_template(js_file, &js_content),
        FileType::Unknown => generate_generic_template(js_file, &js_content),
    };
    
    // Write the Rust template to the output file
    fs::write(rust_file, rust_template)?;
    
    println!("Successfully generated Rust template: {}", rust_file);
    Ok(())
}

/// Determine the type of JavaScript file
fn determine_file_type(file_path: &str, content: &str) -> FileType {
    // Determine file type based on path and content
    if file_path.contains("/models/") || file_path.contains("\\models\\") {
        return FileType::Model;
    } else if file_path.contains("/controllers/") || file_path.contains("\\controllers\\") {
        return FileType::Controller;
    } else if file_path.contains("/services/") || file_path.contains("\\services\\") {
        return FileType::Service;
    } else if file_path.contains("/utils/") || file_path.contains("\\utils\\") {
        return FileType::Utility;
    } else if file_path.contains("/routes/") || file_path.contains("\\routes\\") {
        return FileType::Route;
    } else if file_path.contains("/middleware/") || file_path.contains("\\middleware\\") {
        return FileType::Middleware;
    }
    
    // Check content for clues
    if content.contains("class") && content.contains("extends Model") {
        return FileType::Model;
    } else if content.contains("router.") || content.contains("app.get") || content.contains("app.post") {
        return FileType::Route;
    } else if content.contains("module.exports") && (content.contains("function") || content.contains("=>")) {
        return FileType::Utility;
    }
    
    FileType::Unknown
}

/// Different types of JavaScript files
enum FileType {
    Model,
    Controller,
    Service,
    Utility,
    Route,
    Middleware,
    Unknown,
}

/// Extract functions from JavaScript content
fn extract_functions(content: &str) -> Vec<String> {
    let mut functions = Vec::new();
    
    // Simple regex-like approach to find function definitions
    // In a real implementation, we would use a proper JS parser
    for line in content.lines() {
        let line = line.trim();
        
        // Check for function declarations
        if line.starts_with("function ") && line.contains("(") {
            if let Some(name_end) = line.find("(") {
                let name = line["function ".len()..name_end].trim();
                functions.push(name.to_string());
            }
        }
        
        // Check for arrow functions with assignment
        if line.contains(" = ") && line.contains("=>") {
            if let Some(name_end) = line.find(" = ") {
                let name = line[0..name_end].trim();
                // Remove const, let, var
                let name = name.replace("const ", "").replace("let ", "").replace("var ", "");
                functions.push(name.to_string());
            }
        }
        
        // Check for methods in classes
        if !line.starts_with("//") && line.contains("(") && !line.contains("if") && !line.contains("for") && !line.contains("while") {
            if let Some(name_end) = line.find("(") {
                let name = line[0..name_end].trim();
                if !name.is_empty() && !name.contains(" ") {
                    functions.push(name.to_string());
                }
            }
        }
    }
    
    functions
}

/// Extract imports/requires from JavaScript content
fn extract_imports(content: &str) -> Vec<(String, String)> {
    let mut imports = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        // Check for CommonJS requires
        if line.contains("require(") {
            if let Some(var_end) = line.find(" = ") {
                let var_name = line[0..var_end].trim()
                    .replace("const ", "")
                    .replace("let ", "")
                    .replace("var ", "");
                
                if let Some(module_start) = line.find("require(") {
                    if let Some(module_end) = line[module_start..].find(")") {
                        let module_path = line[module_start + "require(".len()..module_start + module_end]
                            .replace("'", "")
                            .replace("\"", "");
                        
                        imports.push((var_name, module_path));
                    }
                }
            }
        }
        
        // Check for ES6 imports
        if line.starts_with("import ") {
            if line.contains(" from ") {
                if let Some(from_idx) = line.find(" from ") {
                    let import_part = line["import ".len()..from_idx].trim();
                    let module_part = line[from_idx + " from ".len()..].trim()
                        .replace("'", "")
                        .replace("\"", "")
                        .replace(";", "");
                    
                    imports.push((import_part.to_string(), module_part));
                }
            }
        }
    }
    
    imports
}

/// Generate a template for a model file
fn generate_model_template(js_file: &str, content: &str) -> String {
    let js_filename = Path::new(js_file).file_name().unwrap().to_string_lossy();
    let functions = extract_functions(content);
    
    let mut template = format!("//! Rust model migrated from {}\n\n", js_filename);
    template.push_str("use serde::{Deserialize, Serialize};\n");
    template.push_str("use thiserror::Error;\n\n");
    
    // Generate error type
    template.push_str("#[derive(Debug, Error)]\n");
    template.push_str("pub enum ModelError {\n");
    template.push_str("    #[error(\"Invalid data: {0}\")]\n");
    template.push_str("    InvalidData(String),\n\n");
    
    template.push_str("    #[error(\"Database error: {0}\")]\n");
    template.push_str("    DatabaseError(String),\n");
    template.push_str("}\n\n");
    
    // Generate model struct
    let struct_name = js_filename.replace(".js", "");
    template.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    template.push_str(format!("pub struct {} {{\n", struct_name).as_str());
    template.push_str("    // TODO: Add fields based on JavaScript model\n");
    template.push_str("    pub id: Option<String>,\n");
    template.push_str("    pub name: String,\n");
    template.push_str("    pub created_at: Option<String>,\n");
    template.push_str("    pub updated_at: Option<String>,\n");
    template.push_str("}\n\n");
    
    // Generate implementation
    template.push_str(format!("impl {} {{\n", struct_name).as_str());
    
    // Add constructor
    template.push_str("    /// Create a new instance\n");
    template.push_str("    pub fn new(name: String) -> Self {\n");
    template.push_str("        Self {\n");
    template.push_str("            id: None,\n");
    template.push_str("            name,\n");
    template.push_str("            created_at: None,\n");
    template.push_str("            updated_at: None,\n");
    template.push_str("        }\n");
    template.push_str("    }\n\n");
    
    // Add methods based on extracted functions
    for func in &functions {
        let snake_case = to_snake_case(func);
        template.push_str(format!("    /// TODO: Implement {}\n", func).as_str());
        template.push_str(format!("    pub fn {}(&self) -> Result<(), ModelError> {{\n", snake_case).as_str());
        template.push_str("        // TODO: Implement this method\n");
        template.push_str("        Ok(())\n");
        template.push_str("    }\n\n");
    }
    
    template.push_str("}\n\n");
    
    // Add tests
    template.push_str("#[cfg(test)]\n");
    template.push_str("mod tests {\n");
    template.push_str("    use super::*;\n\n");
    
    template.push_str("    #[test]\n");
    template.push_str("    fn test_new() {\n");
    template.push_str(format!("        let model = {}::new(\"Test\".to_string());\n", struct_name).as_str());
    template.push_str("        assert_eq!(model.name, \"Test\");\n");
    template.push_str("        assert!(model.id.is_none());\n");
    template.push_str("    }\n");
    template.push_str("}\n");
    
    template
}

/// Generate a template for a controller file
fn generate_controller_template(js_file: &str, content: &str) -> String {
    let js_filename = Path::new(js_file).file_name().unwrap().to_string_lossy();
    let functions = extract_functions(content);
    
    let mut template = format!("//! Rust controller migrated from {}\n\n", js_filename);
    template.push_str("use actix_web::{web, HttpResponse, Responder};\n");
    template.push_str("use serde::{Deserialize, Serialize};\n");
    template.push_str("use thiserror::Error;\n\n");
    
    // Generate error type
    template.push_str("#[derive(Debug, Error)]\n");
    template.push_str("pub enum ControllerError {\n");
    template.push_str("    #[error(\"Invalid input: {0}\")]\n");
    template.push_str("    InvalidInput(String),\n\n");
    
    template.push_str("    #[error(\"Internal error: {0}\")]\n");
    template.push_str("    InternalError(String),\n");
    template.push_str("}\n\n");
    
    // Add response struct
    template.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    template.push_str("pub struct ApiResponse<T> {\n");
    template.push_str("    pub success: bool,\n");
    template.push_str("    pub message: Option<String>,\n");
    template.push_str("    pub data: Option<T>,\n");
    template.push_str("}\n\n");
    
    // Add controller methods
    for func in &functions {
        let snake_case = to_snake_case(func);
        
        template.push_str(format!("/// Handler for {}\n", func).as_str());
        template.push_str(format!("pub async fn {}(data: web::Json<serde_json::Value>) -> impl Responder {{\n", snake_case).as_str());
        template.push_str("    // TODO: Implement this controller method\n");
        template.push_str("    HttpResponse::Ok().json(ApiResponse {\n");
        template.push_str("        success: true,\n");
        template.push_str("        message: Some(\"Operation successful\".to_string()),\n");
        template.push_str("        data: Some(data.0),\n");
        template.push_str("    })\n");
        template.push_str("}\n\n");
    }
    
    // Add tests
    template.push_str("#[cfg(test)]\n");
    template.push_str("mod tests {\n");
    template.push_str("    use super::*;\n");
    template.push_str("    use actix_web::test;\n\n");
    
    if !functions.is_empty() {
        let first_func = to_snake_case(&functions[0]);
        template.push_str("    #[actix_rt::test]\n");
        template.push_str(format!("    async fn test_{}() {{\n", first_func).as_str());
        template.push_str("        let req = test::TestRequest::post()\n");
        template.push_str("            .set_json(serde_json::json!({\"test\": \"value\"}))\n");
        template.push_str("            .to_http_request();\n\n");
        
        template.push_str("        let resp = web::Json(serde_json::json!({\"test\": \"value\"}))\n");
        template.push_str(format!("            .into_inner();\n").as_str());
        
        template.push_str(format!("        let result = {}(web::Json(resp)).await;\n", first_func).as_str());
        template.push_str("        // TODO: Add assertions\n");
        template.push_str("    }\n");
    }
    
    template.push_str("}\n");
    
    template
}

/// Generate a template for a service file
fn generate_service_template(js_file: &str, content: &str) -> String {
    let js_filename = Path::new(js_file).file_name().unwrap().to_string_lossy();
    let functions = extract_functions(content);
    let imports = extract_imports(content);
    
    let struct_name = js_filename.replace(".js", "Service").replace("Service.js", "Service");
    
    let mut template = format!("//! Rust service migrated from {}\n\n", js_filename);
    template.push_str("use async_trait::async_trait;\n");
    template.push_str("use mockall::predicate::*;\n");
    template.push_str("use mockall::*;\n");
    template.push_str("use serde::{Deserialize, Serialize};\n");
    template.push_str("use thiserror::Error;\n\n");
    
    // Add service dependencies from imports
    for (_, import_path) in imports {
        if import_path.starts_with(".") {
            // Convert relative import to Rust module path
            let path = import_path.replace("./", "crate::")
                .replace("../", "crate::")
                .replace("/", "::");
            
            template.push_str(format!("// TODO: Replace with actual Rust module\n// use {};\n\n", path).as_str());
        }
    }
    
    // Generate error type
    template.push_str("#[derive(Debug, Error)]\n");
    template.push_str("pub enum ServiceError {\n");
    template.push_str("    #[error(\"Invalid input: {0}\")]\n");
    template.push_str("    InvalidInput(String),\n\n");
    
    template.push_str("    #[error(\"Database error: {0}\")]\n");
    template.push_str("    DatabaseError(String),\n\n");
    
    template.push_str("    #[error(\"Not found: {0}\")]\n");
    template.push_str("    NotFound(String),\n");
    template.push_str("}\n\n");
    
    // Create service trait with all methods
    template.push_str("#[async_trait]\n");
    template.push_str("#[automock]\n");
    template.push_str(format!("pub trait {} {{\n", struct_name).as_str());
    
    for func in &functions {
        let snake_case = to_snake_case(func);
        template.push_str(format!("    /// TODO: Document the {} method\n", func).as_str());
        template.push_str(format!("    async fn {}(&self, input: serde_json::Value) -> Result<serde_json::Value, ServiceError>;\n\n", snake_case).as_str());
    }
    
    template.push_str("}\n\n");
    
    // Create concrete service implementation
    template.push_str(format!("pub struct {}Impl {{\n", struct_name).as_str());
    template.push_str("    // TODO: Add service dependencies\n");
    template.push_str("}\n\n");
    
    template.push_str(format!("impl {}Impl {{\n", struct_name).as_str());
    template.push_str("    /// Create a new service instance\n");
    template.push_str("    pub fn new() -> Self {\n");
    template.push_str("        Self {\n");
    template.push_str("            // TODO: Initialize dependencies\n");
    template.push_str("        }\n");
    template.push_str("    }\n");
    template.push_str("}\n\n");
    
    // Implement trait for the concrete service
    template.push_str("#[async_trait]\n");
    template.push_str(format!("impl {} for {}Impl {{\n", struct_name, struct_name).as_str());
    
    for func in &functions {
        let snake_case = to_snake_case(func);
        template.push_str(format!("    async fn {}(&self, input: serde_json::Value) -> Result<serde_json::Value, ServiceError> {{\n", snake_case).as_str());
        template.push_str("        // TODO: Implement this method\n");
        template.push_str("        Ok(input)\n");
        template.push_str("    }\n\n");
    }
    
    template.push_str("}\n\n");
    
    // Add tests
    template.push_str("#[cfg(test)]\n");
    template.push_str("mod tests {\n");
    template.push_str("    use super::*;\n\n");
    
    if !functions.is_empty() {
        let first_func = to_snake_case(&functions[0]);
        template.push_str("    #[tokio::test]\n");
        template.push_str(format!("    async fn test_{}_success() {{\n", first_func).as_str());
        template.push_str(format!("        let service = {}Impl::new();\n", struct_name).as_str());
        template.push_str("        let input = serde_json::json!({\"test\": \"value\"});\n\n");
        
        template.push_str(format!("        let result = service.{}(input).await;\n", first_func).as_str());
        template.push_str("        assert!(result.is_ok());\n");
        template.push_str("    }\n\n");
        
        template.push_str("    #[tokio::test]\n");
        template.push_str(format!("    async fn test_{}_with_mock() {{\n", first_func).as_str());
        template.push_str(format!("        let mut mock = Mock{}::new();\n", struct_name).as_str());
        template.push_str(format!("        mock.expect_{}()\n", first_func).as_str());
        template.push_str("            .with(predicate::always())\n");
        template.push_str("            .returning(|_| Ok(serde_json::json!({\"result\": \"mocked\"})));\n\n");
        
        template.push_str("        let input = serde_json::json!({\"test\": \"value\"});\n");
        template.push_str(format!("        let result = mock.{}(input).await;\n", first_func).as_str());
        template.push_str("        assert!(result.is_ok());\n");
        template.push_str("        assert_eq!(result.unwrap(), serde_json::json!({\"result\": \"mocked\"}));\n");
        template.push_str("    }\n");
    }
    
    template.push_str("}\n");
    
    template
}

/// Generate a template for a utility file
fn generate_utility_template(js_file: &str, content: &str) -> String {
    let js_filename = Path::new(js_file).file_name().unwrap().to_string_lossy();
    let functions = extract_functions(content);
    
    let mut template = format!("//! Rust utility functions migrated from {}\n\n", js_filename);
    
    // Add functions
    for func in &functions {
        let snake_case = to_snake_case(func);
        
        template.push_str(format!("/// TODO: Document the {} function\n", func).as_str());
        template.push_str(format!("pub fn {}(input: &str) -> String {{\n", snake_case).as_str());
        template.push_str("    // TODO: Implement this utility function\n");
        template.push_str("    input.to_string()\n");
        template.push_str("}\n\n");
    }
    
    // Add tests
    template.push_str("#[cfg(test)]\n");
    template.push_str("mod tests {\n");
    template.push_str("    use super::*;\n\n");
    
    for func in &functions {
        let snake_case = to_snake_case(func);
        
        template.push_str("    #[test]\n");
        template.push_str(format!("    fn test_{}() {{\n", snake_case).as_str());
        template.push_str(format!("        let result = {}(\"test\");\n", snake_case).as_str());
        template.push_str("        // TODO: Add assertions\n");
        template.push_str("        assert_eq!(result, \"test\");\n");
        template.push_str("    }\n\n");
    }
    
    template.push_str("}\n");
    
    template
}

/// Generate a template for a route file
fn generate_route_template(js_file: &str, content: &str) -> String {
    let js_filename = Path::new(js_file).file_name().unwrap().to_string_lossy();
    
    let mut template = format!("//! Rust routes migrated from {}\n\n", js_filename);
    template.push_str("use actix_web::{web, HttpResponse, Responder};\n");
    template.push_str("use actix_web::web::ServiceConfig;\n\n");
    
    // Add configure function
    template.push_str("/// Configure routes for this module\n");
    template.push_str("pub fn configure(cfg: &mut ServiceConfig) {\n");
    template.push_str("    cfg.service(\n");
    template.push_str("        web::scope(\"/api\")\n");
    
    // Parse content to find routes
    let mut routes = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        
        if line.contains("router.get") || line.contains("app.get") {
            routes.push(("GET", extract_route_path(line)));
        } else if line.contains("router.post") || line.contains("app.post") {
            routes.push(("POST", extract_route_path(line)));
        } else if line.contains("router.put") || line.contains("app.put") {
            routes.push(("PUT", extract_route_path(line)));
        } else if line.contains("router.delete") || line.contains("app.delete") {
            routes.push(("DELETE", extract_route_path(line)));
        }
    }
    
    // Add routes
    for (i, (method, path)) in routes.iter().enumerate() {
        let handler_name = format!("handle_{}{}", method.to_lowercase(), i);
        
        if let Some(path_str) = path {
            template.push_str(format!("            .route(\"{}\", web::{}().to({}))\n", 
                path_str, method.to_lowercase(), handler_name).as_str());
        }
    }
    
    template.push_str("    );\n");
    template.push_str("}\n\n");
    
    // Add handlers for each route
    for (i, (method, path)) in routes.iter().enumerate() {
        let handler_name = format!("handle_{}{}", method.to_lowercase(), i);
        
        // Create a string to avoid temporary value being dropped
        let path_string = match path {
            Some(p) => p.clone(),
            None => "<unknown>".to_string(),
        };
        
        template.push_str(format!("/// Handler for {} {}\n", method, path_string).as_str());
        template.push_str(format!("async fn {}(data: web::Json<serde_json::Value>) -> impl Responder {{\n", handler_name).as_str());
        template.push_str("    // TODO: Implement this route handler\n");
        template.push_str("    HttpResponse::Ok().json(data.0)\n");
        template.push_str("}\n\n");
    }
    
    // Add tests
    template.push_str("#[cfg(test)]\n");
    template.push_str("mod tests {\n");
    template.push_str("    use super::*;\n");
    template.push_str("    use actix_web::{test, App};\n\n");
    
    if !routes.is_empty() {
        template.push_str("    #[actix_rt::test]\n");
        template.push_str("    async fn test_route_configuration() {\n");
        template.push_str("        let mut app = test::init_service(App::new().configure(configure)).await;\n");
        template.push_str("        // TODO: Add route testing\n");
        template.push_str("    }\n");
    }
    
    template.push_str("}\n");
    
    template
}

/// Extract route path from a line
fn extract_route_path(line: &str) -> Option<String> {
    let start_delimiters = ["'", "\""];
    
    for delim in &start_delimiters {
        if let Some(start_idx) = line.find(delim) {
            if let Some(end_idx) = line[start_idx + 1..].find(delim) {
                return Some(line[start_idx + 1..start_idx + 1 + end_idx].to_string());
            }
        }
    }
    
    None
}

/// Generate a template for a middleware file
fn generate_middleware_template(js_file: &str, _content: &str) -> String {
    let js_filename = Path::new(js_file).file_name().unwrap().to_string_lossy();
    
    let mut template = format!("//! Rust middleware migrated from {}\n\n", js_filename);
    template.push_str("use std::future::{ready, Ready, Future};\n");
    template.push_str("use std::pin::Pin;\n");
    template.push_str("use actix_web::{dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error};\n\n");
    
    // Get middleware name
    let middleware_name = js_filename.replace(".js", "Middleware");
    
    // Create middleware factory struct
    template.push_str(format!("/// {} factory\n", middleware_name).as_str());
    template.push_str(format!("pub struct {} {{\n", middleware_name).as_str());
    template.push_str("    // Add configuration fields here\n");
    template.push_str("}\n\n");
    
    // Add implementation for factory
    template.push_str(format!("impl {} {{\n", middleware_name).as_str());
    template.push_str("    /// Create a new middleware instance\n");
    template.push_str("    pub fn new() -> Self {\n");
    template.push_str("        Self {\n");
    template.push_str("            // Initialize configuration fields\n");
    template.push_str("        }\n");
    template.push_str("    }\n");
    template.push_str("}\n\n");
    
    // Implement Transform trait
    template.push_str(format!("impl<S, B> Transform<S, ServiceRequest> for {}\n", middleware_name).as_str());
    template.push_str("where\n");
    template.push_str("    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,\n");
    template.push_str("    S::Future: 'static,\n");
    template.push_str("    B: 'static,\n");
    template.push_str("{\n");
    template.push_str("    type Response = ServiceResponse<B>;\n");
    template.push_str("    type Error = Error;\n");
    template.push_str("    type Transform = MiddlewareService<S>;\n");
    template.push_str("    type InitError = ();\n");
    template.push_str("    type Future = Ready<Result<Self::Transform, Self::InitError>>;\n\n");
    
    template.push_str("    fn new_transform(&self, service: S) -> Self::Future {\n");
    template.push_str("        ready(Ok(MiddlewareService { service }))\n");
    template.push_str("    }\n");
    template.push_str("}\n\n");
    
    // Create middleware service struct
    template.push_str("pub struct MiddlewareService<S> {\n");
    template.push_str("    service: S,\n");
    template.push_str("}\n\n");
    
    // Implement Service trait for middleware
    template.push_str("impl<S, B> Service<ServiceRequest> for MiddlewareService<S>\n");
    template.push_str("where\n");
    template.push_str("    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,\n");
    template.push_str("    S::Future: 'static,\n");
    template.push_str("    B: 'static,\n");
    template.push_str("{\n");
    template.push_str("    type Response = ServiceResponse<B>;\n");
    template.push_str("    type Error = Error;\n");
    template.push_str("    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;\n\n");
    
    template.push_str("    dev::forward_ready!(service);\n\n");
    
    template.push_str("    fn call(&self, req: ServiceRequest) -> Self::Future {\n");
    template.push_str("        let srv = self.service.clone();\n\n");
    
    template.push_str("        Box::pin(async move {\n");
    template.push_str("            // TODO: Pre-processing logic here\n");
    template.push_str("            println!(\"Request to: {}\", req.path());\n\n");
    
    template.push_str("            let res = srv.call(req).await?;\n\n");
    
    template.push_str("            // TODO: Post-processing logic here\n");
    template.push_str("            println!(\"Response status: {}\", res.status());\n\n");
    
    template.push_str("            Ok(res)\n");
    template.push_str("        })\n");
    template.push_str("    }\n");
    template.push_str("}\n\n");
    
    // Add tests
    template.push_str("#[cfg(test)]\n");
    template.push_str("mod tests {\n");
    template.push_str("    use super::*;\n");
    template.push_str("    use actix_web::{test, web, App, HttpResponse};\n\n");
    
    template.push_str("    #[actix_rt::test]\n");
    template.push_str("    async fn test_middleware() {\n");
    template.push_str("        async fn test_handler() -> HttpResponse {\n");
    template.push_str("            HttpResponse::Ok().body(\"test\")\n");
    template.push_str("        }\n\n");
    
    template.push_str(format!("        let factory = {}::new();\n", middleware_name).as_str());
    template.push_str("        let app = App::new()\n");
    template.push_str(format!("            .wrap(factory)\n").as_str());
    template.push_str("            .route(\"/test\", web::get().to(test_handler));\n\n");
    
    template.push_str("        let app = test::init_service(app).await;\n");
    template.push_str("        let req = test::TestRequest::get().uri(\"/test\").to_request();\n");
    template.push_str("        let resp = test::call_service(&app, req).await;\n\n");
    
    template.push_str("        assert!(resp.status().is_success());\n");
    template.push_str("    }\n");
    template.push_str("}\n");
    
    template
}

/// Generate a generic template for unknown file types
fn generate_generic_template(js_file: &str, content: &str) -> String {
    let js_filename = Path::new(js_file).file_name().unwrap().to_string_lossy();
    let functions = extract_functions(content);
    
    let mut template = format!("//! Rust module migrated from {}\n\n", js_filename);
    template.push_str("// TODO: Determine the purpose of this file and implement appropriately\n\n");
    
    // Add functions
    for func in &functions {
        let snake_case = to_snake_case(func);
        
        template.push_str(format!("/// TODO: Document the {} function\n", func).as_str());
        template.push_str(format!("pub fn {}() {{\n", snake_case).as_str());
        template.push_str("    // TODO: Implement this function\n");
        template.push_str("    println!(\"Function not yet implemented\");\n");
        template.push_str("}\n\n");
    }
    
    template
}

/// Convert a camelCase string to snake_case
fn to_snake_case(camel_case: &str) -> String {
    let mut snake_case = String::new();
    
    for (i, c) in camel_case.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }
        snake_case.push(c.to_lowercase().next().unwrap());
    }
    
    snake_case
}