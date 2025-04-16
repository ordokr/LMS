rust
--- a/tools/unified-analyzer/src/analyzers/modules/ember_analyzer.rs
+++ b/tools/unified-analyzer/src/analyzers/modules/ember_analyzer.rs
use std::{
    collections::HashMap,
    fs,
    io,
    path::{Path, PathBuf},
};

use log::{debug, error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberAnalyzer {
    pub models: HashMap<String, EmberModel>,
    pub controllers: HashMap<String, EmberController>,
    pub components: HashMap<String, EmberComponent>,
    pub routes: HashMap<String, EmberRoute>,
    pub services: HashMap<String, EmberService>,
    pub helpers: HashMap<String, EmberHelper>,
    pub templates: HashMap<String, EmberTemplate>,
    pub initializers: HashMap<String, EmberInitializer>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberModel {
    pub name: String,
    pub attributes: Vec<String>,
    pub relationships: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberController {
    pub name: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberComponent {
    pub name: String,
    pub properties: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberRoute {
    pub name: String,
    pub path: String,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberService {
    pub name: String,
    pub methods: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberHelper {
    pub name: String,
    pub functions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberTemplate {
    pub name: String,
    pub path: String,
    pub bindings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmberInitializer {
    pub name: String,
    pub initialize: String,
}

impl EmberAnalyzer {
    pub fn analyze(&mut self, project_path: &str) -> Result<String, EmberError> {
        for entry in WalkDir::new(project_path).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "js" || ext == "ts" {
                        self.analyze_javascript_file(path)?;
                    } else if ext == "hbs" {
                        self.analyze_template_file(path)?;
                    }
                }
            }
        }
        serde_json::to_string_pretty(&self).map_err(EmberError::JsonError)
    }

    fn analyze_javascript_file(&mut self, path: &Path) -> Result<(), EmberError> {
        let content = fs::read_to_string(path)?;
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let parent_dir_name = parent.file_name().and_then(|s| s.to_str()).unwrap_or("");

        match parent_dir_name {
            "models" => self.extract_model(file_name, &content)?,
            "controllers" => self.extract_controller(file_name, &content)?,
            "components" => self.extract_component(file_name, &content)?,
            "routes" => self.extract_route(file_name, &content)?,
            "services" => self.extract_service(file_name, &content)?,
            "helpers" => self.extract_helper(file_name, &content)?,
            "initializers" => self.extract_initializer(file_name, &content)?,
            _ => debug!("Unhandled directory: {:?}", parent_dir_name),
        }

        Ok(())
    }

    fn analyze_template_file(&mut self, path: &Path) -> Result<(), EmberError> {
        let content = fs::read_to_string(path)?;
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let template = EmberTemplate {
            name: file_name.to_string(),
            path: path.to_string_lossy().into_owned(),
            bindings: self.extract_template_bindings(&content),
        };
        self.templates.insert(file_name.to_string(), template);
        Ok(())
    }

    fn extract_model(&mut self, file_name: &str, content: &str) -> Result<(), EmberError> {
        let mut model = EmberModel {
            name: file_name.to_string(),
            ..Default::default()
        };
        lazy_static::lazy_static! {
            static ref ATTRIBUTE_REGEX: Regex = Regex::new(r"@attr ('|\")(?P<name>\w+)('|\")").unwrap();
            static ref RELATIONSHIP_REGEX: Regex = Regex::new(r"(hasMany|belongsTo)('|\")(?P<name>\w+)('|\")").unwrap();
        }
        for cap in ATTRIBUTE_REGEX.captures_iter(content) {
            if let Some(name) = cap.name("name") {
                model.attributes.push(name.as_str().to_string());
            }
        }
        for cap in RELATIONSHIP_REGEX.captures_iter(content) {
            if let Some(name) = cap.name("name") {
                model.relationships.push(name.as_str().to_string());
            }
        }
        self.models.insert(file_name.to_string(), model);
        Ok(())
    }

    fn extract_controller(&mut self, file_name: &str, content: &str) -> Result<(), EmberError> {
        let mut controller = EmberController {
            name: file_name.to_string(),
            ..Default::default()
        };
        lazy_static::lazy_static! {
            static ref ACTION_REGEX: Regex = Regex::new(r"action: '(?P<name>\w+)'").unwrap();
        }
        for cap in ACTION_REGEX.captures_iter(content) {
            if let Some(name) = cap.name("name") {
                controller.actions.push(name.as_str().to_string());
            }
        }
        self.controllers.insert(file_name.to_string(), controller);
        Ok(())
    }

    fn extract_component(&mut self, file_name: &str, content: &str) -> Result<(), EmberError> {
        let mut component = EmberComponent {
            name: file_name.to_string(),
            ..Default::default()
        };
        lazy_static::lazy_static! {
            static ref PROPERTY_REGEX: Regex = Regex::new(r"attribute: '(?P<name>\w+)'").unwrap();
            static ref ACTION_REGEX: Regex = Regex::new(r"action: '(?P<name>\w+)'").unwrap();
        }
        for cap in PROPERTY_REGEX.captures_iter(content) {
            if let Some(name) = cap.name("name") {
                component.properties.push(name.as_str().to_string());
            }
        }
        for cap in ACTION_REGEX.captures_iter(content) {
            if let Some(name) = cap.name("name") {
                component.actions.push(name.as_str().to_string());
            }
        }
        self.components.insert(file_name.to_string(), component);
        Ok(())
    }

    fn extract_route(&mut self, file_name: &str, content: &str) -> Result<(), EmberError> {
        let mut route = EmberRoute {
            name: file_name.to_string(),
            ..Default::default()
        };
        lazy_static::lazy_static! {
            static ref PATH_REGEX: Regex = Regex::new(r"path: '(?P<path>\S+)'").unwrap();
            static ref MODEL_REGEX: Regex = Regex::new(r"model: '(?P<model>\w+)'").unwrap();
        }
        if let Some(cap) = PATH_REGEX.captures(content) {
            if let Some(path) = cap.name("path") {
                route.path = path.as_str().to_string();
            }
        }
        if let Some(cap) = MODEL_REGEX.captures(content) {
            if let Some(model) = cap.name("model") {
                route.model = Some(model.as_str().to_string());
            }
        }
        self.routes.insert(file_name.to_string(), route);
        Ok(())
    }

    fn extract_service(&mut self, file_name: &str, content: &str) -> Result<(), EmberError> {
        let mut service = EmberService {
            name: file_name.to_string(),
            ..Default::default()
        };
        lazy_static::lazy_static! {
            static ref METHOD_REGEX: Regex = Regex::new(r"method: '(?P<name>\w+)'").unwrap();
        }
        for cap in METHOD_REGEX.captures_iter(content) {
            if let Some(name) = cap.name("name") {
                service.methods.push(name.as_str().to_string());
            }
        }
        self.services.insert(file_name.to_string(), service);
        Ok(())
    }

    fn extract_helper(&mut self, file_name: &str, content: &str) -> Result<(), EmberError> {
        let mut helper = EmberHelper {
            name: file_name.to_string(),
            ..Default::default()
        };
        lazy_static::lazy_static! {
            static ref FUNCTION_REGEX: Regex = Regex::new(r"function: '(?P<name>\w+)'").unwrap();
        }
        for cap in FUNCTION_REGEX.captures_iter(content) {
            if let Some(name) = cap.name("name") {
                helper.functions.push(name.as_str().to_string());
            }
        }
        self.helpers.insert(file_name.to_string(), helper);
        Ok(())
    }

    fn extract_initializer(&mut self, file_name: &str, content: &str) -> Result<(), EmberError> {
        let initializer = EmberInitializer {
            name: file_name.to_string(),
            initialize: content.to_string(), // simplified, can be improved
        };
        self.initializers.insert(file_name.to_string(), initializer);
        Ok(())
    }

    fn extract_template_bindings(&self, content: &str) -> Vec<String> {
        lazy_static::lazy_static! {
            static ref BINDING_REGEX: Regex = Regex::new(r"\{\{(?P<binding>\S+)\}\}").unwrap();
        }
        BINDING_REGEX.captures_iter(content).filter_map(|cap| {
            cap.name("binding").map(|binding| binding.as_str().to_string())
        }).collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmberError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("WalkDir error: {0}")]
    WalkDirError(#[from] walkdir::Error),
}