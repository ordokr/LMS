use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct EnhancedRubyLayout {
    pub name: String,
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyPartial {
    pub name: String,
    pub file_path: String,
    pub content: String,
    pub locals: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyFormField {
    pub field_type: String,
    pub name: String,
    pub label: Option<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyForm {
    pub model: String,
    pub action: Option<String>,
    pub method: Option<String>,
    pub fields: Vec<EnhancedRubyFormField>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyLink {
    pub text: Option<String>,
    pub url: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyView {
    pub name: String,
    pub file_path: String,
    pub layout: Option<String>,
    pub partials: Vec<EnhancedRubyPartial>,
    pub forms: Vec<EnhancedRubyForm>,
    pub links: Vec<EnhancedRubyLink>,
    pub content: String,
}

pub struct EnhancedRubyViewAnalyzer {
    pub views: HashMap<String, EnhancedRubyView>,
    pub layouts: HashMap<String, EnhancedRubyLayout>,
    pub partials: HashMap<String, EnhancedRubyPartial>,
}

impl EnhancedRubyViewAnalyzer {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            layouts: HashMap::new(),
            partials: HashMap::new(),
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
                if extension == "erb" || extension == "html.erb" || extension == "html.haml" {
                    self.analyze_file(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Check if this is a layout file
        if file_path_str.contains("layouts") {
            let name = file_path.file_stem().unwrap().to_string_lossy().to_string();
            
            let layout = EnhancedRubyLayout {
                name: name.clone(),
                file_path: file_path_str,
                content,
            };
            
            self.layouts.insert(name, layout);
            return Ok(());
        }
        
        // Check if this is a partial file
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        if file_name.starts_with("_") {
            let name = file_name[1..].to_string();
            
            let partial = EnhancedRubyPartial {
                name: name.clone(),
                file_path: file_path_str,
                content: content.clone(),
                locals: HashMap::new(),
            };
            
            self.partials.insert(name, partial);
        }
        
        // Parse view
        let name = file_path.file_stem().unwrap().to_string_lossy().to_string();
        
        let mut view = EnhancedRubyView {
            name: name.clone(),
            file_path: file_path_str,
            layout: None,
            partials: Vec::new(),
            forms: Vec::new(),
            links: Vec::new(),
            content,
        };
        
        // Extract layout
        self.extract_layout(&mut view);
        
        // Extract partials
        self.extract_partials(&mut view);
        
        // Extract forms
        self.extract_forms(&mut view);
        
        // Extract links
        self.extract_links(&mut view);
        
        self.views.insert(name, view);
        
        Ok(())
    }
    
    fn extract_layout(&self, view: &mut EnhancedRubyView) {
        // Extract layout from render layout
        lazy_static! {
            static ref LAYOUT_REGEX: Regex =
                Regex::new(r#"<%=\s*render\s+layout:\s*['"]([^'"]+)['"]"#).unwrap();
        }
        
        if let Some(captures) = LAYOUT_REGEX.captures(&view.content) {
            view.layout = Some(captures.get(1).unwrap().as_str().to_string());
        }
    }
    
    fn extract_partials(&self, view: &mut EnhancedRubyView) {
        // Extract partials from render partial
        lazy_static! {
            static ref PARTIAL_REGEX: Regex =
                Regex::new(r#"<%=\s*render\s+(?:partial:\s*)?['"]([^'"]+)['"](?:,\s*(.+?))?%>"#).unwrap();
        }
        
        for captures in PARTIAL_REGEX.captures_iter(&view.content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            
            let mut partial = EnhancedRubyPartial {
                name: name.clone(),
                file_path: String::new(), // Will be filled later if the partial exists
                content: String::new(),   // Will be filled later if the partial exists
                locals: HashMap::new(),
            };
            
            // Extract locals
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract locals from options
                if options.contains("locals:") {
                    // Extract locals hash
                    let locals_start = options.find("locals:").unwrap() + 7;
                    let locals_end = options.find('}').unwrap_or(options.len());
                    let locals_str = &options[locals_start..locals_end];
                    
                    // Parse locals
                    for option_match in OPTION_REGEX.captures_iter(locals_str) {
                        let key = option_match.get(1).unwrap().as_str().to_string();
                        let value = option_match.get(2)
                            .or_else(|| option_match.get(3))
                            .or_else(|| option_match.get(4))
                            .unwrap()
                            .as_str()
                            .to_string();
                        
                        partial.locals.insert(key, value);
                    }
                }
            }
            
            // Fill in file_path and content if the partial exists
            if let Some(existing_partial) = self.partials.get(&name) {
                partial.file_path = existing_partial.file_path.clone();
                partial.content = existing_partial.content.clone();
            }
            
            view.partials.push(partial);
        }
    }
    
    fn extract_forms(&self, view: &mut EnhancedRubyView) {
        // Extract forms from form_for and form_tag
        lazy_static! {
            static ref FORM_FOR_REGEX: Regex =
                Regex::new(r#"<%=\s*form_for\s+([^,]+)(?:,\s*(.+?))?(?:\s+do\s*\|([^|]+)\|)?"#).unwrap();
            static ref FORM_TAG_REGEX: Regex =
                Regex::new(r#"<%=\s*form_tag\s+([^,]+)(?:,\s*(.+?))?(?:\s+do)?"#).unwrap();
            static ref OPTION_REGEX: Regex =
                Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|([^,]+))"#).unwrap();
        }
        
        // Extract form_for forms
        for captures in FORM_FOR_REGEX.captures_iter(&view.content) {
            let model = captures.get(1).unwrap().as_str().trim().to_string();
            
            let mut form = EnhancedRubyForm {
                model,
                action: None,
                method: None,
                fields: Vec::new(),
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract url/action
                if options.contains("url:") {
                    let url_start = options.find("url:").unwrap() + 4;
                    let url_end = options[url_start..].find(',').unwrap_or(options.len() - url_start);
                    let url = &options[url_start..url_start + url_end].trim();
                    form.action = Some(url.to_string());
                }
                
                // Extract method
                if options.contains("method:") {
                    let method_start = options.find("method:").unwrap() + 7;
                    let method_end = options[method_start..].find(',').unwrap_or(options.len() - method_start);
                    let method = &options[method_start..method_start + method_end].trim();
                    form.method = Some(method.to_string());
                }
                
                // Store all options in the options map
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
                    form.options.insert(key, value);
                }
            }
            
            // Extract form fields
            if let Some(form_builder) = captures.get(3) {
                let form_builder_name = form_builder.as_str().trim();
                
                // Extract text fields
                self.extract_form_fields(&view.content, form_builder_name, "text_field", &mut form);
                
                // Extract password fields
                self.extract_form_fields(&view.content, form_builder_name, "password_field", &mut form);
                
                // Extract text areas
                self.extract_form_fields(&view.content, form_builder_name, "text_area", &mut form);
                
                // Extract checkboxes
                self.extract_form_fields(&view.content, form_builder_name, "check_box", &mut form);
                
                // Extract radio buttons
                self.extract_form_fields(&view.content, form_builder_name, "radio_button", &mut form);
                
                // Extract select fields
                self.extract_form_fields(&view.content, form_builder_name, "select", &mut form);
            }
            
            view.forms.push(form);
        }
        
        // Extract form_tag forms
        for captures in FORM_TAG_REGEX.captures_iter(&view.content) {
            let action = captures.get(1).unwrap().as_str().trim().to_string();
            
            let mut form = EnhancedRubyForm {
                model: String::new(),
                action: Some(action),
                method: None,
                fields: Vec::new(),
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract method
                if options.contains("method:") {
                    let method_start = options.find("method:").unwrap() + 7;
                    let method_end = options[method_start..].find(',').unwrap_or(options.len() - method_start);
                    let method = &options[method_start..method_start + method_end].trim();
                    form.method = Some(method.to_string());
                }
                
                // Store all options in the options map
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
                    form.options.insert(key, value);
                }
            }
            
            view.forms.push(form);
        }
    }
    
    fn extract_form_fields(&self, content: &str, form_builder: &str, field_type: &str, form: &mut EnhancedRubyForm) {
        // Create regex for the specific field type
        let field_regex_str = format!(r#"{}\.{}\s+:([a-z0-9_]+)(?:,\s*(.+?))?"#, form_builder, field_type);
        let field_regex = Regex::new(&field_regex_str).unwrap();
        
        lazy_static! {
            static ref OPTION_REGEX: Regex =
                Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|([^,]+))"#).unwrap();
        }
        
        for captures in field_regex.captures_iter(content) {
            let name = captures.get(1).unwrap().as_str().to_string();
            
            let mut field = EnhancedRubyFormField {
                field_type: field_type.to_string(),
                name,
                label: None,
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Extract label
                if let Some(label_match) = Regex::new(r#"label:\s*['"]([^'"]+)['"]"#).unwrap().captures(options) {
                    field.label = Some(label_match.get(1).unwrap().as_str().to_string());
                }
                
                // Store all options in the options map
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
                    field.options.insert(key, value);
                }
            }
            
            form.fields.push(field);
        }
    }
    
    fn extract_links(&self, view: &mut EnhancedRubyView) {
        // Extract links from link_to
        lazy_static! {
            static ref LINK_TO_REGEX: Regex =
                Regex::new(r#"<%=\s*link_to\s+(?:['"]([^'"]+)['"](?:,\s*)?)?([^,]+)(?:,\s*(.+?))?%>"#).unwrap();
            static ref OPTION_REGEX: Regex =
                Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|([^,]+))"#).unwrap();
        }
        
        for captures in LINK_TO_REGEX.captures_iter(&view.content) {
            let text = captures.get(1).map(|m| m.as_str().to_string());
            let url = captures.get(2).unwrap().as_str().trim().to_string();
            
            let mut link = EnhancedRubyLink {
                text,
                url,
                options: HashMap::new(),
            };
            
            // Extract options
            if let Some(options_str) = captures.get(3) {
                let options = options_str.as_str();
                
                // Store all options in the options map
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
                    link.options.insert(key, value);
                }
            }
            
            view.links.push(link);
        }
    }
}
