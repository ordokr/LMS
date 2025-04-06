use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
use crate::db::Database;
use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCategory {
    pub id: String,
    pub canvas_course_id: String,
    pub discourse_category_id: String,
    pub name: String,
    pub last_sync: DateTime<Utc>,
    pub sync_enabled: bool,
    pub sync_direction: SyncDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncDirection {
    CanvasToDiscourse,
    DiscourseToCanvas,
    Bidirectional,
}

impl CourseCategory {
    pub fn new(canvas_course_id: &str, discourse_category_id: &str, name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            canvas_course_id: canvas_course_id.to_string(),
            discourse_category_id: discourse_category_id.to_string(),
            name: name.to_string(),
            last_sync: Utc::now(),
            sync_enabled: true,
            sync_direction: SyncDirection::Bidirectional,
        }
    }
    
    pub async fn sync(
        &mut self, 
        db: &Database,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
    ) -> Result<SyncSummary, Error> {
        if (!self.sync_enabled) {
            return Ok(SyncSummary::new("Sync disabled for this mapping"));
        }

        let mut summary = SyncSummary::new("Starting sync");
        
        // Check sync direction and perform appropriate operations
        match self.sync_direction {
            SyncDirection::CanvasToDiscourse => {
                summary.add_operation(self.sync_canvas_to_discourse(canvas_client, discourse_client).await?);
            },
            SyncDirection::DiscourseToCanvas => {
                summary.add_operation(self.sync_discourse_to_canvas(canvas_client, discourse_client).await?);
            },
            SyncDirection::Bidirectional => {
                summary.add_operation(self.sync_canvas_to_discourse(canvas_client, discourse_client).await?);
                summary.add_operation(self.sync_discourse_to_canvas(canvas_client, discourse_client).await?);
            },
        }
        
        // Update last sync timestamp
        self.last_sync = Utc::now();
        
        // Save updated mapping to database
        db.save_course_category(self).await?;
        
        summary.complete();
        Ok(summary)
    }
    
    async fn sync_canvas_to_discourse(
        &self,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
    ) -> Result<String, Error> {
        // Fetch course data from Canvas
        let canvas_course = canvas_client.get_course(&self.canvas_course_id).await?;
        
        // Update Discourse category with Canvas data
        discourse_client.update_category(
            &self.discourse_category_id,
            &canvas_course.name,
            &canvas_course.description,
        ).await?;
        
        Ok(format!("Synced '{}' from Canvas to Discourse", canvas_course.name))
    }
    
    async fn sync_discourse_to_canvas(
        &self,
        canvas_client: &CanvasClient,
        discourse_client: &DiscourseClient,
    ) -> Result<String, Error> {
        // Fetch category data from Discourse
        let discourse_category = discourse_client.get_category(&self.discourse_category_id).await?;
        
        // Update Canvas course with Discourse data
        canvas_client.update_course(
            &self.canvas_course_id,
            &discourse_category.name,
            &discourse_category.description,
        ).await?;
        
        Ok(format!("Synced '{}' from Discourse to Canvas", discourse_category.name))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SyncSummary {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub operations: Vec<String>,
    pub status: String,
}

impl SyncSummary {
    pub fn new(initial_status: &str) -> Self {
        Self {
            start_time: Utc::now(),
            end_time: None,
            operations: Vec::new(),
            status: initial_status.to_string(),
        }
    }
    
    pub fn add_operation(&mut self, operation: String) {
        self.operations.push(operation);
    }
    
    pub fn complete(&mut self) {
        self.end_time = Some(Utc::now());
        self.status = "Completed".to_string();
    }
}

// Tests for Course-Category mapping
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    
    // Mock APIs for testing
    mock! {
        CanvasClient {}
        
        async fn get_course(&self, course_id: &str) -> Result<CourseData, Error>;
        async fn update_course(&self, course_id: &str, name: &str, description: &str) -> Result<(), Error>;
    }
    
    mock! {
        DiscourseClient {}
        
        async fn get_category(&self, category_id: &str) -> Result<CategoryData, Error>;
        async fn update_category(&self, category_id: &str, name: &str, description: &str) -> Result<(), Error>;
    }
    
    mock! {
        Database {}
        
        async fn save_course_category(&self, mapping: &CourseCategory) -> Result<(), Error>;
    }
    
    #[tokio::test]
    async fn test_sync_from_canvas_to_discourse() {
        // Create test data
        let mut mapping = CourseCategory::new(
            "course123",
            "category456",
            "Test Course"
        );
        
        // Set last_sync to past time
        mapping.last_sync = Utc::now() - chrono::Duration::hours(1);
        
        // Create mock APIs
        let mut mock_canvas = MockCanvasClient::new();
        let mut mock_discourse = MockDiscourseClient::new();
        let mut mock_db = MockDatabase::new();
        
        // Set up Canvas mock to return updated course
        mock_canvas
            .expect_get_course()
            .with(eq("course123"))
            .returning(|_| {
                Ok(CourseData {
                    id: "course123".to_string(),
                    name: "Updated Course Name".to_string(),
                    description: "New description".to_string(),
                    updated_at: Utc::now(),
                })
            });
        
        // Discourse API should be updated with Canvas data
        mock_discourse
            .expect_get_category()
            .with(eq("category456"))
            .returning(|_| {
                Ok(CategoryData {
                    id: "category456".to_string(),
                    name: "Old Category Name".to_string(),
                    description: "Old description".to_string(),
                    updated_at: Utc::now() - chrono::Duration::days(1),
                })
            });
            
        mock_discourse
            .expect_update_category()
            .with(eq("category456"), eq("Updated Course Name"), eq("New description"))
            .returning(|_, _, _| Ok(()));
        
        // Database should save the updated mapping
        mock_db
            .expect_save_course_category()
            .returning(|_| Ok(()));
        
        // Execute sync
        let result = mapping.sync(&mock_db, &mock_canvas, &mock_discourse).await;
        
        // Verify sync succeeded
        assert!(result.is_ok());
        
        // Verify last_sync was updated
        assert!(mapping.last_sync > Utc::now() - chrono::Duration::seconds(10));
    }
}