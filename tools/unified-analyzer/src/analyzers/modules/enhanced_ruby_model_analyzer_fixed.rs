use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct EnhancedRubyAssociation {
    pub association_type: String,
    pub name: String,
    pub class_name: Option<String>,
    pub foreign_key: Option<String>,
    pub dependent: Option<String>,
    pub through: Option<String>,
    pub source: Option<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyValidation {
    pub validation_type: String,
    pub fields: Vec<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyCallback {
    pub callback_type: String,
    pub method: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyScope {
    pub name: String,
    pub lambda: Option<String>,
    pub query: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyMethod {
    pub name: String,
    pub is_class_method: bool,
    pub parameters: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyModel {
    pub name: String,
    pub parent_class: Option<String>,
    pub file_path: String,
    pub attributes: Vec<String>,
    pub associations: Vec<EnhancedRubyAssociation>,
    pub validations: Vec<EnhancedRubyValidation>,
    pub callbacks: Vec<EnhancedRubyCallback>,
    pub scopes: Vec<EnhancedRubyScope>,
    pub methods: Vec<EnhancedRubyMethod>,
    pub included_modules: Vec<String>,
    pub table_name: Option<String>,
    pub primary_key: Option<String>,
}

pub struct EnhancedRubyModelAnalyzer {
    pub models: HashMap<String, EnhancedRubyModel>,
}

impl EnhancedRubyModelAnalyzer {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }
    
    pub fn analyze_directory(&mut self, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(format!("Directory does not exist: {:?}", dir_path).into());
        }
        
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.analyze_directory(&path)?;
            } else if let Some(extension) = path.extension() {
                if extension == "rb" {
                    self.analyze_file(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        
        // Check if this is a model file
        if self.is_model_file(&content) {
            let model = self.parse_model(&content, file_path)?;
            self.models.insert(model.name.clone(), model);
        }
        
        Ok(())
    }
    
    fn is_model_file(&self, content: &str) -> bool {
        // Check if the file contains a class that inherits from ActiveRecord::Base or ApplicationRecord
        lazy_static! {
            static ref CLASS_REGEX: Regex =
                Regex::new(r#"class\s+([A-Za-z0-9:]+)\s*<\s*([A-Za-z0-9:]+)"#).unwrap();
        }
        
        if let Some(captures) = CLASS_REGEX.captures(content) {
            if let Some(parent) = captures.get(2) {
                let parent_class = parent.as_str();
                return parent_class == "ActiveRecord::Base" || parent_class == "ApplicationRecord";
            }
        }
        
        false
    }
    
    fn parse_model(&self, content: &str, file_path: &Path) -> Result<EnhancedRubyModel, Box<dyn std::error::Error>> {
        // Extract class name and parent class
        lazy_static! {
            static ref CLASS_REGEX: Regex =
                Regex::new(r#"class\s+([A-Za-z0-9:]+)\s*<\s*([A-Za-z0-9:]+)"#).unwrap();
        }
        
        let (name, parent_class) = if let Some(captures) = CLASS_REGEX.captures(content) {
            (
                captures.get(1).unwrap().as_str().to_string(),
                Some(captures.get(2).unwrap().as_str().to_string()),
            )
        } else {
            return Err("Could not extract class name and parent class".into());
        };
        
        // Create model
        let mut model = EnhancedRubyModel {
            name,
            parent_class,
            file_path: file_path.to_string_lossy().to_string(),
            attributes: Vec::new(),
            associations: Vec::new(),
            validations: Vec::new(),
            callbacks: Vec::new(),
            scopes: Vec::new(),
            methods: Vec::new(),
            included_modules: Vec::new(),
            table_name: None,
            primary_key: None,
        };
        
        // Extract attributes
        self.extract_attributes(content, &mut model);
        
        // Extract associations
        self.extract_associations(content, &mut model);
        
        // Extract validations
        self.extract_validations(content, &mut model);
        
        // Extract callbacks
        self.extract_callbacks(content, &mut model);
        
        // Extract scopes
        self.extract_scopes(content, &mut model);
        
        // Extract methods
        self.extract_methods(content, &mut model);
        
        // Extract included modules
        self.extract_included_modules(content, &mut model);
        
        // Extract table config
        self.extract_table_config(content, &mut model);
        
        Ok(model)
    }
    
    fn extract_attributes(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract attributes from attr_accessor, attr_reader, attr_writer
        lazy_static! {
            static ref ATTR_ACCESSOR_REGEX: Regex =
                Regex::new(r#"attr_accessor\s+:([a-z0-9_,\s]+)"#).unwrap();
            static ref ATTR_READER_REGEX: Regex =
                Regex::new(r#"attr_reader\s+:([a-z0-9_,\s]+)"#).unwrap();
            static ref ATTR_WRITER_REGEX: Regex =
                Regex::new(r#"attr_writer\s+:([a-z0-9_,\s]+)"#).unwrap();
        }
        
        // Extract attr_accessor
        for captures in ATTR_ACCESSOR_REGEX.captures_iter(content) {
            let attrs = captures.get(1).unwrap().as_str();
            for attr in attrs.split(',') {
                let attr = attr.trim().trim_start_matches(':');
                model.attributes.push(attr.to_string());
            }
        }
        
        // Extract attr_reader
        for captures in ATTR_READER_REGEX.captures_iter(content) {
            let attrs = captures.get(1).unwrap().as_str();
            for attr in attrs.split(',') {
                let attr = attr.trim().trim_start_matches(':');
                model.attributes.push(attr.to_string());
            }
        }
        
        // Extract attr_writer
        for captures in ATTR_WRITER_REGEX.captures_iter(content) {
            let attrs = captures.get(1).unwrap().as_str();
            for attr in attrs.split(',') {
                let attr = attr.trim().trim_start_matches(':');
                model.attributes.push(attr.to_string());
            }
        }
    }
    
    fn extract_associations(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract associations (has_many, has_one, belongs_to, has_and_belongs_to_many)
        lazy_static! {
            static ref ASSOCIATION_REGEX: Regex =
                Regex::new(r#"(has_many|has_one|belongs_to|has_and_belongs_to_many)\s+:([a-z0-9_]+)(?:,\s*(.+?))?(?:\n|\r|$)"#).unwrap();
        }
        
        for captures in ASSOCIATION_REGEX.captures_iter(content) {
            let association_type = captures.get(1).unwrap().as_str().to_string();
            let name = captures.get(2).unwrap().as_str().to_string();
            
            let mut association = EnhancedRubyAssociation {
                association_type,
                name,
                class_name: None,
                foreign_key: None,
                dependent: None,
                through: None,
                source: None,
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();

                // Extract class_name
                lazy_static! {
                    static ref CLASS_NAME_REGEX: Regex =
                        Regex::new(r#"class_name:\s*['"]([^"']+)['"]"#).unwrap();
                }
                if let Some(class_name_match) = CLASS_NAME_REGEX.captures(options) {
                    association.class_name = Some(class_name_match.get(1).unwrap().as_str().to_string());
                }

                // Extract foreign_key
                lazy_static! {
                    static ref FOREIGN_KEY_REGEX: Regex =
                        Regex::new(r#"foreign_key:\s*['"]([^"']+)['"]"#).unwrap();
                }
                if let Some(foreign_key_match) = FOREIGN_KEY_REGEX.captures(options) {
                    association.foreign_key = Some(foreign_key_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract dependent
                lazy_static! {
                    static ref DEPENDENT_REGEX: Regex =
                        Regex::new(r#"dependent:\s*:([a-z_]+)"#).unwrap();
                }
                if let Some(dependent_match) = DEPENDENT_REGEX.captures(options) {
                    association.dependent = Some(dependent_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract through
                lazy_static! {
                    static ref THROUGH_REGEX: Regex =
                        Regex::new(r#"through:\s*:([a-z_]+)"#).unwrap();
                }
                if let Some(through_match) = THROUGH_REGEX.captures(options) {
                    association.through = Some(through_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract source
                lazy_static! {
                    static ref SOURCE_REGEX: Regex =
                        Regex::new(r#"source:\s*:([a-z_]+)"#).unwrap();
                }
                if let Some(source_match) = SOURCE_REGEX.captures(options) {
                    association.source = Some(source_match.get(1).unwrap().as_str().to_string());
                }
                
                // Store all options in the options map
                lazy_static! {
                    static ref OPTION_REGEX: Regex =
                        Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^"']+)['"]|(\d+))"#).unwrap();
                }
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
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
                Regex::new(r#"validates(?:_[a-z_]+)?\s+:([a-z0-9_,\s]+)(?:,\s*(.+?))?(?:\n|\r|$)"#).unwrap();
        }
        
        for captures in VALIDATION_REGEX.captures_iter(content) {
            let validation_type = captures.get(0).unwrap().as_str().to_string();
            let fields_str = captures.get(1).unwrap().as_str();
            
            let mut fields = Vec::new();
            for field in fields_str.split(',') {
                fields.push(field.trim().to_string());
            }
            
            let mut validation = EnhancedRubyValidation {
                validation_type,
                fields,
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Store all options in the options map
                lazy_static! {
                    static ref OPTION_REGEX: Regex =
                        Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^"']+)['"]|(\d+)|true|false)"#).unwrap();
                }
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
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
                Regex::new(r#"(before_save|after_save|before_create|after_create|before_update|after_update|before_destroy|after_destroy|before_validation|after_validation)\s+:([a-z0-9_]+)(?:,\s*(.+?))?(?:\n|\r|$)"#).unwrap();
        }
        
        for captures in CALLBACK_REGEX.captures_iter(content) {
            let callback_type = captures.get(1).unwrap().as_str().to_string();
            let method = captures.get(2).unwrap().as_str().to_string();
            
            let mut callback = EnhancedRubyCallback {
                callback_type,
                method,
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Store all options in the options map
                lazy_static! {
                    static ref OPTION_REGEX: Regex =
                        Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^"']+)['"]|(\d+)|true|false)"#).unwrap();
                }
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
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
                Regex::new(r#"scope\s+:([a-z0-9_]+),\s*(?:->(?:\s*\([^)]*\))?\s*\{(.*?)\}|([^{]+))"#).unwrap();
        }
        
        for captures in SCOPE_REGEX.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let lambda = captures.get(2).map(|m| m.as_str().to_string());
            let query = captures.get(3).map(|m| m.as_str().to_string());
            
            let scope = EnhancedRubyScope {
                name,
                lambda,
                query,
            };
            
            model.scopes.push(scope);
        }
    }
    
    fn extract_methods(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract methods
        lazy_static! {
            static ref METHOD_REGEX: Regex =
                Regex::new(r#"(?:def\s+self\.([a-z0-9_?!]+)(\([^)]*\))?(.*?)end)|(?:def\s+([a-z0-9_?!]+)(\([^)]*\))?(.*?)end)"#).unwrap();
        }
        
        for captures in METHOD_REGEX.captures_iter(content) {
            let is_class_method = captures.get(1).is_some();
            
            let name = if is_class_method {
                captures.get(1).unwrap().as_str().to_string()
            } else {
                captures.get(4).unwrap().as_str().to_string()
            };
            
            let parameters = if is_class_method {
                captures.get(2).map(|m| m.as_str().to_string())
            } else {
                captures.get(5).map(|m| m.as_str().to_string())
            };
            
            let body = if is_class_method {
                captures.get(3).map(|m| m.as_str().to_string())
            } else {
                captures.get(6).map(|m| m.as_str().to_string())
            };
            
            let method = EnhancedRubyMethod {
                name,
                is_class_method,
                parameters,
                body,
            };
            
            model.methods.push(method);
        }
    }
    
    fn extract_included_modules(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract included modules
        lazy_static! {
            static ref INCLUDE_REGEX: Regex =
                Regex::new(r#"include\s+([A-Za-z0-9:]+)"#).unwrap();
        }
        
        for captures in INCLUDE_REGEX.captures_iter(content) {
            let module_name = captures.get(1).unwrap().as_str().to_string();
            model.included_modules.push(module_name);
        }
    }
    
    fn extract_table_config(&self, content: &str, model: &mut EnhancedRubyModel) {
        // Extract table_name
        lazy_static! {
            static ref TABLE_NAME_REGEX: Regex =
                Regex::new(r#"self\.table_name\s*=\s*['"]([^"']+)['"]"#).unwrap();
        }
        
        if let Some(captures) = TABLE_NAME_REGEX.captures(content) {
            model.table_name = Some(captures.get(1).unwrap().as_str().to_string());
        }
        
        // Extract primary_key
        lazy_static! {
            static ref PRIMARY_KEY_REGEX: Regex =
                Regex::new(r#"self\.primary_key\s*=\s*['"]([^"']+)['"]"#).unwrap();
        }
        
        if let Some(captures) = PRIMARY_KEY_REGEX.captures(content) {
            model.primary_key = Some(captures.get(1).unwrap().as_str().to_string());
        }
    }
}
