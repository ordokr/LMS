use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    db::{
        topic_repository::TopicRepository,
        post_repository::PostRepository,
    },
    models::post::Post,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OfflineAction {
    CreatePost {
        local_id: Uuid,
        topic_id: Uuid,
        content: String,
        parent_id: Option<Uuid>,
        created_at: DateTime<Utc>,
    },
    UpdatePost {
        post_id: Uuid,
        content: String,
        updated_at: DateTime<Utc>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SyncResult {
    Success {
        action: OfflineAction,
        server_id: Option<Uuid>,
    },
    Failure {
        action: OfflineAction,
        error: String,
    },
}

pub struct OfflineSyncService {
    pool: PgPool,
    topic_repo: TopicRepository,
    post_repo: PostRepository,
    offline_actions: Arc<RwLock<HashMap<String, Vec<OfflineAction>>>>,
    id_mappings: Arc<RwLock<HashMap<Uuid, Uuid>>>, // local_id -> server_id
}

impl OfflineSyncService {
    pub fn new(pool: PgPool, topic_repo: TopicRepository, post_repo: PostRepository) -> Self {
        Self {
            pool,
            topic_repo,
            post_repo,
            offline_actions: Arc::new(RwLock::new(HashMap::new())),
            id_mappings: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Adds an offline action for a user
    pub async fn add_offline_action(&self, user_id: &str, action: OfflineAction) -> Result<(), String> {
        let mut actions = self.offline_actions.write().await;
        actions
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(action);
        
        Ok(())
    }
    
    /// Creates a post while offline
    pub async fn create_offline_post(
        &self,
        user_id: &str,
        topic_id: Uuid,
        content: String,
        parent_id: Option<Uuid>,
    ) -> Result<Uuid, String> {
        let local_id = Uuid::new_v4();
        let now = Utc::now();
        
        let action = OfflineAction::CreatePost {
            local_id,
            topic_id,
            content,
            parent_id,
            created_at: now,
        };
        
        self.add_offline_action(user_id, action).await?;
        
        Ok(local_id)
    }
    
    /// Updates a post while offline
    pub async fn update_offline_post(
        &self,
        user_id: &str,
        post_id: Uuid,
        content: String,
    ) -> Result<(), String> {
        let now = Utc::now();
        
        let action = OfflineAction::UpdatePost {
            post_id,
            content,
            updated_at: now,
        };
        
        self.add_offline_action(user_id, action).await?;
        
        Ok(())
    }
    
    /// Syncs all offline actions for a user
    pub async fn sync_user_actions(&self, user_id: &str, user_uuid: Uuid) -> Result<Vec<SyncResult>, String> {
        let mut actions = {
            let mut actions_map = self.offline_actions.write().await;
            actions_map.remove(user_id).unwrap_or_default()
        };
        
        if actions.is_empty() {
            return Ok(vec![]);
        }
        
        let mut results = Vec::new();
        
        // Process all actions in a transaction
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        
        for action in actions.drain(..) {
            let result = match action.clone() {
                OfflineAction::CreatePost { local_id, topic_id, content, parent_id, .. } => {
                    match self.post_repo.create_post(
                        &Post {
                            id: Uuid::new_v4(),  // Generate a new server ID
                            topic_id,
                            author_id: user_uuid,
                            content,
                            parent_id,
                            created_at: Utc::now(),
                            updated_at: Utc::now(),
                        },
                        &mut tx,
                    ).await {
                        Ok(post) => {
                            // Store the mapping from local ID to server ID
                            let mut mappings = self.id_mappings.write().await;
                            mappings.insert(local_id, post.id);
                            
                            SyncResult::Success {
                                action: action.clone(),
                                server_id: Some(post.id),
                            }
                        },
                        Err(e) => SyncResult::Failure {
                            action: action.clone(),
                            error: format!("Failed to create post: {}", e),
                        },
                    }
                },
                OfflineAction::UpdatePost { post_id, content, updated_at } => {
                    // Check if post exists
                    match self.post_repo.find_post_by_id(&post_id, &mut tx).await {
                        Ok(Some(existing_post)) => {
                            // Verify the user is the author
                            if existing_post.author_id != user_uuid {
                                SyncResult::Failure {
                                    action: action.clone(),
                                    error: "Not authorized to update this post".to_string(),
                                }
                            } else {
                                // Update the post
                                let updated_post = Post {
                                    content,
                                    updated_at,
                                    ..existing_post
                                };
                                
                                match self.post_repo.update_post_with_tx(&updated_post, &mut tx).await {
                                    Ok(_) => SyncResult::Success {
                                        action: action.clone(),
                                        server_id: None,
                                    },
                                    Err(e) => SyncResult::Failure {
                                        action: action.clone(),
                                        error: format!("Failed to update post: {}", e),
                                    },
                                }
                            }
                        },
                        Ok(None) => {
                            // Check if this post was created offline and has a local->server mapping
                            let mappings = self.id_mappings.read().await;
                            if let Some(&server_id) = mappings.get(&post_id) {
                                // Update using the server ID instead
                                match self.post_repo.find_post_by_id(&server_id, &mut tx).await {
                                    Ok(Some(existing_post)) => {
                                        let updated_post = Post {
                                            content,
                                            updated_at,
                                            ..existing_post
                                        };
                                        
                                        match self.post_repo.update_post_with_tx(&updated_post, &mut tx).await {
                                            Ok(_) => SyncResult::Success {
                                                action: action.clone(),
                                                server_id: None,
                                            },
                                            Err(e) => SyncResult::Failure {
                                                action: action.clone(),
                                                error: format!("Failed to update post: {}", e),
                                            },
                                        }
                                    },
                                    _ => SyncResult::Failure {
                                        action: action.clone(),
                                        error: "Post not found with mapped server ID".to_string(),
                                    },
                                }
                            } else {
                                SyncResult::Failure {
                                    action: action.clone(),
                                    error: "Post not found".to_string(),
                                }
                            }
                        },
                        Err(e) => SyncResult::Failure {
                            action: action.clone(),
                            error: format!("Database error: {}", e),
                        },
                    }
                }
            };
            
            results.push(result);
        }
        
        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
        Ok(results)
    }
    
    /// Gets pending offline actions for a user
    pub async fn get_pending_actions(&self, user_id: &str) -> Vec<OfflineAction> {
        let actions = self.offline_actions.read().await;
        actions.get(user_id).cloned().unwrap_or_default()
    }
    
    /// Clears all offline actions for a user
    pub async fn clear_offline_actions(&self, user_id: &str) -> Result<(), String> {
        let mut actions = self.offline_actions.write().await;
        actions.remove(user_id);
        
        Ok(())
    }
    
    /// Gets server ID for a local ID if it exists
    pub async fn get_server_id(&self, local_id: &Uuid) -> Option<Uuid> {
        let mappings = self.id_mappings.read().await;
        mappings.get(local_id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::forum::post::Post;
    use mockall::predicate::*;
    use mockall::mock;
    
    mock! {
        PostRepository {}
        impl PostRepository {
            pub async fn create_post<'a, E>(&self, post: &Post, executor: E) -> Result<Post, sqlx::Error>
            where
                E: sqlx::Executor<'a, Database = sqlx::Postgres>;
                
            pub async fn find_post_by_id<'a, E>(&self, id: &Uuid, executor: E) -> Result<Option<Post>, sqlx::Error>
            where
                E: sqlx::Executor<'a, Database = sqlx::Postgres>;
                
            pub async fn update_post<'a, E>(&self, post: &Post, executor: E) -> Result<(), sqlx::Error>
            where
                E: sqlx::Executor<'a, Database = sqlx::Postgres>;
        }
    }
    
    #[tokio::test]
    async fn test_create_offline_post() {
        let user_id = "user123";
        let topic_id = Uuid::new_v4();
        let content = "Offline test post".to_string();
        
        let service = OfflineSyncService::new(
            PgPool::connect("postgres://localhost").await.unwrap(),
            TopicRepository::new(PgPool::connect("postgres://localhost").await.unwrap()),
            PostRepository::new(PgPool::connect("postgres://localhost").await.unwrap()),
        );
        
        let local_id = service.create_offline_post(user_id, topic_id, content.clone(), None).await.unwrap();
        
        let actions = service.get_pending_actions(user_id).await;
        assert_eq!(actions.len(), 1);
        
        match &actions[0] {
            OfflineAction::CreatePost { local_id: stored_id, topic_id: stored_topic_id, content: stored_content, .. } => {
                assert_eq!(*stored_id, local_id);
                assert_eq!(*stored_topic_id, topic_id);
                assert_eq!(*stored_content, content);
            },
            _ => panic!("Expected CreatePost action"),
        }
    }
}