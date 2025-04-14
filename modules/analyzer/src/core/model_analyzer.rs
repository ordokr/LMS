use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;

/// Analyzer for data models in the codebase
pub struct ModelAnalyzer {
    /// Base directory for analysis
    base_dir: PathBuf,
    
    /// Regex patterns for detecting models
    model_patterns: HashMap<String, Regex>,
    
    /// Regex patterns for detecting model fields
    field_patterns: HashMap<String, Regex>,
    
    /// Regex patterns for detecting model relationships
    relationship_patterns: HashMap<String, Regex>,
    
    /// Directories to exclude from analysis
    exclude_dirs: Vec<String>,
}

/// Model information
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Model name
    pub name: String,
    
    /// File path
    pub file_path: String,
    
    /// Source system (Canvas, Discourse, or Native)
    pub source_system: String,
    
    /// Model fields
    pub fields: Vec<ModelField>,
    
    /// Model relationships
    pub relationships: Vec<ModelRelationship>,
}

/// Model field
#[derive(Debug, Clone)]
pub struct ModelField {
    /// Field name
    pub name: String,
    
    /// Field type
    pub field_type: String,
    
    /// Whether the field is required
    pub required: bool,
    
    /// Field description
    pub description: Option<String>,
}

/// Model relationship
#[derive(Debug, Clone)]
pub struct ModelRelationship {
    /// Relationship type (OneToOne, OneToMany, ManyToOne, ManyToMany)
    pub relationship_type: String,
    
    /// Related model
    pub related_model: String,
    
    /// Relationship description
    pub description: Option<String>,
}

/// Model metrics
#[derive(Debug, Clone)]
pub struct ModelMetrics {
    /// Total number of models
    pub total_models: usize,
    
    /// Number of Canvas models
    pub canvas_models: usize,
    
    /// Number of Discourse models
    pub discourse_models: usize,
    
    /// Number of Native models
    pub native_models: usize,
    
    /// Models by source system
    pub models_by_source: HashMap<String, Vec<ModelInfo>>,
    
    /// Models by file
    pub models_by_file: HashMap<String, Vec<ModelInfo>>,
    
    /// Model relationships
    pub relationships: Vec<ModelRelationship>,
}

impl ModelAnalyzer {
    /// Create a new model analyzer
    pub fn new(base_dir: PathBuf) -> Self {
        let mut model_patterns = HashMap::new();
        let mut field_patterns = HashMap::new();
        let mut relationship_patterns = HashMap::new();
        
        // Add regex patterns for detecting models in Rust
        model_patterns.insert(
            "rust_struct".to_string(),
            Regex::new(r"(?m)^(?:#\[derive\([^\)]*\)\s*)?struct\s+([A-Za-z0-9_]+)").unwrap()
        );
        model_patterns.insert(
            "rust_enum".to_string(),
            Regex::new(r"(?m)^(?:#\[derive\([^\)]*\)\s*)?enum\s+([A-Za-z0-9_]+)").unwrap()
        );
        
        // Add regex patterns for detecting models in Haskell
        model_patterns.insert(
            "haskell_data".to_string(),
            Regex::new(r"(?m)^data\s+([A-Za-z0-9_]+)").unwrap()
        );
        model_patterns.insert(
            "haskell_newtype".to_string(),
            Regex::new(r"(?m)^newtype\s+([A-Za-z0-9_]+)").unwrap()
        );
        
        // Add regex patterns for detecting model fields in Rust
        field_patterns.insert(
            "rust_field".to_string(),
            Regex::new(r"(?m)^\s+(?:#\[.*\]\s*)?pub\s+([a-z0-9_]+):\s+([A-Za-z0-9_<>:,\s]+)").unwrap()
        );
        
        // Add regex patterns for detecting model fields in Haskell
        field_patterns.insert(
            "haskell_field".to_string(),
            Regex::new(r"(?m)^\s+([a-z0-9_]+)\s+::\s+([A-Za-z0-9_\s]+)").unwrap()
        );
        
        // Add regex patterns for detecting model relationships in Rust
        relationship_patterns.insert(
            "rust_one_to_one".to_string(),
            Regex::new(r"(?m)^\s+(?:#\[.*\]\s*)?pub\s+([a-z0-9_]+):\s+([A-Za-z0-9_]+)").unwrap()
        );
        relationship_patterns.insert(
            "rust_one_to_many".to_string(),
            Regex::new(r"(?m)^\s+(?:#\[.*\]\s*)?pub\s+([a-z0-9_]+):\s+Vec<([A-Za-z0-9_]+)>").unwrap()
        );
        relationship_patterns.insert(
            "rust_many_to_many".to_string(),
            Regex::new(r"(?m)^\s+(?:#\[.*\]\s*)?pub\s+([a-z0-9_]+):\s+Vec<([A-Za-z0-9_]+)>").unwrap()
        );
        
        // Add regex patterns for detecting model relationships in Haskell
        relationship_patterns.insert(
            "haskell_one_to_one".to_string(),
            Regex::new(r"(?m)^\s+([a-z0-9_]+)\s+::\s+([A-Za-z0-9_]+)").unwrap()
        );
        relationship_patterns.insert(
            "haskell_one_to_many".to_string(),
            Regex::new(r"(?m)^\s+([a-z0-9_]+)\s+::\s+\[([A-Za-z0-9_]+)\]").unwrap()
        );
        relationship_patterns.insert(
            "haskell_many_to_many".to_string(),
            Regex::new(r"(?m)^\s+([a-z0-9_]+)\s+::\s+\[([A-Za-z0-9_]+)\]").unwrap()
        );
        
        // Add directories to exclude
        let exclude_dirs = vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "build-output".to_string(),
        ];
        
        Self {
            base_dir,
            model_patterns,
            field_patterns,
            relationship_patterns,
            exclude_dirs,
        }
    }
    
    /// Analyze a file for models
    pub fn analyze_file(&self, file_path: &Path) -> Result<Vec<ModelInfo>, String> {
        // Skip files in excluded directories
        for exclude_dir in &self.exclude_dirs {
            if file_path.to_string_lossy().contains(exclude_dir) {
                return Ok(Vec::new());
            }
        }
        
        // Skip non-Rust and non-Haskell files
        let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        if extension != "rs" && extension != "hs" {
            return Ok(Vec::new());
        }
        
        // Read file content
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
        
        // Create relative path
        let relative_path = file_path.strip_prefix(&self.base_dir)
            .map_err(|_| "Failed to create relative path".to_string())?
            .to_string_lossy()
            .to_string();
        
        let mut models = Vec::new();
        
        // Determine source system based on file path
        let source_system = if relative_path.contains("canvas") {
            "Canvas".to_string()
        } else if relative_path.contains("discourse") {
            "Discourse".to_string()
        } else {
            "Native".to_string()
        };
        
        // Find models in Rust files
        if extension == "rs" {
            // Find structs
            for captures in self.model_patterns.get("rust_struct").unwrap().captures_iter(&content) {
                let model_name = captures.get(1).unwrap().as_str().to_string();
                
                // Find fields
                let mut fields = Vec::new();
                for field_captures in self.field_patterns.get("rust_field").unwrap().captures_iter(&content) {
                    let field_name = field_captures.get(1).unwrap().as_str().to_string();
                    let field_type = field_captures.get(2).unwrap().as_str().to_string();
                    
                    fields.push(ModelField {
                        name: field_name,
                        field_type: field_type.trim().to_string(),
                        required: !field_type.contains("Option<"),
                        description: None,
                    });
                }
                
                // Find relationships
                let mut relationships = Vec::new();
                
                // One-to-one relationships
                for rel_captures in self.relationship_patterns.get("rust_one_to_one").unwrap().captures_iter(&content) {
                    let field_name = rel_captures.get(1).unwrap().as_str().to_string();
                    let related_model = rel_captures.get(2).unwrap().as_str().to_string();
                    
                    // Skip primitive types
                    if is_primitive_type(&related_model) {
                        continue;
                    }
                    
                    relationships.push(ModelRelationship {
                        relationship_type: "OneToOne".to_string(),
                        related_model,
                        description: None,
                    });
                }
                
                // One-to-many relationships
                for rel_captures in self.relationship_patterns.get("rust_one_to_many").unwrap().captures_iter(&content) {
                    let field_name = rel_captures.get(1).unwrap().as_str().to_string();
                    let related_model = rel_captures.get(2).unwrap().as_str().to_string();
                    
                    relationships.push(ModelRelationship {
                        relationship_type: "OneToMany".to_string(),
                        related_model,
                        description: None,
                    });
                }
                
                models.push(ModelInfo {
                    name: model_name,
                    file_path: relative_path.clone(),
                    source_system: source_system.clone(),
                    fields,
                    relationships,
                });
            }
            
            // Find enums
            for captures in self.model_patterns.get("rust_enum").unwrap().captures_iter(&content) {
                let model_name = captures.get(1).unwrap().as_str().to_string();
                
                models.push(ModelInfo {
                    name: model_name,
                    file_path: relative_path.clone(),
                    source_system: source_system.clone(),
                    fields: Vec::new(),
                    relationships: Vec::new(),
                });
            }
        }
        
        // Find models in Haskell files
        if extension == "hs" {
            // Find data types
            for captures in self.model_patterns.get("haskell_data").unwrap().captures_iter(&content) {
                let model_name = captures.get(1).unwrap().as_str().to_string();
                
                // Find fields
                let mut fields = Vec::new();
                for field_captures in self.field_patterns.get("haskell_field").unwrap().captures_iter(&content) {
                    let field_name = field_captures.get(1).unwrap().as_str().to_string();
                    let field_type = field_captures.get(2).unwrap().as_str().to_string();
                    
                    fields.push(ModelField {
                        name: field_name,
                        field_type: field_type.trim().to_string(),
                        required: !field_type.contains("Maybe"),
                        description: None,
                    });
                }
                
                // Find relationships
                let mut relationships = Vec::new();
                
                // One-to-one relationships
                for rel_captures in self.relationship_patterns.get("haskell_one_to_one").unwrap().captures_iter(&content) {
                    let field_name = rel_captures.get(1).unwrap().as_str().to_string();
                    let related_model = rel_captures.get(2).unwrap().as_str().to_string();
                    
                    // Skip primitive types
                    if is_primitive_type(&related_model) {
                        continue;
                    }
                    
                    relationships.push(ModelRelationship {
                        relationship_type: "OneToOne".to_string(),
                        related_model,
                        description: None,
                    });
                }
                
                // One-to-many relationships
                for rel_captures in self.relationship_patterns.get("haskell_one_to_many").unwrap().captures_iter(&content) {
                    let field_name = rel_captures.get(1).unwrap().as_str().to_string();
                    let related_model = rel_captures.get(2).unwrap().as_str().to_string();
                    
                    relationships.push(ModelRelationship {
                        relationship_type: "OneToMany".to_string(),
                        related_model,
                        description: None,
                    });
                }
                
                models.push(ModelInfo {
                    name: model_name,
                    file_path: relative_path.clone(),
                    source_system: source_system.clone(),
                    fields,
                    relationships,
                });
            }
            
            // Find newtypes
            for captures in self.model_patterns.get("haskell_newtype").unwrap().captures_iter(&content) {
                let model_name = captures.get(1).unwrap().as_str().to_string();
                
                models.push(ModelInfo {
                    name: model_name,
                    file_path: relative_path.clone(),
                    source_system: source_system.clone(),
                    fields: Vec::new(),
                    relationships: Vec::new(),
                });
            }
        }
        
        Ok(models)
    }
    
    /// Analyze a directory for models
    pub fn analyze_directory(&self, dir_path: &Path, models: &mut Vec<ModelInfo>) -> Result<(), String> {
        // Skip excluded directories
        for exclude_dir in &self.exclude_dirs {
            if dir_path.to_string_lossy().contains(exclude_dir) {
                return Ok(());
            }
        }
        
        // Walk through the directory
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            
            // Analyze the file
            let mut file_models = self.analyze_file(path)?;
            models.append(&mut file_models);
        }
        
        Ok(())
    }
    
    /// Analyze the entire codebase for models
    pub fn analyze_codebase(&self) -> Result<ModelMetrics, String> {
        let mut models = Vec::new();
        
        // Analyze the base directory
        self.analyze_directory(&self.base_dir, &mut models)?;
        
        // Count models by source system
        let mut canvas_models = 0;
        let mut discourse_models = 0;
        let mut native_models = 0;
        
        let mut models_by_source = HashMap::new();
        let mut models_by_file = HashMap::new();
        
        for model in &models {
            match model.source_system.as_str() {
                "Canvas" => canvas_models += 1,
                "Discourse" => discourse_models += 1,
                "Native" => native_models += 1,
                _ => {}
            }
            
            models_by_source.entry(model.source_system.clone())
                .or_insert_with(Vec::new)
                .push(model.clone());
            
            models_by_file.entry(model.file_path.clone())
                .or_insert_with(Vec::new)
                .push(model.clone());
        }
        
        // Collect all relationships
        let mut relationships = Vec::new();
        for model in &models {
            for relationship in &model.relationships {
                relationships.push(relationship.clone());
            }
        }
        
        Ok(ModelMetrics {
            total_models: models.len(),
            canvas_models,
            discourse_models,
            native_models,
            models_by_source,
            models_by_file,
            relationships,
        })
    }
    
    /// Generate a model report
    pub fn generate_report(&self) -> Result<String, String> {
        // Analyze the codebase
        let metrics = self.analyze_codebase()?;
        
        // Generate the report
        let mut report = String::new();
        
        // Header
        report.push_str("# Data Models Report\n\n");
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("**Total Models: {}**\n\n", metrics.total_models));
        report.push_str("| Source System | Count |\n");
        report.push_str("|--------------|-------|\n");
        report.push_str(&format!("| Canvas | {} |\n", metrics.canvas_models));
        report.push_str(&format!("| Discourse | {} |\n", metrics.discourse_models));
        report.push_str(&format!("| Native | {} |\n\n", metrics.native_models));
        
        // Models by Source System
        for (source, models) in &metrics.models_by_source {
            report.push_str(&format!("## {} Models\n\n", source));
            
            if !models.is_empty() {
                report.push_str("| Model | File | Fields | Relationships |\n");
                report.push_str("|-------|------|--------|---------------|\n");
                
                for model in models {
                    report.push_str(&format!("| {} | {} | {} | {} |\n",
                        model.name,
                        model.file_path,
                        model.fields.len(),
                        model.relationships.len()));
                }
            } else {
                report.push_str("No models found.\n");
            }
            
            report.push_str("\n");
        }
        
        // Model Details
        report.push_str("## Model Details\n\n");
        
        for (source, models) in &metrics.models_by_source {
            for model in models {
                report.push_str(&format!("### {}\n\n", model.name));
                report.push_str(&format!("**Source System:** {}\n\n", source));
                report.push_str(&format!("**File:** {}\n\n", model.file_path));
                
                // Fields
                report.push_str("#### Fields\n\n");
                
                if !model.fields.is_empty() {
                    report.push_str("| Name | Type | Required |\n");
                    report.push_str("|------|------|----------|\n");
                    
                    for field in &model.fields {
                        report.push_str(&format!("| {} | {} | {} |\n",
                            field.name,
                            field.field_type,
                            if field.required { "Yes" } else { "No" }));
                    }
                } else {
                    report.push_str("No fields found.\n");
                }
                
                report.push_str("\n");
                
                // Relationships
                report.push_str("#### Relationships\n\n");
                
                if !model.relationships.is_empty() {
                    report.push_str("| Type | Related Model |\n");
                    report.push_str("|------|---------------|\n");
                    
                    for relationship in &model.relationships {
                        report.push_str(&format!("| {} | {} |\n",
                            relationship.relationship_type,
                            relationship.related_model));
                    }
                } else {
                    report.push_str("No relationships found.\n");
                }
                
                report.push_str("\n");
            }
        }
        
        // Relationship Graph
        report.push_str("## Relationship Graph\n\n");
        report.push_str("```mermaid\nclassDiagram\n");
        
        // Add classes
        for (_, models) in &metrics.models_by_source {
            for model in models {
                report.push_str(&format!("    class {} {{\n", model.name));
                
                for field in &model.fields {
                    report.push_str(&format!("        {} {}\n",
                        field.field_type,
                        field.name));
                }
                
                report.push_str("    }\n");
            }
        }
        
        // Add relationships
        for (_, models) in &metrics.models_by_source {
            for model in models {
                for relationship in &model.relationships {
                    let arrow = match relationship.relationship_type.as_str() {
                        "OneToOne" => "--",
                        "OneToMany" => "-->",
                        "ManyToOne" => "<--",
                        "ManyToMany" => "<-->",
                        _ => "--",
                    };
                    
                    report.push_str(&format!("    {} {} {}\n",
                        model.name,
                        arrow,
                        relationship.related_model));
                }
            }
        }
        
        report.push_str("```\n");
        
        Ok(report)
    }
}

/// Check if a type is a primitive type
fn is_primitive_type(type_name: &str) -> bool {
    let primitive_types = [
        "i8", "i16", "i32", "i64", "i128", "isize",
        "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64",
        "bool", "char", "str", "String",
        "Int", "Integer", "Float", "Double", "Bool", "Char", "String",
    ];
    
    primitive_types.contains(&type_name)
}
