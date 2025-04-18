use crate::analyzers::modules::enhanced_ruby_model_analyzer::{EnhancedRubyModel, ModelAssociation, ModelValidation};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct RubyToRustModelGenerator {
    pub output_dir: PathBuf,
    pub models_dir: PathBuf,
    pub type_mappings: HashMap<String, String>,
}

impl RubyToRustModelGenerator {
    pub fn new(output_dir: &Path) -> Self {
        let models_dir = output_dir.join("models");
        
        // Create type mappings
        let mut type_mappings = HashMap::new();
        type_mappings.insert("string".to_string(), "String".to_string());
        type_mappings.insert("text".to_string(), "String".to_string());
        type_mappings.insert("integer".to_string(), "i32".to_string());
        type_mappings.insert("bigint".to_string(), "i64".to_string());
        type_mappings.insert("float".to_string(), "f32".to_string());
        type_mappings.insert("decimal".to_string(), "f64".to_string());
        type_mappings.insert("datetime".to_string(), "DateTime<Utc>".to_string());
        type_mappings.insert("timestamp".to_string(), "DateTime<Utc>".to_string());
        type_mappings.insert("date".to_string(), "NaiveDate".to_string());
        type_mappings.insert("time".to_string(), "NaiveTime".to_string());
        type_mappings.insert("boolean".to_string(), "bool".to_string());
        type_mappings.insert("binary".to_string(), "Vec<u8>".to_string());
        type_mappings.insert("json".to_string(), "serde_json::Value".to_string());
        type_mappings.insert("jsonb".to_string(), "serde_json::Value".to_string());
        
        Self {
            output_dir: output_dir.to_path_buf(),
            models_dir,
            type_mappings,
        }
    }
    
    pub fn generate_model(&self, model: &EnhancedRubyModel) -> Result<(), Box<dyn std::error::Error>> {
        // Create models directory if it doesn't exist
        fs::create_dir_all(&self.models_dir)?;
        
        // Generate file name (snake_case)
        let file_name = to_snake_case(&model.name);
        let file_path = self.models_dir.join(format!("{}.rs", file_name));
        
        // Generate Rust code
        let rust_code = self.generate_rust_code(model)?;
        
        // Write to file
        fs::write(file_path, rust_code)?;
        
        Ok(())
    }
    
    fn generate_rust_code(&self, model: &EnhancedRubyModel) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        
        // Add imports
        code.push_str("use serde::{Deserialize, Serialize};\n");
        code.push_str("use chrono::{DateTime, Utc, NaiveDate, NaiveTime};\n");
        code.push_str("use sqlx::FromRow;\n");
        
        // Add any additional imports based on associations
        let mut has_many_associations = false;
        let mut belongs_to_associations = false;
        
        for association in &model.associations {
            match association.association_type.as_str() {
                "has_many" | "has_one" => has_many_associations = true,
                "belongs_to" => belongs_to_associations = true,
                _ => {}
            }
        }
        
        if has_many_associations || belongs_to_associations {
            code.push_str("use std::collections::HashMap;\n");
        }
        
        code.push_str("\n");
        
        // Add struct definition
        code.push_str(&format!("#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]\n"));
        code.push_str(&format!("pub struct {} {{\n", model.name));
        
        // Add fields
        for attribute in &model.attributes {
            let field_name = &attribute.name;
            let field_type = self.map_type(&attribute.attr_type);
            
            code.push_str(&format!("    pub {}: {},\n", field_name, field_type));
        }
        
        // Add fields for belongs_to associations
        for association in &model.associations {
            if association.association_type == "belongs_to" {
                let field_name = format!("{}_id", association.name);
                code.push_str(&format!("    pub {}: Option<i64>,\n", field_name));
            }
        }
        
        // Add timestamps if they exist in the model
        if model.attributes.iter().any(|attr| attr.name == "created_at") {
            code.push_str("    pub created_at: Option<DateTime<Utc>>,\n");
        }
        
        if model.attributes.iter().any(|attr| attr.name == "updated_at") {
            code.push_str("    pub updated_at: Option<DateTime<Utc>>,\n");
        }
        
        code.push_str("}\n\n");
        
        // Add implementation
        code.push_str(&format!("impl {} {{\n", model.name));
        
        // Add new method
        code.push_str("    pub fn new() -> Self {\n");
        code.push_str("        Self {\n");
        
        // Add default values for fields
        for attribute in &model.attributes {
            let field_name = &attribute.name;
            let default_value = self.get_default_value(&attribute.attr_type);
            
            code.push_str(&format!("            {}: {},\n", field_name, default_value));
        }
        
        // Add default values for belongs_to associations
        for association in &model.associations {
            if association.association_type == "belongs_to" {
                let field_name = format!("{}_id", association.name);
                code.push_str(&format!("            {}: None,\n", field_name));
            }
        }
        
        // Add default values for timestamps
        if model.attributes.iter().any(|attr| attr.name == "created_at") {
            code.push_str("            created_at: None,\n");
        }
        
        if model.attributes.iter().any(|attr| attr.name == "updated_at") {
            code.push_str("            updated_at: None,\n");
        }
        
        code.push_str("        }\n");
        code.push_str("    }\n\n");
        
        // Add validation method
        if !model.validations.is_empty() {
            code.push_str("    pub fn validate(&self) -> Result<(), String> {\n");
            
            for validation in &model.validations {
                self.generate_validation_code(&mut code, validation);
            }
            
            code.push_str("        Ok(())\n");
            code.push_str("    }\n\n");
        }
        
        // Add association methods
        for association in &model.associations {
            self.generate_association_method(&mut code, association, &model.name);
        }
        
        code.push_str("}\n");
        
        Ok(code)
    }
    
    fn map_type(&self, ruby_type: &str) -> String {
        if let Some(rust_type) = self.type_mappings.get(ruby_type) {
            rust_type.clone()
        } else {
            // Default to String for unknown types
            "String".to_string()
        }
    }
    
    fn get_default_value(&self, ruby_type: &str) -> String {
        match ruby_type {
            "string" | "text" => "String::new()".to_string(),
            "integer" | "bigint" => "0".to_string(),
            "float" | "decimal" => "0.0".to_string(),
            "datetime" | "timestamp" => "None".to_string(),
            "date" => "None".to_string(),
            "time" => "None".to_string(),
            "boolean" => "false".to_string(),
            "binary" => "Vec::new()".to_string(),
            "json" | "jsonb" => "serde_json::Value::Null".to_string(),
            _ => "String::new()".to_string(),
        }
    }
    
    fn generate_validation_code(&self, code: &mut String, validation: &ModelValidation) {
        for field in &validation.fields {
            match validation.validation_type.as_str() {
                "validates_presence_of" | "presence" => {
                    code.push_str(&format!("        if self.{}.is_empty() {{\n", field));
                    code.push_str(&format!("            return Err(\"{}cannot be empty\".to_string());\n", field));
                    code.push_str("        }\n\n");
                },
                "validates_length_of" | "length" => {
                    if let Some(max) = validation.options.get("maximum") {
                        code.push_str(&format!("        if self.{}.len() > {} {{\n", field, max));
                        code.push_str(&format!("            return Err(\"{} is too long (maximum is {})\".to_string());\n", field, max));
                        code.push_str("        }\n\n");
                    }
                    
                    if let Some(min) = validation.options.get("minimum") {
                        code.push_str(&format!("        if self.{}.len() < {} {{\n", field, min));
                        code.push_str(&format!("            return Err(\"{} is too short (minimum is {})\".to_string());\n", field, min));
                        code.push_str("        }\n\n");
                    }
                },
                "validates_numericality_of" | "numericality" => {
                    if let Some(greater_than) = validation.options.get("greater_than") {
                        code.push_str(&format!("        if self.{} <= {} {{\n", field, greater_than));
                        code.push_str(&format!("            return Err(\"{} must be greater than {}\".to_string());\n", field, greater_than));
                        code.push_str("        }\n\n");
                    }
                    
                    if let Some(less_than) = validation.options.get("less_than") {
                        code.push_str(&format!("        if self.{} >= {} {{\n", field, less_than));
                        code.push_str(&format!("            return Err(\"{} must be less than {}\".to_string());\n", field, less_than));
                        code.push_str("        }\n\n");
                    }
                },
                _ => {}
            }
        }
    }
    
    fn generate_association_method(&self, code: &mut String, association: &ModelAssociation, model_name: &str) {
        match association.association_type.as_str() {
            "has_many" => {
                let association_name = &association.name;
                let association_type = to_pascal_case(association_name);
                
                // Remove trailing 's' for singular type name
                let singular_type = if association_type.ends_with('s') {
                    association_type[..association_type.len() - 1].to_string()
                } else {
                    association_type.clone()
                };
                
                code.push_str(&format!("    pub async fn {}(&self, db: &sqlx::SqlitePool) -> Result<Vec<{}>, sqlx::Error> {{\n", association_name, singular_type));
                code.push_str(&format!("        sqlx::query_as!({})\n", singular_type));
                code.push_str(&format!("            .fetch_all(db)\n"));
                code.push_str(&format!("            .await\n"));
                code.push_str("    }\n\n");
            },
            "belongs_to" => {
                let association_name = &association.name;
                let association_type = to_pascal_case(association_name);
                
                code.push_str(&format!("    pub async fn {}(&self, db: &sqlx::SqlitePool) -> Result<Option<{}>, sqlx::Error> {{\n", association_name, association_type));
                code.push_str(&format!("        if let Some(id) = self.{}_id {{\n", association_name));
                code.push_str(&format!("            sqlx::query_as!({})\n", association_type));
                code.push_str(&format!("                .fetch_optional(db)\n"));
                code.push_str(&format!("                .await\n"));
                code.push_str("        } else {\n");
                code.push_str("            Ok(None)\n");
                code.push_str("        }\n");
                code.push_str("    }\n\n");
            },
            "has_one" => {
                let association_name = &association.name;
                let association_type = to_pascal_case(association_name);
                
                code.push_str(&format!("    pub async fn {}(&self, db: &sqlx::SqlitePool) -> Result<Option<{}>, sqlx::Error> {{\n", association_name, association_type));
                code.push_str(&format!("        sqlx::query_as!({})\n", association_type));
                code.push_str(&format!("            .fetch_optional(db)\n"));
                code.push_str(&format!("            .await\n"));
                code.push_str("    }\n\n");
            },
            _ => {}
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
