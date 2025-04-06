use std::sync::Arc;
use crate::models::mapping::{CourseCategory, SyncDirection, SyncSummary};
use crate::error::Error;
use crate::api::{canvas::CanvasClient, discourse::DiscourseClient};
use crate::db::Database;

pub struct MappingService {
    db: Arc<Database>,
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
}

impl MappingService {
    pub fn new(
        db: Arc<Database>,
        canvas_client: Arc<CanvasClient>,
        discourse_client: Arc<DiscourseClient>,
    ) -> Self {
        Self {
            db,
            canvas_client,
            discourse_client,
        }
    }
    
    pub async fn create_mapping(
        &self,
        canvas_course_id: &str,
        discourse_category_id: &str,
        name: &str,
        sync_direction: Option<SyncDirection>,
    ) -> Result<CourseCategory, Error> {
        // Check if mapping already exists
        if self.db.mapping_exists(canvas_course_id, discourse_category_id).await? {
            return Err(Error::DuplicateMapping);
        }
        
        // Verify course exists in Canvas
        let _ = self.canvas_client.get_course(canvas_course_id).await?;
        
        // Verify category exists in Discourse
        let _ = self.discourse_client.get_category(discourse_category_id).await?;
        
        // Create new mapping
        let mut mapping = CourseCategory::new(canvas_course_id, discourse_category_id, name);
        
        // Set sync direction if provided
        if let Some(direction) = sync_direction {
            mapping.sync_direction = direction;
        }
        
        // Save to database
        self.db.save_course_category(&mapping).await?;
        
        Ok(mapping)
    }
    
    pub async fn get_mapping(&self, id: &str) -> Result<CourseCategory, Error> {
        self.db.get_course_category(id).await
    }
    
    pub async fn update_mapping(
        &self,
        id: &str,
        name: Option<&str>,
        sync_enabled: Option<bool>,
        sync_direction: Option<SyncDirection>,
    ) -> Result<CourseCategory, Error> {
        let mut mapping = self.db.get_course_category(id).await?;
        
        if let Some(name) = name {
            mapping.name = name.to_string();
        }
        
        if let Some(enabled) = sync_enabled {
            mapping.sync_enabled = enabled;
        }
        
        if let Some(direction) = sync_direction {
            mapping.sync_direction = direction;
        }
        
        self.db.save_course_category(&mapping).await?;
        
        Ok(mapping)
    }
    
    pub async fn sync_mapping(&self, id: &str) -> Result<SyncSummary, Error> {
        let mut mapping = self.db.get_course_category(id).await?;
        
        mapping.sync(
            &self.db,
            &self.canvas_client,
            &self.discourse_client,
        ).await
    }
    
    pub async fn sync_all_mappings(&self) -> Result<Vec<(String, Result<SyncSummary, Error>)>, Error> {
        let mappings = self.db.get_all_course_categories().await?;
        let mut results = Vec::new();
        
        for mut mapping in mappings {
            let result = mapping.sync(
                &self.db,
                &self.canvas_client,
                &self.discourse_client,
            ).await;
            
            results.push((mapping.id.clone(), result));
        }
        
        Ok(results)
    }
    
    pub async fn delete_mapping(&self, id: &str) -> Result<(), Error> {
        self.db.delete_course_category(id).await
    }
}