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
