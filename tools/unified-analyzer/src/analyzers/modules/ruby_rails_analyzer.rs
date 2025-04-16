rust
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    io,
    path::{Path, PathBuf},
};
use lazy_static::lazy_static;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    pub verb: String,
    pub path: String,
    pub controller: String,
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Callback {
    pub model: String,
    pub r#type: String,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hook {
    pub name: String,
    pub r#type: String,
    pub target: String,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseSchema {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: String,
    pub r#type: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RailsModel {
    pub name: String,
    pub associations: Vec<String>,
    pub validations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RailsController {
    pub name: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RubyRailsAnalyzer {
    pub models: HashMap<String, RailsModel>,
    pub controllers: HashMap<String, RailsController>,
    pub routes: Vec<Route>,
    pub callbacks: Vec<Callback>,
    pub hooks: Vec<Hook>,
    pub database_schemas: HashMap<String, DatabaseSchema>,
}

impl RubyRailsAnalyzer {
    pub fn analyze(&self, project_path: &str) -> Result<String, RubyRailsError> {
        let mut analyzer = RubyRailsAnalyzer::default();

        analyzer.extract_routes(project_path)?;
        analyzer.extract_database_schema(project_path)?;
        analyzer.extract_callbacks_and_hooks(project_path)?;


        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".rb") {
                            let content = fs::read_to_string(path)?;
                            if file_name_str.ends_with("_controller.rb") {
                                analyzer.extract_controller(file_name_str, &content)?;
                            } else if file_name_str.ends_with(".rb") {
                                analyzer.extract_model(file_name_str, &content)?;
                            }
                        }
                    }
                }
            }
        }
        Ok(serde_json::to_string_pretty(&analyzer)?)
    }

    fn extract_routes(&mut self, project_path: &str) -> Result<(), RubyRailsError> {
        let routes_path = Path::new(project_path).join("config").join("routes.rb");
        if !routes_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&routes_path)?;
        lazy_static! {
            static ref ROUTE_REGEX: Regex =
                Regex::new(r#"^\s*(get|post|put|patch|delete)\s+(['"].*?['"]),\s+to:\s+(['"].*?['"])"#).unwrap();
        }

        for line in content.lines() {
            if let Some(caps) = ROUTE_REGEX.captures(line) {
                let verb = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let path = caps.get(2).map_or("", |m| m.as_str()).trim_matches(|c| c == '\'' || c == '"').to_string();
                let to = caps.get(3).map_or("", |m| m.as_str()).trim_matches(|c| c == '\'' || c == '"').to_string();

                let parts: Vec<&str> = to.split('#').collect();
                if parts.len() == 2 {
                    let controller = parts[0].to_string();
                    let action = parts[1].to_string();
                    self.routes.push(Route {
                        verb,
                        path,
                        controller,
                        action,
                    });
                }
            }
        }
        Ok(())
    }

    fn extract_database_schema(&mut self, project_path: &str) -> Result<(), RubyRailsError> {
        let schema_path = Path::new(project_path).join("db").join("schema.rb");
        if !schema_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(schema_path)?;
        lazy_static! {
            static ref TABLE_REGEX: Regex = Regex::new(r#"create_table\s+"(\w+)"\s+do\s+\|t\|"#).unwrap();
            static ref COLUMN_REGEX: Regex = Regex::new(r#"t\.(\w+)\s+"(\w+)"(,\s+(.*?))?\s*$"#).unwrap();
        }

        let mut current_table: Option<String> = None;
        let mut schemas: HashMap<String, DatabaseSchema> = HashMap::new();

        for line in content.lines() {
            if let Some(table_caps) = TABLE_REGEX.captures(line) {
                if let Some(table_name) = table_caps.get(1).map(|m| m.as_str().to_string()) {
                    schemas.insert(table_name.clone(), DatabaseSchema {
                        name: table_name.clone(),
                        columns: Vec::new(),
                    });
                    current_table = Some(table_name);
                }
            } else if let Some(column_caps) = COLUMN_REGEX.captures(line) {
                if let Some(table_name) = &current_table {
                    if let (Some(column_type), Some(column_name)) = (column_caps.get(1), column_caps.get(2)) {
                        let column_type = column_type.as_str().to_string();
                        let column_name = column_name.as_str().to_string();
                        let mut options: HashMap<String, String> = HashMap::new();

                        if let Some(options_match) = column_caps.get(4) {
                            let options_str = options_match.as_str();
                            lazy_static! {
                                static ref OPTIONS_REGEX: Regex = Regex::new(r#"(\w+):\s+([\w"']+)"#).unwrap();
                            }
                            for option_caps in OPTIONS_REGEX.captures_iter(options_str) {
                                if let (Some(key), Some(value)) = (option_caps.get(1), option_caps.get(2)) {
                                    options.insert(key.as_str().to_string(), value.as_str().trim_matches(|c| c == '\'' || c == '"').to_string());
                                }
                            }
                        }

                        if let Some(schema) = schemas.get_mut(table_name) {
                            schema.columns.push(Column {
                                name: column_name,
                                r#type: column_type,
                                options,
                            });
                        }
                    }
                }
            } else if line.trim() == "end" {
                current_table = None;
            }
        }

        self.database_schemas = schemas;
        Ok(())
    }

    fn extract_callbacks_and_hooks(&mut self, project_path: &str) -> Result<(), RubyRailsError> {
        let callback_hook_regex = Regex::new(
            r"^(after_initialize|after_find|before_validation|after_validation|before_save|around_save|after_save|before_create|around_create|after_create|before_update|around_update|after_update|before_destroy|around_destroy|after_destroy|after_touch)\s+:(.+)$",
        )
        .unwrap();
        let model_regex = Regex::new(r"class\s+([A-Za-z:]+)\s*<.*").unwrap();

        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".rb") {
                            let content = fs::read_to_string(path)?;
                            let mut current_model: Option<String> = None;

                            if let Some(model_match) = model_regex.captures(&content) {
                                current_model =
                                    model_match.get(1).map(|m| m.as_str().to_string());
                            }
                            for line in content.lines() {
                                if let Some(caps) = callback_hook_regex.captures(line) {
                                    let hook_type = caps.get(1).map_or("", |m| m.as_str()).to_string();
                                    let methods_str = caps.get(2).map_or("", |m| m.as_str()).to_string();
                                    let methods: Vec<&str> = methods_str.split(",").map(|s| s.trim()).collect();
                                    for method in methods {
                                        if method.starts_with(":") {
                                            if let Some(model) = &current_model {
                                                self.callbacks.push(Callback {
                                                    model: model.clone(),
                                                    r#type: hook_type.clone(),
                                                    method: method.trim_start_matches(":").to_string(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let hook_regex = Regex::new(r"add_to_class\s*\(:([\w]+),\s*&([\w:]+)\)").unwrap();
        let initializer_regex = Regex::new(r"initializer\s+:'([\w]+)'\s+do\s+\|([\w]+)\|([\s\S]*?)end").unwrap();

        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".rb") {
                            let content = fs::read_to_string(path)?;

                            for line in content.lines(){
                                if let Some(caps) = hook_regex.captures(line) {
                                    if let (Some(hook_type), Some(method)) = (caps.get(1), caps.get(2)) {
                                        let hook_type_str = hook_type.as_str();
                                        let method_str = method.as_str();

                                        self.hooks.push(Hook{
                                            name: "".to_string(),
                                            r#type: hook_type_str.to_string(),
                                            target: "".to_string(),
                                            method: method_str.to_string(),
                                        })
                                    }
                                }
                            }

                            if let Some(caps) = initializer_regex.captures(&content) {
                                if let (Some(name), Some(target), Some(content)) = (caps.get(1), caps.get(2), caps.get(3)) {
                                    let name_str = name.as_str();
                                    let target_str = target.as_str();
                                    let content_str = content.as_str();

                                    lazy_static! {
                                        static ref DEFINE_METHOD_REGEX: Regex = Regex::new(r"define_method\s+\"?([\w]+)\"?\s+do([\s\S]*?)end").unwrap();
                                    }

                                    for method_caps in DEFINE_METHOD_REGEX.captures_iter(content_str) {
                                        if let Some(method_name) = method_caps.get(1) {
                                            self.hooks.push(Hook{
                                                name: name_str.to_string(),
                                                target: target_str.to_string(),
                                                r#type: "initializer".to_string(),
                                                method: method_name.as_str().to_string(),
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_controller(&mut self, file_name: &str,
        content: &str,
    ) -> Result<(), RubyRailsError> {
        // Regex to find the class name for controllers
        lazy_static! {
            static ref CONTROLLER_CLASS_REGEX: Regex =
                Regex::new(r"class\s+([A-Za-z:]+Controller)\s*<.*").unwrap();
        }
        // Regex to find method names within the controller class.
        lazy_static! {
            static ref CONTROLLER_METHODS_REGEX: Regex = Regex::new(r"def\s+([a-z_]+)").unwrap();
        }

        if let Some(captures) = CONTROLLER_CLASS_REGEX.captures(content) {
            if let Some(class_name_match) = captures.get(1) {
                let class_name = class_name_match.as_str();

                let mut actions: Vec<String> = Vec::new();
                for method_captures in CONTROLLER_METHODS_REGEX.captures_iter(content) {
                    if let Some(method_name_match) = method_captures.get(1) {
                        actions.push(method_name_match.as_str().to_string());
                    }
                }

                self.controllers.insert(class_name.to_string(), RailsController {
                    name: class_name.to_string(),
                    actions: actions,
                });
            }
        }
        Ok(())
    }
    fn extract_model(&mut self, file_name: &str,
        content: &str,
    ) -> Result<(), RubyRailsError> {
        // Regex to find the class name for models
        lazy_static! {
            static ref MODEL_CLASS_REGEX: Regex =
                Regex::new(r"class\s+([A-Za-z:]+)\s*<.*").unwrap();
        }
        // Regex to find Active Record associations
        lazy_static! {
            static ref MODEL_ASSOCIATIONS_REGEX: Regex =
                Regex::new(r"^\s*(has_many|has_one|belongs_to)\s+(:[a-z_]+)").unwrap();
        }

        // Regex to find Active Record validations
        lazy_static! {
            static ref MODEL_VALIDATIONS_REGEX: Regex =
                Regex::new(r"^\s*validates\s+(:[a-z_]+)").unwrap();
        }
        if let Some(captures) = MODEL_CLASS_REGEX.captures(content) {
            if let Some(class_name_match) = captures.get(1) {
                let class_name = class_name_match.as_str();

                let mut associations: Vec<String> = Vec::new();
                for association_captures in MODEL_ASSOCIATIONS_REGEX.captures_iter(content) {
                    if let Some(association_name_match) = association_captures.get(2) {
                        associations.push(association_name_match.as_str().to_string());
                    }
                }

                let mut validations: Vec<String> = Vec::new();
                for validation_captures in MODEL_VALIDATIONS_REGEX.captures_iter(content) {
                    if let Some(validation_name_match) = validation_captures.get(1) {
                        validations.push(validation_name_match.as_str().to_string());
                    }
                }

                self.models.insert(class_name.to_string(), RailsModel {
                    name: class_name.to_string(),
                    associations: associations,
                    validations: validations,
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RubyRailsError {
    IoError(io::Error),
    RegexError(String),
    JsonError(serde_json::Error),
    WalkDirError(walkdir::Error),
}

impl From<io::Error> for RubyRailsError {
    fn from(error: io::Error) -> Self {
        RubyRailsError::IoError(error)
    }
}

impl From<regex::Error> for RubyRailsError {
    fn from(error: regex::Error) -> Self {
        RubyRailsError::RegexError(error.to_string())
    }
}
impl From<serde_json::Error> for RubyRailsError {
    fn from(error: serde_json::Error) -> Self {
        RubyRailsError::JsonError(error)
    }
}

impl From<walkdir::Error> for RubyRailsError {
    fn from(error: walkdir::Error) -> Self {
        RubyRailsError::WalkDirError(error)
    }
}