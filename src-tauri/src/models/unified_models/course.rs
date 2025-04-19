use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Course status enum representing the possible states of a course
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CourseStatus {
    Draft,
    Active,
    Archived,
    Unpublished,
    Deleted,
}

impl Default for CourseStatus {
    fn default() -> Self {
        CourseStatus::Draft
    }
}

impl std::fmt::Display for CourseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CourseStatus::Draft => write!(f, "draft"),
            CourseStatus::Active => write!(f, "active"),
            CourseStatus::Archived => write!(f, "archived"),
            CourseStatus::Unpublished => write!(f, "unpublished"),
            CourseStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl From<&str> for CourseStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "draft" => CourseStatus::Draft,
            "active" => CourseStatus::Active,
            "archived" => CourseStatus::Archived,
            "unpublished" => CourseStatus::Unpublished,
            "deleted" => CourseStatus::Deleted,
            _ => CourseStatus::Draft,
        }
    }
}

/// Course visibility enum representing who can see the course
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CourseVisibility {
    Public,
    Institution,
    Course,
    Private,
}

impl Default for CourseVisibility {
    fn default() -> Self {
        CourseVisibility::Course
    }
}

impl std::fmt::Display for CourseVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CourseVisibility::Public => write!(f, "public"),
            CourseVisibility::Institution => write!(f, "institution"),
            CourseVisibility::Course => write!(f, "course"),
            CourseVisibility::Private => write!(f, "private"),
        }
    }
}

/// Homepage type enum representing the default landing page for a course
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HomepageType {
    Feed,
    Modules,
    Assignments,
    Syllabus,
    Custom,
}

impl Default for HomepageType {
    fn default() -> Self {
        HomepageType::Modules
    }
}

impl std::fmt::Display for HomepageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomepageType::Feed => write!(f, "feed"),
            HomepageType::Modules => write!(f, "modules"),
            HomepageType::Assignments => write!(f, "assignments"),
            HomepageType::Syllabus => write!(f, "syllabus"),
            HomepageType::Custom => write!(f, "custom"),
        }
    }
}

/// Unified Course model that harmonizes all existing course implementations
/// This model is designed to be compatible with both Canvas and Discourse course/category models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    // Core fields
    pub id: String,                           // Primary identifier (UUID)
    pub name: String,                         // Course name
    pub code: String,                         // Course code
    pub description: Option<String>,          // Course description
    pub created_at: DateTime<Utc>,            // Creation timestamp
    pub updated_at: DateTime<Utc>,            // Last update timestamp
    
    // Status and visibility
    pub status: CourseStatus,                 // Course status
    pub visibility: CourseVisibility,         // Course visibility
    pub is_public: bool,                      // Whether the course is public
    pub is_published: bool,                   // Whether the course is published
    
    // Dates
    pub start_date: Option<DateTime<Utc>>,    // Start date
    pub end_date: Option<DateTime<Utc>>,      // End date
    
    // Instructor and enrollment
    pub instructor_id: Option<String>,        // Primary instructor ID
    pub allow_self_enrollment: bool,          // Whether self-enrollment is allowed
    pub enrollment_code: Option<String>,      // Enrollment access code
    pub enrollment_count: Option<i32>,        // Number of enrollments
    
    // Content and display
    pub syllabus_body: Option<String>,        // Syllabus content
    pub homepage_type: HomepageType,          // Default homepage type
    pub default_view: String,                 // Default view
    pub theme_color: Option<String>,          // Theme color
    pub banner_image_url: Option<String>,     // Banner image URL
    pub timezone: Option<String>,             // Course timezone
    pub license: Option<String>,              // Content license
    
    // External system IDs
    pub canvas_id: Option<String>,            // Canvas course ID
    pub discourse_id: Option<String>,         // Discourse category ID
    pub category_id: Option<String>,          // Category ID
    
    // Discourse-specific fields
    pub slug: Option<String>,                 // URL slug
    pub color: Option<String>,                // Category color
    pub position: Option<i32>,                // Display position
    pub parent_id: Option<String>,            // Parent category ID
    
    // Integration fields
    pub last_sync: Option<DateTime<Utc>>,     // Last sync timestamp
    pub source_system: Option<String>,        // Source system (canvas, discourse, etc.)
    
    // Metadata and extensibility
    pub metadata: HashMap<String, serde_json::Value>, // Extensible metadata
}

impl Course {
    /// Create a new Course with default values
    pub fn new(
        id: Option<String>,
        name: String,
        code: String,
    ) -> Self {
        let now = Utc::now();
        let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        Self {
            id,
            name,
            code,
            description: None,
            created_at: now,
            updated_at: now,
            status: CourseStatus::Draft,
            visibility: CourseVisibility::Course,
            is_public: false,
            is_published: false,
            start_date: None,
            end_date: None,
            instructor_id: None,
            allow_self_enrollment: false,
            enrollment_code: None,
            enrollment_count: None,
            syllabus_body: None,
            homepage_type: HomepageType::Modules,
            default_view: "modules".to_string(),
            theme_color: None,
            banner_image_url: None,
            timezone: None,
            license: None,
            canvas_id: None,
            discourse_id: None,
            category_id: None,
            slug: None,
            color: None,
            position: None,
            parent_id: None,
            last_sync: None,
            source_system: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a Course from a Canvas course JSON
    pub fn from_canvas_course(canvas_course: &serde_json::Value) -> Self {
        let id = Uuid::new_v4().to_string();
        let canvas_id = canvas_course["id"].as_str()
            .or_else(|| canvas_course["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let name = canvas_course["name"].as_str().unwrap_or("").to_string();
        let code = canvas_course["course_code"].as_str().unwrap_or("").to_string();
        let description = canvas_course["description"].as_str().map(|s| s.to_string());
        let syllabus_body = canvas_course["syllabus_body"].as_str().map(|s| s.to_string());
        
        // Parse dates
        let start_date = canvas_course["start_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let end_date = canvas_course["end_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        // Parse status
        let workflow_state = canvas_course["workflow_state"].as_str().unwrap_or("unpublished");
        let status = match workflow_state {
            "available" => CourseStatus::Active,
            "completed" => CourseStatus::Archived,
            "deleted" => CourseStatus::Deleted,
            _ => CourseStatus::Unpublished,
        };
        
        // Parse visibility
        let is_public = canvas_course["is_public"].as_bool().unwrap_or(false);
        let visibility = if is_public {
            CourseVisibility::Public
        } else {
            CourseVisibility::Course
        };
        
        // Parse default view
        let default_view = canvas_course["default_view"].as_str()
            .unwrap_or("modules")
            .to_string();
            
        let homepage_type = match default_view.as_str() {
            "feed" => HomepageType::Feed,
            "wiki" | "syllabus" => HomepageType::Syllabus,
            "assignments" => HomepageType::Assignments,
            _ => HomepageType::Modules,
        };
        
        // Convert the canvas_course to a HashMap for metadata
        let metadata = serde_json::to_value(canvas_course).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();
        
        let now = Utc::now();
        
        Self {
            id,
            name,
            code,
            description,
            created_at: now,
            updated_at: now,
            status,
            visibility,
            is_public,
            is_published: workflow_state == "available",
            start_date,
            end_date,
            instructor_id: None, // Canvas doesn't provide this directly
            allow_self_enrollment: false,
            enrollment_code: None,
            enrollment_count: None,
            syllabus_body,
            homepage_type,
            default_view,
            theme_color: None,
            banner_image_url: None,
            timezone: None,
            license: None,
            canvas_id: Some(canvas_id),
            discourse_id: None,
            category_id: None,
            slug: None,
            color: None,
            position: None,
            parent_id: None,
            last_sync: None,
            source_system: Some("canvas".to_string()),
            metadata,
        }
    }
    
    /// Create a Course from a Discourse category JSON
    pub fn from_discourse_category(discourse_category: &serde_json::Value) -> Self {
        let id = Uuid::new_v4().to_string();
        let discourse_id = discourse_category["id"].as_str()
            .or_else(|| discourse_category["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let name = discourse_category["name"].as_str().unwrap_or("").to_string();
        
        // Generate a code from the name if not available
        let code = discourse_category["course_code"].as_str()
            .unwrap_or_else(|| {
                // Generate a code from the first letters of each word in the name
                name.split_whitespace()
                    .filter_map(|word| word.chars().next())
                    .collect::<String>()
                    .to_uppercase()
            })
            .to_string();
            
        let description = discourse_category["description"].as_str()
            .or_else(|| discourse_category["description_text"].as_str())
            .map(|s| s.to_string());
            
        let slug = discourse_category["slug"].as_str().map(|s| s.to_string());
        let color = discourse_category["color"].as_str().map(|s| s.to_string());
        let position = discourse_category["position"].as_i64().map(|p| p as i32);
        let parent_id = discourse_category["parent_category_id"].as_str()
            .or_else(|| discourse_category["parent_category_id"].as_i64().map(|id| id.to_string()));
        
        // Convert the discourse_category to a HashMap for metadata
        let metadata = serde_json::to_value(discourse_category).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();
        
        let now = Utc::now();
        
        Self {
            id,
            name,
            code,
            description,
            created_at: now,
            updated_at: now,
            status: CourseStatus::Active, // Default for Discourse categories
            visibility: CourseVisibility::Public, // Default for Discourse categories
            is_public: true,
            is_published: true,
            start_date: None,
            end_date: None,
            instructor_id: None,
            allow_self_enrollment: true,
            enrollment_code: None,
            enrollment_count: None,
            syllabus_body: None,
            homepage_type: HomepageType::Feed, // Default for Discourse
            default_view: "feed".to_string(),
            theme_color: None,
            banner_image_url: None,
            timezone: None,
            license: None,
            canvas_id: None,
            discourse_id: Some(discourse_id),
            category_id: Some(discourse_id.clone()),
            slug,
            color,
            position,
            parent_id,
            last_sync: None,
            source_system: Some("discourse".to_string()),
            metadata,
        }
    }
    
    /// Convert Course to Canvas course JSON
    pub fn to_canvas_course(&self) -> serde_json::Value {
        // Map status to Canvas workflow_state
        let workflow_state = match self.status {
            CourseStatus::Active => "available",
            CourseStatus::Archived => "completed",
            CourseStatus::Deleted => "deleted",
            _ => "unpublished",
        };
        
        serde_json::json!({
            "id": self.canvas_id,
            "name": self.name,
            "course_code": self.code,
            "description": self.description,
            "syllabus_body": self.syllabus_body,
            "start_at": self.start_date.map(|dt| dt.to_rfc3339()),
            "end_at": self.end_date.map(|dt| dt.to_rfc3339()),
            "workflow_state": workflow_state,
            "is_public": self.is_public,
            "default_view": self.default_view,
            "license": self.license,
        })
    }
    
    /// Convert Course to Discourse category JSON
    pub fn to_discourse_category(&self) -> serde_json::Value {
        // Generate slug if not available
        let slug = self.slug.clone().unwrap_or_else(|| {
            self.name.to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>()
        });
        
        serde_json::json!({
            "id": self.discourse_id,
            "name": self.name,
            "description": self.description,
            "slug": slug,
            "color": self.color,
            "position": self.position,
            "parent_category_id": self.parent_id,
        })
    }
    
    /// Activate the course
    pub fn activate(&mut self) {
        self.status = CourseStatus::Active;
        self.is_published = true;
        self.updated_at = Utc::now();
    }
    
    /// Archive the course
    pub fn archive(&mut self) {
        self.status = CourseStatus::Archived;
        self.updated_at = Utc::now();
    }
    
    /// Unpublish the course
    pub fn unpublish(&mut self) {
        self.status = CourseStatus::Unpublished;
        self.is_published = false;
        self.updated_at = Utc::now();
    }
    
    /// Delete the course
    pub fn delete(&mut self) {
        self.status = CourseStatus::Deleted;
        self.is_published = false;
        self.updated_at = Utc::now();
    }
    
    /// Check if the course is active
    pub fn is_active(&self) -> bool {
        self.status == CourseStatus::Active
    }
    
    /// Check if the course is archived
    pub fn is_archived(&self) -> bool {
        self.status == CourseStatus::Archived
    }
    
    /// Check if the course is deleted
    pub fn is_deleted(&self) -> bool {
        self.status == CourseStatus::Deleted
    }
    
    /// Get the course URL slug
    pub fn get_slug(&self) -> String {
        self.slug.clone().unwrap_or_else(|| {
            self.name.to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_course() {
        let course = Course::new(
            None,
            "Introduction to Computer Science".to_string(),
            "CS101".to_string(),
        );
        
        assert_eq!(course.name, "Introduction to Computer Science");
        assert_eq!(course.code, "CS101");
        assert_eq!(course.status, CourseStatus::Draft);
        assert_eq!(course.is_published, false);
        assert_eq!(course.homepage_type, HomepageType::Modules);
        assert_eq!(course.default_view, "modules");
    }
    
    #[test]
    fn test_from_canvas_course() {
        let canvas_json = serde_json::json!({
            "id": "12345",
            "name": "Advanced Mathematics",
            "course_code": "MATH301",
            "description": "Advanced topics in mathematics",
            "syllabus_body": "<p>Course syllabus content</p>",
            "start_at": "2023-01-15T00:00:00Z",
            "end_at": "2023-05-15T00:00:00Z",
            "workflow_state": "available",
            "is_public": true,
            "default_view": "syllabus"
        });
        
        let course = Course::from_canvas_course(&canvas_json);
        
        assert_eq!(course.name, "Advanced Mathematics");
        assert_eq!(course.code, "MATH301");
        assert_eq!(course.description, Some("Advanced topics in mathematics".to_string()));
        assert_eq!(course.syllabus_body, Some("<p>Course syllabus content</p>".to_string()));
        assert_eq!(course.status, CourseStatus::Active);
        assert_eq!(course.is_public, true);
        assert_eq!(course.is_published, true);
        assert_eq!(course.homepage_type, HomepageType::Syllabus);
        assert_eq!(course.default_view, "syllabus");
        assert_eq!(course.canvas_id, Some("12345".to_string()));
        assert_eq!(course.source_system, Some("canvas".to_string()));
        
        // Check date parsing
        assert!(course.start_date.is_some());
        assert!(course.end_date.is_some());
        if let Some(start) = course.start_date {
            assert_eq!(start.date().to_string(), "2023-01-15");
        }
        if let Some(end) = course.end_date {
            assert_eq!(end.date().to_string(), "2023-05-15");
        }
    }
    
    #[test]
    fn test_from_discourse_category() {
        let discourse_json = serde_json::json!({
            "id": "67890",
            "name": "Physics Discussion",
            "description": "Forum for physics discussions",
            "slug": "physics-discussion",
            "color": "3498DB",
            "position": 3,
            "parent_category_id": "54321"
        });
        
        let course = Course::from_discourse_category(&discourse_json);
        
        assert_eq!(course.name, "Physics Discussion");
        assert_eq!(course.description, Some("Forum for physics discussions".to_string()));
        assert_eq!(course.status, CourseStatus::Active);
        assert_eq!(course.is_public, true);
        assert_eq!(course.is_published, true);
        assert_eq!(course.homepage_type, HomepageType::Feed);
        assert_eq!(course.default_view, "feed");
        assert_eq!(course.discourse_id, Some("67890".to_string()));
        assert_eq!(course.category_id, Some("67890".to_string()));
        assert_eq!(course.slug, Some("physics-discussion".to_string()));
        assert_eq!(course.color, Some("3498DB".to_string()));
        assert_eq!(course.position, Some(3));
        assert_eq!(course.parent_id, Some("54321".to_string()));
        assert_eq!(course.source_system, Some("discourse".to_string()));
    }
    
    #[test]
    fn test_to_canvas_course() {
        let mut course = Course::new(
            Some("abcd1234".to_string()),
            "Biology 101".to_string(),
            "BIO101".to_string(),
        );
        
        course.canvas_id = Some("54321".to_string());
        course.description = Some("Introduction to biology".to_string());
        course.syllabus_body = Some("<p>Biology syllabus</p>".to_string());
        course.start_date = Some(DateTime::parse_from_rfc3339("2023-09-01T00:00:00Z").unwrap().with_timezone(&Utc));
        course.end_date = Some(DateTime::parse_from_rfc3339("2023-12-15T00:00:00Z").unwrap().with_timezone(&Utc));
        course.status = CourseStatus::Active;
        course.is_public = true;
        course.default_view = "syllabus".to_string();
        course.license = Some("CC-BY".to_string());
        
        let canvas_course = course.to_canvas_course();
        
        assert_eq!(canvas_course["id"], "54321");
        assert_eq!(canvas_course["name"], "Biology 101");
        assert_eq!(canvas_course["course_code"], "BIO101");
        assert_eq!(canvas_course["description"], "Introduction to biology");
        assert_eq!(canvas_course["syllabus_body"], "<p>Biology syllabus</p>");
        assert_eq!(canvas_course["workflow_state"], "available");
        assert_eq!(canvas_course["is_public"], true);
        assert_eq!(canvas_course["default_view"], "syllabus");
        assert_eq!(canvas_course["license"], "CC-BY");
        
        // Check date formatting
        assert!(canvas_course["start_at"].as_str().unwrap().starts_with("2023-09-01"));
        assert!(canvas_course["end_at"].as_str().unwrap().starts_with("2023-12-15"));
    }
    
    #[test]
    fn test_to_discourse_category() {
        let mut course = Course::new(
            Some("efgh5678".to_string()),
            "Chemistry Forum".to_string(),
            "CHEM".to_string(),
        );
        
        course.discourse_id = Some("98765".to_string());
        course.description = Some("Forum for chemistry discussions".to_string());
        course.slug = Some("chemistry-forum".to_string());
        course.color = Some("E74C3C".to_string());
        course.position = Some(2);
        course.parent_id = Some("12345".to_string());
        
        let discourse_category = course.to_discourse_category();
        
        assert_eq!(discourse_category["id"], "98765");
        assert_eq!(discourse_category["name"], "Chemistry Forum");
        assert_eq!(discourse_category["description"], "Forum for chemistry discussions");
        assert_eq!(discourse_category["slug"], "chemistry-forum");
        assert_eq!(discourse_category["color"], "E74C3C");
        assert_eq!(discourse_category["position"], 2);
        assert_eq!(discourse_category["parent_category_id"], "12345");
    }
    
    #[test]
    fn test_course_status_methods() {
        let mut course = Course::new(
            None,
            "Test Course".to_string(),
            "TEST101".to_string(),
        );
        
        // Test activate
        course.activate();
        assert_eq!(course.status, CourseStatus::Active);
        assert_eq!(course.is_published, true);
        assert!(course.is_active());
        
        // Test archive
        course.archive();
        assert_eq!(course.status, CourseStatus::Archived);
        assert!(course.is_archived());
        
        // Test unpublish
        course.unpublish();
        assert_eq!(course.status, CourseStatus::Unpublished);
        assert_eq!(course.is_published, false);
        
        // Test delete
        course.delete();
        assert_eq!(course.status, CourseStatus::Deleted);
        assert_eq!(course.is_published, false);
        assert!(course.is_deleted());
    }
    
    #[test]
    fn test_get_slug() {
        let mut course = Course::new(
            None,
            "Advanced Programming Concepts".to_string(),
            "CS301".to_string(),
        );
        
        // Test auto-generated slug
        assert_eq!(course.get_slug(), "advanced-programming-concepts");
        
        // Test custom slug
        course.slug = Some("adv-programming".to_string());
        assert_eq!(course.get_slug(), "adv-programming");
    }
}
