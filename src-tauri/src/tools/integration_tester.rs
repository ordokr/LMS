use crate::db::DB;
use crate::services::integration::canvas_integration::{CanvasIntegration, CanvasIntegrationService};
use crate::services::integration::discourse_integration::{DiscourseIntegration, DiscourseIntegrationService};
use crate::services::integration::sync_service::IntegrationSyncService;
use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::user::user::User;
use crate::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct IntegrationTester {
    db: DB,
    canvas_service: Arc<CanvasIntegrationService>,
    discourse_service: Arc<DiscourseIntegrationService>,
    sync_service: Arc<IntegrationSyncService<CanvasIntegrationService, DiscourseIntegrationService>>,
}

impl IntegrationTester {
    pub fn new(
        db: DB,
        canvas_api_url: String,
        canvas_api_token: String,
        discourse_api_url: String,
        discourse_api_key: String,
        discourse_api_username: String,
    ) -> Self {
        let canvas_service = Arc::new(CanvasIntegrationService::new(
            db.clone(),
            canvas_api_url,
            canvas_api_token,
        ));
        
        let discourse_service = Arc::new(DiscourseIntegrationService::new(
            db.clone(),
            discourse_api_url,
            discourse_api_key,
            discourse_api_username,
        ));
        
        let sync_service = Arc::new(IntegrationSyncService::new(
            db.clone(),
            canvas_service.as_ref().clone(),
            discourse_service.as_ref().clone(),
        ));
        
        IntegrationTester {
            db,
            canvas_service,
            discourse_service,
            sync_service,
        }
    }
    
    pub async fn test_canvas_connectivity(&self) -> Result<bool, Error> {
        println!("Testing Canvas connectivity...");
        
        // Try to fetch a known user
        match self.canvas_service.fetch_canvas_user("1").await {
            Ok(_) => {
                println!("✅ Canvas connectivity test passed");
                Ok(true)
            },
            Err(e) => {
                println!("❌ Canvas connectivity test failed: {}", e);
                Ok(false)
            }
        }
    }
    
    pub async fn test_discourse_connectivity(&self) -> Result<bool, Error> {
        println!("Testing Discourse connectivity...");
        
        // Try to fetch a known category
        match self.discourse_service.fetch_discourse_category(1).await {
            Ok(_) => {
                println!("✅ Discourse connectivity test passed");
                Ok(true)
            },
            Err(e) => {
                println!("❌ Discourse connectivity test failed: {}", e);
                Ok(false)
            }
        }
    }
    
    pub async fn create_test_topic(&self) -> Result<Topic, Error> {
        println!("Creating test topic...");
        
        // Create a test user if we don't have one
        let user = match User::find_by_email(&self.db, "test@example.com").await {
            Ok(user) => user,
            Err(_) => {
                let mut user = User::new(
                    "Test User".to_string(),
                    "test@example.com".to_string(),
                    "testuser".to_string(),
                );
                
                user.create(&self.db).await?;
                user
            }
        };
        
        // Create a test topic
        let mut topic = Topic::new(
            format!("Test Topic {}", Utc::now().timestamp()),
            user.id,
            "This is a test topic created by the integration tester.".to_string(),
        );
        
        topic.create(&self.db).await?;
        
        // Create a test post
        let mut post = Post::new(
            topic.id,
            user.id,
            "This is a test reply to the topic.".to_string(),
        );
        
        post.create(&self.db).await?;
        
        println!("✅ Created test topic with ID: {}", topic.id);
        
        Ok(topic)
    }
    
    pub async fn test_canvas_to_discourse_sync(&self) -> Result<(), Error> {
        println!("Testing Canvas to Discourse sync...");
        
        // Create a test topic
        let topic = self.create_test_topic().await?;
        
        // Mark it as Canvas-synced
        let mut canvas_topic = topic.clone();
        canvas_topic.canvas_topic_id = Some(format!("test_canvas_id_{}", Utc::now().timestamp()));
        canvas_topic.sync_status = crate::models::forum::topic::SyncStatus::SyncedWithCanvas;
        canvas_topic.update(&self.db).await?;
        
        // Sync to Discourse
        match self.sync_service
            .sync_topic_canvas_to_discourse(&canvas_topic.canvas_topic_id.unwrap()).await
        {
            Ok(mapping) => {
                println!("✅ Canvas to Discourse sync successful");
                println!("   Canvas ID: {}", mapping.canvas_topic_id);
                println!("   Discourse ID: {}", mapping.discourse_topic_id);
                
                // Verify the topic was updated with Discourse ID
                let updated_topic = Topic::find(&self.db, topic.id).await?;
                if updated_topic.discourse_topic_id.is_some() {
                    println!("✅ Topic correctly updated with Discourse ID");
                } else {
                    println!("❌ Topic missing Discourse ID");
                }
                
                Ok(())
            },
            Err(e) => {
                println!("❌ Canvas to Discourse sync failed: {}", e);
                Err(e)
            }
        }
    }
    
    pub async fn test_discourse_to_canvas_sync(&self) -> Result<(), Error> {
        println!("Testing Discourse to Canvas sync...");
        
        // Create a test topic
        let topic = self.create_test_topic().await?;
        
        // Mark it as Discourse-synced
        let mut discourse_topic = topic.clone();
        discourse_topic.discourse_topic_id = Some(1000 + rand::random::<i64>() % 9000);
        discourse_topic.sync_status = crate::models::forum::topic::SyncStatus::SyncedWithDiscourse;
        discourse_topic.update(&self.db).await?;
        
        // Sync to Canvas
        match self.sync_service
            .sync_topic_discourse_to_canvas(discourse_topic.discourse_topic_id.unwrap()).await
        {
            Ok(mapping) => {
                println!("✅ Discourse to Canvas sync successful");
                println!("   Canvas ID: {}", mapping.canvas_topic_id);
                println!("   Discourse ID: {}", mapping.discourse_topic_id);
                
                // Verify the topic was updated with Canvas ID
                let updated_topic = Topic::find(&self.db, topic.id).await?;
                if updated_topic.canvas_topic_id.is_some() {
                    println!("✅ Topic correctly updated with Canvas ID");
                } else {
                    println!("❌ Topic missing Canvas ID");
                }
                
                Ok(())
            },
            Err(e) => {
                println!("❌ Discourse to Canvas sync failed: {}", e);
                Err(e)
            }
        }
    }
    
    pub async fn test_sync_all_pending(&self) -> Result<(), Error> {
        println!("Testing sync_all_pending...");
        
        // Create test topics that need sync
        let user = match User::find_by_email(&self.db, "test@example.com").await {
            Ok(user) => user,
            Err(_) => {
                let mut user = User::new(
                    "Test User".to_string(),
                    "test@example.com".to_string(),
                    "testuser".to_string(),
                );
                
                user.create(&self.db).await?;
                user
            }
        };
        
        // Create a canvas topic that needs to sync to discourse
        let mut canvas_topic = Topic::new(
            format!("Canvas Topic {}", Utc::now().timestamp()),
            user.id,
            "This is a test canvas topic that needs to sync.".to_string(),
        );
        canvas_topic.canvas_topic_id = Some(format!("pending_canvas_id_{}", Utc::now().timestamp()));
        canvas_topic.sync_status = crate::models::forum::topic::SyncStatus::PendingSync;
        canvas_topic.create(&self.db).await?;
        
        // Create a discourse topic that needs to sync to canvas
        let mut discourse_topic = Topic::new(
            format!("Discourse Topic {}", Utc::now().timestamp()),
            user.id,
            "This is a test discourse topic that needs to sync.".to_string(),
        );
        discourse_topic.discourse_topic_id = Some(1000 + rand::random::<i64>() % 9000);
        discourse_topic.sync_status = crate::models::forum::topic::SyncStatus::PendingSync;
        discourse_topic.create(&self.db).await?;
        
        // Sync all pending
        match self.sync_service.sync_all_pending().await {
            Ok(_) => {
                println!("✅ Sync all pending completed successfully");
                
                // Verify topics were synced
                let updated_canvas_topic = Topic::find(&self.db, canvas_topic.id).await?;
                let updated_discourse_topic = Topic::find(&self.db, discourse_topic.id).await?;
                
                if updated_canvas_topic.discourse_topic_id.is_some() {
                    println!("✅ Canvas topic correctly synced to Discourse");
                } else {
                    println!("❌ Canvas topic not synced to Discourse");
                }
                
                if updated_discourse_topic.canvas_topic_id.is_some() {
                    println!("✅ Discourse topic correctly synced to Canvas");
                } else {
                    println!("❌ Discourse topic not synced to Canvas");
                }
                
                Ok(())
            },
            Err(e) => {
                println!("❌ Sync all pending failed: {}", e);
                Err(e)
            }
        }
    }
    
    pub async fn run_all_tests(&self) -> Result<(), Error> {
        println!("\n===== Running Integration Tests =====\n");
        
        let mut all_passed = true;
        
        // Test connectivity
        match self.test_canvas_connectivity().await {
            Ok(true) => {},
            _ => all_passed = false,
        }
        
        match self.test_discourse_connectivity().await {
            Ok(true) => {},
            _ => all_passed = false,
        }
        
        // Test syncs
        if let Err(_) = self.test_canvas_to_discourse_sync().await {
            all_passed = false;
        }
        
        if let Err(_) = self.test_discourse_to_canvas_sync().await {
            all_passed = false;
        }
        
        if let Err(_) = self.test_sync_all_pending().await {
            all_passed = false;
        }
        
        println!("\n===== Test Summary =====");
        if all_passed {
            println!("✅ All integration tests passed!");
        } else {
            println!("❌ Some tests failed. See above for details.");
        }
        
        Ok(())
    }
}