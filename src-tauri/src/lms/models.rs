use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

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
    pub syllabus: Option<String>,
    pub visibility: CourseVisibility,
    pub allow_self_enrollment: bool,
    pub enrollment_code: Option<String>,
    pub homepage_type: HomepageType,
    pub theme_color: Option<String>,
    pub banner_image_url: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CourseStatus {
    Draft,
    Active,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CourseVisibility {
    Public,
    InstitutionOnly,
    CourseMembers,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum HomepageType {
    ActivityStream,
    Syllabus,
    Modules,
    Assignments,
    CustomPage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: Option<i64>,
    pub course_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub position: i32,
    pub published: bool,
    pub unlock_at: Option<String>,
    pub require_sequential_progress: bool,
    pub prerequisite_module_ids: Vec<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: Option<i64>,
    pub module_id: i64,
    pub title: String,
    pub item_type: ModuleItemType,
    pub content_id: Option<i64>,
    pub content_type: Option<String>,
    pub page_url: Option<String>,
    pub external_url: Option<String>,
    pub position: i32,
    pub indent_level: i32,
    pub published: bool,
    pub completion_requirement: Option<CompletionRequirement>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ModuleItemType {
    Assignment,
    Quiz,
    File,
    Page,
    Discussion,
    ExternalUrl,
    ExternalTool,
    Header,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequirement {
    pub requirement_type: CompletionRequirementType,
    pub min_score: Option<f64>,
    pub completed: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CompletionRequirementType {
    MustView,
    MustSubmit,
    MustContribute,
    MinScore,
    MarkDone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Option<i64>,
    pub course_id: i64,
    pub name: String,
    pub description: String,
    pub due_at: Option<String>,
    pub unlock_at: Option<String>,
    pub lock_at: Option<String>,
    pub points_possible: Option<f64>,
    pub grading_type: GradingType,
    pub submission_types: Vec<SubmissionType>,
    pub position: i32,
    pub published: bool,
    pub group_category_id: Option<i64>,
    pub peer_reviews: bool,
    pub automatic_peer_reviews: bool,
    pub peer_review_count: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GradingType {
    Points,
    Percentage,
    LetterGrade,
    GpaScale,
    PassFail,
    NotGraded,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SubmissionType {
    None,
    OnlineText,
    OnlineUrl,
    OnlineUpload,
    MediaRecording,
    Discussion,
    Quiz,
    ExternalTool,
    NotGraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: Option<i64>,
    pub assignment_id: i64,
    pub user_id: i64,
    pub submitted_at: Option<String>,
    pub submission_type: Option<SubmissionType>,
    pub submission_data: Option<String>,
    pub url: Option<String>,
    pub grade: Option<String>,
    pub score: Option<f64>,
    pub graded_at: Option<String>,
    pub grader_id: Option<i64>,
    pub attempt: i32,
    pub feedback_comment: Option<String>,
    pub feedback_files: Vec<SubmissionFile>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionFile {
    pub id: Option<i64>,
    pub submission_id: i64,
    pub filename: String,
    pub display_name: String,
    pub content_type: String,
    pub size: i64,
    pub url: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPage {
    pub id: Option<i64>,
    pub course_id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub front_page: bool,
    pub url: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: Option<i64>,
    pub course_id: i64,
    pub user_id: i64,
    pub role: EnrollmentRole,
    pub enrollment_state: EnrollmentState,
    pub limit_privileges_to_course_section: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EnrollmentRole {
    Student,
    Teacher,
    TeachingAssistant,
    Designer,
    Observer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EnrollmentState {
    Active,
    Invited,
    Inactive,
    Completed,
    Rejected,
}