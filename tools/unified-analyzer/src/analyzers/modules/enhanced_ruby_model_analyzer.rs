use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelAttribute {
    pub name: String,
    pub attr_type: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelAssociation {
    pub association_type: String,
    pub name: String,
    pub class_name: Option<String>,
    pub foreign_key: Option<String>,
    pub dependent: Option<String>,
    pub through: Option<String>,
    pub source: Option<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelValidation {
    pub validation_type: String,
    pub fields: Vec<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelCallback {
    pub callback_type: String,
    pub method_name: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelScope {
    pub name: String,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelMethod {
    pub name: String,
    pub is_class_method: bool,
    pub parameters: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnhancedRubyModel {
    pub name: String,
    pub file_path: String,
    pub parent_class: String,
    pub attributes: Vec<ModelAttribute>,
    pub associations: Vec<ModelAssociation>,
    pub validations: Vec<ModelValidation>,
    pub callbacks: Vec<ModelCallback>,
    pub scopes: Vec<ModelScope>,
    pub methods: Vec<ModelMethod>,
    pub concerns: Vec<String>,
    pub table_name: Option<String>,
    pub primary_key: Option<String>,
}

#[derive(Debug, Default)]
pub struct EnhancedRubyModelAnalyzer {
    pub models: HashMap<String, EnhancedRubyModel>,
}

impl EnhancedRubyModelAnalyzer {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Ruby models in directory: {:?}", directory);
        
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "rb" {
                        // Check if it's likely a model file
                        if let Some(file_name) = path.file_name() {
                            let file_name_str = file_name.to_string_lossy();
                            // Models are typically singular and don't end with _controller.rb
                            if !file_name_str.ends_with("_controller.rb") && 
                               !file_name_str.contains("_") {
                                self.analyze_model_file(path)?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn analyze_model_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing model file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        
        // Extract model name and parent class
        lazy_static! {
            static ref MODEL_CLASS_REGEX: Regex = 
                Regex::new(r"class\s+([A-Za-z0-9:]+)\s*<\s*([A-Za-z0-9:]+)").unwrap();
        }
        
        if let Some(captures) = MODEL_CLASS_REGEX.captures(&content) {
            let model_name = captures.get(1).unwrap().as_str().to_string();
            let parent_class = captures.get(2).unwrap().as_str().to_string();
            
            let mut model = EnhancedRubyModel {
                name: model_name.clone(),
                file_path: file_path.to_string_lossy().to_string(),
                parent_class,
                ..Default::default()
            };
            
            // Extract attributes
            self.extract_attributes(&content, &mut model);
            
            // Extract associations
            self.extract_associations(&content, &mut model);
            
            // Extract validations
            self.extract_validations(&content, &mut model);
            
            // Extract callbacks
            self.extract_callbacks(&content, &mut model);
            
            // Extract scopes
            self.extract_scopes(&content, &mut model);
            
            // Extract methods
            self.extract_methods(&content, &mut model);
            
            // Extract concerns
            self.extract_concerns(&content, &mut model);
            
            // Extract table configuration
            self.extract_table_config(&content, &mut model);
            
            // Add model to the collection
            self.models.insert(model_name, model);
        }
        
        Ok(())
    }

    fn extract_attributes(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract attr_accessor, attr_reader, attr_writer
        lazy_static! {
            static ref ATTR_ACCESSOR_REGEX: Regex = 
                Regex::new(r"attr_accessor\s+:([a-z0-9_,\s]+)").unwrap();
            static ref ATTR_READER_REGEX: Regex = 
                Regex::new(r"attr_reader\s+:([a-z0-9_,\s]+)").unwrap();
            static ref ATTR_WRITER_REGEX: Regex = 
                Regex::new(r"attr_writer\s+:([a-z0-9_,\s]+)").unwrap();
        }
        
        // Process attr_accessor
        for captures in ATTR_ACCESSOR_REGEX.captures_iter(content) {
            let attrs_str = captures.get(1).unwrap().as_str();
            for attr in attrs_str.split(',') {
                let attr_name = attr.trim().to_string();
                if !attr_name.is_empty() {
                    model.attributes.push(ModelAttribute {
                        name: attr_name,
                        attr_type: "accessor".to_string(),
                        options: HashMap::new(),
                    });
                }
            }
        }
        
        // Process attr_reader
        for captures in ATTR_READER_REGEX.captures_iter(content) {
            let attrs_str = captures.get(1).unwrap().as_str();
            for attr in attrs_str.split(',') {
                let attr_name = attr.trim().to_string();
                if !attr_name.is_empty() {
                    model.attributes.push(ModelAttribute {
                        name: attr_name,
                        attr_type: "reader".to_string(),
                        options: HashMap::new(),
                    });
                }
            }
        }
        
        // Process attr_writer
        for captures in ATTR_WRITER_REGEX.captures_iter(content) {
            let attrs_str = captures.get(1).unwrap().as_str();
            for attr in attrs_str.split(',') {
                let attr_name = attr.trim().to_string();
                if !attr_name.is_empty() {
                    model.attributes.push(ModelAttribute {
                        name: attr_name,
                        attr_type: "writer".to_string(),
                        options: HashMap::new(),
                    });
                }
            }
        }
    }

    fn extract_associations(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract associations: has_many, has_one, belongs_to, has_and_belongs_to_many
        lazy_static! {
            static ref ASSOCIATION_REGEX: Regex = 
                Regex::new(r"(has_many|has_one|belongs_to|has_and_belongs_to_many)\s+:([a-z0-9_]+)(?:,\s*(.+?))?(?:\n|\r|$)").unwrap();
        }
        
        for captures in ASSOCIATION_REGEX.captures_iter(content) {
            let assoc_type = captures.get(1).unwrap().as_str().to_string();
            let assoc_name = captures.get(2).unwrap().as_str().to_string();
            
            let mut association = ModelAssociation {
                association_type: assoc_type,
                name: assoc_name,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Extract class_name
                if let Some(class_name_match) = Regex::new(r"class_name:\s*['\"](.*?)['\"]").unwrap().captures(options) {
                    association.class_name = Some(class_name_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract foreign_key
                if let Some(foreign_key_match) = Regex::new(r"foreign_key:\s*['\"](.*?)['\"]").unwrap().captures(options) {
                    association.foreign_key = Some(foreign_key_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract dependent
                if let Some(dependent_match) = Regex::new(r"dependent:\s*:([a-z_]+)").unwrap().captures(options) {
                    association.dependent = Some(dependent_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract through
                if let Some(through_match) = Regex::new(r"through:\s*:([a-z_]+)").unwrap().captures(options) {
                    association.through = Some(through_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract source
                if let Some(source_match) = Regex::new(r"source:\s*:([a-z_]+)").unwrap().captures(options) {
                    association.source = Some(source_match.get(1).unwrap().as_str().to_string());
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|(\d+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    association.options.insert(key, value);
                }
            }
            
            model.associations.push(association);
        }
    }

    fn extract_validations(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract validations
        lazy_static! {
            static ref VALIDATION_REGEX: Regex = 
                Regex::new(r"validates(?:_[a-z_]+)?\s+:([a-z0-9_,\s]+)(?:,\s*(.+?))?(?:\n|\r|$)").unwrap();
        }
        
        for captures in VALIDATION_REGEX.captures_iter(content) {
            let fields_str = captures.get(1).unwrap().as_str();
            let validation_type = if captures.get(0).unwrap().as_str().starts_with("validates_") {
                captures.get(0).unwrap().as_str().split("_").nth(1).unwrap_or("validates").to_string()
            } else {
                "validates".to_string()
            };
            
            let mut fields = Vec::new();
            for field in fields_str.split(',') {
                let field_name = field.trim().to_string();
                if !field_name.is_empty() {
                    fields.push(field_name);
                }
            }
            
            let mut validation = ModelValidation {
                validation_type,
                fields,
                options: HashMap::new(),
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|(\d+)|true|false)").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    validation.options.insert(key, value);
                }
            }
            
            model.validations.push(validation);
        }
    }

    fn extract_callbacks(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract callbacks
        lazy_static! {
            static ref CALLBACK_REGEX: Regex = 
                Regex::new(r"(before_save|after_save|before_create|after_create|before_update|after_update|before_destroy|after_destroy|before_validation|after_validation)\s+:([a-z0-9_]+)(?:,\s*(.+?))?(?:\n|\r|$)").unwrap();
        }
        
        for captures in CALLBACK_REGEX.captures_iter(content) {
            let callback_type = captures.get(1).unwrap().as_str().to_string();
            let method_name = captures.get(2).unwrap().as_str().to_string();
            
            let mut callback = ModelCallback {
                callback_type,
                method_name,
                options: HashMap::new(),
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|(\d+)|true|false)").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    callback.options.insert(key, value);
                }
            }
            
            model.callbacks.push(callback);
        }
    }

    fn extract_scopes(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract scopes
        lazy_static! {
            static ref SCOPE_REGEX: Regex = 
                Regex::new(r"scope\s+:([a-z0-9_]+),\s*(?:->(?:\s*\([^)]*\))?\s*\{(.*?)\}|([^{]+))").unwrap();
        }
        
        for captures in SCOPE_REGEX.captures_iter(content) {
            let scope_name = captures.get(1).unwrap().as_str().to_string();
            let query = captures.get(2)
                .or_else(|| captures.get(3))
                .map_or("".to_string(), |m| m.as_str().trim().to_string());
            
            model.scopes.push(ModelScope {
                name: scope_name,
                query,
            });
        }
    }

    fn extract_methods(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract methods
        lazy_static! {
            static ref METHOD_REGEX: Regex = 
                Regex::new(r"(?:def\s+self\.([a-z0-9_?!]+)(\([^)]*\))?(.*?)end)|(?:def\s+([a-z0-9_?!]+)(\([^)]*\))?(.*?)end)").unwrap();
        }
        
        for captures in METHOD_REGEX.captures_iter(content) {
            let is_class_method = captures.get(1).is_some();
            let method_name = if is_class_method {
                captures.get(1).unwrap().as_str().to_string()
            } else {
                captures.get(4).unwrap().as_str().to_string()
            };
            
            let parameters_str = if is_class_method {
                captures.get(2).map_or("".to_string(), |m| m.as_str().to_string())
            } else {
                captures.get(5).map_or("".to_string(), |m| m.as_str().to_string())
            };
            
            let body = if is_class_method {
                captures.get(3).map_or("".to_string(), |m| m.as_str().trim().to_string())
            } else {
                captures.get(6).map_or("".to_string(), |m| m.as_str().trim().to_string())
            };
            
            // Extract parameters
            let mut parameters = Vec::new();
            if !parameters_str.is_empty() {
                let params = parameters_str.trim_start_matches('(').trim_end_matches(')');
                for param in params.split(',') {
                    let param_name = param.trim().to_string();
                    if !param_name.is_empty() {
                        parameters.push(param_name);
                    }
                }
            }
            
            model.methods.push(ModelMethod {
                name: method_name,
                is_class_method,
                parameters,
                body,
            });
        }
    }

    fn extract_concerns(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract concerns
        lazy_static! {
            static ref CONCERN_REGEX: Regex = 
                Regex::new(r"include\s+([A-Za-z0-9:]+)").unwrap();
        }
        
        for captures in CONCERN_REGEX.captures_iter(content) {
            let concern = captures.get(1).unwrap().as_str().to_string();
            model.concerns.push(concern);
        }
    }

    fn extract_table_config(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract table_name
        lazy_static! {
            static ref TABLE_NAME_REGEX: Regex = 
                Regex::new(r"self\.table_name\s*=\s*['\"]([^'\"]+)['\"]").unwrap();
        }
        
        if let Some(captures) = TABLE_NAME_REGEX.captures(content) {
            model.table_name = Some(captures.get(1).unwrap().as_str().to_string());
        }
        
        // Extract primary_key
        lazy_static! {
            static ref PRIMARY_KEY_REGEX: Regex = 
                Regex::new(r"self\.primary_key\s*=\s*['\"]([^'\"]+)['\"]").unwrap();
        }
        
        if let Some(captures) = PRIMARY_KEY_REGEX.captures(content) {
            model.primary_key = Some(captures.get(1).unwrap().as_str().to_string());
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.models)
    }
}
