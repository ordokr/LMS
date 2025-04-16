rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn setup() -> PathBuf {
        let temp_dir = PathBuf::from("test_files");
        fs::create_dir_all(&temp_dir).unwrap();

        // Module file
        let module_content = r#"
            <h2 class="page-title">Test Module</h2>
            <div class="module-sequence-footer"></div>
            <h2>Test Module Overview</h2>
        "#;
        let mut file = File::create(temp_dir.join("module.html")).unwrap();
        file.write_all(module_content.as_bytes()).unwrap();

        // Assignment file
        let assignment_content = r#"
            <h2 class="page-title">Test Module</h2>
            <div class="assignment-details"></div>
            <h2>Test Assignment</h2>
            <span class="due_date_local">Jan 1, 2024</span>
        "#;
        let mut file = File::create(temp_dir.join("assignment.html")).unwrap();
        file.write_all(assignment_content.as_bytes()).unwrap();

        // Quiz file
        let quiz_content = r#"
            <h2 class="page-title">Test Module</h2>
            <div class="quiz_details"></div>
            <h2>Test Quiz</h2>
            <span class="display_date">Dec 31, 2023</span>
        "#;
        let mut file = File::create(temp_dir.join("quiz.html")).unwrap();
        file.write_all(quiz_content.as_bytes()).unwrap();

        // Page file
        let page_content = r#"
            <h2 class="page-title">Test Module</h2>
            <h2>Test Page</h2>
        "#;
        let mut file = File::create(temp_dir.join("page.html")).unwrap();
        file.write_all(page_content.as_bytes()).unwrap();

        temp_dir
    }

    fn teardown(temp_dir: PathBuf) {
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_module_identification() {
        let temp_dir = setup();
        let analyzer = CanvasAnalyzer::default();
        let result = analyzer.analyze(temp_dir.to_str().unwrap()).unwrap();
        let analysis: CanvasAnalysis = serde_json::from_str(&result).unwrap();
        assert_eq!(analysis.modules.len(), 1);
        assert_eq!(analysis.modules[0].name, "Test Module Overview");
        assert_eq!(analysis.modules[0].module, "Test Module");
        assert_eq!(analysis.modules[0].due_date, None);
        teardown(temp_dir);
    }

    #[test]
    fn test_assignment_identification() {
        let temp_dir = setup();
        let analyzer = CanvasAnalyzer::default();
        let result = analyzer.analyze(temp_dir.to_str().unwrap()).unwrap();
        let analysis: CanvasAnalysis = serde_json::from_str(&result).unwrap();
        assert_eq!(analysis.assignments.len(), 1);
        assert_eq!(analysis.assignments[0].name, "Test Assignment");
        assert_eq!(analysis.assignments[0].module, "Test Module");
        assert_eq!(analysis.assignments[0].due_date, Some("Jan 1, 2024".to_string()));
        teardown(temp_dir);
    }

    #[test]
    fn test_quiz_identification() {
        let temp_dir = setup();
        let analyzer = CanvasAnalyzer::default();
        let result = analyzer.analyze(temp_dir.to_str().unwrap()).unwrap();
        let analysis: CanvasAnalysis = serde_json::from_str(&result).unwrap();
        assert_eq!(analysis.quizzes.len(), 1);
        assert_eq!(analysis.quizzes[0].name, "Test Quiz");
        assert_eq!(analysis.quizzes[0].module, "Test Module");
        assert_eq!(analysis.quizzes[0].due_date, Some("Dec 31, 2023".to_string()));
        teardown(temp_dir);
    }

    #[test]
    fn test_page_identification() {
        let temp_dir = setup();
        let analyzer = CanvasAnalyzer::default();
        let result = analyzer.analyze(temp_dir.to_str().unwrap()).unwrap();
        let analysis: CanvasAnalysis = serde_json::from_str(&result).unwrap();
        assert_eq!(analysis.pages.len(), 1);
        assert_eq!(analysis.pages[0].name, "Test Page");
        assert_eq!(analysis.pages[0].module, "Test Module");
        assert_eq!(analysis.pages[0].due_date, None);
        teardown(temp_dir);
    }

    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::HashMap,
        fs,
        io::{self, Error},
        path::{Path, PathBuf},
    };
    use lazy_static::lazy_static;
    use walkdir::WalkDir;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CanvasElement {
        pub name: String,
        pub module: String,
        pub due_date: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CanvasAnalysis {
        pub modules: Vec<CanvasElement>,
        pub assignments: Vec<CanvasElement>,
        pub quizzes: Vec<CanvasElement>,
        pub pages: Vec<CanvasElement>,
    }

    #[derive(Debug, Default)]
    pub struct CanvasAnalyzer {}

    impl CanvasAnalyzer {
        pub fn analyze(&self, project_path: &str) -> Result<String, CanvasError> {
            let mut modules: Vec<CanvasElement> = Vec::new();
            let mut assignments: Vec<CanvasElement> = Vec::new();
            let mut quizzes: Vec<CanvasElement> = Vec::new();
            let mut pages: Vec<CanvasElement> = Vec::new();

            for entry in WalkDir::new(project_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if file_name_str.ends_with(".html") {
                                let content = fs::read_to_string(path)?;

                                // Extract module name
                                let module_name = self.extract_module_name(&content).unwrap_or_else(|| "Unknown Module".to_string());

                                // Check if it's a module overview page
                                if self.is_module_overview(&content) {
                                    if let Some(name) = self.extract_element_name(&content) {
                                        modules.push(CanvasElement {
                                            name,
                                            module: module_name.clone(),
                                            due_date: None,
                                        });
                                    }
                                }

                                // Check if it's an assignment
                                if self.is_assignment(&content) {
                                    if let Some((name, due_date)) = self.extract_assignment_details(&content) {
                                        assignments.push(CanvasElement {
                                            name,
                                            module: module_name.clone(),
                                            due_date: Some(due_date),
                                        });
                                    }
                                }

                                // Check if it's a quiz
                                if self.is_quiz(&content) {
                                    if let Some((name, due_date)) = self.extract_quiz_details(&content) {
                                        quizzes.push(CanvasElement {
                                            name,
                                            module: module_name.clone(),
                                            due_date: Some(due_date),
                                        });
                                    }
                                }

                                // Check if it's a page (not a module, assignment, or quiz)
                                if !(self.is_module_overview(&content) || self.is_assignment(&content) || self.is_quiz(&content)) {
                                    if let Some(name) = self.extract_element_name(&content) {
                                        pages.push(CanvasElement {
                                            name,
                                            module: module_name.clone(),
                                            due_date: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let result = serde_json::json!({
                "modules": modules,
                "assignments": assignments,
                "quizzes": quizzes,
                "pages": pages,
            });

            Ok(serde_json::to_string_pretty(&result)?)
        }

        // Helper functions for extracting information using regex

        // Extracts the module name from the content
        fn extract_module_name(&self, content: &str) -> Option<String> {
            lazy_static! {
                // Regex to find the module name within an h2 tag with class 'page-title'
                static ref MODULE_NAME_REGEX: Regex =
                    Regex::new(r#"<h2[^>]*class="page-title"[^>]*>\s*(.*?)\s*</h2>"#).unwrap();
            }
            MODULE_NAME_REGEX.captures(content).and_then(|caps| {
                caps.get(1).map(|m| m.as_str().trim().to_string())
            })
        }

        // Checks if the content represents a module overview page
        fn is_module_overview(&self, content: &str) -> bool {
            lazy_static! {
                // Regex to check for a div with the class 'module-sequence-footer'
                static ref MODULE_OVERVIEW_REGEX: Regex =
                    Regex::new(r#"<div[^>]*class="module-sequence-footer"[^>]*>"#).unwrap();
            }
            MODULE_OVERVIEW_REGEX.is_match(content)
        }

        // Checks if the content represents an assignment
        fn is_assignment(&self, content: &str) -> bool {
            lazy_static! {
                // Regex to check for a div with the class 'assignment-details'
                static ref ASSIGNMENT_REGEX: Regex =
                    Regex::new(r#"<div[^>]*class="assignment-details"[^>]*>"#).unwrap();
            }
            ASSIGNMENT_REGEX.is_match(content)
        }

        // Extracts assignment name and due date
        fn extract_assignment_details(&self, content: &str) -> Option<(String, String)> {
            lazy_static! {
                // Regex to find assignment name in an h2 tag
                static ref ASSIGNMENT_NAME_REGEX: Regex =
                    Regex::new(r#"<h2[^>]*>(.*?)</h2>"#).unwrap();
                // Regex to find the due date in a span with class 'due_date_local'
                static ref ASSIGNMENT_DUE_DATE_REGEX: Regex =
                    Regex::new(r#"<span[^>]*class="due_date_local"[^>]*>(.*?)</span>"#).unwrap();
            }
            let name = ASSIGNMENT_NAME_REGEX.captures(content).and_then(|caps| {
                caps.get(1).map(|m| m.as_str().trim().to_string())
            });
            let due_date = ASSIGNMENT_DUE_DATE_REGEX.captures(content).and_then(|caps| {
                caps.get(1).map(|m| m.as_str().trim().to_string())
            });
            if let (Some(name), Some(due_date)) = (name, due_date) {
                Some((name, due_date))
            } else {
                None
            }
        }

        // Checks if the content represents a quiz
        fn is_quiz(&self, content: &str) -> bool {
            lazy_static! {
                // Regex to check for a div with the class 'quiz_details'
                static ref QUIZ_REGEX: Regex =
                    Regex::new(r#"<div[^>]*class="quiz_details"[^>]*>"#).unwrap();
            }
            QUIZ_REGEX.is_match(content)
        }

        // Extracts quiz name and due date
        fn extract_quiz_details(&self, content: &str) -> Option<(String, String)> {
            lazy_static! {
                // Regex to find quiz name in an h2 tag
                static ref QUIZ_NAME_REGEX: Regex =
                    Regex::new(r#"<h2[^>]*>(.*?)</h2>"#).unwrap();
                // Regex to find the due date
                static ref QUIZ_DUE_DATE_REGEX: Regex =
                    Regex::new(r#"<span[^>]*class="display_date"[^>]*>(.*?)</span>"#).unwrap();
            }
            let name = QUIZ_NAME_REGEX.captures(content).and_then(|caps| {
                caps.get(1).map(|m| m.as_str().trim().to_string())
            });
            let due_date = QUIZ_DUE_DATE_REGEX.captures(content).and_then(|caps| {
                caps.get(1).map(|m| m.as_str().trim().to_string())
            });
            if let (Some(name), Some(due_date)) = (name, due_date) {
                Some((name, due_date))
            } else {
                None
            }
        }

        // Extracts the element name from the content (for modules and pages)
        fn extract_element_name(&self, content: &str) -> Option<String> {
            lazy_static! {
                // Broader regex to find the main title within an h2 or h1 tag
                static ref ELEMENT_NAME_REGEX: Regex =
                    Regex::new(r#"<(h1|h2)[^>]*>\s*(.*?)\s*</(h1|h2)>"#).unwrap();
            }
            ELEMENT_NAME_REGEX.captures(content).and_then(|caps| {
                caps.get(2).map(|m| m.as_str().trim().to_string())
            })
        }
    }

    #[derive(Debug)]
    pub enum CanvasError {
        IoError(io::Error),
        RegexError(String),
        JsonError(serde_json::Error),
        WalkDirError(walkdir::Error),
    }

    impl From<io::Error> for CanvasError {
        fn from(error: io::Error) -> Self {
            CanvasError::IoError(error)
        }
    }

    impl From<regex::Error> for CanvasError {
        fn from(error: regex::Error) -> Self {
            CanvasError::RegexError(error.to_string())
        }
    }

    impl From<serde_json::Error> for CanvasError {
        fn from(error: serde_json::Error) -> Self {
            CanvasError::JsonError(error)
        }
    }

    impl From<walkdir::Error> for CanvasError {
        fn from(error: walkdir::Error) -> Self {
            CanvasError::WalkDirError(error)
        }
    }
}