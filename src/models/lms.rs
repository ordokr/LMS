use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub instructor_id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: CourseStatus,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    Draft,
    Active,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: Option<i64>,
    pub course_id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub published: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: Option<i64>,
    pub module_id: i64,
    pub title: Option<String>,
    pub content_type: Option<String>, // "page", "assignment", "file", "discussion", "quiz", "url"
    pub content_id: Option<i64>,
    pub position: Option<i32>,
    pub published: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Option<i64>,
    pub course_id: i64,
    pub title: String,
    pub description: String,
    pub due_date: Option<String>,
    pub available_from: Option<String>,
    pub available_until: Option<String>,
    pub points_possible: Option<f64>,
    pub submission_types: Vec<String>,
    pub published: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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
    pub feedback: Option<String>,
    pub graded_by: Option<i64>,
    pub graded_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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