//! Synchronization Service
//!
//! Core service for synchronizing data between Canvas and Discourse

use crate::api::canvas_client::CanvasClient;
use crate::api::discourse_client::DiscourseClient;
use crate::models::sync_state::SyncState;
use crate::models::sync_transaction::{EntityType, SyncDirection, SyncStatus, SyncTransaction};

use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Service for synchronizing data between Canvas and Discourse
pub struct SyncService {
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
    sync_state: Arc<SyncState>,
    queue_url: String,
    active_transactions: Mutex<Vec<SyncTransaction>>,
}

impl SyncService {
    /// Create a new synchronization service
    pub async fn new(
        canvas_client: &Arc<CanvasClient>,
        discourse_client: &Arc<DiscourseClient>,
        sync_state: &Arc<SyncState>,
        queue_url: &str,
    ) -> Result<Self> {
        Ok(Self {
            canvas_client: Arc::clone(canvas_client),
            discourse_client: Arc::clone(discourse_client),
            sync_state: Arc::clone(sync_state),
            queue_url: queue_url.to_string(),
            active_transactions: Mutex::new(Vec::new()),
        })
    }
    
    /// Start the synchronization service
    pub async fn start(&self) -> Result<()> {
        info!("Starting LMS integration synchronization service");
        
        // TODO: Connect to message queue and start processing messages
        
        // For now, just perform a sample synchronization
        self.sync_courses().await?;
        
        Ok(())
    }
    
    /// Synchronize courses from Canvas to Discourse
    pub async fn sync_courses(&self) -> Result<()> {
        info!("Synchronizing courses from Canvas to Discourse");
        
        // Get courses from Canvas
        let courses = self.canvas_client.list_courses().await
            .context("Failed to list courses from Canvas")?;
            
        info!("Found {} courses in Canvas", courses.len());
        
        // Create or update categories in Discourse
        for course in courses {
            let transaction = SyncTransaction::new(
                EntityType::Course,
                &course.id,
                SyncDirection::CanvasToDiscourse,
            );
            
            // Check if we already have a mapping for this course
            if let Some(category_id) = self.sync_state.get_category_for_course(&course.id) {
                info!("Course {} already mapped to category {}", course.id, category_id);
                continue;
            }
            
            info!("Creating category for course: {}", course.name);
            
            // Create a new category in Discourse
            match self.discourse_client.create_category(
                &course.name,
                "0088CC", // Default blue color
                "FFFFFF", // White text
                &format!("Category for Canvas course: {}", course.course_code),
                None,
            ).await {
                Ok(category) => {
                    info!("Created category {} for course {}", category.id, course.id);
                    
                    // Store the mapping
                    self.sync_state.set_course_category_mapping(&course.id, &category.id.to_string())
                        .context("Failed to store course-category mapping")?;
                }
                Err(e) => {
                    error!("Failed to create category for course {}: {:?}", course.id, e);
                }
            }
        }
        
        info!("Course synchronization completed");
        Ok(())
    }
    
    /// Queue a new synchronization transaction
    pub async fn queue_sync_transaction(
        &self,
        entity_type: EntityType,
        entity_id: &str,
        direction: SyncDirection,
    ) -> Result<SyncTransaction> {
        let transaction = SyncTransaction::new(entity_type, entity_id, direction);
        
        // TODO: Send to message queue
        
        info!(
            "Queued sync transaction: {} {} {}",
            transaction.transaction_id, entity_type, direction
        );
        
        Ok(transaction)
    }
    
    /// Process a synchronization transaction
    pub async fn process_transaction(&self, transaction: &mut SyncTransaction) -> Result<()> {
        info!(
            "Processing transaction {}: {} {} {}",
            transaction.transaction_id, transaction.entity_type, transaction.source_entity_id, transaction.direction
        );
        
        transaction.update_status(SyncStatus::InProgress);
        
        match transaction.entity_type {
            EntityType::Course => {
                match transaction.direction {
                    SyncDirection::CanvasToDiscourse => {
                        // Get course from Canvas
                        let course = self.canvas_client.get_course(&transaction.source_entity_id)
                            .await
                            .context("Failed to get course from Canvas")?;
                            
                        // Create category in Discourse
                        let category = self.discourse_client.create_category(
                            &course.name,
                            "0088CC", // Default blue color
                            "FFFFFF", // White text
                            &format!("Category for Canvas course: {}", course.course_code),
                            None,
                        ).await
                        .context("Failed to create category in Discourse")?;
                        
                        // Store the mapping
                        self.sync_state.set_course_category_mapping(
                            &transaction.source_entity_id,
                            &category.id.to_string()
                        ).context("Failed to store course-category mapping")?;
                        
                        transaction.mark_completed(&category.id.to_string());
                    }
                    _ => {
                        transaction.mark_failed("Unsupported direction for course synchronization");
                        return Ok(());
                    }
                }
            }
            // Handle other entity types
            _ => {
                transaction.mark_failed(&format!("Unsupported entity type: {}", transaction.entity_type));
                return Ok(());
            }
        }
        
        Ok(())
    }
}
