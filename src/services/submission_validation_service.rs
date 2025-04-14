use crate::models::lms::{Assignment, Submission};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};

/// Validation result for a submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn with_error(error: &str) -> Self {
        let mut result = Self::new();
        result.is_valid = false;
        result.errors.push(error.to_string());
        result
    }
    
    pub fn with_warning(warning: &str) -> Self {
        let mut result = Self::new();
        result.warnings.push(warning.to_string());
        result
    }
    
    pub fn add_error(&mut self, error: &str) {
        self.is_valid = false;
        self.errors.push(error.to_string());
    }
    
    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }
}

/// Service for validating assignment submissions
pub struct SubmissionValidationService;

impl SubmissionValidationService {
    /// Validate a submission against its assignment
    pub fn validate_submission(&self, submission: &Submission, assignment: &Assignment) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Check if submission is allowed based on assignment status
        if let Some(false) = assignment.published {
            result.add_error("Cannot submit to an unpublished assignment");
            return result;
        }
        
        // Check if submission is on time
        if let Some(due_date) = self.parse_date(&assignment.due_date) {
            let now = Utc::now();
            if now > due_date {
                result.add_warning("Submission is past the due date");
            }
        }
        
        // Check if submission has any content
        if submission.content.is_empty() && submission.attachments.is_empty() {
            result.add_error("Submission must have content or attachments");
        }
        
        // Check attachment types if specified
        if !submission.attachments.is_empty() {
            self.validate_attachments(&submission.attachments, assignment, &mut result);
        }
        
        // Check submission type
        if let Some(allowed_types) = self.get_allowed_submission_types(assignment) {
            let actual_type = self.determine_submission_type(submission);
            if !allowed_types.contains(&actual_type) {
                result.add_error(&format!("Submission type '{}' is not allowed for this assignment", actual_type));
            }
        }
        
        // Check word count if required
        if let Some(min_word_count) = self.get_min_word_count(assignment) {
            let word_count = submission.content.split_whitespace().count();
            if word_count < min_word_count {
                result.add_error(&format!("Submission requires at least {} words", min_word_count));
            }
        }
        
        result
    }
    
    /// Parse date string to DateTime
    fn parse_date(&self, date_str: &Option<String>) -> Option<DateTime<Utc>> {
        date_str
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }
    
    /// Validate submission attachments
    fn validate_attachments(&self, attachments: &[String], assignment: &Assignment, result: &mut ValidationResult) {
        // Get allowed file extensions from assignment
        let allowed_extensions = self.get_allowed_file_extensions(assignment);
        
        if let Some(extensions) = allowed_extensions {
            for attachment in attachments {
                // Extract file extension
                let extension = attachment
                    .split('.')
                    .last()
                    .map(|s| s.to_lowercase());
                
                // Check if extension is allowed
                if let Some(ext) = extension {
                    if !extensions.contains(&ext) {
                        result.add_error(&format!("File type '.{}' is not allowed for this assignment", ext));
                    }
                } else {
                    result.add_error("Attachment must have a file extension");
                }
            }
        }
        
        // Check attachment count if there's a limit
        if let Some(max_attachments) = self.get_max_attachments(assignment) {
            if attachments.len() > max_attachments {
                result.add_error(&format!("Maximum of {} attachments allowed", max_attachments));
            }
        }
    }
    
    /// Get allowed submission types from assignment
    fn get_allowed_submission_types(&self, assignment: &Assignment) -> Option<Vec<String>> {
        if let Some(submission_types) = &assignment.submission_types {
            if !submission_types.is_empty() {
                return Some(submission_types.clone());
            }
        }
        None
    }
    
    /// Determine submission type from submitted content
    fn determine_submission_type(&self, submission: &Submission) -> String {
        if !submission.attachments.is_empty() {
            return "file_upload".to_string();
        }
        
        if submission.content.starts_with("http") {
            return "url".to_string();
        }
        
        "text_entry".to_string()
    }
    
    /// Get minimum word count from assignment if specified
    fn get_min_word_count(&self, assignment: &Assignment) -> Option<usize> {
        // This would be based on assignment configuration
        // For now, return None as this field might not exist yet
        None
    }
    
    /// Get allowed file extensions from assignment
    fn get_allowed_file_extensions(&self, assignment: &Assignment) -> Option<Vec<String>> {
        // This would be based on assignment configuration
        // For now, return a default list of common allowed extensions
        Some(vec![
            "pdf".to_string(),
            "doc".to_string(),
            "docx".to_string(),
            "txt".to_string(),
            "rtf".to_string(),
            "odt".to_string(),
            "zip".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "png".to_string(),
        ])
    }
    
    /// Get maximum number of attachments allowed
    fn get_max_attachments(&self, assignment: &Assignment) -> Option<usize> {
        // This would be based on assignment configuration
        // For now, return a default value
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_assignment() -> Assignment {
        Assignment {
            id: 123,
            course_id: 456,
            title: "Test Assignment".to_string(),
            description: "Test description".to_string(),
            due_date: Some(Utc::now().to_rfc3339()),
            available_from: None,
            available_until: None,
            points_possible: Some(100.0),
            submission_types: Some(vec![
                "text_entry".to_string(),
                "file_upload".to_string(),
                "url".to_string(),
            ]),
            published: Some(true),
            created_at: Some(Utc::now().to_rfc3339()),
            updated_at: Some(Utc::now().to_rfc3339()),
            grading_type: None,
            position: None,
            peer_review_count: None,
            group_category_id: None,
            grade_group_students_individually: None,
            anonymous_grading: None,
            allowed_attempts: None,
            omit_from_final_grade: None,
            assignment_group_id: None,
            parent_assignment_id: None,
            sub_assignment_tag: None,
        }
    }
    
    fn create_test_submission() -> Submission {
        Submission {
            id: "123".to_string(),
            assignment_id: "123".to_string(),
            user_id: "456".to_string(),
            content: "This is a test submission".to_string(),
            attachments: Vec::new(),
            status: "submitted".to_string(),
            score: None,
            feedback: None,
            submitted_at: Utc::now().to_rfc3339(),
            graded_at: None,
        }
    }
    
    #[test]
    fn test_validate_valid_submission() {
        let validation_service = SubmissionValidationService;
        let assignment = create_test_assignment();
        let submission = create_test_submission();
        
        let result = validation_service.validate_submission(&submission, &assignment);
        
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }
    
    #[test]
    fn test_validate_empty_submission() {
        let validation_service = SubmissionValidationService;
        let assignment = create_test_assignment();
        let mut submission = create_test_submission();
        submission.content = "".to_string();
        
        let result = validation_service.validate_submission(&submission, &assignment);
        
        assert!(!result.is_valid);
        assert!(result.errors.contains(&"Submission must have content or attachments".to_string()));
    }
    
    #[test]
    fn test_validate_unpublished_assignment() {
        let validation_service = SubmissionValidationService;
        let mut assignment = create_test_assignment();
        assignment.published = Some(false);
        let submission = create_test_submission();
        
        let result = validation_service.validate_submission(&submission, &assignment);
        
        assert!(!result.is_valid);
        assert!(result.errors.contains(&"Cannot submit to an unpublished assignment".to_string()));
    }
    
    #[test]
    fn test_validate_file_extension() {
        let validation_service = SubmissionValidationService;
        let assignment = create_test_assignment();
        let mut submission = create_test_submission();
        submission.attachments = vec!["file.exe".to_string()];
        
        let result = validation_service.validate_submission(&submission, &assignment);
        
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("File type '.exe' is not allowed")));
    }
}
