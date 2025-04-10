// src/models/assignment.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unified Assignment model
/// Maps between Canvas Assignment and Discourse CustomField
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    // Core fields
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Canvas-specific fields
    pub canvas_id: Option<String>,
    pub course_id: Option<String>,
    pub due_at: Option<DateTime<Utc>>,
    pub lock_at: Option<DateTime<Utc>>,
    pub unlock_at: Option<DateTime<Utc>>,
    pub points_possible: f64,
    pub submission_types: Vec<String>,
    pub grading_type: String,
    
    // Discourse-specific fields
    pub discourse_id: Option<String>,
    pub topic_id: Option<String>,
    pub category_id: Option<String>,
    
    // Integration fields
    pub last_sync: Option<DateTime<Utc>>,
    pub source_system: String,
}

impl Assignment {
    /// Create a new Assignment instance
    pub fn new(
        id: Option<String>,
        title: Option<String>,
        description: Option<String>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        canvas_id: Option<String>,
        course_id: Option<String>,
        due_at: Option<DateTime<Utc>>,
        lock_at: Option<DateTime<Utc>>,
        unlock_at: Option<DateTime<Utc>>,
        points_possible: Option<f64>,
        submission_types: Option<Vec<String>>,
        grading_type: Option<String>,
        discourse_id: Option<String>,
        topic_id: Option<String>,
        category_id: Option<String>,
        last_sync: Option<DateTime<Utc>>,
        source_system: Option<String>,
    ) -> Self {
        let now = Utc::now();
        
        Assignment {
            id,
            title: title.unwrap_or_default(),
            description: description.unwrap_or_default(),
            created_at: created_at.unwrap_or(now),
            updated_at: updated_at.unwrap_or(now),
            canvas_id,
            course_id,
            due_at,
            lock_at,
            unlock_at,
            points_possible: points_possible.unwrap_or(0.0),
            submission_types: submission_types.unwrap_or_default(),
            grading_type: grading_type.unwrap_or_else(|| "points".to_string()),
            discourse_id,
            topic_id,
            category_id,
            last_sync,
            source_system: source_system.unwrap_or_else(|| "canvas".to_string()),
        }
    }

    /// Create an Assignment from Canvas data
    pub fn from_canvas(canvas_data: &serde_json::Value) -> Self {
        let id = canvas_data["id"].as_str().map(String::from);
        
        let title = canvas_data["name"].as_str()
            .or_else(|| canvas_data["title"].as_str())
            .map(String::from);
            
        let description = canvas_data["description"].as_str().map(String::from);
            
        let created_at = canvas_data["created_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let updated_at = canvas_data["updated_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let course_id = canvas_data["course_id"].as_str().map(String::from);
        
        let due_at = canvas_data["due_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let lock_at = canvas_data["lock_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let unlock_at = canvas_data["unlock_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let points_possible = canvas_data["points_possible"].as_f64();
        
        let submission_types = canvas_data["submission_types"].as_array()
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect());
                
        let grading_type = canvas_data["grading_type"].as_str().map(String::from);
        
        Self::new(
            None,
            title,
            description,
            created_at,
            updated_at,
            id,
            course_id,
            due_at,
            lock_at,
            unlock_at,
            points_possible,
            submission_types,
            grading_type,
            None,
            None,
            None,
            None,
            Some("canvas".to_string()),
        )
    }
    
    /// Create an Assignment from Discourse data
    pub fn from_discourse(discourse_data: &serde_json::Value) -> Self {
        // In a real implementation, we'd extract assignment data from Discourse custom fields
        // This is a simplified example
        let id = discourse_data["id"].as_str().map(String::from);
        
        let topic_id = discourse_data["topic_id"].as_str().map(String::from);
        let category_id = discourse_data["category_id"].as_str().map(String::from);
        
        // Extract assignment metadata from custom fields
        let custom_fields = discourse_data["custom_fields"].as_object();
        let assignment_data = custom_fields
            .and_then(|fields| fields.get("assignment"))
            .and_then(|a| a.as_object());
            
        let title = assignment_data
            .and_then(|a| a.get("title"))
            .and_then(|t| t.as_str())
            .or_else(|| discourse_data["title"].as_str())
            .map(String::from);
            
        let description = assignment_data
            .and_then(|a| a.get("description"))
            .and_then(|d| d.as_str())
            .map(String::from);
            
        let created_at = discourse_data["created_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let updated_at = discourse_data["updated_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        let canvas_id = assignment_data
            .and_then(|a| a.get("canvas_id"))
            .and_then(|c| c.as_str())
            .map(String::from);
            
        let course_id = assignment_data
            .and_then(|a| a.get("course_id"))
            .and_then(|c| c.as_str())
            .map(String::from);
            
        let due_at = assignment_data
            .and_then(|a| a.get("due_at"))
            .and_then(|d| d.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
            
        Self::new(
            None,
            title,
            description,
            created_at,
            updated_at,
            canvas_id,
            course_id,
            due_at,
            None,
            None,
            None,
            None,
            None,
            id,
            topic_id,
            category_id,
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
            "course_id": self.course_id,
            "due_at": self.due_at,
            "lock_at": self.lock_at,
            "unlock_at": self.unlock_at,
            "points_possible": self.points_possible,
            "submission_types": self.submission_types,
            "grading_type": self.grading_type,
            "created_at": self.created_at,
            "updated_at": self.updated_at
        })
    }
    
    /// Convert to Discourse format
    pub fn to_discourse(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "topic_id": self.topic_id,
            "category_id": self.category_id,
            "custom_fields": {
                "assignment": {
                    "title": self.title,
                    "description": self.description,
                    "canvas_id": self.canvas_id,
                    "course_id": self.course_id,
                    "due_at": self.due_at,
                    "points_possible": self.points_possible,
                    "grading_type": self.grading_type
                }
            }
        })
    }
}
