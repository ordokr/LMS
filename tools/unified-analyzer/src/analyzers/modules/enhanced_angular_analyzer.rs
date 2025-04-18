use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AngularInput {
    pub name: String,
    pub input_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub decorator_options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AngularOutput {
    pub name: String,
    pub event_type: String,
    pub decorator_options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AngularProperty {
    pub name: String,
    pub property_type: String,
    pub initial_value: Option<String>,
    pub is_private: bool,
    pub is_readonly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AngularMethod {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub code: String,
    pub is_private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AngularLifecycleHook {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AngularDependency {
    pub name: String,
    pub dependency_type: String,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AngularComponent {
    pub name: String,
    pub file_path: String,
    pub selector: String,
    pub template_path: Option<String>,
    pub template_content: Option<String>,
    pub style_paths: Vec<String>,
    pub style_content: Option<String>,
    pub inputs: Vec<AngularInput>,
    pub outputs: Vec<AngularOutput>,
    pub properties: Vec<AngularProperty>,
    pub methods: Vec<AngularMethod>,
    pub lifecycle_hooks: Vec<AngularLifecycleHook>,
    pub dependencies: Vec<AngularDependency>,
    pub child_components: Vec<String>,
    pub directives: Vec<String>,
    pub pipes: Vec<String>,
    pub imports: HashMap<String, String>,
    pub raw_content: String,
}

#[derive(Debug, Default)]
pub struct EnhancedAngularAnalyzer {
    pub components: HashMap<String, AngularComponent>,
}

impl EnhancedAngularAnalyzer {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Angular components in directory: {:?}", directory);

        // First, find all component files
        let mut component_files = Vec::new();
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Check for Angular component files
                    if (extension == "ts") &&
                       path.to_string_lossy().contains(".component.") {
                        component_files.push(path.to_path_buf());
                    }
                }
            }
        }

        // Then, analyze each component file
        for file_path in component_files {
            self.analyze_component_file(&file_path)?;
        }

        Ok(())
    }

    fn analyze_component_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Angular component file: {:?}", file_path);

        let content = fs::read_to_string(file_path)?;

        // Extract component name from file name
        let component_name = self.extract_component_name(file_path);

        let mut component = AngularComponent {
            name: component_name,
            file_path: file_path.to_string_lossy().to_string(),
            raw_content: content.clone(),
            ..Default::default()
        };

        // Extract imports
        self.extract_imports(&content, &mut component);

        // Extract component decorator
        self.extract_component_decorator(&content, &mut component, file_path);

        // Extract inputs
        self.extract_inputs(&content, &mut component);

        // Extract outputs
        self.extract_outputs(&content, &mut component);

        // Extract properties
        self.extract_properties(&content, &mut component);

        // Extract methods
        self.extract_methods(&content, &mut component);

        // Extract lifecycle hooks
        self.extract_lifecycle_hooks(&content, &mut component);

        // Extract dependencies from constructor
        self.extract_dependencies(&content, &mut component);

        // Extract child components, directives, and pipes from template
        if let Some(template_content) = &component.template_content {
            self.extract_template_dependencies(template_content, &mut component);
        }

        // Add component to the collection
        self.components.insert(component.file_path.clone(), component);

        Ok(())
    }

    fn extract_component_name(&self, file_path: &Path) -> String {
        // Extract component name from file name
        // Example: user-profile.component.ts -> UserProfileComponent
        if let Some(file_name) = file_path.file_stem() {
            let file_name_str = file_name.to_string_lossy();

            // Remove .component suffix if present
            let base_name = if file_name_str.ends_with(".component") {
                &file_name_str[..file_name_str.len() - 10]
            } else {
                &file_name_str
            };

            // Convert kebab-case to PascalCase
            if base_name.contains('-') {
                return base_name
                    .split('-')
                    .map(|part| {
                        let mut chars = part.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("") + "Component";
            }

            // If already PascalCase, just add Component suffix
            if let Some(first_char) = base_name.chars().next() {
                if first_char.is_uppercase() {
                    return base_name.to_string() + "Component";
                }
            }

            // Convert from camelCase to PascalCase and add Component suffix
            if let Some(first_char) = base_name.chars().next() {
                return first_char.to_uppercase().to_string() + &base_name[1..] + "Component";
            }

            return base_name.to_string() + "Component";
        }

        "UnknownComponent".to_string()
    }

    fn extract_imports(&self, content: &str, component: &mut AngularComponent) {
        lazy_static! {
            static ref IMPORT_REGEX: Regex =
                Regex::new(r"import\s+\{([^}]+)\}\s+from\s+['"]([^'"]+)['"];").unwrap();
        }

        for captures in IMPORT_REGEX.captures_iter(content) {
            let imports_str = captures.get(1).unwrap().as_str();
            let module_path = captures.get(2).unwrap().as_str().to_string();

            // Split multiple imports
            for import in imports_str.split(',') {
                let import_name = import.trim().to_string();
                component.imports.insert(import_name, module_path.clone());
            }
        }
    }

    fn extract_component_decorator(&self, content: &str, component: &mut AngularComponent, file_path: &Path) {
        lazy_static! {
            static ref COMPONENT_DECORATOR_REGEX: Regex =
                Regex::new(r"@Component\(\{([\s\S]*?)\}\)").unwrap();
        }

        if let Some(captures) = COMPONENT_DECORATOR_REGEX.captures(content) {
            let decorator_content = captures.get(1).unwrap().as_str();

            // Extract selector
            if let Some(selector_match) = Regex::new(r"selector:\s*['"]([^'"]+)['"]")
                .unwrap()
                .captures(decorator_content) {
                component.selector = selector_match.get(1).unwrap().as_str().to_string();
            }

            // Extract template or templateUrl
            if let Some(template_match) = Regex::new(r"template:\s*`([\s\S]*?)`")
                .unwrap()
                .captures(decorator_content) {
                component.template_content = Some(template_match.get(1).unwrap().as_str().to_string());
            } else if let Some(template_url_match) = Regex::new(r"templateUrl:\s*['"]([^'"]+)['"]")
                .unwrap()
                .captures(decorator_content) {
                let template_url = template_url_match.get(1).unwrap().as_str();
                component.template_path = Some(template_url.to_string());

                // Try to load template content
                let template_path = if template_url.starts_with("./") {
                    // Relative path
                    let parent_dir = file_path.parent().unwrap_or(Path::new(""));
                    parent_dir.join(&template_url[2..])
                } else {
                    // Absolute path or other format
                    PathBuf::from(template_url)
                };

                if template_path.exists() {
                    if let Ok(template_content) = fs::read_to_string(&template_path) {
                        component.template_content = Some(template_content);
                    }
                }
            }

            // Extract styles or styleUrls
            if let Some(styles_match) = Regex::new(r"styles:\s*\[([\s\S]*?)\]")
                .unwrap()
                .captures(decorator_content) {
                let styles_content = styles_match.get(1).unwrap().as_str();
                component.style_content = Some(styles_content.to_string());
            } else if let Some(style_urls_match) = Regex::new(r"styleUrls:\s*\[([\s\S]*?)\]")
                .unwrap()
                .captures(decorator_content) {
                let style_urls_str = style_urls_match.get(1).unwrap().as_str();

                // Extract individual style URLs
                lazy_static! {
                    static ref STYLE_URL_REGEX: Regex =
                        Regex::new(r"['"]([^'"]+)['"]")
                        .unwrap();
                }

                for url_match in STYLE_URL_REGEX.captures_iter(style_urls_str) {
                    let style_url = url_match.get(1).unwrap().as_str().to_string();
                    component.style_paths.push(style_url);
                }

                // Try to load first style content
                if !component.style_paths.is_empty() {
                    let style_url = &component.style_paths[0];
                    let style_path = if style_url.starts_with("./") {
                        // Relative path
                        let parent_dir = file_path.parent().unwrap_or(Path::new(""));
                        parent_dir.join(&style_url[2..])
                    } else {
                        // Absolute path or other format
                        PathBuf::from(style_url)
                    };

                    if style_path.exists() {
                        if let Ok(style_content) = fs::read_to_string(&style_path) {
                            component.style_content = Some(style_content);
                        }
                    }
                }
            }
        }
    }

    fn extract_inputs(&self, content: &str, component: &mut AngularComponent) {
        // Extract @Input() decorators
        lazy_static! {
            static ref INPUT_REGEX: Regex =
                Regex::new(r"@Input\((?:['"]([^'"]+)['"])?\)\s*(?:readonly\s+)?([a-zA-Z0-9_]+)(?:!|\?)?:\s*([a-zA-Z0-9_<>\[\]|]+)(?:\s*=\s*([^;]+))?;").unwrap();
        }

        for captures in INPUT_REGEX.captures_iter(content) {
            let input_alias = captures.get(1).map(|m| m.as_str().to_string());
            let input_name = captures.get(2).unwrap().as_str().to_string();
            let input_type = captures.get(3).unwrap().as_str().to_string();
            let default_value = captures.get(4).map(|m| m.as_str().trim().to_string());

            let mut input = AngularInput {
                name: input_name,
                input_type,
                required: !content.contains(&format!("{}: {}|", input_name, input_type)) &&
                          !content.contains(&format!("{}: {} |", input_name, input_type)) &&
                          !content.contains(&format!("{}: {} | ", input_name, input_type)) &&
                          !content.contains(&format!("{}: {}?", input_name, input_type)) &&
                          !content.contains(&format!("{}: {} ?", input_name, input_type)) &&
                          !content.contains(&format!("{}: {} ? ", input_name, input_type)),
                default_value,
                ..Default::default()
            };

            // Add alias to decorator options if present
            if let Some(alias) = input_alias {
                input.decorator_options.insert("alias".to_string(), alias);
            }

            component.inputs.push(input);
        }

        // Also check for inputs in the @Component decorator
        lazy_static! {
            static ref COMPONENT_INPUTS_REGEX: Regex =
                Regex::new(r"@Component\(\{[\s\S]*?inputs:\s*\[([\s\S]*?)\][\s\S]*?\}\)").unwrap();
        }

        if let Some(captures) = COMPONENT_INPUTS_REGEX.captures(content) {
            let inputs_list = captures.get(1).unwrap().as_str();

            // Extract individual inputs
            lazy_static! {
                static ref INPUT_ITEM_REGEX: Regex =
                    Regex::new(r"['"]([^'"]+)['"]")
                    .unwrap();
            }

            for input_match in INPUT_ITEM_REGEX.captures_iter(inputs_list) {
                let input_str = input_match.get(1).unwrap().as_str();

                // Check if it's in the format 'propertyName: bindingName'
                let parts: Vec<&str> = input_str.split(':').collect();
                if parts.len() == 2 {
                    let property_name = parts[0].trim().to_string();
                    let binding_name = parts[1].trim().to_string();

                    // Skip if already added via @Input decorator
                    if !component.inputs.iter().any(|i| i.name == property_name) {
                        let mut input = AngularInput {
                            name: property_name,
                            input_type: "any".to_string(), // Can't determine type from this syntax
                            required: false,
                            ..Default::default()
                        };

                        input.decorator_options.insert("alias".to_string(), binding_name);
                        component.inputs.push(input);
                    }
                } else {
                    // Simple input name
                    let input_name = input_str.to_string();

                    // Skip if already added via @Input decorator
                    if !component.inputs.iter().any(|i| i.name == input_name) {
                        component.inputs.push(AngularInput {
                            name: input_name,
                            input_type: "any".to_string(), // Can't determine type from this syntax
                            required: false,
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    fn extract_outputs(&self, content: &str, component: &mut AngularComponent) {
        // Extract @Output() decorators
        lazy_static! {
            static ref OUTPUT_REGEX: Regex =
                Regex::new(r"@Output\((?:['"]([^'"]+)['"])?\)\s*(?:readonly\s+)?([a-zA-Z0-9_]+)\s*=\s*new\s+EventEmitter<([^>]*)>\(\);").unwrap();
        }

        for captures in OUTPUT_REGEX.captures_iter(content) {
            let output_alias = captures.get(1).map(|m| m.as_str().to_string());
            let output_name = captures.get(2).unwrap().as_str().to_string();
            let event_type = captures.get(3).map_or("void".to_string(), |m| m.as_str().trim().to_string());

            let mut output = AngularOutput {
                name: output_name,
                event_type,
                ..Default::default()
            };

            // Add alias to decorator options if present
            if let Some(alias) = output_alias {
                output.decorator_options.insert("alias".to_string(), alias);
            }

            component.outputs.push(output);
        }

        // Also check for outputs in the @Component decorator
        lazy_static! {
            static ref COMPONENT_OUTPUTS_REGEX: Regex =
                Regex::new(r"@Component\(\{[\s\S]*?outputs:\s*\[([\s\S]*?)\][\s\S]*?\}\)").unwrap();
        }

        if let Some(captures) = COMPONENT_OUTPUTS_REGEX.captures(content) {
            let outputs_list = captures.get(1).unwrap().as_str();

            // Extract individual outputs
            lazy_static! {
                static ref OUTPUT_ITEM_REGEX: Regex =
                    Regex::new(r"['"]([^'"]+)['"]")
                    .unwrap();
            }

            for output_match in OUTPUT_ITEM_REGEX.captures_iter(outputs_list) {
                let output_str = output_match.get(1).unwrap().as_str();

                // Check if it's in the format 'propertyName: bindingName'
                let parts: Vec<&str> = output_str.split(':').collect();
                if parts.len() == 2 {
                    let property_name = parts[0].trim().to_string();
                    let binding_name = parts[1].trim().to_string();

                    // Skip if already added via @Output decorator
                    if !component.outputs.iter().any(|o| o.name == property_name) {
                        let mut output = AngularOutput {
                            name: property_name,
                            event_type: "any".to_string(), // Can't determine type from this syntax
                            ..Default::default()
                        };

                        output.decorator_options.insert("alias".to_string(), binding_name);
                        component.outputs.push(output);
                    }
                } else {
                    // Simple output name
                    let output_name = output_str.to_string();

                    // Skip if already added via @Output decorator
                    if !component.outputs.iter().any(|o| o.name == output_name) {
                        component.outputs.push(AngularOutput {
                            name: output_name,
                            event_type: "any".to_string(), // Can't determine type from this syntax
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    fn extract_properties(&self, content: &str, component: &mut AngularComponent) {
        // Extract class properties
        lazy_static! {
            static ref PROPERTY_REGEX: Regex =
                Regex::new(r"(?:private|protected|public|readonly)?\s*([a-zA-Z0-9_]+)(?:!|\?)?:\s*([a-zA-Z0-9_<>\[\]|]+)(?:\s*=\s*([^;]+))?;").unwrap();
        }

        for captures in PROPERTY_REGEX.captures_iter(content) {
            let property_name = captures.get(1).unwrap().as_str().to_string();

            // Skip if this is an input or output
            if component.inputs.iter().any(|i| i.name == property_name) ||
               component.outputs.iter().any(|o| o.name == property_name) {
                continue;
            }

            // Skip if this is a constructor parameter
            if content.contains(&format!("constructor(private {}", property_name)) ||
               content.contains(&format!("constructor(protected {}", property_name)) ||
               content.contains(&format!("constructor(public {}", property_name)) ||
               content.contains(&format!("constructor(readonly {}", property_name)) {
                continue;
            }

            let property_type = captures.get(2).unwrap().as_str().to_string();
            let initial_value = captures.get(3).map(|m| m.as_str().trim().to_string());

            let is_private = content.contains(&format!("private {}", property_name));
            let is_readonly = content.contains(&format!("readonly {}", property_name));

            component.properties.push(AngularProperty {
                name: property_name,
                property_type,
                initial_value,
                is_private,
                is_readonly,
            });
        }
    }

    fn extract_methods(&self, content: &str, component: &mut AngularComponent) {
        // Extract class methods
        lazy_static! {
            static ref METHOD_REGEX: Regex =
                Regex::new(r"(?:private|protected|public)?\s+([a-zA-Z0-9_]+)\s*\(([^)]*)\)(?:\s*:\s*([a-zA-Z0-9_<>\[\]|]+))?\s*\{([\s\S]*?)\}").unwrap();
        }

        for captures in METHOD_REGEX.captures_iter(content) {
            let method_name = captures.get(1).unwrap().as_str().to_string();

            // Skip lifecycle hooks
            if ["ngOnInit", "ngOnChanges", "ngDoCheck", "ngAfterContentInit",
                "ngAfterContentChecked", "ngAfterViewInit", "ngAfterViewChecked",
                "ngOnDestroy"].contains(&method_name.as_str()) {
                continue;
            }

            // Skip constructor
            if method_name == "constructor" {
                continue;
            }

            let params_str = captures.get(2).map_or("", |m| m.as_str());
            let return_type = captures.get(3).map(|m| m.as_str().to_string());
            let method_body = captures.get(4).unwrap().as_str().trim().to_string();

            let parameters = if params_str.is_empty() {
                Vec::new()
            } else {
                params_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };

            let is_private = content.contains(&format!("private {}", method_name));

            component.methods.push(AngularMethod {
                name: method_name,
                parameters,
                return_type,
                code: method_body,
                is_private,
            });
        }
    }

    fn extract_lifecycle_hooks(&self, content: &str, component: &mut AngularComponent) {
        // Extract lifecycle hooks
        let lifecycle_hooks = vec![
            "ngOnInit", "ngOnChanges", "ngDoCheck", "ngAfterContentInit",
            "ngAfterContentChecked", "ngAfterViewInit", "ngAfterViewChecked",
            "ngOnDestroy"
        ];

        for hook in lifecycle_hooks {
            lazy_static! {
                static ref HOOK_REGEX: Regex =
                    Regex::new(&format!(r"{}\s*\([^)]*\)\s*\{{([\s\S]*?)\}}", hook)).unwrap();
            }

            if let Some(captures) = HOOK_REGEX.captures(content) {
                let hook_body = captures.get(1).unwrap().as_str().trim().to_string();

                component.lifecycle_hooks.push(AngularLifecycleHook {
                    name: hook.to_string(),
                    code: hook_body,
                });
            }
        }

        // Also check for implements OnInit, etc.
        lazy_static! {
            static ref IMPLEMENTS_REGEX: Regex =
                Regex::new(r"implements\s+([^{]+)").unwrap();
        }

        if let Some(captures) = IMPLEMENTS_REGEX.captures(content) {
            let implements_str = captures.get(1).unwrap().as_str();

            // Check for each lifecycle interface
            let lifecycle_interfaces = vec![
                "OnInit", "OnChanges", "DoCheck", "AfterContentInit",
                "AfterContentChecked", "AfterViewInit", "AfterViewChecked",
                "OnDestroy"
            ];

            for interface in lifecycle_interfaces {
                if implements_str.contains(interface) {
                    let hook_name = format!("ng{}", interface);

                    // Skip if already added
                    if !component.lifecycle_hooks.iter().any(|h| h.name == hook_name) {
                        // Try to find the hook implementation
                        lazy_static! {
                            static ref HOOK_IMPL_REGEX: Regex =
                                Regex::new(&format!(r"{}\s*\([^)]*\)\s*\{{([\s\S]*?)\}}", hook_name)).unwrap();
                        }

                        if let Some(captures) = HOOK_IMPL_REGEX.captures(content) {
                            let hook_body = captures.get(1).unwrap().as_str().trim().to_string();

                            component.lifecycle_hooks.push(AngularLifecycleHook {
                                name: hook_name,
                                code: hook_body,
                            });
                        }
                    }
                }
            }
        }
    }

    fn extract_dependencies(&self, content: &str, component: &mut AngularComponent) {
        // Extract dependencies from constructor
        lazy_static! {
            static ref CONSTRUCTOR_REGEX: Regex =
                Regex::new(r"constructor\s*\(([^)]*)\)").unwrap();
        }

        if let Some(captures) = CONSTRUCTOR_REGEX.captures(content) {
            let params_str = captures.get(1).unwrap().as_str();

            // Extract individual parameters
            for param in params_str.split(',') {
                let param = param.trim();
                if param.is_empty() {
                    continue;
                }

                // Check for access modifiers
                let is_required = !param.contains('?');

                // Extract parameter name and type
                lazy_static! {
                    static ref PARAM_REGEX: Regex =
                        Regex::new(r"(?:private|protected|public|readonly)?\s*([a-zA-Z0-9_]+)(?:!|\?)?:\s*([a-zA-Z0-9_<>\[\]|]+)").unwrap();
                }

                if let Some(param_captures) = PARAM_REGEX.captures(param) {
                    let dependency_name = param_captures.get(1).unwrap().as_str().to_string();
                    let dependency_type = param_captures.get(2).unwrap().as_str().to_string();

                    component.dependencies.push(AngularDependency {
                        name: dependency_name,
                        dependency_type,
                        is_required,
                    });
                }
            }
        }
    }

    fn extract_template_dependencies(&self, template_content: &str, component: &mut AngularComponent) {
        // Extract child components from template
        // Look for custom elements with dash in the name (Angular components)
        lazy_static! {
            static ref COMPONENT_REGEX: Regex =
                Regex::new(r"<([a-z][a-z0-9]*-[a-z0-9-]*)(?:\s|>|/)").unwrap();
        }

        let mut child_components = std::collections::HashSet::new();

        for captures in COMPONENT_REGEX.captures_iter(template_content) {
            let component_name = captures.get(1).unwrap().as_str().to_string();
            child_components.insert(component_name);
        }

        component.child_components = child_components.into_iter().collect();

        // Extract directives from template
        // Look for *ngIf, *ngFor, etc.
        lazy_static! {
            static ref DIRECTIVE_REGEX: Regex =
                Regex::new(r"\*([a-zA-Z0-9]+)=").unwrap();
        }

        let mut directives = std::collections::HashSet::new();

        for captures in DIRECTIVE_REGEX.captures_iter(template_content) {
            let directive_name = captures.get(1).unwrap().as_str().to_string();
            directives.insert(directive_name);
        }

        // Also look for [directive] and (directive) syntax
        lazy_static! {
            static ref PROPERTY_DIRECTIVE_REGEX: Regex =
                Regex::new(r"\[([a-zA-Z0-9.]+)\]=").unwrap();
        }

        for captures in PROPERTY_DIRECTIVE_REGEX.captures_iter(template_content) {
            let directive_name = captures.get(1).unwrap().as_str().to_string();
            // Skip standard property bindings like [class], [style], [id], etc.
            if !["class", "style", "id", "hidden", "disabled", "checked", "selected", "value", "href", "src"].contains(&directive_name.as_str()) {
                directives.insert(directive_name);
            }
        }

        component.directives = directives.into_iter().collect();

        // Extract pipes from template
        // Look for | pipe syntax
        lazy_static! {
            static ref PIPE_REGEX: Regex =
                Regex::new(r"\|\s*([a-zA-Z0-9_]+)(?:\s*:\s*|"}}"|\s+|$)").unwrap();
        }

        let mut pipes = std::collections::HashSet::new();

        for captures in PIPE_REGEX.captures_iter(template_content) {
            let pipe_name = captures.get(1).unwrap().as_str().to_string();
            pipes.insert(pipe_name);
        }

        component.pipes = pipes.into_iter().collect();
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.components)
    }
