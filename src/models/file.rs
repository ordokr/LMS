use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// File model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// Unique identifier
    pub id: Uuid,
    /// File name
    pub name: String,
    /// File size in bytes
    pub size: u64,
    /// File content type
    pub content_type: String,
    /// File URL
    pub url: String,
    /// File display name
    pub display_name: String,
    /// Course ID
    pub course_id: Option<Uuid>,
    /// User ID
    pub user_id: Option<Uuid>,
    /// Assignment ID
    pub assignment_id: Option<Uuid>,
    /// Submission ID
    pub submission_id: Option<Uuid>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Canvas ID
    pub canvas_id: Option<String>,
    /// Discourse upload ID
    pub discourse_upload_id: Option<String>,
}

impl File {
    /// Create a new file
    pub fn new(
        name: String,
        size: u64,
        content_type: String,
        url: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.clone(),
            size,
            content_type,
            url,
            display_name: name,
            course_id: None,
            user_id: None,
            assignment_id: None,
            submission_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canvas_id: None,
            discourse_upload_id: None,
        }
    }
    
    /// Set display name
    pub fn with_display_name(mut self, display_name: &str) -> Self {
        self.display_name = display_name.to_string();
        self
    }
    
    /// Set course ID
    pub fn with_course_id(mut self, course_id: Uuid) -> Self {
        self.course_id = Some(course_id);
        self
    }
    
    /// Set user ID
    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    /// Set assignment ID
    pub fn with_assignment_id(mut self, assignment_id: Uuid) -> Self {
        self.assignment_id = Some(assignment_id);
        self
    }
    
    /// Set submission ID
    pub fn with_submission_id(mut self, submission_id: Uuid) -> Self {
        self.submission_id = Some(submission_id);
        self
    }
    
    /// Set Canvas ID
    pub fn with_canvas_id(mut self, canvas_id: &str) -> Self {
        self.canvas_id = Some(canvas_id.to_string());
        self
    }
    
    /// Set Discourse upload ID
    pub fn with_discourse_upload_id(mut self, discourse_upload_id: &str) -> Self {
        self.discourse_upload_id = Some(discourse_upload_id.to_string());
        self
    }
    
    /// Get file extension
    pub fn extension(&self) -> Option<&str> {
        self.name.split('.').last()
    }
    
    /// Check if the file is an image
    pub fn is_image(&self) -> bool {
        self.content_type.starts_with("image/")
    }
    
    /// Check if the file is a document
    pub fn is_document(&self) -> bool {
        matches!(
            self.content_type.as_str(),
            "application/pdf"
                | "application/msword"
                | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                | "application/vnd.ms-excel"
                | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                | "application/vnd.ms-powerpoint"
                | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                | "text/plain"
        )
    }
    
    /// Check if the file is a video
    pub fn is_video(&self) -> bool {
        self.content_type.starts_with("video/")
    }
    
    /// Check if the file is an audio
    pub fn is_audio(&self) -> bool {
        self.content_type.starts_with("audio/")
    }
    
    /// Format file size for display
    pub fn formatted_size(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.2} KB", self.size as f64 / 1024.0)
        } else if self.size < 1024 * 1024 * 1024 {
            format!("{:.2} MB", self.size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", self.size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}
