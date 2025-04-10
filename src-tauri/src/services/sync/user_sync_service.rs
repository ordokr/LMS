use crate::models::unified::User;
use crate::repositories::unified::UserRepository;
use crate::services::canvas::CanvasClient;
use crate::services::discourse::DiscourseClient;
use crate::core::errors::AppError;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};

pub struct UserSyncService {
    user_repository: Arc<UserRepository>,
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
    sync_lock: Arc<Mutex<()>>,
}

impl UserSyncService {
    pub fn new(
        user_repository: Arc<UserRepository>,
        canvas_client: Arc<CanvasClient>,
        discourse_client: Arc<DiscourseClient>,
    ) -> Self {
        Self {
            user_repository,
            canvas_client,
            discourse_client,
            sync_lock: Arc::new(Mutex::new(())),
        }
    }
    
    pub async fn sync_user(&self, user_id: &str) -> Result<User, AppError> {
        let _lock = self.sync_lock.lock().await;
        
        // Find user in our database
        let mut user = match self.user_repository.find_by_id(user_id).await? {
            Some(user) => user,
            None => return Err(AppError::NotFound("User not found".to_string())),
        };
        
        // Sync with Canvas if we have a Canvas ID
        if let Some(canvas_id) = &user.canvas_id {
            match self.sync_with_canvas(&user, canvas_id).await {
                Ok(updated_user) => user = updated_user,
                Err(e) => error!("Error syncing with Canvas: {}", e),
            }
        }
        
        // Sync with Discourse if we have a Discourse ID
        if let Some(discourse_id) = &user.discourse_id {
            match self.sync_with_discourse(&user, discourse_id).await {
                Ok(updated_user) => user = updated_user,
                Err(e) => error!("Error syncing with Discourse: {}", e),
            }
        }
        
        // Save updated user
        self.user_repository.update(&user).await?;
        
        Ok(user)
    }
    
    async fn sync_with_canvas(&self, user: &User, canvas_id: &str) -> Result<User, AppError> {
        info!("Syncing user {} with Canvas ID {}", user.id, canvas_id);
        
        // Fetch user data from Canvas
        let canvas_user = self.canvas_client.get_user(canvas_id).await?;
        
        // Convert to JSON for User::from_canvas_user
        let canvas_user_json = serde_json::to_value(canvas_user)?;
        
        // Create a new user from Canvas data
        let canvas_unified_user = User::from_canvas_user(&canvas_user_json);
        
        // Merge data, prioritizing some fields from our system and others from Canvas
        let mut updated_user = user.clone();
        
        // Update fields that we want to sync from Canvas
        updated_user.name = canvas_unified_user.name;
        updated_user.email = canvas_unified_user.email;
        updated_user.avatar = canvas_unified_user.avatar;
        
        // Keep track of sync metadata
        updated_user.metadata.insert(
            "last_canvas_sync".to_string(),
            serde_json::json!(chrono::Utc::now().to_rfc3339()),
        );
        
        Ok(updated_user)
    }
    
    async fn sync_with_discourse(&self, user: &User, discourse_id: &str) -> Result<User, AppError> {
        info!("Syncing user {} with Discourse ID {}", user.id, discourse_id);
        
        // Fetch user data from Discourse
        let discourse_user = self.discourse_client.get_user(discourse_id).await?;
        
        // Convert to JSON for User::from_discourse_user
        let discourse_user_json = serde_json::to_value(discourse_user)?;
        
        // Create a new user from Discourse data
        let discourse_unified_user = User::from_discourse_user(&discourse_user_json);
        
        // Merge data, prioritizing some fields from our system and others from Discourse
        let mut updated_user = user.clone();
        
        // Update fields that we want to sync from Discourse
        // For Discourse users, we might want different fields than from Canvas
        if updated_user.avatar.is_empty() {
            updated_user.avatar = discourse_unified_user.avatar;
        }
        
        // Merge roles without duplicates
        for role in &discourse_unified_user.roles {
            if !updated_user.roles.contains(role) {
                updated_user.roles.push(role.clone());
            }
        }
        
        // Keep track of sync metadata
        updated_user.metadata.insert(
            "last_discourse_sync".to_string(),
            serde_json::json!(chrono::Utc::now().to_rfc3339()),
        );
        
        Ok(updated_user)
    }
    
    pub async fn sync_all_users(&self) -> Result<Vec<User>, AppError> {
        let _lock = self.sync_lock.lock().await;
        
        info!("Starting sync of all users");
        
        let users = self.user_repository.find_all().await?;
        let mut updated_users = Vec::new();
        
        for user in users {
            match self.sync_user(&user.id).await {
                Ok(updated_user) => {
                    updated_users.push(updated_user);
                }
                Err(e) => {
                    error!("Error syncing user {}: {}", user.id, e);
                }
            }
        }
        
        info!("Completed sync of {} users", updated_users.len());
        
        Ok(updated_users)
    }
}