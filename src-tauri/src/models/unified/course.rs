use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use slugify::slugify;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub canvas_specific_fields: Option<CanvasSpecificFields>,
    pub discourse_specific_fields: Option<DiscourseSpecificFields>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSpecificFields {
    // Add Canvas-specific fields here
    pub course_code: Option<String>,
    pub syllabus_body: Option<String>,
    // Add more fields as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseSpecificFields {
    pub slug: String,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub parent_category_id: Option<i64>,
    // Add more fields as needed
}

impl Course {
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
            canvas_specific_fields: None,
            discourse_specific_fields: None,
        }
    }
    
    fn generate_slug(&self) -> String {
        slugify!(&self.title)
    }
    
    pub fn to_canvas_course(&self) -> serde_json::Value {
        let canvas_fields = self.canvas_specific_fields.clone().unwrap_or(CanvasSpecificFields {
            course_code: None,
            syllabus_body: None,
        });
        
        serde_json::json!({
            "id": self.id,
            "name": self.title,
            "course_code": canvas_fields.course_code,
            "syllabus_body": canvas_fields.syllabus_body.unwrap_or_else(|| self.description.clone()),
            "description": self.description,
            "created_at": self.created_at,
            "updated_at": self.updated_at
            // Add more fields as needed
        })
    }
    
    pub fn to_discourse_category(&self) -> serde_json::Value {
        let discourse_fields = self.discourse_specific_fields.clone()
            .unwrap_or_else(|| DiscourseSpecificFields {
                slug: self.generate_slug(),
                color: None,
                text_color: None,
                parent_category_id: None,
            });
        
        serde_json::json!({
            "id": self.id,
            "name": self.title,
            "description": self.description,
            "slug": discourse_fields.slug,
            "color": discourse_fields.color.unwrap_or_else(|| "#38B2AC".to_string()),
            "text_color": discourse_fields.text_color.unwrap_or_else(|| "#FFFFFF".to_string()),
            "parent_category_id": discourse_fields.parent_category_id,
            "created_at": self.created_at,
            "updated_at": self.updated_at
        })
    }
    
    pub fn from_canvas_course(canvas_course: &serde_json::Value) -> Self {
        let id = canvas_course["id"].as_str().map(|s| s.to_string());
        let title = canvas_course["name"].as_str().map(|s| s.to_string());
        let description = canvas_course["description"].as_str()
            .or_else(|| canvas_course["syllabus_body"].as_str())
            .map(|s| s.to_string());
        
        let created_at_str = canvas_course["created_at"].as_str();
        let updated_at_str = canvas_course["updated_at"].as_str();
        
        let created_at = created_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let updated_at = updated_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let course_code = canvas_course["course_code"].as_str()
            .map(|s| s.to_string());
        
        let syllabus_body = canvas_course["syllabus_body"].as_str()
            .map(|s| s.to_string());
        
        let canvas_fields = CanvasSpecificFields {
            course_code,
            syllabus_body,
        };
        
        let mut course = Self::new(
            id,
            title,
            description,
            created_at,
            updated_at,
        );
        
        course.canvas_specific_fields = Some(canvas_fields);
        course
    }
    
    pub fn from_discourse_category(category: &serde_json::Value) -> Self {
        let id = category["id"].as_str().map(|s| s.to_string());
        let title = category["name"].as_str().map(|s| s.to_string());
        let description = category["description"].as_str()
            .or_else(|| category["description_text"].as_str())
            .map(|s| s.to_string());
        
        let created_at_str = category["created_at"].as_str();
        let updated_at_str = category["updated_at"].as_str();
        
        let created_at = created_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let updated_at = updated_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        let slug = category["slug"].as_str().map(|s| s.to_string());
        let color = category["color"].as_str().map(|s| s.to_string());
        let text_color = category["text_color"].as_str().map(|s| s.to_string());
        let parent_category_id = category["parent_category_id"].as_i64();
        
        let discourse_fields = DiscourseSpecificFields {
            slug: slug.unwrap_or_else(|| {
                let title_str = title.clone().unwrap_or_default();
                slugify!(&title_str)
            }),
            color,
            text_color,
            parent_category_id,
        };
        
        let mut course = Self::new(
            id,
            title,
            description,
            created_at,
            updated_at,
        );
        
        course.discourse_specific_fields = Some(discourse_fields);
        course
    }
}