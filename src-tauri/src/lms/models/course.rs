use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i64>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub instructor_id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub published: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseSettings {
    pub id: Option<i64>,
    pub course_id: i64,
    pub allow_student_discussion_topics: bool,
    pub allow_student_discussion_editing: bool,
    pub allow_student_forum_attachments: bool,
    pub restrict_student_past_view: bool,
    pub restrict_student_future_view: bool,
    pub hide_final_grades: bool,
    pub hide_distribution_graphs: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseSection {
    pub id: Option<i64>,
    pub course_id: i64,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourseUserRole {
    Student,
    Teacher,
    TeacherAssistant,
    Designer,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseUser {
    pub id: Option<i64>,
    pub course_id: i64,
    pub user_id: i64,
    pub role: CourseUserRole,
    pub section_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}