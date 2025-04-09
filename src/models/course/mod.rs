use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub instructor_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: CourseStatus,
    pub access_code: Option<String>,
    pub enrollment_count: i32,
    pub category_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub canvas_course_id: Option<String>,
    pub discourse_category_id: Option<String>,
    pub integration_status: IntegrationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    Active,
    Upcoming,
    Completed,
    Archived,
    Draft,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationStatus {
    NotIntegrated,
    PartiallyIntegrated,
    FullyIntegrated,
    SyncError,
}

impl Course {
    // Associate course with Canvas
    pub fn set_canvas_integration(&mut self, canvas_id: String) {
        self.canvas_course_id = Some(canvas_id);
        self.update_integration_status();
    }
    
    // Associate course with Discourse
    pub fn set_discourse_integration(&mut self, discourse_id: String) {
        self.discourse_category_id = Some(discourse_id);
        self.update_integration_status();
    }
    
    // Update integration status based on current state
    fn update_integration_status(&mut self) {
        self.integration_status = match (self.canvas_course_id.is_some(), self.discourse_category_id.is_some()) {
            (true, true) => IntegrationStatus::FullyIntegrated,
            (true, false) | (false, true) => IntegrationStatus::PartiallyIntegrated,
            (false, false) => IntegrationStatus::NotIntegrated,
        };
    }
}

// Course module sub-types
mod enrollment;
mod assignment;
mod material;
mod integration;

pub use enrollment::Enrollment;
pub use assignment::Assignment;
pub use material::Material;
pub use integration::CourseIntegrationSettings;