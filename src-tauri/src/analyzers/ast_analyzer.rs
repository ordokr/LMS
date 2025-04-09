use crate::utils::file_system::FileSystemUtils;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use regex::Regex;

pub struct CodeMetrics {
    pub file_complexity: HashMap<PathBuf, u32>,
    pub components: Vec<ComponentInfo>,
    pub total_complexity: u32,
    pub average_complexity: f32,
    pub file_count: usize,
    // Other metrics
}

impl Default for CodeMetrics {
    fn default() -> Self {
        Self {
            file_complexity: HashMap::new(),
            components: Vec::new(),
            total_complexity: 0,
            average_complexity: 0.0,
            file_count: 0,
        }
    }
}

impl CodeMetrics {
    pub fn add_file_complexity(&mut self, path: PathBuf, complexity: u32) {
        self.file_complexity.insert(path, complexity);
    }
}

pub struct ComponentInfo {
    pub name: String,
    pub file_path: PathBuf,
    pub complexity: u32,
    pub type_name: String,  // "React", "Vue", "Rust", etc.
    pub props: Vec<PropInfo>,
    pub lines_of_code: usize,
}

pub struct PropInfo {
    pub name: String,
    pub prop_type: String,
    pub is_required: bool,
}

pub struct AstAnalyzer;

impl AstAnalyzer {
    pub fn new() -> Self {
        Self
    }
      pub fn analyze_project_code(&self, fs_utils: &FileSystemUtils) -> CodeMetrics {
        let file_contents = fs_utils.get_file_contents();
        
        let mut metrics = CodeMetrics::default();
        
        for (path, content) in file_contents {
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            
            let complexity = match extension {
                "rs" => self.calculate_rust_complexity(content),
                "js" | "jsx" | "ts" | "tsx" => self.estimate_js_complexity(content),
                "vue" => self.estimate_vue_complexity(content),
                _ => 0, // Skip unknown file types for complexity analysis
            };
            
            if complexity > 0 {
                metrics.add_file_complexity(path.clone(), complexity);
                metrics.total_complexity += complexity;
                metrics.file_count += 1;
                
                // Analyze for components
                if extension == "jsx" || extension == "tsx" || extension == "vue" {
                    if let Some(component) = self.detect_component(path, content, complexity) {
                        metrics.components.push(component);
                    }
                } else if extension == "rs" && content.contains("impl Component") {
                    if let Some(component) = self.detect_rust_component(path, content, complexity) {
                        metrics.components.push(component);
                    }
                }
            }
        }
        
        // Calculate average complexity if we have files
        if metrics.file_count > 0 {
            metrics.average_complexity = metrics.total_complexity as f32 / metrics.file_count as f32;
        }
        
        metrics
    }
    
    pub fn calculate_rust_complexity(&self, content: &str) -> u32 {
        let mut complexity = 1;
        
        // Count decision points in Rust code
        complexity += (content.matches("if ").count() + 
                      content.matches("else if").count() + 
                      content.matches("for ").count() + 
                      content.matches("while ").count() + 
                      content.matches("match ").count() + 
                      content.matches(" ? ").count()) as u32;
                      
        // Add complexity for closures
        complexity += content.matches("|").count() as u32 / 2; // Rough estimate for pairs of |
        
        // Add complexity for function definitions (excluding the main one)
        let fn_count = content.matches("fn ").count();
        if fn_count > 1 {
            complexity += (fn_count - 1) as u32;
        }
        
        complexity
    }

    pub fn estimate_js_complexity(&self, content: &str) -> u32 {
        let mut complexity = 1;
        
        // Count decision points in JS/TS code
        complexity += (content.matches("if ").count() + 
                      content.matches("else if").count() + 
                      content.matches("for ").count() + 
                      content.matches("while ").count() + 
                      content.matches("switch").count() +
                      content.matches("case ").count() +
                      content.matches(" ? ").count()) as u32;
                      
        // Add complexity for function definitions
        complexity += (content.matches("function").count() + 
                       content.matches("=>").count() + 
                       content.matches("class ").count()) as u32;
        
        // Handle JSX complexity
        if content.contains("import React") || content.contains("<") && content.contains("/>") {
            // Add complexity for conditional rendering in JSX
            complexity += content.matches("&&").count() as u32 / 2; // Rough estimate
            complexity += content.matches("\\{\\s*\\w+").count() as u32; // Expressions in JSX
        }
        
        complexity
    }
    
    pub fn estimate_vue_complexity(&self, content: &str) -> u32 {
        let mut complexity = 1;
        
        // Basic JS complexity
        complexity += self.estimate_js_complexity(content) / 2; // Divide by 2 since Vue files are split
        
        // Vue specific features
        complexity += (content.matches("v-if").count() + 
                      content.matches("v-for").count() + 
                      content.matches("v-model").count() + 
                      content.matches(":").count() / 3) as u32;
        
        // Vue lifecycle hooks
        complexity += (content.matches("created").count() + 
                      content.matches("mounted").count() + 
                      content.matches("updated").count() + 
                      content.matches("destroyed").count()) as u32;
                      
        complexity
    }
    
    pub fn detect_component(&self, path: &Path, content: &str, complexity: u32) -> Option<ComponentInfo> {
        // Detect React components
        if content.contains("import React") || content.contains("from 'react'") || content.contains("from \"react\"") {
            return self.detect_react_component(path, content, complexity);
        }
        
        // Detect Vue components
        if content.contains("<template>") && content.contains("<script>") {
            return self.detect_vue_component(path, content, complexity);
        }
        
        None
    }
    
    pub fn detect_react_component(&self, path: &Path, content: &str, complexity: u32) -> Option<ComponentInfo> {
        // Try to extract component name from the file
        let file_name = path.file_stem()?.to_string_lossy().to_string();
        let component_name = if file_name.starts_with("index") {
            // Use parent directory name for index files
            path.parent()?.file_name()?.to_string_lossy().to_string()
        } else {
            file_name
        };
        
        // Count lines of code
        let lines_of_code = content.lines().count();
        
        // Extract props (basic implementation)
        let props = self.extract_react_props(content);
        
        Some(ComponentInfo {
            name: component_name,
            file_path: path.to_path_buf(),
            complexity,
            type_name: "React".to_string(),
            props,
            lines_of_code,
        })
    }
    
    pub fn detect_vue_component(&self, path: &Path, content: &str, complexity: u32) -> Option<ComponentInfo> {
        // Extract component name from file
        let component_name = path.file_stem()?.to_string_lossy().to_string();
        
        // Count lines of code
        let lines_of_code = content.lines().count();
        
        // Extract props (basic implementation)
        let props = self.extract_vue_props(content);
        
        Some(ComponentInfo {
            name: component_name,
            file_path: path.to_path_buf(),
            complexity,
            type_name: "Vue".to_string(),
            props,
            lines_of_code,
        })
    }
    
    pub fn detect_rust_component(&self, path: &Path, content: &str, complexity: u32) -> Option<ComponentInfo> {
        // Extract component name - look for struct definitions
        let struct_pattern = Regex::new(r"struct\s+(\w+)").ok()?;
        let component_name = match struct_pattern.captures(content) {
            Some(caps) => caps.get(1)?.as_str().to_string(),
            None => path.file_stem()?.to_string_lossy().to_string(),
        };
        
        // Count lines of code
        let lines_of_code = content.lines().count();
        
        // For Rust, try to extract fields from the struct as "props"
        let props = self.extract_rust_fields(content);
        
        Some(ComponentInfo {
            name: component_name,
            file_path: path.to_path_buf(),
            complexity,
            type_name: "Rust".to_string(),
            props,
            lines_of_code,
        })
    }
    
    fn extract_react_props(&self, content: &str) -> Vec<PropInfo> {
        let mut props = Vec::new();
        
        // Look for PropTypes definitions
        let prop_types_pattern = Regex::new(r"PropTypes\.(\w+)(?:\.isRequired)?").ok();
        if let Some(pattern) = prop_types_pattern {
            for cap in pattern.captures_iter(content) {
                if let Some(prop_type) = cap.get(1) {
                    let prop_text = cap.get(0).unwrap().as_str();
                    props.push(PropInfo {
                        name: "unknown".to_string(), // Without AST, exact name is hard to get
                        prop_type: prop_type.as_str().to_string(),
                        is_required: prop_text.contains("isRequired"),
                    });
                }
            }
        }
        
        // Look for TypeScript interface/type definitions
        let ts_prop_pattern = Regex::new(r"(\w+)(?:\?)?:\s*(\w+)").ok();
        if let Some(pattern) = ts_prop_pattern {
            for cap in pattern.captures_iter(content) {
                if let (Some(name), Some(prop_type)) = (cap.get(1), cap.get(2)) {
                    let prop_text = cap.get(0).unwrap().as_str();
                    props.push(PropInfo {
                        name: name.as_str().to_string(),
                        prop_type: prop_type.as_str().to_string(),
                        is_required: !prop_text.contains("?"),
                    });
                }
            }
        }
        
        props
    }
    
    fn extract_vue_props(&self, content: &str) -> Vec<PropInfo> {
        let mut props = Vec::new();
        
        // Look for props in the script section
        let props_pattern = Regex::new(r"props:\s*\{([^}]+)\}").ok();
        if let Some(pattern) = props_pattern {
            if let Some(caps) = pattern.captures(content) {
                if let Some(props_block) = caps.get(1) {
                    let props_text = props_block.as_str();
                    
                    // Extract individual props
                    let prop_line_pattern = Regex::new(r"(\w+):\s*(?:\{\s*type:\s*(\w+)(?:,\s*required:\s*(true|false))?\s*\}|(\w+))").ok();
                    if let Some(line_pattern) = prop_line_pattern {
                        for prop_cap in line_pattern.captures_iter(props_text) {
                            let name = prop_cap.get(1).map_or("unknown", |m| m.as_str());
                            
                            // Handle both formats: { type: String, required: true } and just String
                            let prop_type = prop_cap.get(2)
                                .or_else(|| prop_cap.get(4))
                                .map_or("any", |m| m.as_str());
                                
                            let is_required = prop_cap.get(3)
                                .map_or(false, |m| m.as_str() == "true");
                                
                            props.push(PropInfo {
                                name: name.to_string(),
                                prop_type: prop_type.to_string(),
                                is_required,
                            });
                        }
                    }
                }
            }
        }
        
        props
    }
    
    fn extract_rust_fields(&self, content: &str) -> Vec<PropInfo> {
        let mut props = Vec::new();
        
        // Look for struct fields
        let struct_fields_pattern = Regex::new(r"struct\s+\w+\s*\{([^}]+)\}").ok();
        if let Some(pattern) = struct_fields_pattern {
            if let Some(caps) = pattern.captures(content) {
                if let Some(struct_block) = caps.get(1) {
                    let fields_text = struct_block.as_str();
                    
                    // Extract individual fields
                    let field_pattern = Regex::new(r"(?:pub\s+)?(\w+):\s*([^,]+)").ok();
                    if let Some(field_pattern) = field_pattern {
                        for field_cap in field_pattern.captures_iter(fields_text) {
                            if let (Some(name), Some(field_type)) = (field_cap.get(1), field_cap.get(2)) {
                                props.push(PropInfo {
                                    name: name.as_str().to_string(),
                                    prop_type: field_type.as_str().trim().to_string(),
                                    is_required: !field_type.as_str().contains("Option<"),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        props
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_rust_complexity() {
        let analyzer = AstAnalyzer::new();
        
        let simple_code = "
            fn main() {
                println!(\"Hello, world!\");
            }
        ";
        
        let complex_code = "
            fn complex_function(input: u32) -> Result<u32, String> {
                if input == 0 {
                    return Err(\"Input cannot be zero\".to_string());
                }
                
                let result = match input {
                    1 => 1,
                    2 => 2,
                    _ => {
                        let mut sum = 0;
                        for i in 1..=input {
                            sum += i;
                        }
                        sum
                    }
                };
                
                let multiplier = |x: u32| x * 2;
                
                Ok(multiplier(result))
            }
        ";
        
        let simple_complexity = analyzer.calculate_rust_complexity(simple_code);
        let complex_complexity = analyzer.calculate_rust_complexity(complex_code);
        
        assert!(simple_complexity <= 2, "Simple code should have low complexity");
        assert!(complex_complexity >= 5, "Complex code should have higher complexity");
        assert!(complex_complexity > simple_complexity, "Complex code should be more complex than simple code");
    }
    
    #[test]
    fn test_js_complexity() {
        let analyzer = AstAnalyzer::new();
        
        let simple_js = "
            function greet() {
                console.log(\"Hello\");
            }
        ";
        
        let complex_js = "
            function processData(data) {
                if (!data || data.length === 0) {
                    return [];
                }
                
                const result = [];
                
                for (let i = 0; i < data.length; i++) {
                    const item = data[i];
                    
                    if (item.active) {
                        switch (item.type) {
                            case 'user':
                                result.push(processUser(item));
                                break;
                            case 'post':
                                result.push(processPost(item));
                                break;
                            default:
                                result.push(item);
                        }
                    }
                }
                
                return result;
            }
            
            function processUser(user) {
                return { ...user, processed: true };
            }
            
            function processPost(post) {
                return { ...post, processed: true };
            }
        ";
        
        let simple_complexity = analyzer.estimate_js_complexity(simple_js);
        let complex_complexity = analyzer.estimate_js_complexity(complex_js);
        
        assert!(simple_complexity <= 2, "Simple JS should have low complexity");
        assert!(complex_complexity >= 8, "Complex JS should have higher complexity");
        assert!(complex_complexity > simple_complexity, "Complex JS should be more complex than simple JS");
    }
    
    #[test]
    fn test_component_detection() {
        let analyzer = AstAnalyzer::new();
        let temp = tempdir().unwrap();
        
        // Create a React component file
        let react_path = temp.path().join("Button.jsx");
        let react_content = "
import React from 'react';
import PropTypes from 'prop-types';

const Button = ({ label, onClick, disabled }) => {
  return (
    <button 
      className=\"btn\"
      onClick={onClick}
      disabled={disabled}
    >
      {label}
    </button>
  );
};

Button.propTypes = {
  label: PropTypes.string.isRequired,
  onClick: PropTypes.func,
  disabled: PropTypes.bool
};

export default Button;
        ";
        
        let mut file = File::create(&react_path).unwrap();
        write!(file, "{}", react_content).unwrap();
        
        // Create a Rust component file
        let rust_path = temp.path().join("counter.rs");
        let rust_content = "
use yew::prelude::*;

pub struct Counter {
    value: i64,
    onclick: Callback<()>,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            onclick: Callback::noop(),
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{ self.value }</p>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ \"+1\" }</button>
                <button onclick={ctx.link().callback(|_| Msg::Decrement)}>{ \"-1\" }</button>
            </div>
        }
    }
}
        ";
        
        let mut file = File::create(&rust_path).unwrap();
        write!(file, "{}", rust_content).unwrap();
        
        // Detect React component
        let complexity = analyzer.estimate_js_complexity(&react_content);
        let react_component = analyzer.detect_component(&react_path, &react_content, complexity);
        
        assert!(react_component.is_some(), "Should detect React component");
        if let Some(component) = react_component {
            assert_eq!(component.name, "Button");
            assert_eq!(component.type_name, "React");
            assert!(component.props.len() > 0, "Should detect props");
        }
        
        // Detect Rust component
        let complexity = analyzer.calculate_rust_complexity(&rust_content);
        let rust_component = analyzer.detect_rust_component(&rust_path, &rust_content, complexity);
        
        assert!(rust_component.is_some(), "Should detect Rust component");
        if let Some(component) = rust_component {
            assert_eq!(component.name, "Counter");
            assert_eq!(component.type_name, "Rust");
        }
    }
}