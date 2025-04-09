use crate::db::DB;
use crate::services::integration::canvas_integration::CanvasIntegrationService;
use crate::services::integration::discourse_integration::DiscourseIntegrationService;
use crate::services::integration::sync_service::IntegrationSyncService;

pub async fn run_integration_example() {
    // Set up database connection
    let db = DB::connect("sqlite:lms.db").await.unwrap();
    
    // Create Canvas integration service
    let canvas_service = CanvasIntegrationService::new(
        db.clone(),
        "https://canvas.example.com".to_string(),
        "canvas_api_token".to_string(),
    );
    
    // Create Discourse integration service
    let discourse_service = DiscourseIntegrationService::new(
        db.clone(),
        "https://discourse.example.com".to_string(),
        "discourse_api_key".to_string(),
        "system".to_string(),
    );
    
    // Create the sync service
    let sync_service = IntegrationSyncService::new(
        db.clone(),
        canvas_service,
        discourse_service,
    );
    
    // Sync a specific topic from Canvas to Discourse
    let canvas_topic_id = "12345";
    match sync_service.sync_topic_canvas_to_discourse(canvas_topic_id).await {
        Ok(mapping) => {
            println!("Successfully synced Canvas topic {} to Discourse topic {}", 
                     mapping.canvas_topic_id, mapping.discourse_topic_id);
        },
        Err(e) => {
            eprintln!("Failed to sync topic: {}", e);
        }
    }
    
    // Sync a specific topic from Discourse to Canvas
    let discourse_topic_id = 67890;
    match sync_service.sync_topic_discourse_to_canvas(discourse_topic_id).await {
        Ok(mapping) => {
            println!("Successfully synced Discourse topic {} to Canvas topic {}", 
                     mapping.discourse_topic_id, mapping.canvas_topic_id);
        },
        Err(e) => {
            eprintln!("Failed to sync topic: {}", e);
        }
    }
    
    // Sync all pending topics
    match sync_service.sync_all_pending().await {
        Ok(_) => {
            println!("Successfully synced all pending topics");
        },
        Err(e) => {
            eprintln!("Failed to sync pending topics: {}", e);
        }
    }
}