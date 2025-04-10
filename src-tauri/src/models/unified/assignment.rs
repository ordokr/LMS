use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub course_id: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub points_possible: Option<f64>,
    pub submission_types: Option<Vec<String>>,
    pub canvas_specific_fields: Option<CanvasSpecificFields>,
    pub discourse_specific_fields: Option<DiscourseSpecificFields>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSpecificFields {
    pub unlock_at: Option<DateTime<Utc>>,
    pub lock_at: Option<DateTime<Utc>>,
    pub position: Option<i64>,
    pub published: Option<bool>,
    // Add more Canvas-specific fields as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseSpecificFields {
    pub topic_id: Option<String>,
    pub category_id: Option<String>,
    // Add more Discourse-specific fields as needed
}

impl Assignment {
    pub fn new(
        id: Option<String>,
        title: Option<String>,
        description: Option<String>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            title: title.unwrap_or_default(),
            description: description.unwrap_or_default(),
            created_at: created_at.unwrap_or(now),
            updated_at: updated_at.unwrap_or(now),
            course_id: None,
            due_date: None,
            points_possible: None,
            submission_types: None,
            canvas_specific_fields: None,
            discourse_specific_fields: None,
        }
    }
    
    pub fn to_canvas_assignment(&self) -> serde_json::Value {
        let canvas_fields = self.canvas_specific_fields.clone().unwrap_or(CanvasSpecificFields {
            unlock_at: None,
            lock_at: None,
            position: None,
            published: None,
        });
        
        serde_json::json!({
            "id": self.id,
            "name": self.title,
            "description": self.description,
            "course_id": self.course_id,
            "due_at": self.due_date,
            "points_possible": self.points_possible,
            "submission_types": self.submission_types,
            "unlock_at": canvas_fields.unlock_at,
            "lock_at": canvas_fields.lock_at,
            "position": canvas_fields.position,
            "published": canvas_fields.published,
            "created_at": self.created_at,
            "updated_at": self.updated_at
        })
    }
    
    pub fn to_discourse_custom_fields(&self) -> HashMap<String, serde_json::Value> {
        let mut custom_fields = HashMap::new();
        
        custom_fields.insert("assignment_id".to_string(), serde_json::json!(self.id));
        custom_fields.insert("assignment_title".to_string(), serde_json::json!(self.title));
        custom_fields.insert("assignment_description".to_string(), serde_json::json!(self.description));
        custom_fields.insert("assignment_due_date".to_string(), serde_json::json!(self.due_date));
        custom_fields.insert("assignment_points".to_string(), serde_json::json!(self.points_possible));
        custom_fields.insert("assignment_course_id".to_string(), serde_json::json!(self.course_id));
        
        custom_fields
    }
    
    pub fn from_canvas_assignment(canvas_assignment: &serde_json::Value) -> Self {
        let id = canvas_assignment["id"].as_str().map(|s| s.to_string());
        let title = canvas_assignment["name"].as_str()
            .or_else(|| canvas_assignment["title"].as_str())
            .map(|s| s.to_string());
        let description = canvas_assignment["description"].as_str().map(|s| s.to_string());
        
        let created_at_str = canvas_assignment["created_at"].as_str();
        let updated_at_str = canvas_assignment["updated_at"].as_str();
        
        let created_at = created_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let updated_at = updated_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let course_id = canvas_assignment["course_id"].as_str().map(|s| s.to_string());
        
        let due_date_str = canvas_assignment["due_at"].as_str();
        let due_date = due_date_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let points_possible = canvas_assignment["points_possible"].as_f64();
        
        // Extract submission types if available
        let submission_types = canvas_assignment["submission_types"].as_array()
            .map(|array| {
                array.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            });
        
        // Extract Canvas-specific fields
        let unlock_at_str = canvas_assignment["unlock_at"].as_str();
        let unlock_at = unlock_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let lock_at_str = canvas_assignment["lock_at"].as_str();
        let lock_at = lock_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let position = canvas_assignment["position"].as_i64();
        let published = canvas_assignment["published"].as_bool();
        
        let canvas_fields = CanvasSpecificFields {
            unlock_at,
            lock_at,
            position,
            published,
        };
        
        let mut assignment = Self::new(
            id,
            title,
            description,
            created_at,
            updated_at,
        );
        
        assignment.course_id = course_id;
        assignment.due_date = due_date;
        assignment.points_possible = points_possible;
        assignment.submission_types = submission_types;
        assignment.canvas_specific_fields = Some(canvas_fields);
        
        assignment
    }
    
    pub fn from_discourse_topic(topic: &serde_json::Value) -> Self {
        let id = None; // Assignment ID typically comes from custom fields
        let title = topic["title"].as_str().map(|s| s.to_string());
        let description = topic["raw"].as_str().map(|s| s.to_string());
        
        let created_at_str = topic["created_at"].as_str();
        let updated_at_str = topic["updated_at"].as_str();
        
        let created_at = created_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let updated_at = updated_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        // Extract custom fields if available
        let custom_fields = topic["custom_fields"].as_object();
        
        let assignment_id = custom_fields
            .and_then(|fields| fields.get("assignment_id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let course_id = custom_fields
            .and_then(|fields| fields.get("assignment_course_id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let due_date_str = custom_fields
            .and_then(|fields| fields.get("assignment_due_date"))
            .and_then(|v| v.as_str());
        
        let due_date = due_date_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let points_possible = custom_fields
            .and_then(|fields| fields.get("assignment_points"))
            .and_then(|v| v.as_f64());
        
        let topic_id = topic["id"].as_str().map(|s| s.to_string());
        let category_id = topic["category_id"].as_str().map(|s| s.to_string());
        
        let discourse_fields = DiscourseSpecificFields {
            topic_id,
            category_id,
        };
        
        let mut assignment = Self::new(
            assignment_id.or(id),
            title,
            description,
            created_at,
            updated_at,
        );
        
        assignment.course_id = course_id;
        assignment.due_date = due_date;
        assignment.points_possible = points_possible;
        assignment.discourse_specific_fields = Some(discourse_fields);
        
        assignment
    }
}