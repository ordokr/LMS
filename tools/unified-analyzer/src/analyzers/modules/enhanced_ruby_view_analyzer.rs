use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ViewFormField {
    pub name: String,
    pub field_type: String,
    pub label: Option<String>,
    pub required: bool,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ViewPartial {
    pub name: String,
    pub path: String,
    pub locals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ViewLink {
    pub text: Option<String>,
    pub url: String,
    pub controller: Option<String>,
    pub action: Option<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ViewForm {
    pub model: Option<String>,
    pub action: Option<String>,
    pub method: Option<String>,
    pub fields: Vec<ViewFormField>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnhancedRubyView {
    pub name: String,
    pub file_path: String,
    pub controller: Option<String>,
    pub action: Option<String>,
    pub layout: Option<String>,
    pub partials: Vec<ViewPartial>,
    pub forms: Vec<ViewForm>,
    pub links: Vec<ViewLink>,
    pub instance_variables: Vec<String>,
    pub helpers: Vec<String>,
}

#[derive(Debug, Default)]
pub struct EnhancedRubyViewAnalyzer {
    pub views: HashMap<String, EnhancedRubyView>,
}

impl EnhancedRubyViewAnalyzer {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Ruby views in directory: {:?}", directory);
        
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Check for ERB templates
                    if extension == "erb" || 
                       (extension == "html" && path.to_string_lossy().contains(".html.erb")) {
                        self.analyze_view_file(path)?;
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn analyze_view_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing view file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        
        // Extract view name from path
        let view_name = extract_view_name(file_path);
        
        // Extract controller and action from path
        let (controller, action) = extract_controller_action(file_path);
        
        let mut view = EnhancedRubyView {
            name: view_name,
            file_path: file_path.to_string_lossy().to_string(),
            controller,
            action,
            ..Default::default()
        };
        
        // Extract layout
        self.extract_layout(&content, &mut view);
        
        // Extract partials
        self.extract_partials(&content, &mut view);
        
        // Extract forms
        self.extract_forms(&content, &mut view);
        
        // Extract links
        self.extract_links(&content, &mut view);
        
        // Extract instance variables
        self.extract_instance_variables(&content, &mut view);
        
        // Extract helpers
        self.extract_helpers(&content, &mut view);
        
        // Add view to the collection
        self.views.insert(view.file_path.clone(), view);
        
        Ok(())
    }

    fn extract_layout(&self, content: &str, view: &mut EnhancedRubyView) {
        // Extract layout from content
        lazy_static! {
            static ref LAYOUT_REGEX: Regex = 
                Regex::new(r"<%=\s*render\s+layout:\s*['\"]([^'\"]+)['\"]").unwrap();
        }
        
        if let Some(captures) = LAYOUT_REGEX.captures(content) {
            view.layout = Some(captures.get(1).unwrap().as_str().to_string());
        }
    }

    fn extract_partials(&self, content: &str, view: &mut EnhancedRubyView) {
        // Extract partials from content
        lazy_static! {
            static ref PARTIAL_REGEX: Regex = 
                Regex::new(r"<%=\s*render\s+(?:partial:\s*)?['\"]([^'\"]+)['\"](?:,\s*(.+?))?%>").unwrap();
        }
        
        for captures in PARTIAL_REGEX.captures_iter(content) {
            let partial_name = captures.get(1).unwrap().as_str().to_string();
            
            let mut partial = ViewPartial {
                name: partial_name.clone(),
                path: format!("_{}.", partial_name),
                locals: Vec::new(),
            };
            
            // Extract locals if present
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract locals
                if let Some(locals_match) = Regex::new(r"locals:\s*\{([^}]+)\}").unwrap().captures(options) {
                    let locals_str = locals_match.get(1).unwrap().as_str();
                    for local in locals_str.split(',') {
                        if let Some(local_match) = Regex::new(r"([a-z_]+):\s*([^,]+)").unwrap().captures(local) {
                            let local_name = local_match.get(1).unwrap().as_str().to_string();
                            partial.locals.push(local_name);
                        }
                    }
                }
            }
            
            view.partials.push(partial);
        }
    }

    fn extract_forms(&self, content: &str, view: &mut EnhancedRubyView) {
        // Extract forms from content
        lazy_static! {
            static ref FORM_FOR_REGEX: Regex = 
                Regex::new(r"<%=\s*form_for\s+(?:@([a-z_]+)|:([a-z_]+))(?:,\s*(.+?))?\s*do\s*\|([a-z_]+)\|\s*%>(.*?)<%\s*end\s*%>").unwrap();
        }
        
        for captures in FORM_FOR_REGEX.captures_iter(content) {
            let model_name = captures.get(1)
                .or_else(|| captures.get(2))
                .map(|m| m.as_str().to_string());
            
            let form_builder = captures.get(4).unwrap().as_str().to_string();
            let form_content = captures.get(5).unwrap().as_str();
            
            let mut form = ViewForm {
                model: model_name,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Extract url/action
                if let Some(url_match) = Regex::new(r"url:\s*([^,]+)").unwrap().captures(options) {
                    form.action = Some(url_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract method
                if let Some(method_match) = Regex::new(r"method:\s*:([a-z_]+)").unwrap().captures(options) {
                    form.method = Some(method_match.get(1).unwrap().as_str().to_string());
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    form.options.insert(key, value);
                }
            }
            
            // Extract form fields
            self.extract_form_fields(form_content, &form_builder, &mut form);
            
            view.forms.push(form);
        }
    }

    fn extract_form_fields(&self, content: &str, form_builder: &str, form: &mut ViewForm) {
        // Extract text fields
        lazy_static! {
            static ref TEXT_FIELD_REGEX: Regex = 
                Regex::new(&format!(r"<%=\s*{}\.text_field\s+:([a-z_]+)(?:,\s*(.+?))?%>", form_builder)).unwrap();
        }
        
        for captures in TEXT_FIELD_REGEX.captures_iter(content) {
            let field_name = captures.get(1).unwrap().as_str().to_string();
            
            let mut field = ViewFormField {
                name: field_name,
                field_type: "text_field".to_string(),
                required: false,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract label
                if let Some(label_match) = Regex::new(r"label:\s*['\"]([^'\"]+)['\"]").unwrap().captures(options) {
                    field.label = Some(label_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract required
                if let Some(_) = Regex::new(r"required:\s*true").unwrap().captures(options) {
                    field.required = true;
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    field.options.insert(key, value);
                }
            }
            
            form.fields.push(field);
        }
        
        // Extract password fields
        lazy_static! {
            static ref PASSWORD_FIELD_REGEX: Regex = 
                Regex::new(&format!(r"<%=\s*{}\.password_field\s+:([a-z_]+)(?:,\s*(.+?))?%>", form_builder)).unwrap();
        }
        
        for captures in PASSWORD_FIELD_REGEX.captures_iter(content) {
            let field_name = captures.get(1).unwrap().as_str().to_string();
            
            let mut field = ViewFormField {
                name: field_name,
                field_type: "password_field".to_string(),
                required: false,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract label
                if let Some(label_match) = Regex::new(r"label:\s*['\"]([^'\"]+)['\"]").unwrap().captures(options) {
                    field.label = Some(label_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract required
                if let Some(_) = Regex::new(r"required:\s*true").unwrap().captures(options) {
                    field.required = true;
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    field.options.insert(key, value);
                }
            }
            
            form.fields.push(field);
        }
        
        // Extract select fields
        lazy_static! {
            static ref SELECT_FIELD_REGEX: Regex = 
                Regex::new(&format!(r"<%=\s*{}\.select\s+:([a-z_]+)(?:,\s*(.+?))?%>", form_builder)).unwrap();
        }
        
        for captures in SELECT_FIELD_REGEX.captures_iter(content) {
            let field_name = captures.get(1).unwrap().as_str().to_string();
            
            let mut field = ViewFormField {
                name: field_name,
                field_type: "select".to_string(),
                required: false,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract label
                if let Some(label_match) = Regex::new(r"label:\s*['\"]([^'\"]+)['\"]").unwrap().captures(options) {
                    field.label = Some(label_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract required
                if let Some(_) = Regex::new(r"required:\s*true").unwrap().captures(options) {
                    field.required = true;
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    field.options.insert(key, value);
                }
            }
            
            form.fields.push(field);
        }
        
        // Extract checkbox fields
        lazy_static! {
            static ref CHECKBOX_FIELD_REGEX: Regex = 
                Regex::new(&format!(r"<%=\s*{}\.check_box\s+:([a-z_]+)(?:,\s*(.+?))?%>", form_builder)).unwrap();
        }
        
        for captures in CHECKBOX_FIELD_REGEX.captures_iter(content) {
            let field_name = captures.get(1).unwrap().as_str().to_string();
            
            let mut field = ViewFormField {
                name: field_name,
                field_type: "check_box".to_string(),
                required: false,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract label
                if let Some(label_match) = Regex::new(r"label:\s*['\"]([^'\"]+)['\"]").unwrap().captures(options) {
                    field.label = Some(label_match.get(1).unwrap().as_str().to_string());
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    field.options.insert(key, value);
                }
            }
            
            form.fields.push(field);
        }
        
        // Extract text area fields
        lazy_static! {
            static ref TEXT_AREA_FIELD_REGEX: Regex = 
                Regex::new(&format!(r"<%=\s*{}\.text_area\s+:([a-z_]+)(?:,\s*(.+?))?%>", form_builder)).unwrap();
        }
        
        for captures in TEXT_AREA_FIELD_REGEX.captures_iter(content) {
            let field_name = captures.get(1).unwrap().as_str().to_string();
            
            let mut field = ViewFormField {
                name: field_name,
                field_type: "text_area".to_string(),
                required: false,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract label
                if let Some(label_match) = Regex::new(r"label:\s*['\"]([^'\"]+)['\"]").unwrap().captures(options) {
                    field.label = Some(label_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract required
                if let Some(_) = Regex::new(r"required:\s*true").unwrap().captures(options) {
                    field.required = true;
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    field.options.insert(key, value);
                }
            }
            
            form.fields.push(field);
        }
    }

    fn extract_links(&self, content: &str, view: &mut EnhancedRubyView) {
        // Extract links from content
        lazy_static! {
            static ref LINK_TO_REGEX: Regex = 
                Regex::new(r"<%=\s*link_to\s+(?:['\"]([^'\"]+)['\"](?:,\s*)?)?([^,]+)(?:,\s*(.+?))?%>").unwrap();
        }
        
        for captures in LINK_TO_REGEX.captures_iter(content) {
            let text = captures.get(1).map(|m| m.as_str().to_string());
            let url = captures.get(2).unwrap().as_str().to_string();
            
            let mut link = ViewLink {
                text,
                url,
                ..Default::default()
            };
            
            // Extract options if present
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Extract controller
                if let Some(controller_match) = Regex::new(r"controller:\s*:([a-z_]+)").unwrap().captures(options) {
                    link.controller = Some(controller_match.get(1).unwrap().as_str().to_string());
                }
                
                // Extract action
                if let Some(action_match) = Regex::new(r"action:\s*:([a-z_]+)").unwrap().captures(options) {
                    link.action = Some(action_match.get(1).unwrap().as_str().to_string());
                }
                
                // Store all options in the options map
                for option_match in Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap().captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .map_or("".to_string(), |m| m.as_str().to_string());
                    
                    link.options.insert(key, value);
                }
            }
            
            view.links.push(link);
        }
    }

    fn extract_instance_variables(&self, content: &str, view: &mut EnhancedRubyView) {
        // Extract instance variables from content
        lazy_static! {
            static ref INSTANCE_VAR_REGEX: Regex = 
                Regex::new(r"@([a-z_][a-z0-9_]*)").unwrap();
        }
        
        for captures in INSTANCE_VAR_REGEX.captures_iter(content) {
            let var_name = captures.get(1).unwrap().as_str().to_string();
            if !view.instance_variables.contains(&var_name) {
                view.instance_variables.push(var_name);
            }
        }
    }

    fn extract_helpers(&self, content: &str, view: &mut EnhancedRubyView) {
        // Extract helpers from content
        lazy_static! {
            static ref HELPER_REGEX: Regex = 
                Regex::new(r"<%=\s*([a-z_]+)(?:\(|[\s(])").unwrap();
        }
        
        for captures in HELPER_REGEX.captures_iter(content) {
            let helper_name = captures.get(1).unwrap().as_str().to_string();
            
            // Skip common Rails helpers
            if !["link_to", "form_for", "render", "content_tag", "image_tag", "javascript_include_tag", "stylesheet_link_tag"].contains(&helper_name.as_str()) {
                if !view.helpers.contains(&helper_name) {
                    view.helpers.push(helper_name);
                }
            }
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.views)
    }
}

// Helper function to extract view name from path
fn extract_view_name(file_path: &Path) -> String {
    if let Some(file_name) = file_path.file_name() {
        let file_name_str = file_name.to_string_lossy();
        if let Some(dot_pos) = file_name_str.find('.') {
            return file_name_str[..dot_pos].to_string();
        }
        return file_name_str.to_string();
    }
    "unknown".to_string()
}

// Helper function to extract controller and action from path
fn extract_controller_action(file_path: &Path) -> (Option<String>, Option<String>) {
    let path_str = file_path.to_string_lossy();
    
    // Try to extract controller and action from path
    // Typical path: app/views/controller_name/action_name.html.erb
    lazy_static! {
        static ref PATH_REGEX: Regex = 
            Regex::new(r"app/views/([a-z_]+)/([a-z_]+)\.").unwrap();
    }
    
    if let Some(captures) = PATH_REGEX.captures(&path_str) {
        let controller = captures.get(1).map(|m| m.as_str().to_string());
        let action = captures.get(2).map(|m| m.as_str().to_string());
        return (controller, action);
    }
    
    (None, None)
}
