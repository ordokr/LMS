use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::models::user::User;
use crate::services::api::{ApiClient, ApiError};

/// Represents a Canvas Course
/// Based on Canvas LMS Course model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: i64,
    pub name: Option<String>,
    pub course_code: Option<String>,
    pub workflow_state: Option<String>,
    pub account_id: Option<i64>,
    pub root_account_id: Option<i64>,
    pub enrollment_term_id: Option<i64>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub grading_standard_id: Option<i64>,
    pub is_public: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub course_format: Option<String>,
    pub restrict_enrollments_to_course_dates: Option<bool>,
    pub enrollment_count: Option<i32>,
    pub allow_student_forum_attachments: Option<bool>,
    pub open_enrollment: Option<bool>,
    pub self_enrollment: Option<bool>,
    pub license: Option<String>,
    pub allow_wiki_comments: Option<bool>,
    pub hide_final_grade: Option<bool>,
    pub time_zone: Option<String>,
    pub uuid: Option<String>,
    pub default_view: Option<String>,
    pub syllabus_body: Option<String>,
    pub course_color: Option<String>,
}

impl Course {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            course_code: None,
            workflow_state: None,
            account_id: None,
            root_account_id: None,
            enrollment_term_id: None,
            start_at: None,
            end_at: None,
            grading_standard_id: None,
            is_public: None,
            created_at: None,
            course_format: None,
            restrict_enrollments_to_course_dates: None,
            enrollment_count: None,
            allow_student_forum_attachments: None,
            open_enrollment: None,
            self_enrollment: None,
            license: None,
            allow_wiki_comments: None,
            hide_final_grade: None,
            time_zone: None,
            uuid: None,
            default_view: None,
            syllabus_body: None,
            course_color: None,
        }
    }
    
    /// Find published courses
    pub async fn find_published(api: &ApiClient) -> Result<Vec<Self>, ApiError> {
        let courses = api.get_courses().await?;
        Ok(courses.into_iter()
            .filter(|c| c.workflow_state.as_deref() == Some("available"))
            .collect())
    }
    
    /// Get all enrollments for this course
    pub async fn enrollments(&self, api: &ApiClient) -> Result<Vec<Enrollment>, ApiError> {
        api.get_course_enrollments(self.id).await
    }
    
    /// Get assignments for this course
    pub async fn assignments(&self, api: &ApiClient) -> Result<Vec<Assignment>, ApiError> {
        api.get_course_assignments(self.id).await
    }
    
    /// Get modules for this course
    pub async fn modules(&self, api: &ApiClient) -> Result<Vec<Module>, ApiError> {
        api.get_course_modules(self.id).await
    }
    
    /// Check if a user is enrolled in this course
    pub fn user_is_enrolled(&self, user_id: i64) -> bool {
        // Implementation would connect to backend service
        false
    }
    
    /// Check if the course is available to the user
    pub fn available_to_user(&self, user: &User) -> bool {
        // Implementation would check dates, enrollment, visibility
        match &self.workflow_state {
            Some(state) if state == "available" => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    Draft,
    Active,
    Archived,
}

/// Represents a Module in a Canvas course
/// Based on Canvas LMS ContextModule model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: i64,
    pub course_id: Option<i64>,
    pub name: Option<String>,
    pub position: Option<i32>,
    pub unlock_at: Option<DateTime<Utc>>,
    pub require_sequential_progress: Option<bool>,
    pub prerequisite_module_ids: Option<Vec<i64>>,
    pub items_count: Option<i32>,
    pub items_url: Option<String>,
    pub state: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub publish_final_grade: Option<bool>,
    pub workflow_state: Option<String>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            id: 0,
            course_id: None,
            name: None,
            position: None,
            unlock_at: None,
            require_sequential_progress: None,
            prerequisite_module_ids: None,
            items_count: None,
            items_url: None,
            state: None,
            completed_at: None,
            publish_final_grade: None,
            workflow_state: None,
        }
    }
    
    /// Get module items for this module
    pub fn items(&self) -> Vec<ModuleItem> {
        // Implementation would connect to backend service
        Vec::new()
    }
    
    /// Check if module is accessible to the user
    pub fn accessible_to_user(&self, user: &User) -> bool {
        // Implementation for accessibility logic
        true
    }
    
    /// Check if module is completed by the user
    pub fn completed_by_user(&self, user: &User) -> bool {
        // Implementation for completion status
        false
    }
}

/// Represents an item in a course module
/// Based on Canvas LMS ContextModuleItem model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: i64,
    pub module_id: Option<i64>,
    pub position: Option<i32>,
    pub title: Option<String>,
    pub indent: Option<i32>,
    pub content_type: Option<String>,
    pub content_id: Option<i64>,
    pub html_url: Option<String>,
    pub url: Option<String>,
    pub page_url: Option<String>,
    pub external_url: Option<String>,
    pub new_tab: Option<bool>,
    pub completion_requirement: Option<CompletionRequirement>,
    pub published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequirement {
    pub r#type: Option<String>, // min_score, must_view, must_submit, must_contribute
    pub min_score: Option<f32>,
    pub completed: Option<bool>,
}

impl ModuleItem {
    pub fn new() -> Self {
        Self {
            id: 0,
            module_id: None,
            position: None,
            title: None,
            indent: None,
            content_type: None,
            content_id: None,
            html_url: None,
            url: None,
            page_url: None,
            external_url: None,
            new_tab: None,
            completion_requirement: None,
            published: None,
        }
    }
    
    /// Get the content object for this module item
    pub fn content(&self) -> Result<serde_json::Value, String> {
        // Implementation would fetch the appropriate content based on content_type
        Err("Not implemented".to_string())
    }
    
    /// Mark this module item as viewed by the user
    pub fn mark_as_viewed(&self, user: &User) -> Result<bool, String> {
        // Implementation for view tracking
        Ok(true)
    }
    
    /// Check if this item is completed by the user
    pub fn completed_by_user(&self, user: &User) -> bool {
        match &self.completion_requirement {
            Some(req) => req.completed.unwrap_or(false),
            None => false,
        }
    }
}

// Update your Assignment struct with more fields from Canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Assignment model - ported from Canvas LMS
/// Reference: Canvas app/models/assignment.rb
pub struct Assignment {
    pub id: i64,
    pub course_id: i64,                     // Maps to context_id in Canvas (when context_type is 'Course')
    pub title: String,
    pub description: String,
    pub due_date: Option<String>,           // Maps to due_at in Canvas
    pub available_from: Option<String>,     // Maps to unlock_at in Canvas
    pub available_until: Option<String>,    // Maps to lock_at in Canvas
    pub points_possible: Option<f64>,
    pub submission_types: Vec<String>,
    pub published: bool,                    // Maps to workflow_state in Canvas
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    
    // Additional fields based on Canvas Assignment model
    pub grading_type: Option<String>,       // points, percent, letter_grade, etc.
    pub position: Option<i32>,              // Order within assignment groups
    pub peer_review_count: Option<i32>,     // Number of peer reviews
    pub group_category_id: Option<i64>,     // For group assignments
    pub grade_group_students_individually: Option<bool>,
    pub anonymous_grading: Option<bool>,
    pub allowed_attempts: Option<i32>,
    pub omit_from_final_grade: Option<bool>,
    pub assignment_group_id: Option<i64>,
    pub parent_assignment_id: Option<i64>,  // For sub-assignments
    pub sub_assignment_tag: Option<String>, // Tag for sub-assignments
}

impl Assignment {
    pub fn new(course_id: i64, title: String) -> Self {
        Self {
            id: 0, // Will be set when saved to database
            course_id,
            title,
            description: String::new(),
            due_date: None,
            available_from: None,
            available_until: None,
            points_possible: None,
            submission_types: vec!["none".to_string()],
            published: false,
            created_at: None,
            updated_at: None,
            
            // Initialize additional fields
            grading_type: Some("points".to_string()),
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
    
    /// Check if assignment can be unpublished
    /// Port of Canvas's can_unpublish? method
    pub fn can_unpublish(&self) -> bool {
        // In Canvas, this checks if there are any submissions
        // For now we'll return true, but this should be implemented
        // by checking if any Submission records exist for this assignment
        true // Replace with actual submission check
    }
    
    /// Check if this is a checkpoints parent assignment
    /// Port of Canvas's checkpoints_parent? method
    pub fn is_checkpoints_parent(&self) -> bool {
        self.parent_assignment_id.is_none() && self.sub_assignment_tag.is_none()
    }
    
    /// Check if this assignment has any student submissions
    /// This would usually involve a database query
    pub fn has_student_submissions(&self) -> bool {
        // In a real implementation, this would query the submissions table
        false // Replace with actual implementation
    }
    
    /// Get the effective group category ID
    /// Port of Canvas's effective_group_category_id method
    pub fn effective_group_category_id(&self) -> Option<i64> {
        self.group_category_id
    }
}

// After your Assignment struct...

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Assignment Group model - ported from Canvas LMS
/// Reference: Canvas app/models/assignment_group.rb
pub struct AssignmentGroup {
    pub id: i64,
    pub course_id: i64,
    pub name: String,
    pub position: Option<i32>,
    pub group_weight: Option<f64>,
    pub rules: Option<AssignmentGroupRules>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentGroupRules {
    pub drop_lowest: Option<i32>,
    pub drop_highest: Option<i32>,
    pub never_drop: Vec<i64>, // Assignment IDs that should never be dropped
}

impl AssignmentGroup {
    pub fn new(course_id: i64, name: String) -> Self {
        Self {
            id: 0,
            course_id,
            name,
            position: None,
            group_weight: None,
            rules: None,
            created_at: None,
            updated_at: None,
        }
    }
    
    pub fn with_rules(mut self, drop_lowest: Option<i32>, drop_highest: Option<i32>) -> Self {
        self.rules = Some(AssignmentGroupRules {
            drop_lowest,
            drop_highest,
            never_drop: Vec::new(),
        });
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: Option<i64>,
    pub user_id: i64,
    pub course_id: i64,
    pub role: EnrollmentRole,
    pub status: EnrollmentStatus,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentRole {
    Student,
    Teacher,
    TeachingAssistant,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EnrollmentStatus {
    Active,
    Invited,
    Completed,
    Rejected,
}

// Course creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCreationRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: CourseStatus,
}

// Module with items combined for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleWithItems {
    pub module: Module,
    pub items: Vec<ModuleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: Option<i64>,
    pub assignment_id: i64,
    pub user_id: i64,
    pub content: String,
    pub submission_type: String,
    pub submitted_at: Option<String>,
    pub grade: Option<f64>,
    pub score: Option<f64>,                  // Raw score before curve
    pub feedback: Option<String>,
    pub graded_by: Option<i64>,
    pub graded_at: Option<String>,
    pub attempt: Option<i32>,                // Which attempt this is
    pub late: bool,                          // If submission was late
    pub missing: bool,                       // If submission is missing
    pub excused: bool,                       // If submission is excused
    pub workflow_state: String,              // "submitted", "graded", etc.
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    
    // For group submissions
    pub group_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: Option<i64>,
    pub course_id: i64,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub front_page: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Represents a submission for an assignment
/// Based on Canvas LMS Submission model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: i64,
    pub assignment_id: Option<i64>,
    pub user_id: Option<i64>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub grade: Option<String>,
    pub score: Option<f32>,
    pub workflow_state: Option<String>,
    pub grade_matches_current_submission: Option<bool>,
    pub url: Option<String>,
    pub preview_url: Option<String>,
    pub submission_type: Option<String>,
    pub body: Option<String>,
    pub grade_status: Option<String>,
    pub attempt: Option<i32>,
    pub submission_comments: Option<Vec<SubmissionComment>>,
    pub late: Option<bool>,
    pub missing: Option<bool>,
    pub late_policy_status: Option<String>,
    pub points_deducted: Option<f32>,
    pub graded_at: Option<DateTime<Utc>>,
    pub grader_id: Option<i64>,
    pub excused: Option<bool>,
    pub posted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionComment {
    pub id: i64,
    pub author_id: Option<i64>,
    pub author_name: Option<String>,
    pub comment: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub edited_at: Option<DateTime<Utc>>,
}

impl Submission {
    pub fn new() -> Self {
        Self {
            id: 0,
            assignment_id: None,
            user_id: None,
            submitted_at: None,
            grade: None,
            score: None,
            workflow_state: None,
            grade_matches_current_submission: None,
            url: None,
            preview_url: None,
            submission_type: None,
            body: None,
            grade_status: None,
            attempt: None,
            submission_comments: None,
            late: None,
            missing: None,
            late_policy_status: None,
            points_deducted: None,
            graded_at: None,
            grader_id: None,
            excused: None,
            posted_at: None,
        }
    }
    
    /// Get the assignment for this submission
    pub fn assignment(&self) -> Option<Assignment> {
        // Implementation would fetch the assignment
        None
    }
    
    /// Get the user who created this submission
    pub fn user(&self) -> Option<User> {
        // Implementation would fetch the user
        None
    }
    
    /// Check if submission is late
    pub fn is_late(&self) -> bool {
        self.late.unwrap_or(false)
    }
    
    /// Add a comment to this submission
    pub fn add_comment(&mut self, user: &User, comment: &str) -> Result<SubmissionComment, String> {
        let new_comment = SubmissionComment {
            id: 0, // This would be set by the backend
            author_id: Some(user.id),
            author_name: user.name.clone(),
            comment: Some(comment.to_string()),
            created_at: Some(Utc::now()),
            edited_at: None,
        };
        
        match &mut self.submission_comments {
            Some(comments) => comments.push(new_comment.clone()),
            None => self.submission_comments = Some(vec![new_comment.clone()]),
        }
        
        Ok(new_comment)
    }
}

/// Represents a page in Canvas
/// Based on Canvas LMS WikiPage model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub editing_roles: Option<String>,
    pub published: Option<bool>,
    pub hide_from_students: Option<bool>,
    pub front_page: Option<bool>,
    pub locked_for_user: Option<bool>,
    pub lock_explanation: Option<String>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: None,
            body: None,
            url: None,
            created_at: None,
            updated_at: None,
            editing_roles: None,
            published: None,
            hide_from_students: None,
            front_page: None,
            locked_for_user: None,
            lock_explanation: None,
        }
    }
    
    /// Check if the page is visible to the user
    pub fn visible_to(&self, user: &User) -> bool {
        // Students can't see unpublished pages or hidden pages
        if user.has_role("student") {
            if let Some(published) = self.published {
                if !published {
                    return false;
                }
            }
            
            if let Some(hidden) = self.hide_from_students {
                if hidden {
                    return false;
                }
            }
        }
        
        // Check if locked for the user
        !self.locked_for_user.unwrap_or(false)
    }
    
    /// Check if the user can edit this page
    pub fn can_edit(&self, user: &User) -> bool {
        // Teachers and admins can always edit
        if user.has_role("teacher") || user.has_role("admin") {
            return true;
        }
        
        // Check editing_roles for other users
        match &self.editing_roles {
            Some(roles) => {
                if roles == "teachers,students" || roles == "anyone" {
                    return true;
                }
                
                if roles == "teachers,tas" && user.has_role("ta") {
                    return true;
                }
            },
            None => return false,
        }
        
        false
    }
}

