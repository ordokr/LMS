use crate::analyzers::modules::enhanced_vue_analyzer::{VueComponent, VueProp, VueData, VueComputed, VueMethod, VueWatch, VueLifecycleHook};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use lazy_static::lazy_static;

pub struct VueToLeptosGenerator {
    pub output_dir: PathBuf,
    pub components_dir: PathBuf,
}

impl VueToLeptosGenerator {
    pub fn new(output_dir: &Path) -> Self {
        let components_dir = output_dir.join("components");

        Self {
            output_dir: output_dir.to_path_buf(),
            components_dir,
        }
    }

    pub fn generate_component(&self, component: &VueComponent) -> Result<(), Box<dyn std::error::Error>> {
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

    fn generate_leptos_code(&self, component: &VueComponent) -> Result<String, Box<dyn std::error::Error>> {
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
        for (name, component_ref) in &component.components {
            let child_module = to_snake_case(component_ref);
            additional_imports.insert(component_ref.clone(), format!("use crate::components::{}::{};\n", child_module, component_ref));
        }

        // Add the additional imports
        for (_, import) in additional_imports.iter() {
            code.push_str(import);
        }

        code.push_str("\n");

        // Add component props struct
        if !component.props.is_empty() {
            code.push_str("#[derive(Props, Clone)]\n");
            code.push_str("pub struct ");
            code.push_str(&component.name);
            code.push_str("Props {\n");

            for prop in &component.props {
                let prop_type = self.map_prop_type(&prop.prop_type);

                if prop.required {
                    code.push_str(&format!("    pub {}: {},\n", prop.name, prop_type));
                } else {
                    code.push_str(&format!("    #[prop(optional)]\n"));
                    code.push_str(&format!("    pub {}: Option<{}>,\n", prop.name, prop_type));
                }
            }

            // Add children prop if component likely accepts children
            if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("<slot>")) {
                code.push_str("    #[prop(optional)]\n");
                code.push_str("    pub children: Option<Children>,\n");
            }

            code.push_str("}\n\n");
        }

        // Add component function
        code.push_str("#[component]\n");
        code.push_str("pub fn ");
        code.push_str(&component.name);

        if !component.props.is_empty() {
            code.push_str("(cx: Scope, props: ");
            code.push_str(&component.name);
            code.push_str("Props");
        } else {
            code.push_str("(cx: Scope");
        }

        code.push_str(") -> impl IntoView {\n");

        // Add state signals
        for data in &component.data {
            let data_type = self.map_data_type(&data.data_type);
            let initial_value = data.initial_value.as_ref().map_or("Default::default()", |v| v.as_str());

            code.push_str(&format!("    let ({}, set_{}) = create_signal(cx, {});\n",
                data.name, data.name, initial_value));
        }

        // Add computed properties as memos
        for computed in &component.computed {
            code.push_str(&format!("    let {} = create_memo(cx, move |_| {{\n", computed.name));

            // Add computed code as comment
            for line in computed.getter.lines() {
                code.push_str(&format!("        // {}\n", line.trim()));
            }

            // Add placeholder implementation
            code.push_str("        // TODO: Implement computed property\n");

            // Try to convert the computed property logic
            if computed.getter.contains("return") {
                // Extract the return statement
                if let Some(return_stmt) = Regex::new(r"return\s+([^;]+);").unwrap().captures(&computed.getter) {
                    let return_expr = return_stmt.get(1).unwrap().as_str();
                    let converted_expr = self.convert_vue_expression(return_expr, component);
                    code.push_str(&format!("        {}\n", converted_expr));
                } else {
                    code.push_str("        String::new() // Placeholder return value\n");
                }
            } else {
                code.push_str("        String::new() // Placeholder return value\n");
            }

            code.push_str("    });\n\n");
        }

        // Add watches as effects
        for watch in &component.watches {
            code.push_str(&format!("    // Watch for changes to {}\n", watch.target));
            code.push_str("    create_effect(cx, move |_| {\n");

            // Add dependencies
            if component.data.iter().any(|d| d.name == watch.target) {
                code.push_str(&format!("        let _ = {}();\n", watch.target));
            } else if component.props.iter().any(|p| p.name == watch.target) {
                code.push_str(&format!("        let _ = props.{};\n", watch.target));
            }

            // Add watch handler code as comment
            for line in watch.handler.lines() {
                code.push_str(&format!("        // {}\n", line.trim()));
            }

            // Add placeholder implementation
            code.push_str("        // TODO: Implement watch handler\n");

            code.push_str("    });\n\n");
        }

        // Add lifecycle hooks
        if component.lifecycle_hooks.iter().any(|h| h.name == "created" || h.name == "mounted") {
            code.push_str("    // Initialize component\n");
            code.push_str("    let _ = use_effect(cx, (), |_| {\n");

            // Add created/mounted code as comment
            let created_hook = component.lifecycle_hooks.iter().find(|h| h.name == "created");
            if let Some(hook) = created_hook {
                code.push_str("        // created lifecycle hook\n");
                for line in hook.code.lines() {
                    code.push_str(&format!("        // {}\n", line.trim()));
                }
            }

            let mounted_hook = component.lifecycle_hooks.iter().find(|h| h.name == "mounted");
            if let Some(hook) = mounted_hook {
                code.push_str("        // mounted lifecycle hook\n");
                for line in hook.code.lines() {
                    code.push_str(&format!("        // {}\n", line.trim()));
                }
            }

            code.push_str("        // TODO: Implement initialization\n");

            // Add cleanup for beforeDestroy/destroyed
            let destroy_hook = component.lifecycle_hooks.iter().find(|h| h.name == "beforeDestroy" || h.name == "destroyed");
            if let Some(hook) = destroy_hook {
                code.push_str("        || {\n");
                code.push_str(&format!("            // {} lifecycle hook\n", hook.name));
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
            for data in &component.data {
                if method.code.contains(&format!("this.{} =", data.name)) {
                    code.push_str(&format!("        // set_{}(new_value);\n", data.name));
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

    fn map_prop_type(&self, vue_type: &str) -> String {
        // Check for array types first
        if vue_type.starts_with("Array<") && vue_type.ends_with(">") {
            let inner_type = &vue_type[6..vue_type.len()-1];
            return format!("Vec<{}>", self.map_prop_type(inner_type));
        }

        // Check for object types with specific fields
        if vue_type.starts_with("{") && vue_type.ends_with("}") {
            return "serde_json::Value".to_string(); // For complex objects, use serde_json::Value
        }

        // Check for union types
        if vue_type.contains("|") {
            let types: Vec<&str> = vue_type.split('|').map(|t| t.trim()).collect();
            if types.contains(&"null") || types.contains(&"undefined") {
                // If union includes null/undefined, make it an Option
                let non_null_types: Vec<&str> = types.iter()
                    .filter(|&&t| t != "null" && t != "undefined")
                    .copied()
                    .collect();

                if non_null_types.len() == 1 {
                    return format!("Option<{}>", self.map_prop_type(non_null_types[0]));
                } else {
                    // For multiple non-null types, use an enum or Any
                    return "Option<Box<dyn std::any::Any>>".to_string();
                }
            } else {
                // For unions without null, use an enum or Any
                return "Box<dyn std::any::Any>".to_string();
            }
        }

        // Handle basic types
        match vue_type {
            "String" => "String".to_string(),
            "Number" => "f64".to_string(),
            "Boolean" | "boolean" => "bool".to_string(),
            "Array" => "Vec<serde_json::Value>".to_string(),
            "Object" => "serde_json::Value".to_string(),
            "Function" => "Callback<()>".to_string(),
            "Date" => "chrono::DateTime<chrono::Utc>".to_string(),
            "Promise" => "futures::future::BoxFuture<'static, Result<serde_json::Value, String>>".to_string(),
            "Map" => "std::collections::HashMap<String, serde_json::Value>".to_string(),
            "Set" => "std::collections::HashSet<String>".to_string(),
            // Model relationships
            t if t.ends_with("Model") || t.ends_with("Entity") => {
                format!("Rc<{0}>", t)
            },
            // Collection relationships
            t if t.starts_with("Vec<") && (t.contains("Model") || t.contains("Entity")) => {
                t.to_string()
            },
            _ => "String".to_string(), // Default to String for unknown types
        }
    }

    fn map_data_type(&self, vue_type: &str) -> String {
        // Reuse the prop type mapping logic for consistency
        self.map_prop_type(vue_type)
    }

    fn convert_template_to_leptos(&self, template: &str, component: &VueComponent) -> String {
        let mut leptos_view = String::new();

        // Simple Vue template to Leptos conversion
        // This is a basic implementation and would need to be enhanced for complex templates

        // Process each line
        for line in template.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                leptos_view.push_str("        \n");
                continue;
            }

            // Convert <slot> to {children}
            if trimmed.contains("<slot>") || trimmed.contains("<slot/>") || trimmed.contains("<slot />") {
                let converted = trimmed
                    .replace("<slot>", "{children}")
                    .replace("</slot>", "")
                    .replace("<slot/>", "{children}")
                    .replace("<slot />", "{children}");
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert v-if to <Show>
            lazy_static! {
                static ref V_IF_REGEX: Regex = Regex::new(r#"v-if="([^"]+)""#).unwrap();
            }

            if V_IF_REGEX.is_match(trimmed) {
                let captures = V_IF_REGEX.captures(trimmed).unwrap();
                let condition = captures.get(1).unwrap().as_str();

                let tag_start = if let Some(tag_match) = Regex::new(r"<([a-zA-Z0-9_-]+)").unwrap().captures(trimmed) {
                    tag_match.get(1).unwrap().as_str()
                } else {
                    "div"
                };

                let converted = V_IF_REGEX.replace(trimmed, "");
                let converted = converted.replace(&format!("<{}", tag_start), &format!("<Show when=move || {} fallback=|_| view! {{ cx, }}>", self.convert_vue_expression(condition, component)));
                let converted = converted.replace(&format!("</{}>", tag_start), "</Show>");

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert v-for to <For>
            lazy_static! {
                static ref V_FOR_REGEX: Regex = Regex::new(r#"v-for="([^"]+)""#).unwrap();
            }

            if V_FOR_REGEX.is_match(trimmed) {
                let captures = V_FOR_REGEX.captures(trimmed).unwrap();
                let for_expr = captures.get(1).unwrap().as_str();

                let tag_start = if let Some(tag_match) = Regex::new(r"<([a-zA-Z0-9_-]+)").unwrap().captures(trimmed) {
                    tag_match.get(1).unwrap().as_str()
                } else {
                    "div"
                };

                // Parse v-for expression (item in items)
                let parts: Vec<&str> = for_expr.split(" in ").collect();
                if parts.len() == 2 {
                    let item = parts[0].trim();
                    let items = parts[1].trim();

                    let converted = V_FOR_REGEX.replace(trimmed, "");
                    let converted = converted.replace(&format!("<{}", tag_start),
                        &format!("<For each=move || {} key=|{}| {}.id let:{}>",
                            self.convert_vue_expression(items, component),
                            item,
                            item,
                            item));
                    let converted = converted.replace(&format!("</{}>", tag_start), "</For>");

                    leptos_view.push_str(&format!("        {}\n", converted));
                    continue;
                }
            }

            // Convert v-model to value and on:input
            lazy_static! {
                static ref V_MODEL_REGEX: Regex = Regex::new(r#"v-model="([^"]+)""#).unwrap();
            }

            if V_MODEL_REGEX.is_match(trimmed) {
                let captures = V_MODEL_REGEX.captures(trimmed).unwrap();
                let model = captures.get(1).unwrap().as_str();

                let converted = V_MODEL_REGEX.replace(trimmed, &format!("value={{{0}()}} on:input=move |ev| set_{0}(event_target_value(&ev))", model));
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert :class binding
            lazy_static! {
                static ref CLASS_BINDING_REGEX: Regex = Regex::new(r#":class="([^"]+)""#).unwrap();
            }

            if CLASS_BINDING_REGEX.is_match(trimmed) {
                let captures = CLASS_BINDING_REGEX.captures(trimmed).unwrap();
                let class_expr = captures.get(1).unwrap().as_str();

                let converted = CLASS_BINDING_REGEX.replace(trimmed, &format!("class={{{}}}", self.convert_vue_expression(class_expr, component)));
                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert other : bindings
            lazy_static! {
                static ref BINDING_REGEX: Regex = Regex::new(r#":([a-zA-Z0-9_-]+)="([^"]+)""#).unwrap();
            }

            if BINDING_REGEX.is_match(trimmed) {
                let mut converted = trimmed.to_string();

                for captures in BINDING_REGEX.captures_iter(trimmed) {
                    let attr_name = captures.get(1).unwrap().as_str();
                    let attr_value = captures.get(2).unwrap().as_str();

                    let binding = format!(":{}=\"{}\"", attr_name, attr_value);
                    let replacement = format!("{}={{{}}}", attr_name, self.convert_vue_expression(attr_value, component));

                    converted = converted.replace(&binding, &replacement);
                }

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Convert @ event handlers
            lazy_static! {
                static ref EVENT_REGEX: Regex = Regex::new(r#"@([a-zA-Z0-9_-]+)="([^"]+)""#).unwrap();
            }

            if EVENT_REGEX.is_match(trimmed) {
                let mut converted = trimmed.to_string();

                for captures in EVENT_REGEX.captures_iter(trimmed) {
                    let event_name = captures.get(1).unwrap().as_str();
                    let handler = captures.get(2).unwrap().as_str();

                    let event_binding = format!("@{}=\"{}\"", event_name, handler);
                    let replacement = format!("on:{}={}", event_name, self.convert_vue_expression(handler, component));

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
                    let replacement = format!("{{{}}}", self.convert_vue_expression(expr, component));

                    converted = converted.replace(&mustache, &replacement);
                }

                leptos_view.push_str(&format!("        {}\n", converted));
                continue;
            }

            // Pass through other HTML
            leptos_view.push_str(&format!("        {}\n", trimmed));
        }

        leptos_view
    }

    fn convert_vue_expression(&self, expr: &str, component: &VueComponent) -> String {
        let expr = expr.trim();

        // Convert this.property to property()
        if expr.starts_with("this.") {
            let property = &expr[5..];

            // Check if it's a computed property
            if component.computed.iter().any(|cp| cp.name == property) {
                return format!("{}()", property);
            }

            // Check if it's a data property
            if component.data.iter().any(|d| d.name == property) {
                return format!("{}()", property);
            }

            // Otherwise, just return as is
            return property.to_string();
        }

        // Handle simple property references
        if component.data.iter().any(|d| d.name == expr) {
            return format!("{}()", expr);
        }

        // Handle computed properties
        if component.computed.iter().any(|cp| cp.name == expr) {
            return format!("{}()", expr);
        }

        // Handle props
        if component.props.iter().any(|p| p.name == expr) {
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

    fn generate_test_code(&self, component: &VueComponent) -> Result<String, Box<dyn std::error::Error>> {
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
        if !component.props.is_empty() {
            code.push_str("    // Create test props\n");
            code.push_str(&format!("    let props = {}Props {{\n", component_name));

            for prop in &component.props {
                let prop_name = &prop.name;
                let prop_type = self.map_prop_type(&prop.prop_type);

                if prop.required {
                    // Generate a default value based on type
                    let default_value = match prop_type.as_str() {
                        "String" => "\"test\".to_string()",
                        "f64" => "0.0",
                        "bool" => "false",
                        t if t.starts_with("Vec<") => "vec![]",
                        t if t.starts_with("Option<") => "None",
                        _ => "Default::default()",
                    };

                    code.push_str(&format!("        {}: {},\n", prop_name, default_value));
                } else {
                    code.push_str(&format!("        {}: None,\n", prop_name));
                }
            }

            // Add children if needed
            if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("<slot>")) {
                code.push_str("        children: None,\n");
            }

            code.push_str("    };\n\n");
        }

        // Mount the component
        if !component.props.is_empty() {
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

        // Add props test if there are props
        if !component.props.is_empty() {
            code.push_str(&format!("#[wasm_bindgen_test]\n"));
            code.push_str(&format!("fn test_{}_props() {{\n", file_name));
            code.push_str("    // Create a test mount point\n");
            code.push_str("    let document = web_sys::window().unwrap().document().unwrap();\n");
            code.push_str("    let mount = document.create_element(\"div\").unwrap();\n");
            code.push_str("    document.body().unwrap().append_child(&mount).unwrap();\n\n");

            // Create test props with specific values
            code.push_str("    // Create test props with specific values\n");
            code.push_str(&format!("    let props = {}Props {{\n", component_name));

            for prop in &component.props {
                let prop_name = &prop.name;
                let prop_type = self.map_prop_type(&prop.prop_type);

                if prop.required {
                    // Generate a specific test value based on type
                    let test_value = match prop_type.as_str() {
                        "String" => "\"test-value\".to_string()",
                        "f64" => "42.0",
                        "bool" => "true",
                        t if t.starts_with("Vec<") => "vec![\"test\".to_string()]",
                        _ => "Default::default()",
                    };

                    code.push_str(&format!("        {}: {},\n", prop_name, test_value));
                } else {
                    code.push_str(&format!("        {}: Some(\"test-value\".to_string()),\n", prop_name));
                }
            }

            // Add children if needed
            if component.template_content.as_ref().map_or(false, |tpl| tpl.contains("<slot>")) {
                code.push_str("        children: Some(|cx| view! { cx, <p>\"Test Child\"</p> }),\n");
            }

            code.push_str("    };\n\n");

            // Mount the component
            code.push_str(&format!("    mount_to(mount, |cx| view! {{ cx, <{} props={{props}} /> }});\n\n", component_name));

            // Add assertions for props
            code.push_str("    // Test that props were applied\n");
            code.push_str("    let component_el = document.query_selector(\"div\").unwrap().unwrap();\n");
            code.push_str("    // Add specific assertions for props\n");

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
