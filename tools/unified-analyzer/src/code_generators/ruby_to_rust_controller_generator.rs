use crate::analyzers::modules::enhanced_ruby_controller_analyzer::{EnhancedRubyController, ControllerAction, ControllerFilter};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct RubyToRustControllerGenerator {
    pub output_dir: PathBuf,
    pub services_dir: PathBuf,
    pub handlers_dir: PathBuf,
}

impl RubyToRustControllerGenerator {
    pub fn new(output_dir: &Path) -> Self {
        let services_dir = output_dir.join("services");
        let handlers_dir = output_dir.join("handlers");
        
        Self {
            output_dir: output_dir.to_path_buf(),
            services_dir,
            handlers_dir,
        }
    }
    
    pub fn generate_controller(&self, controller: &EnhancedRubyController) -> Result<(), Box<dyn std::error::Error>> {
        // Create directories if they don't exist
        fs::create_dir_all(&self.services_dir)?;
        fs::create_dir_all(&self.handlers_dir)?;
        
        // Generate service name (remove "Controller" suffix and convert to snake_case)
        let service_name = controller.name.trim_end_matches("Controller");
        let service_file_name = to_snake_case(service_name);
        
        // Generate service file path
        let service_file_path = self.services_dir.join(format!("{}_service.rs", service_file_name));
        
        // Generate handler file path
        let handler_file_path = self.handlers_dir.join(format!("{}_handler.rs", service_file_name));
        
        // Generate Rust service code
        let service_code = self.generate_service_code(controller, service_name)?;
        
        // Generate Rust handler code
        let handler_code = self.generate_handler_code(controller, service_name)?;
        
        // Write to files
        fs::write(service_file_path, service_code)?;
        fs::write(handler_file_path, handler_code)?;
        
        Ok(())
    }
    
    fn generate_service_code(&self, controller: &EnhancedRubyController, service_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        // Add imports
        code.push_str("use sqlx::SqlitePool;\n");
        code.push_str("use anyhow::Result;\n");
        
        // Add model imports based on controller name
        let model_name = if service_name.ends_with('s') {
            // Remove trailing 's' for singular model name
            &service_name[..service_name.len() - 1]
        } else {
            service_name
        };
        
        code.push_str(&format!("use crate::models::{}::{};\n", to_snake_case(model_name), model_name));
        code.push_str("\n");
        
        // Add service struct
        code.push_str(&format!("pub struct {}Service {{\n", service_name));
        code.push_str("    db: SqlitePool,\n");
        code.push_str("}\n\n");
        
        // Add implementation
        code.push_str(&format!("impl {}Service {{\n", service_name));
        
        // Add new method
        code.push_str("    pub fn new(db: SqlitePool) -> Self {\n");
        code.push_str("        Self { db }\n");
        code.push_str("    }\n\n");
        
        // Add methods for each controller action
        for action in &controller.actions {
            self.generate_service_method(&mut code, action, model_name);
        }
        
        code.push_str("}\n");
        
        Ok(code)
    }
    
    fn generate_handler_code(&self, controller: &EnhancedRubyController, service_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        // Add imports
        code.push_str("use axum::{extract::State, Json, response::IntoResponse};\n");
        code.push_str("use axum::extract::Path;\n");
        code.push_str("use axum::http::StatusCode;\n");
        code.push_str("use serde::{Deserialize, Serialize};\n");
        code.push_str("use std::sync::Arc;\n");
        
        // Add service import
        code.push_str(&format!("use crate::services::{}_service::{}Service;\n", to_snake_case(service_name), service_name));
        
        // Add model import
        let model_name = if service_name.ends_with('s') {
            // Remove trailing 's' for singular model name
            &service_name[..service_name.len() - 1]
        } else {
            service_name
        };
        
        code.push_str(&format!("use crate::models::{}::{};\n", to_snake_case(model_name), model_name));
        code.push_str("use crate::AppState;\n\n");
        
        // Add handler functions for each controller action
        for action in &controller.actions {
            self.generate_handler_function(&mut code, action, service_name, model_name);
        }
        
        Ok(code)
    }
    
    fn generate_service_method(&self, code: &mut String, action: &ControllerAction, model_name: &str) {
        match action.name.as_str() {
            "index" => {
                code.push_str(&format!("    pub async fn list_{}s(&self) -> Result<Vec<{}>> {{\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("        let {}s = sqlx::query_as!({})\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("            .fetch_all(&self.db)\n"));
                code.push_str(&format!("            .await?;\n\n"));
                code.push_str(&format!("        Ok({}s)\n", to_snake_case(model_name)));
                code.push_str("    }\n\n");
            },
            "show" => {
                code.push_str(&format!("    pub async fn get_{}(&self, id: i64) -> Result<Option<{}>> {{\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("        let {} = sqlx::query_as!({})\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("            .fetch_optional(&self.db)\n"));
                code.push_str(&format!("            .await?;\n\n"));
                code.push_str(&format!("        Ok({})\n", to_snake_case(model_name)));
                code.push_str("    }\n\n");
            },
            "create" => {
                code.push_str(&format!("    pub async fn create_{}(&self, {}: &{}) -> Result<i64> {{\n", to_snake_case(model_name), to_snake_case(model_name), model_name));
                code.push_str(&format!("        let result = sqlx::query!()\n"));
                code.push_str(&format!("            .execute(&self.db)\n"));
                code.push_str(&format!("            .await?;\n\n"));
                code.push_str("        Ok(result.last_insert_rowid())\n");
                code.push_str("    }\n\n");
            },
            "update" => {
                code.push_str(&format!("    pub async fn update_{}(&self, id: i64, {}: &{}) -> Result<bool> {{\n", to_snake_case(model_name), to_snake_case(model_name), model_name));
                code.push_str(&format!("        let result = sqlx::query!()\n"));
                code.push_str(&format!("            .execute(&self.db)\n"));
                code.push_str(&format!("            .await?;\n\n"));
                code.push_str("        Ok(result.rows_affected() > 0)\n");
                code.push_str("    }\n\n");
            },
            "destroy" => {
                code.push_str(&format!("    pub async fn delete_{}(&self, id: i64) -> Result<bool> {{\n", to_snake_case(model_name)));
                code.push_str(&format!("        let result = sqlx::query!(\"DELETE FROM {} WHERE id = ?\", id)\n", to_snake_case(model_name)));
                code.push_str(&format!("            .execute(&self.db)\n"));
                code.push_str(&format!("            .await?;\n\n"));
                code.push_str("        Ok(result.rows_affected() > 0)\n");
                code.push_str("    }\n\n");
            },
            _ => {
                // Custom action
                code.push_str(&format!("    pub async fn {}(&self) -> Result<()> {{\n", to_snake_case(&action.name)));
                code.push_str("        // TODO: Implement custom action\n");
                code.push_str("        Ok(())\n");
                code.push_str("    }\n\n");
            }
        }
    }
    
    fn generate_handler_function(&self, code: &mut String, action: &ControllerAction, service_name: &str, model_name: &str) {
        match action.name.as_str() {
            "index" => {
                code.push_str(&format!("pub async fn list_{}s(State(state): State<Arc<AppState>>) -> impl IntoResponse {{\n", to_snake_case(model_name)));
                code.push_str(&format!("    let service = {}Service::new(state.db.clone());\n\n", service_name));
                code.push_str(&format!("    match service.list_{}s().await {{\n", to_snake_case(model_name)));
                code.push_str("        Ok(items) => Json(items).into_response(),\n");
                code.push_str("        Err(err) => {\n");
                code.push_str("            eprintln!(\"Error: {}\", err);\n");
                code.push_str("            StatusCode::INTERNAL_SERVER_ERROR.into_response()\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
                code.push_str("}\n\n");
            },
            "show" => {
                code.push_str(&format!("pub async fn get_{}(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> impl IntoResponse {{\n", to_snake_case(model_name)));
                code.push_str(&format!("    let service = {}Service::new(state.db.clone());\n\n", service_name));
                code.push_str(&format!("    match service.get_{}(id).await {{\n", to_snake_case(model_name)));
                code.push_str("        Ok(Some(item)) => Json(item).into_response(),\n");
                code.push_str("        Ok(None) => StatusCode::NOT_FOUND.into_response(),\n");
                code.push_str("        Err(err) => {\n");
                code.push_str("            eprintln!(\"Error: {}\", err);\n");
                code.push_str("            StatusCode::INTERNAL_SERVER_ERROR.into_response()\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
                code.push_str("}\n\n");
            },
            "create" => {
                code.push_str("#[derive(Debug, Deserialize)]\n");
                code.push_str(&format!("pub struct Create{}Request {{\n", model_name));
                code.push_str("    // TODO: Add fields\n");
                code.push_str("}\n\n");
                
                code.push_str(&format!("pub async fn create_{}(State(state): State<Arc<AppState>>, Json(payload): Json<Create{}Request>) -> impl IntoResponse {{\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("    let service = {}Service::new(state.db.clone());\n\n", service_name));
                code.push_str(&format!("    // TODO: Convert payload to {}\n", model_name));
                code.push_str(&format!("    let {} = {}::new();\n\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("    match service.create_{}(&{}).await {{\n", to_snake_case(model_name), to_snake_case(model_name)));
                code.push_str("        Ok(id) => StatusCode::CREATED.into_response(),\n");
                code.push_str("        Err(err) => {\n");
                code.push_str("            eprintln!(\"Error: {}\", err);\n");
                code.push_str("            StatusCode::INTERNAL_SERVER_ERROR.into_response()\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
                code.push_str("}\n\n");
            },
            "update" => {
                code.push_str("#[derive(Debug, Deserialize)]\n");
                code.push_str(&format!("pub struct Update{}Request {{\n", model_name));
                code.push_str("    // TODO: Add fields\n");
                code.push_str("}\n\n");
                
                code.push_str(&format!("pub async fn update_{}(State(state): State<Arc<AppState>>, Path(id): Path<i64>, Json(payload): Json<Update{}Request>) -> impl IntoResponse {{\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("    let service = {}Service::new(state.db.clone());\n\n", service_name));
                code.push_str(&format!("    // TODO: Convert payload to {}\n", model_name));
                code.push_str(&format!("    let {} = {}::new();\n\n", to_snake_case(model_name), model_name));
                code.push_str(&format!("    match service.update_{}(id, &{}).await {{\n", to_snake_case(model_name), to_snake_case(model_name)));
                code.push_str("        Ok(true) => StatusCode::OK.into_response(),\n");
                code.push_str("        Ok(false) => StatusCode::NOT_FOUND.into_response(),\n");
                code.push_str("        Err(err) => {\n");
                code.push_str("            eprintln!(\"Error: {}\", err);\n");
                code.push_str("            StatusCode::INTERNAL_SERVER_ERROR.into_response()\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
                code.push_str("}\n\n");
            },
            "destroy" => {
                code.push_str(&format!("pub async fn delete_{}(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> impl IntoResponse {{\n", to_snake_case(model_name)));
                code.push_str(&format!("    let service = {}Service::new(state.db.clone());\n\n", service_name));
                code.push_str(&format!("    match service.delete_{}(id).await {{\n", to_snake_case(model_name)));
                code.push_str("        Ok(true) => StatusCode::NO_CONTENT.into_response(),\n");
                code.push_str("        Ok(false) => StatusCode::NOT_FOUND.into_response(),\n");
                code.push_str("        Err(err) => {\n");
                code.push_str("            eprintln!(\"Error: {}\", err);\n");
                code.push_str("            StatusCode::INTERNAL_SERVER_ERROR.into_response()\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
                code.push_str("}\n\n");
            },
            _ => {
                // Custom action
                code.push_str(&format!("pub async fn {}(State(state): State<Arc<AppState>>) -> impl IntoResponse {{\n", to_snake_case(&action.name)));
                code.push_str(&format!("    let service = {}Service::new(state.db.clone());\n\n", service_name));
                code.push_str(&format!("    match service.{}().await {{\n", to_snake_case(&action.name)));
                code.push_str("        Ok(_) => StatusCode::OK.into_response(),\n");
                code.push_str("        Err(err) => {\n");
                code.push_str("            eprintln!(\"Error: {}\", err);\n");
                code.push_str("            StatusCode::INTERNAL_SERVER_ERROR.into_response()\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
                code.push_str("}\n\n");
            }
        }
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
