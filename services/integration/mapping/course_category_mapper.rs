use crate::api::canvas_api::CanvasClient;
use crate::api::discourse_api::DiscourseClient;
use crate::db::Database;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use thiserror::Error;
use log::{error, info};

/// Error types for the course category mapper
#[derive(Error, Debug)]
pub enum MapperError {
    #[error("Canvas API error: {0}")]
    CanvasApiError(String),
    
    #[error("Discourse API error: {0}")]
    DiscourseApiError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Course {0} not found in Canvas")]
    CourseNotFound(String),
    
    #[error("Category {0} not found in Discourse")]
    CategoryNotFound(String),
    
    #[error("Mapping error: {0}")]
    MappingError(String),
}

/// Permission levels for Discourse categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPermissions {
    pub everyone: i32,
    pub staff: i32,
}

/// Course-Category Mapper for Canvas-Discourse Integration
/// Handles mapping between Canvas courses and Discourse categories
pub struct CourseCategoryMapper {
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
    db: Arc<Database>,
}

impl CourseCategoryMapper {
    /// Create a new course-category mapper
    pub fn new(
        canvas_client: Arc<CanvasClient>,
        discourse_client: Arc<DiscourseClient>,
        db: Arc<Database>,
    ) -> Result<Self, MapperError> {
        if canvas_client.is_none() {
            return Err(MapperError::MappingError("Canvas API client is required".to_string()));
        }
        
        if discourse_client.is_none() {
            return Err(MapperError::MappingError("Discourse API client is required".to_string()));
        }
        
        Ok(CourseCategoryMapper {
            canvas_client,
            discourse_client,
            db,
        })
    }
    
    /// Get Discourse category for Canvas course
    pub async fn get_discourse_category(&self, course_id: &str) -> Result<Option<serde_json::Value>, MapperError> {
        let query = "SELECT discourse_category_id FROM course_category_mappings WHERE canvas_course_id = $1";
        
        // Query the database for existing mapping
        let result = self.db.query(query, &[&course_id])
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))?;
        
        // If no mapping found, return None
        if result.is_empty() {
            return Ok(None);
        }
        
        // Extract category ID from result
        let category_id = result[0]["discourse_category_id"].as_str()
            .ok_or_else(|| MapperError::DatabaseError("Invalid database result format".to_string()))?;
        
        // Get category details from Discourse
        self.discourse_client.get_category(category_id)
            .await
            .map_err(|e| MapperError::DiscourseApiError(e.to_string()))
            .map(Some)
    }
    
    /// Create Discourse category for Canvas course
    pub async fn create_discourse_category(&self, course_id: &str) -> Result<serde_json::Value, MapperError> {
        // Get course details from Canvas
        let course = self.canvas_client.get_course(course_id)
            .await
            .map_err(|e| MapperError::CanvasApiError(e.to_string()))?;
        
        if course.is_null() {
            return Err(MapperError::CourseNotFound(course_id.to_string()));
        }
        
        // Check if mapping already exists
        if let Some(existing) = self.get_discourse_category(course_id).await? {
            return Ok(existing);
        }
        
        // Extract course name
        let course_name = course["name"].as_str()
            .ok_or_else(|| MapperError::MappingError("Course name not found".to_string()))?;
        
        // Generate category data
        let color = self.generate_color_from_course(course_id);
        
        let category_data = serde_json::json!({
            "name": course_name,
            "color": color,
            "text_color": "FFFFFF",
            "description": format!("Discussion forum for {} (Canvas Course ID: {})", course_name, course_id),
            "permissions": {
                "everyone": 0, // no access
                "staff": 3     // full access (see, reply, create)
            }
        });
        
        // Create category in Discourse
        let category = self.discourse_client.create_category(&category_data)
            .await
            .map_err(|e| MapperError::DiscourseApiError(e.to_string()))?;
        
        // Extract category ID
        let category_id = category["id"].as_str()
            .ok_or_else(|| MapperError::MappingError("Category ID not found in response".to_string()))?;
        
        // Store mapping in database
        let query = "INSERT INTO course_category_mappings (canvas_course_id, discourse_category_id) VALUES ($1, $2)";
        self.db.query(query, &[&course_id, &category_id])
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))?;
        
        Ok(category)
    }
    
    /// Update Discourse category for Canvas course
    pub async fn update_discourse_category(&self, course_id: &str) -> Result<serde_json::Value, MapperError> {
        // Get course details from Canvas
        let course = self.canvas_client.get_course(course_id)
            .await
            .map_err(|e| MapperError::CanvasApiError(e.to_string()))?;
        
        if course.is_null() {
            return Err(MapperError::CourseNotFound(course_id.to_string()));
        }
        
        // Get existing mapping
        let existing = self.get_discourse_category(course_id).await?
            .ok_or_else(|| MapperError::CategoryNotFound(format!("No Discourse category found for course {}", course_id)))?;
        
        // Extract course and category IDs
        let course_name = course["name"].as_str()
            .ok_or_else(|| MapperError::MappingError("Course name not found".to_string()))?;
        
        let category_id = existing["id"].as_str()
            .ok_or_else(|| MapperError::MappingError("Category ID not found".to_string()))?;
        
        // Prepare update data
        let update_data = serde_json::json!({
            "name": course_name,
            "description": format!("Discussion forum for {} (Canvas Course ID: {})", course_name, course_id)
        });
        
        // Update category in Discourse
        let updated_category = self.discourse_client.update_category(category_id, &update_data)
            .await
            .map_err(|e| MapperError::DiscourseApiError(e.to_string()))?;
        
        Ok(updated_category)
    }
    
    /// Delete mapping between Canvas course and Discourse category
    pub async fn delete_mapping(&self, course_id: &str) -> Result<bool, MapperError> {
        // Check if mapping exists
        let existing = self.get_discourse_category(course_id).await?;
        
        if existing.is_none() {
            return Ok(false); // Nothing to delete
        }
        
        // Delete mapping from database
        let query = "DELETE FROM course_category_mappings WHERE canvas_course_id = $1";
        let result = self.db.query(query, &[&course_id])
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))?;
        
        Ok(!result.is_empty())
    }
    
    /// Get Canvas course for Discourse category
    pub async fn get_canvas_course(&self, category_id: &str) -> Result<Option<serde_json::Value>, MapperError> {
        let query = "SELECT canvas_course_id FROM course_category_mappings WHERE discourse_category_id = $1";
        
        // Query the database for existing mapping
        let result = self.db.query(query, &[&category_id])
            .await
            .map_err(|e| MapperError::DatabaseError(e.to_string()))?;
        
        // If no mapping found, return None
        if result.is_empty() {
            return Ok(None);
        }
        
        // Extract course ID from result
        let course_id = result[0]["canvas_course_id"].as_str()
            .ok_or_else(|| MapperError::DatabaseError("Invalid database result format".to_string()))?;
        
        // Get course details from Canvas
        self.canvas_client.get_course(course_id)
            .await
            .map_err(|e| MapperError::CanvasApiError(e.to_string()))
            .map(Some)
    }
    
    /// Generate a color code for a course
    fn generate_color_from_course(&self, course_id: &str) -> String {
        // Simple but deterministic way to generate a color from course ID
        // In a real implementation, this might be more sophisticated
        let id_num = course_id.parse::<u64>().unwrap_or(0);
        let hue = (id_num % 360) as u8;
        
        // Convert HSV to RGB (simplified - fixed saturation and value)
        // This is a simplified approach - in practice, you might want a better
        // color generation algorithm that ensures readability and aesthetics
        let h = hue as f32 / 60.0;
        let s = 0.8;
        let v = 0.9;
        
        let c = v * s;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r1, g1, b1) = match h as i32 {
            0..=1 => (c, x, 0.0),
            1..=2 => (x, c, 0.0),
            2..=3 => (0.0, c, x),
            3..=4 => (0.0, x, c),
            4..=5 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        
        let r = ((r1 + m) * 255.0) as u8;
        let g = ((g1 + m) * 255.0) as u8;
        let b = ((b1 + m) * 255.0) as u8;
        
        format!("{:02X}{:02X}{:02X}", r, g, b)
    }
}
