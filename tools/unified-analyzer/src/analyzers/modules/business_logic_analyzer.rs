use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BusinessLogicPattern {
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
    pub code_snippets: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DomainAlgorithm {
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
    pub complexity: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub files: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub description: String,
    pub actor: Option<String>,
    pub triggers: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EdgeCase {
    pub scenario: String,
    pub handling: String,
    pub files: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BusinessRule {
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BusinessLogicAnalyzer {
    pub patterns: Vec<BusinessLogicPattern>,
    pub algorithms: Vec<DomainAlgorithm>,
    pub workflows: Vec<Workflow>,
    pub edge_cases: Vec<EdgeCase>,
    pub business_rules: Vec<BusinessRule>,
}

impl BusinessLogicAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = BusinessLogicAnalyzer::default();

        // Identify core business logic patterns
        analyzer.identify_patterns(base_dir);

        // Map domain-specific algorithms
        analyzer.map_algorithms(base_dir);

        // Document critical workflows
        analyzer.document_workflows(base_dir);

        // Identify edge cases and error handling
        analyzer.identify_edge_cases(base_dir);

        // Map business rules and constraints
        analyzer.map_business_rules(base_dir);

        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize BusinessLogicAnalyzer: {}", e)),
        }
    }

    fn identify_patterns(&mut self, base_dir: &PathBuf) {
        // Look for common business logic patterns in the codebase

        // Pattern: Service Objects
        self.find_pattern(
            base_dir,
            "Service Objects",
            "Encapsulated business operations in dedicated service classes",
            vec!["app/services", "app/lib"],
            vec!["Service", "Manager", "Processor", "Handler"],
        );

        // Pattern: Form Objects
        self.find_pattern(
            base_dir,
            "Form Objects",
            "Dedicated objects for handling form submissions and validations",
            vec!["app/forms", "app/lib"],
            vec!["Form", "FormObject"],
        );

        // Pattern: Query Objects
        self.find_pattern(
            base_dir,
            "Query Objects",
            "Encapsulated database queries in dedicated objects",
            vec!["app/queries", "app/lib"],
            vec!["Query", "Finder", "Searcher"],
        );

        // Pattern: Presenters/Decorators
        self.find_pattern(
            base_dir,
            "Presenters/Decorators",
            "Objects that enhance models with presentation logic",
            vec!["app/presenters", "app/decorators", "app/lib"],
            vec!["Presenter", "Decorator", "Representation"],
        );

        // Pattern: Policies
        self.find_pattern(
            base_dir,
            "Policy Objects",
            "Objects that encapsulate authorization rules",
            vec!["app/policies"],
            vec!["Policy"],
        );
    }

    fn find_pattern(&mut self, base_dir: &PathBuf, name: &str, description: &str, dirs: Vec<&str>, suffixes: Vec<&str>) {
        let mut pattern = BusinessLogicPattern {
            name: name.to_string(),
            description: description.to_string(),
            files: Vec::new(),
            code_snippets: Vec::new(),
        };

        for dir_name in dirs {
            let dir_path = base_dir.join(dir_name);
            if dir_path.exists() {
                for entry in WalkDir::new(&dir_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name_str) = file_name.to_str() {
                                // Check if file name contains any of the pattern suffixes
                                if suffixes.iter().any(|&suffix| file_name_str.contains(suffix)) {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();
                                        pattern.files.push(file_path.clone());

                                        // Extract a code snippet
                                        if let Ok(content) = fs::read_to_string(path) {
                                            let lines: Vec<&str> = content.lines().collect();
                                            if !lines.is_empty() {
                                                let start = 0;
                                                let end = std::cmp::min(15, lines.len()); // First 15 lines or less
                                                let snippet = lines[start..end].join("\n");
                                                pattern.code_snippets.push(format!("File: {}\n{}", file_path, snippet));
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

        if !pattern.files.is_empty() {
            self.patterns.push(pattern);
        }
    }

    fn map_algorithms(&mut self, base_dir: &PathBuf) {
        // Look for complex algorithms in the codebase
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "rb" || ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Look for algorithm indicators
                                if (content.contains("algorithm") || content.contains("calculate") || content.contains("compute")) &&
                                   (content.lines().count() > 30) // Longer files are more likely to contain complex algorithms
                                {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        // Try to extract algorithm name from class/method names
                                        let mut algorithm_name = "Unknown Algorithm";
                                        let class_regex = Regex::new(r"class\s+(\w+)").unwrap();
                                        if let Some(class_capture) = class_regex.captures(&content) {
                                            if let Some(class_name) = class_capture.get(1) {
                                                algorithm_name = class_name.as_str();
                                            }
                                        }

                                        // Determine complexity based on code metrics
                                        let complexity = if content.lines().count() > 100 {
                                            "High"
                                        } else if content.lines().count() > 50 {
                                            "Medium"
                                        } else {
                                            "Low"
                                        };

                                        self.algorithms.push(DomainAlgorithm {
                                            name: algorithm_name.to_string(),
                                            description: format!("Algorithm found in {}", file_path),
                                            files: vec![file_path],
                                            complexity: complexity.to_string(),
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

    fn document_workflows(&mut self, base_dir: &PathBuf) {
        // Look for workflow definitions or indicators

        // Check for controller actions that might represent workflow steps
        let controllers_dir = base_dir.join("app").join("controllers");
        if controllers_dir.exists() {
            for entry in WalkDir::new(&controllers_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            // Extract controller name
                            let controller_name = file_name_str.replace("_controller.rb", "");

                            if let Ok(content) = fs::read_to_string(path) {
                                // Extract actions
                                let action_regex = Regex::new(r"def\s+([\w_]+)").unwrap();
                                let mut workflow_steps = Vec::new();

                                for action_capture in action_regex.captures_iter(&content) {
                                    if let Some(action) = action_capture.get(1) {
                                        let action_name = action.as_str();

                                        // Skip common CRUD actions
                                        if action_name == "index" || action_name == "show" || action_name == "new" ||
                                           action_name == "create" || action_name == "edit" || action_name == "update" ||
                                           action_name == "destroy" {
                                            continue;
                                        }

                                        // This might be a workflow step
                                        workflow_steps.push(WorkflowStep {
                                            name: action_name.to_string(),
                                            description: format!("Action in {} controller", controller_name),
                                            actor: None,
                                            triggers: Vec::new(),
                                        });
                                    }
                                }

                                if !workflow_steps.is_empty() {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        self.workflows.push(Workflow {
                                            name: format!("{} Workflow", controller_name),
                                            description: format!("Workflow extracted from {} controller", controller_name),
                                            steps: workflow_steps,
                                            files: vec![file_path],
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Look for state machines or workflow definitions
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "rb" || ext == "js" || ext == "ts" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Check for state machine indicators
                                if content.contains("state_machine") || content.contains("aasm") ||
                                   content.contains("workflow") || content.contains("state :")
                                {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        // Extract workflow name
                                        let workflow_name = if let Some(file_name) = path.file_name() {
                                            if let Some(file_name_str) = file_name.to_str() {
                                                file_name_str.replace(".rb", "").replace(".js", "").replace(".ts", "")
                                            } else {
                                                "State Machine Workflow".to_string()
                                            }
                                        } else {
                                            "State Machine Workflow".to_string()
                                        };

                                        // Extract states and transitions
                                        let state_regex = Regex::new(r"(?:state|event)\s+:([\w_]+)").unwrap();
                                        let mut steps = Vec::new();

                                        for state_capture in state_regex.captures_iter(&content) {
                                            if let Some(state) = state_capture.get(1) {
                                                steps.push(WorkflowStep {
                                                    name: state.as_str().to_string(),
                                                    description: format!("State in {} workflow", workflow_name),
                                                    actor: None,
                                                    triggers: Vec::new(),
                                                });
                                            }
                                        }

                                        if !steps.is_empty() {
                                            self.workflows.push(Workflow {
                                                name: format!("{} Workflow", workflow_name),
                                                description: format!("State machine workflow extracted from {}", file_path),
                                                steps,
                                                files: vec![file_path],
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

    fn identify_edge_cases(&mut self, base_dir: &PathBuf) {
        // Look for error handling and edge case handling
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if ext == "rb" || ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" {
                            if let Ok(content) = fs::read_to_string(path) {
                                // Look for error handling patterns
                                if content.contains("rescue") || content.contains("begin") ||
                                   content.contains("catch") || content.contains("try") ||
                                   content.contains("if error") || content.contains("handle error")
                                {
                                    if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                                        let file_path = rel_path.to_string_lossy().to_string();

                                        // Extract error handling blocks
                                        let error_regex = if ext == "rb" {
                                            Regex::new(r"rescue\s+([^\n]+)\s+do([\s\S]*?)end").unwrap()
                                        } else {
                                            Regex::new(r"catch\s*\(([^\)]+)\)\s*\{([\s\S]*?)\}").unwrap()
                                        };

                                        for error_capture in error_regex.captures_iter(&content) {
                                            if let (Some(error_type), Some(handling)) = (error_capture.get(1), error_capture.get(2)) {
                                                self.edge_cases.push(EdgeCase {
                                                    scenario: format!("Error: {}", error_type.as_str().trim()),
                                                    handling: handling.as_str().trim().to_string(),
                                                    files: vec![file_path.clone()],
                                                });
                                            }
                                        }

                                        // Look for conditional checks that might indicate edge cases
                                        let condition_regex = Regex::new(r"if\s+([^\n]+)\s+(?:then|do|\{)([\s\S]*?)(?:end|\})").unwrap();
                                        for condition_capture in condition_regex.captures_iter(&content) {
                                            if let (Some(condition), Some(handling)) = (condition_capture.get(1), condition_capture.get(2)) {
                                                let condition_str = condition.as_str().trim();

                                                // Only consider conditions that look like edge case checks
                                                if condition_str.contains("nil?") || condition_str.contains("empty?") ||
                                                   condition_str.contains("blank?") || condition_str.contains("present?") ||
                                                   condition_str.contains("==") || condition_str.contains("!=") ||
                                                   condition_str.contains(">") || condition_str.contains("<")
                                                {
                                                    self.edge_cases.push(EdgeCase {
                                                        scenario: format!("Condition: {}", condition_str),
                                                        handling: handling.as_str().trim().to_string(),
                                                        files: vec![file_path.clone()],
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
        }
    }

    fn map_business_rules(&mut self, base_dir: &PathBuf) {
        // Look for business rules in models (validations, callbacks)
        let models_dir = base_dir.join("app").join("models");
        if models_dir.exists() {
            for entry in WalkDir::new(&models_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                    if let Ok(content) = fs::read_to_string(path) {
                        if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                            let file_path = rel_path.to_string_lossy().to_string();

                            // Extract model name
                            let mut model_name = "Unknown";
                            let model_regex = Regex::new(r"class\s+(\w+)\s+<").unwrap();
                            if let Some(model_capture) = model_regex.captures(&content) {
                                if let Some(name) = model_capture.get(1) {
                                    model_name = name.as_str();
                                }
                            }

                            // Extract validations
                            let validation_regex = Regex::new(r"validates(?:_[\w_]+)?\s+:([^,\n]+)(?:,\s*([^\n]+))?").unwrap();
                            for validation_capture in validation_regex.captures_iter(&content) {
                                if let Some(field) = validation_capture.get(1) {
                                    let field_str = field.as_str().trim();
                                    let options_str = validation_capture.get(2).map_or("", |m| m.as_str().trim());

                                    self.business_rules.push(BusinessRule {
                                        name: format!("{} {} Validation", model_name, field_str),
                                        description: format!("Validation rule: {} {}", field_str, options_str),
                                        files: vec![file_path.clone()],
                                    });
                                }
                            }

                            // Extract callbacks
                            let callback_regex = Regex::new(r"(before|after|around)_([\w_]+)\s+:([^,\n]+)(?:,\s*([^\n]+))?").unwrap();
                            for callback_capture in callback_regex.captures_iter(&content) {
                                if let (Some(timing), Some(event), Some(method)) = (callback_capture.get(1), callback_capture.get(2), callback_capture.get(3)) {
                                    self.business_rules.push(BusinessRule {
                                        name: format!("{} {} Callback", model_name, method.as_str().trim()),
                                        description: format!("{} {} callback: {}", timing.as_str(), event.as_str(), method.as_str().trim()),
                                        files: vec![file_path.clone()],
                                    });
                                }
                            }

                            // Extract scopes (which often represent business rules)
                            let scope_regex = Regex::new(r"scope\s+:([^,\n]+)(?:,\s*([^\n]+))?").unwrap();
                            for scope_capture in scope_regex.captures_iter(&content) {
                                if let Some(scope_name) = scope_capture.get(1) {
                                    self.business_rules.push(BusinessRule {
                                        name: format!("{} {} Scope", model_name, scope_name.as_str().trim()),
                                        description: format!("Database query scope: {}", scope_name.as_str().trim()),
                                        files: vec![file_path.clone()],
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Look for business rules in service objects
        let services_dir = base_dir.join("app").join("services");
        if services_dir.exists() {
            for entry in WalkDir::new(&services_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                    if let Ok(content) = fs::read_to_string(path) {
                        if let Some(rel_path) = path.strip_prefix(base_dir).ok() {
                            let file_path = rel_path.to_string_lossy().to_string();

                            // Extract service name
                            let mut service_name = "Unknown Service";
                            let service_regex = Regex::new(r"class\s+(\w+)(?:Service)?\s+<").unwrap();
                            if let Some(service_capture) = service_regex.captures(&content) {
                                if let Some(name) = service_capture.get(1) {
                                    service_name = name.as_str();
                                }
                            }

                            // Service objects often implement business rules
                            self.business_rules.push(BusinessRule {
                                name: format!("{} Business Logic", service_name),
                                description: format!("Business logic encapsulated in {} service", service_name),
                                files: vec![file_path],
                            });
                        }
                    }
                }
            }
        }
    }
}