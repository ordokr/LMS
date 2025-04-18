use crate::analyzers::modules::enhanced_angular_analyzer::{AngularComponent, AngularInput, AngularOutput, AngularProperty, AngularMethod, AngularLifecycleHook, AngularDependency};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use lazy_static::lazy_static;

pub struct AngularToLeptosGenerator {
    pub output_dir: PathBuf,
    pub components_dir: PathBuf,
}

impl AngularToLeptosGenerator {
    pub fn new(output_dir: &Path) -> Self {
        let components_dir = output_dir.join("components");

        Self {
            output_dir: output_dir.to_path_buf(),
            components_dir,
        }
    }

    pub fn generate_component(&self, component: &AngularComponent) -> Result<(), Box<dyn std::error::Error>> {
        // Create components directory if it doesn't exist
        fs::create_dir_all(&self.components_dir)?;

        // Generate file name (snake_case)
        let file_name = to_snake_case(&component.name);
        let file_path = self.components_dir.join(format!("{}.rs", file_name));

        // Generate Leptos component
        let leptos_code = self.generate_leptos_code(component)?;

        // Write to file
        fs::write(&file_path, leptos_code)?;

        // Generate test file
        let test_dir = self.components_dir.join("tests");
        fs::create_dir_all(&test_dir)?;

        let test_file_path = test_dir.join(format!("{}_test.rs", file_name));
        let test_code = self.generate_test_code(component)?;

        // Write test file
        fs::write(test_file_path, test_code)?;

        Ok(())
    }

    fn generate_leptos_code(&self, component: &AngularComponent) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();

        // Add imports
        code.push_str("use leptos::*;\n");
        code.push_str("use leptos_dom::*;\n");
        code.push_str("use leptos_router::*;\n");
        code.push_str("use std::rc::Rc;\n");
        code.push_str("use std::cell::RefCell;\n");
        code.push_str("use serde::{Serialize, Deserialize};\n");

        // Add styling imports
        code.push_str("// Import styling\n");
        code.push_str("use crate::components::styles::use_stylesheet;\n");

        // Add any additional imports based on component dependencies
        let mut additional_imports = HashMap::new();

        // Add imports for child components
        for child_component in &component.child_components {
            let child_module = to_snake_case(&pascal_case_from_kebab(child_component));
            let child_component_name = pascal_case_from_kebab(child_component);
            additional_imports.insert(child_component.clone(), format!("use crate::components::{}::{};\n", child_module, child_component_name));
        }

        // Add the additional imports
        for (_, import) in additional_imports.iter() {
            code.push_str(import);
        }

        code.push_str("\n");

        // Add component props struct
        if !component.inputs.is_empty() {
            code.push_str("#[derive(Props, Clone)]\n");
            code.push_str("pub struct ");
            code.push_str(&component.name);
            code.push_str("Props {\n");

            for input in &component.inputs {
                let input_type = self.map_input_type(&input.input_type);

                if input.required {
                    code.push_str(&format!("    pub {}: {},\n", input.name, input_type));
                } else {
                    code.push_str(&format!("    #[prop(optional)]\n"));
                    code.push_str(&format!("    pub {}: Option<{}>,\n", input.name, input_type));
                }
            }

            // Add children prop if component likely accepts ng-content
            if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("<ng-content>")) {
                code.push_str("    #[prop(optional)]\n");
                code.push_str("    pub children: Option<Children>,\n");
            }

            code.push_str("}\n\n");
        }

        // Add component function
        code.push_str("#[component]\n");
        code.push_str("pub fn ");
        code.push_str(&component.name);

        if !component.inputs.is_empty() {
            code.push_str("(cx: Scope, props: ");
            code.push_str(&component.name);
            code.push_str("Props");
        } else {
            code.push_str("(cx: Scope");
        }

        code.push_str(") -> impl IntoView {\n");

        // Add state signals
        for property in &component.properties {
            if property.is_private {
                continue; // Skip private properties for now
            }

            let property_type = self.map_property_type(&property.property_type);
            let initial_value = property.initial_value.as_ref().map_or("Default::default()", |v| v.as_str());

            code.push_str(&format!("    let ({}, set_{}) = create_signal(cx, {});\n",
                property.name, property.name, initial_value));
        }

        // Add output event emitters
        for output in &component.outputs {
            code.push_str(&format!("    let (on_{}, set_on_{}) = create_signal(cx, None);\n",
                output.name, output.name));
        }

        // Add getters as memos
        for method in &component.methods {
            if method.name.starts_with("get") && method.parameters.is_empty() {
                let getter_name = method.name[3..].to_lowercase();

                code.push_str(&format!("    let {} = create_memo(cx, move |_| {{\n", getter_name));

                // Add method code as comment
                for line in method.code.lines() {
                    code.push_str(&format!("        // {}\n", line.trim()));
                }

                // Add placeholder implementation
                code.push_str("        // TODO: Implement getter\n");

                // Try to convert the getter logic
                if method.code.contains("return") {
                    // Extract the return statement
                    if let Some(return_stmt) = Regex::new(r"return\s+([^;]+);").unwrap().captures(&method.code) {
                        let return_expr = return_stmt.get(1).unwrap().as_str();
                        let converted_expr = self.convert_angular_expression(return_expr, component);
                        code.push_str(&format!("        {}\n", converted_expr));
                    } else {
                        code.push_str("        String::new() // Placeholder return value\n");
                    }
                } else {
                    code.push_str("        String::new() // Placeholder return value\n");
                }

                code.push_str("    });\n\n");
            }
        }

        // Add lifecycle hooks
        if component.lifecycle_hooks.iter().any(|h| h.name == "ngOnInit" || h.name == "ngAfterViewInit") {
            code.push_str("    // Initialize component\n");
            code.push_str("    let _ = use_effect(cx, (), |_| {\n");

            // Add ngOnInit/ngAfterViewInit code as comment
            let on_init_hook = component.lifecycle_hooks.iter().find(|h| h.name == "ngOnInit");
            if let Some(hook) = on_init_hook {
                code.push_str("        // ngOnInit lifecycle hook\n");
                for line in hook.code.lines() {
                    code.push_str(&format!("        // {}\n", line.trim()));
                }
            }

            let after_view_init_hook = component.lifecycle_hooks.iter().find(|h| h.name == "ngAfterViewInit");
            if let Some(hook) = after_view_init_hook {
                code.push_str("        // ngAfterViewInit lifecycle hook\n");
                for line in hook.code.lines() {
                    code.push_str(&format!("        // {}\n", line.trim()));
                }
            }

            code.push_str("        // TODO: Implement initialization\n");

            // Add cleanup for ngOnDestroy
            let destroy_hook = component.lifecycle_hooks.iter().find(|h| h.name == "ngOnDestroy");
            if let Some(hook) = destroy_hook {
                code.push_str("        || {\n");
                code.push_str("            // ngOnDestroy lifecycle hook\n");
                for line in hook.code.lines() {
                    code.push_str(&format!("            // {}\n", line.trim()));
                }
                code.push_str("            // TODO: Implement cleanup\n");
                code.push_str("        }\n");
            } else {
                code.push_str("        || {}\n");
            }

            code.push_str("    });\n\n");
        }

        // Add methods as event handlers
        for method in &component.methods {
            // Skip getters
            if method.name.starts_with("get") && method.parameters.is_empty() {
                continue;
            }

            // Skip lifecycle hooks
            if ["ngOnInit", "ngOnChanges", "ngDoCheck", "ngAfterContentInit",
                "ngAfterContentChecked", "ngAfterViewInit", "ngAfterViewChecked",
                "ngOnDestroy"].contains(&method.name.as_str()) {
                continue;
            }

            code.push_str(&format!("    let {} = move |", method.name));

            // Add parameters
            if !method.parameters.is_empty() {
                code.push_str(&method.parameters.join(", "));
            }

            code.push_str("| {\n");

            // Add method code as comment
            for line in method.code.lines() {
                code.push_str(&format!("        // {}\n", line.trim()));
            }

            // Add placeholder implementation
            code.push_str("        // TODO: Implement method\n");

            // For state updates, add signal updates
            for property in &component.properties {
                if method.code.contains(&format!("this.{} =", property.name)) {
                    code.push_str(&format!("        // set_{}(new_value);\n", property.name));
                }
            }

            // For event emitters, add signal updates
            for output in &component.outputs {
                if method.code.contains(&format!("this.{}.emit", output.name)) {
                    code.push_str(&format!("        // set_on_{}(Some(event_data));\n", output.name));
                }
            }

            code.push_str("    };\n\n");
        }

        // Add stylesheet
        code.push_str("    // Apply component styles\n");
        code.push_str("    use_stylesheet(cx);\n\n");

        // Add view
        code.push_str("    view! { cx,\n");

        // Convert template to Leptos view
        if let Some(template) = &component.template_content {
            let leptos_view = self.convert_template_to_leptos(template, component);
            code.push_str(&leptos_view);
        } else {
            code.push_str("        <div class=\"card w-full bg-base-100 shadow-xl\">\n");
            code.push_str("            <div class=\"card-body\">\n");
            code.push_str("                <h2 class=\"card-title\">{&component.name}</h2>\n");
            code.push_str("                <p>\"TODO: Implement component view\"</p>\n");
            code.push_str("                <div class=\"card-actions justify-end\">\n");
            code.push_str("                    <button class=\"btn btn-primary\">\"Button\"</button>\n");
            code.push_str("                </div>\n");
            code.push_str("            </div>\n");
            code.push_str("        </div>\n");
        }

        code.push_str("    }\n");
        code.push_str("}\n");

        Ok(code)
    }

    fn map_input_type(&self, angular_type: &str) -> String {
        // Check for array types first
        if (angular_type.starts_with("Array<") && angular_type.ends_with(">")) || angular_type.ends_with("[]") {
            let inner_type = angular_type
                .trim_start_matches("Array<")
                .trim_end_matches(">")
                .trim_end_matches("[]");

            return format!("Vec<{}>", self.map_input_type(inner_type));
        }

        // Check for object types with specific fields
        if angular_type.starts_with("{") && angular_type.ends_with("}") {
            return "serde_json::Value".to_string(); // For complex objects, use serde_json::Value
        }

        // Check for union types
        if angular_type.contains("|") {
            let types: Vec<&str> = angular_type.split('|').map(|t| t.trim()).collect();
            if types.contains(&"null") || types.contains(&"undefined") {
                // If union includes null/undefined, make it an Option
                let non_null_types: Vec<&str> = types.iter()
                    .filter(|&&t| t != "null" && t != "undefined")
                    .copied()
                    .collect();

                if non_null_types.len() == 1 {
                    return format!("Option<{}>", self.map_input_type(non_null_types[0]));
                } else {
                    // For multiple non-null types, use an enum or Any
                    return "Option<Box<dyn std::any::Any>>".to_string();
                }
            } else {
                // For unions without null, use an enum or Any
                return "Box<dyn std::any::Any>".to_string();
            }
        }

        // Handle generic types
        if angular_type.contains("<") && angular_type.contains(">") {
            let base_type = angular_type.split('<').next().unwrap();
            let generic_part = angular_type
                .trim_start_matches(base_type)
                .trim_start_matches("<")
                .trim_end_matches(">");

            match base_type {
                "Map" | "Record" => return format!("std::collections::HashMap<String, {}>", self.map_input_type(generic_part)),
                "Set" => return format!("std::collections::HashSet<{}>", self.map_input_type(generic_part)),
                "Promise" => return format!("futures::future::BoxFuture<'static, Result<{}, String>>", self.map_input_type(generic_part)),
                "Observable" => return format!("leptos::RwSignal<{}>", self.map_input_type(generic_part)),
                _ => return format!("{}<{}>", base_type, self.map_input_type(generic_part)),
            }
        }

        // Handle basic types
        match angular_type {
            "string" => "String".to_string(),
            "number" | "int" | "float" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "any" => "serde_json::Value".to_string(),
            "void" => "()".to_string(),
            "Date" | "date" => "chrono::DateTime<chrono::Utc>".to_string(),
            "object" => "serde_json::Value".to_string(),
            "Function" | "function" => "Callback<()>".to_string(),
            // Model relationships
            t if t.ends_with("Model") || t.ends_with("Entity") || t.ends_with("DTO") => {
                format!("Rc<{0}>", t)
            },
            // Collection relationships
            t if t.contains("Model[]") || t.contains("Entity[]") || t.contains("DTO[]") => {
                let base_type = t.trim_end_matches("[]");
                format!("Vec<Rc<{}>>", base_type)
            },
            _ => "String".to_string(), // Default to String for unknown types
        }
    }

    fn map_property_type(&self, angular_type: &str) -> String {
        // Reuse the input type mapping logic for consistency
        self.map_input_type(angular_type)
    }

    fn convert_template_to_leptos(&self, template: &str, component: &AngularComponent) -> String {
        let mut leptos_view = String::new();

        // Simple Angular template to Leptos conversion
        // This is a basic implementation and would need to be enhanced for complex templates

        // Process each line
        for line in template.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                leptos_view.push_str("        \n");
                continue;
            }

            // Convert <ng-content> to {children}
            if trimmed.contains("<ng-content>") || trimmed.contains("<ng-content/>") || trimmed.contains("<ng-content />") {
                let converted = trimmed
                    .replace("<ng-content>", "{children}")
                    .replace("</ng-content>", "")
                    .replace("<ng-content/>", "{children}")
                    .replace("<ng-content />", "{children}");
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert *ngIf to <Show>
            lazy_static! {
                static ref NG_IF_REGEX: Regex = Regex::new(r#"\*ngIf="([^"]+)""#).unwrap();
            }

            if NG_IF_REGEX.is_match(trimmed) {
                let captures = NG_IF_REGEX.captures(trimmed).unwrap();
                let condition = captures.get(1).unwrap().as_str();

                let tag_start = if let Some(tag_match) = Regex::new(r"<([a-zA-Z0-9_-]+)").unwrap().captures(trimmed) {
                    tag_match.get(1).unwrap().as_str()
                } else {
                    "div"
                };

                let converted = NG_IF_REGEX.replace(trimmed, "");
                let converted = converted.replace(&format!("<{}", tag_start), &format!("<Show when=move || {} fallback=|_| view! {{ cx, }}>", self.convert_angular_expression(condition, component)));
                let converted = converted.replace(&format!("</{}>", tag_start), "</Show>");

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert *ngFor to <For>
            lazy_static! {
                static ref NG_FOR_REGEX: Regex = Regex::new(r#"\*ngFor="let ([a-zA-Z0-9_]+) of ([^"]+)""#).unwrap();
            }

            if NG_FOR_REGEX.is_match(trimmed) {
                let captures = NG_FOR_REGEX.captures(trimmed).unwrap();
                let item = captures.get(1).unwrap().as_str();
                let items = captures.get(2).unwrap().as_str();

                let tag_start = if let Some(tag_match) = Regex::new(r"<([a-zA-Z0-9_-]+)").unwrap().captures(trimmed) {
                    tag_match.get(1).unwrap().as_str()
                } else {
                    "div"
                };

                let converted = NG_FOR_REGEX.replace(trimmed, "");
                let converted = converted.replace(&format!("<{}", tag_start),
                    &format!("<For each=move || {} key=|{}| {}.id let:{}>",
                        self.convert_angular_expression(items, component),
                        item,
                        item,
                        item));
                let converted = converted.replace(&format!("</{}>", tag_start), "</For>");

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert [ngClass] to class
            lazy_static! {
                static ref NG_CLASS_REGEX: Regex = Regex::new(r#"\[ngClass]="([^"]+)""#).unwrap();
            }

            if NG_CLASS_REGEX.is_match(trimmed) {
                let captures = NG_CLASS_REGEX.captures(trimmed).unwrap();
                let class_expr = captures.get(1).unwrap().as_str();

                let converted = NG_CLASS_REGEX.replace(trimmed, &format!("class={{{}}}", self.convert_angular_expression(class_expr, component)));
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert [(ngModel)] to value and on:input
            lazy_static! {
                static ref NG_MODEL_REGEX: Regex = Regex::new(r#"\[\(ngModel\)]="([^"]+)""#).unwrap();
            }

            if NG_MODEL_REGEX.is_match(trimmed) {
                let captures = NG_MODEL_REGEX.captures(trimmed).unwrap();
                let model = captures.get(1).unwrap().as_str();

                let converted = NG_MODEL_REGEX.replace(trimmed, &format!("value={{{0}()}} on:input=move |ev| set_{0}(event_target_value(&ev))", model));
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert other [] bindings
            lazy_static! {
                static ref BINDING_REGEX: Regex = Regex::new(r#"\[([a-zA-Z0-9_-]+)]="([^"]+)""#).unwrap();
            }

            if BINDING_REGEX.is_match(trimmed) {
                let mut converted = trimmed.to_string();

                for captures in BINDING_REGEX.captures_iter(trimmed) {
                    let attr_name = captures.get(1).unwrap().as_str();
                    let attr_value = captures.get(2).unwrap().as_str();

                    let binding = format!("[{}]=\"{}\"", attr_name, attr_value);
                    let replacement = format!("{}={{{}}}", attr_name, self.convert_angular_expression(attr_value, component));

                    converted = converted.replace(&binding, &replacement);
                }

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert () event handlers
            lazy_static! {
                static ref EVENT_REGEX: Regex = Regex::new(r#"\(([a-zA-Z0-9_-]+)\)="([^"]+)""#).unwrap();
            }

            if EVENT_REGEX.is_match(trimmed) {
                let mut converted = trimmed.to_string();

                for captures in EVENT_REGEX.captures_iter(trimmed) {
                    let event_name = captures.get(1).unwrap().as_str();
                    let handler = captures.get(2).unwrap().as_str();

                    let event_binding = format!("({}]=\"{}\"", event_name, handler);
                    let replacement = format!("on:{}={}", event_name, self.convert_angular_expression(handler, component));

                    converted = converted.replace(&event_binding, &replacement);
                }

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert {{ expressions }}
            lazy_static! {
                static ref MUSTACHE_REGEX: Regex = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
            }

            if MUSTACHE_REGEX.is_match(trimmed) {
                let mut converted = trimmed.to_string();

                for captures in MUSTACHE_REGEX.captures_iter(trimmed) {
                    let expr = captures.get(1).unwrap().as_str().trim();

                    let mustache = format!("{{{{{}}}}}", expr);
                    let replacement = format!("{{{}}}", self.convert_angular_expression(expr, component));

                    converted = converted.replace(&mustache, &replacement);
                }

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert pipes
            lazy_static! {
                static ref PIPE_REGEX: Regex = Regex::new(r"([^|]+)\s*\|\s*([a-zA-Z0-9_]+)(?::([^|]+))?").unwrap();
            }

            if PIPE_REGEX.is_match(trimmed) {
                let mut converted = trimmed.to_string();

                for captures in PIPE_REGEX.captures_iter(trimmed) {
                    let expr = captures.get(1).unwrap().as_str().trim();
                    let pipe = captures.get(2).unwrap().as_str();
                    let args = captures.get(3).map_or("", |m| m.as_str());

                    let pipe_expr = if args.is_empty() {
                        format!("{} | {}", expr, pipe)
                    } else {
                        format!("{} | {}:{}", expr, pipe, args)
                    };

                    let replacement = match pipe {
                        "uppercase" => format!("{}.to_uppercase()", self.convert_angular_expression(expr, component)),
                        "lowercase" => format!("{}.to_lowercase()", self.convert_angular_expression(expr, component)),
                        "date" => format!("format_date({}, \"{}\")", self.convert_angular_expression(expr, component), args),
                        "json" => format!("serde_json::to_string(&{}).unwrap_or_default()", self.convert_angular_expression(expr, component)),
                        _ => format!("{}_{}_pipe({})", pipe, args.replace(" ", "_"), self.convert_angular_expression(expr, component)),
                    };

                    converted = converted.replace(&pipe_expr, &replacement);
                }

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Pass through other HTML
            leptos_view.push_str(&format!("        {}\n", trimmed));
        }

        leptos_view
    }

    fn convert_angular_expression(&self, expr: &str, component: &AngularComponent) -> String {
        let expr = expr.trim();

        // Convert this.property to property()
        if expr.starts_with("this.") {
            let property = &expr[5..];

            // Check if it's a property
            if component.properties.iter().any(|p| p.name == property) {
                return format!("{}()", property);
            }

            // Check if it's an input
            if component.inputs.iter().any(|i| i.name == property) {
                return format!("props.{}", property);
            }

            // Otherwise, just return as is
            return property.to_string();
        }

        // Handle simple property references
        if component.properties.iter().any(|p| p.name == expr) {
            return format!("{}()", expr);
        }

        // Handle inputs
        if component.inputs.iter().any(|i| i.name == expr) {
            return format!("props.{}", expr);
        }

        // Handle method calls
        if let Some(method_call) = Regex::new(r"([a-zA-Z0-9_]+)\(([^)]*)\)").unwrap().captures(expr) {
            let method_name = method_call.get(1).unwrap().as_str();
            let args = method_call.get(2).unwrap().as_str();

            if component.methods.iter().any(|m| m.name == method_name) {
                return format!("{}({})", method_name, args);
            }
        }

        // Pass through other expressions
        expr.to_string()
    }
}

    fn generate_test_code(&self, component: &AngularComponent) -> Result<String, Box<dyn std::error::Error>> {
        let mut code = String::new();
        let component_name = &component.name;
        let file_name = to_snake_case(component_name);

        // Add imports
        code.push_str("use leptos::*;\n");
        code.push_str("use leptos_dom::*;\n");
        code.push_str("use leptos_meta::*;\n");
        code.push_str("use wasm_bindgen_test::*;\n");
        code.push_str("use wasm_bindgen::JsCast;\n");
        code.push_str("use web_sys::{HtmlElement, HtmlInputElement};\n");
        code.push_str("use std::rc::Rc;\n\n");

        // Import the component
        code.push_str(&format!("use crate::components::{}::{};", file_name, component_name));
        code.push_str("\n\n");

        // Add wasm_bindgen_test_configure
        code.push_str("wasm_bindgen_test_configure!(run_in_browser);\n\n");

        // Add basic render test
        code.push_str(&format!("#[wasm_bindgen_test]\n"));
        code.push_str(&format!("fn test_{}_renders() {{\n", file_name));
        code.push_str("    // Create a test mount point\n");
        code.push_str("    let document = web_sys::window().unwrap().document().unwrap();\n");
        code.push_str("    let mount = document.create_element(\"div\").unwrap();\n");
        code.push_str("    document.body().unwrap().append_child(&mount).unwrap();\n\n");

        // Create test props if needed
        if !component.inputs.is_empty() {
            code.push_str("    // Create test props\n");
            code.push_str(&format!("    let props = {}Props {{\n", component_name));

            for input in &component.inputs {
                let input_name = &input.name;
                let input_type = self.map_input_type(&input.input_type);

                if input.required {
                    // Generate a default value based on type
                    let default_value = match input_type.as_str() {
                        "String" => "\"test\".to_string()",
                        "f64" => "0.0",
                        "bool" => "false",
                        t if t.starts_with("Vec<") => "vec![]",
                        t if t.starts_with("Option<") => "None",
                        _ => "Default::default()",
                    };

                    code.push_str(&format!("        {}: {},\n", input_name, default_value));
                } else {
                    code.push_str(&format!("        {}: None,\n", input_name));
                }
            }

            // Add children if needed
            if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("<ng-content>")) {
                code.push_str("        children: None,\n");
            }

            code.push_str("    };\n\n");
        }

        // Mount the component
        if !component.inputs.is_empty() {
            code.push_str(&format!("    mount_to(mount, |cx| view! {{ cx, <{} props={{props}} /> }});\n\n", component_name));
        } else {
            code.push_str(&format!("    mount_to(mount, |cx| view! {{ cx, <{}  /> }});\n\n", component_name));
        }

        // Add assertions
        code.push_str("    // Basic assertion that component rendered\n");
        code.push_str("    let component_el = document.query_selector(\"div\").unwrap().unwrap();\n");
        code.push_str("    assert!(component_el.is_instance_of::<HtmlElement>());\n");

        // Add interaction tests if there are methods
        if !component.methods.is_empty() {
            let method = &component.methods[0];
            if method.name.starts_with("handle") || method.name.contains("click") {
                code.push_str("\n    // Test interaction\n");
                code.push_str("    let button = document.query_selector(\"button\");\n");
                code.push_str("    if let Some(button) = button {\n");
                code.push_str("        // Simulate click\n");
                code.push_str("        let event = web_sys::Event::new(\"click\").unwrap();\n");
                code.push_str("        button.dispatch_event(&event).unwrap();\n");
                code.push_str("        // Add assertions for expected behavior after click\n");
                code.push_str("    }\n");
            }
        }

        code.push_str("}\n\n");

        // Add inputs test if there are inputs
        if !component.inputs.is_empty() {
            code.push_str(&format!("#[wasm_bindgen_test]\n"));
            code.push_str(&format!("fn test_{}_inputs() {{\n", file_name));
            code.push_str("    // Create a test mount point\n");
            code.push_str("    let document = web_sys::window().unwrap().document().unwrap();\n");
            code.push_str("    let mount = document.create_element(\"div\").unwrap();\n");
            code.push_str("    document.body().unwrap().append_child(&mount).unwrap();\n\n");

            // Create test props with specific values
            code.push_str("    // Create test props with specific values\n");
            code.push_str(&format!("    let props = {}Props {{\n", component_name));

            for input in &component.inputs {
                let input_name = &input.name;
                let input_type = self.map_input_type(&input.input_type);

                if input.required {
                    // Generate a specific test value based on type
                    let test_value = match input_type.as_str() {
                        "String" => "\"test-value\".to_string()",
                        "f64" => "42.0",
                        "bool" => "true",
                        t if t.starts_with("Vec<") => "vec![\"test\".to_string()]",
                        _ => "Default::default()",
                    };

                    code.push_str(&format!("        {}: {},\n", input_name, test_value));
                } else {
                    code.push_str(&format!("        {}: Some(\"test-value\".to_string()),\n", input_name));
                }
            }

            // Add children if needed
            if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("<ng-content>")) {
                code.push_str("        children: Some(|cx| view! { cx, <p>\"Test Child\"</p> }),\n");
            }

            code.push_str("    };\n\n");

            // Mount the component
            code.push_str(&format!("    mount_to(mount, |cx| view! {{ cx, <{} props={{props}} /> }});\n\n", component_name));

            // Add assertions for inputs
            code.push_str("    // Test that inputs were applied\n");
            code.push_str("    let component_el = document.query_selector(\"div\").unwrap().unwrap();\n");
            code.push_str("    // Add specific assertions for inputs\n");

            code.push_str("}\n");
        }

        // Add outputs test if there are outputs
        if !component.outputs.is_empty() {
            code.push_str(&format!("\n#[wasm_bindgen_test]\n"));
            code.push_str(&format!("fn test_{}_outputs() {{\n", file_name));
            code.push_str("    // Create a test mount point\n");
            code.push_str("    let document = web_sys::window().unwrap().document().unwrap();\n");
            code.push_str("    let mount = document.create_element(\"div\").unwrap();\n");
            code.push_str("    document.body().unwrap().append_child(&mount).unwrap();\n\n");

            // Create test props
            if !component.inputs.is_empty() {
                code.push_str("    // Create test props\n");
                code.push_str(&format!("    let props = {}Props {{\n", component_name));

                for input in &component.inputs {
                    let input_name = &input.name;
                    code.push_str(&format!("        {}: None,\n", input_name));
                }

                // Add children if needed
                if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("<ng-content>")) {
                    code.push_str("        children: None,\n");
                }

                code.push_str("    };\n\n");
            }

            // Mount the component
            if !component.inputs.is_empty() {
                code.push_str(&format!("    mount_to(mount, |cx| view! {{ cx, <{} props={{props}} /> }});\n\n", component_name));
            } else {
                code.push_str(&format!("    mount_to(mount, |cx| view! {{ cx, <{}  /> }});\n\n", component_name));
            }

            // Add test for outputs
            code.push_str("    // Test outputs\n");
            code.push_str("    let button = document.query_selector(\"button\");\n");
            code.push_str("    if let Some(button) = button {\n");
            code.push_str("        // Set up event listener to capture output event\n");
            code.push_str("        // This is a simplified example - in a real test, you would need to set up\n");
            code.push_str("        // proper event listeners and assertions\n");
            code.push_str("        let event = web_sys::Event::new(\"click\").unwrap();\n");
            code.push_str("        button.dispatch_event(&event).unwrap();\n");
            code.push_str("    }\n");

            code.push_str("}\n");
        }

        Ok(code)
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

// Helper function to convert kebab-case to PascalCase
fn pascal_case_from_kebab(kebab_case: &str) -> String {
    let mut pascal_case = String::new();

    let mut capitalize_next = true;
    for c in kebab_case.chars() {
        if c == '-' {
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
