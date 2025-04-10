// src/models/course.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unified Course model
/// Maps between Canvas Course and Discourse Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    // Core fields
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Canvas-specific fields
    pub canvas_id: Option<String>,
    pub course_code: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub enrollments: Vec<serde_json::Value>,
    pub term: serde_json::Value,
    
    // Discourse-specific fields
    pub discourse_id: Option<String>,
    pub slug: String,
    pub color: String,
    pub position: i32,
    pub parent_id: Option<String>,
    
    // Integration fields
    pub last_sync: Option<DateTime<Utc>>,
    pub source_system: String,
}

impl Course {
    /// Create a new Course instance
    pub fn new(
        id: Option<String>,
        title: Option<String>,
        description: Option<String>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        canvas_id: Option<String>,
        course_code: Option<String>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        enrollments: Option<Vec<serde_json::Value>>,
        term: Option<serde_json::Value>,
        discourse_id: Option<String>,
        slug: Option<String>,
        color: Option<String>,
        position: Option<i32>,
        parent_id: Option<String>,
        last_sync: Option<DateTime<Utc>>,
        source_system: Option<String>,
    ) -> Self {
        let title_str = title.unwrap_or_default();
        let now = Utc::now();
        
        Course {
            id,
            title: title_str.clone(),
            description: description.unwrap_or_default(),
            created_at: created_at.unwrap_or(now),
            updated_at: updated_at.unwrap_or(now),
            canvas_id,
            course_code: course_code.unwrap_or_default(),
            start_date,
            end_date,
            enrollments: enrollments.unwrap_or_default(),
            term: term.unwrap_or_else(|| serde_json::json!({})),
            discourse_id,
            slug: slug.unwrap_or_else(|| Self::generate_slug(&title_str)),
            color: color.unwrap_or_else(|| "#0073A7".to_string()), // Default Canvas blue
            position: position.unwrap_or(0),
            parent_id,
            last_sync,
            source_system: source_system.unwrap_or_else(|| "canvas".to_string()),
        }
    }
    
    /// Generate a slug from title
    fn generate_slug(title: &str) -> String {
        if title.is_empty() {
            return String::new();
        }
        
        title.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .replace("--", "-")
    }

    /// Create a Course from Canvas data
    pub fn from_canvas(canvas_data: &serde_json::Value) -> Self {
        let id = canvas_data["id"].as_str().map(String::from);
        
        let title = canvas_data["name"].as_str().map(String::from);
        
        let description = canvas_data["description"].as_str()
            .or_else(|| canvas_data["about"].as_str())
            .map(String::from);
            
        let created_at = canvas_data["created_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let updated_at = canvas_data["updated_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let course_code = canvas_data["course_code"].as_str().map(String::from);
        
        let start_date = canvas_data["start_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let end_date = canvas_data["end_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let enrollments = canvas_data["enrollments"].as_array()
            .map(|arr| arr.clone());
            
        let term = canvas_data["term"].as_object()
            .map(|obj| serde_json::Value::Object(obj.clone()));
        
        Self::new(
            None,
            title,
            description,
            created_at,
            updated_at,
            id,
            course_code,
            start_date,
            end_date,
            enrollments,
            term,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("canvas".to_string()),
        )
    }
    
    /// Create a Course from Discourse data
    pub fn from_discourse(discourse_data: &serde_json::Value) -> Self {
        let id = discourse_data["id"].as_str().map(String::from);
        
        let title = discourse_data["name"].as_str()
            .or_else(|| discourse_data["title"].as_str())
            .map(String::from);
            
        let description = discourse_data["description"].as_str()
            .or_else(|| discourse_data["about"].as_str())
            .map(String::from);
            
        let created_at = discourse_data["created_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let updated_at = discourse_data["updated_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let slug = discourse_data["slug"].as_str().map(String::from);
        
        let color = discourse_data["color"].as_str()
            .or_else(|| discourse_data["category_color"].as_str())
            .map(String::from);
            
        let position = discourse_data["position"].as_i64().map(|v| v as i32);
        
        let parent_id = discourse_data["parent_category_id"].as_str()
            .or_else(|| discourse_data["parent_id"].as_str())
            .map(String::from);
        
        Self::new(
            None,
            title,
            description,
            created_at,
            updated_at,
            None,
            None,
            None,
            None,
            None,
            None,
            id,
            slug,
            color,
            position,
            parent_id,
            None,
            Some("discourse".to_string()),
        )
    }
    
    /// Convert to Canvas format
    pub fn to_canvas(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.canvas_id,
            "name": self.title,
            "description": self.description,
            "course_code": self.course_code,
            "start_at": self.start_date,
            "end_at": self.end_date,
            "enrollments": self.enrollments,
            "term": self.term,
            "created_at": self.created_at,
            "updated_at": self.updated_at
        })
    }
    
    /// Convert to Discourse format
    pub fn to_discourse(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "name": self.title,
            "description": self.description,
            "slug": self.slug,
            "color": self.color,
            "position": self.position,
            "parent_category_id": self.parent_id,
            "created_at": self.created_at,
            "updated_at": self.updated_at
        })
    }
}
