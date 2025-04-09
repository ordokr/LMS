use crate::models::discussion::{Discussion, DiscussionCreate, DiscussionStatus};
use crate::db::discussion_repository::DiscussionRepository;
use tauri::State;
use std::sync::Arc;
use tracing::{info, warn, error, instrument};
use uuid::Uuid;

/// Gets discussions for a course
///
/// # Arguments
/// * `course_id` - ID of the course
///
/// # Returns
/// * `Vec<Discussion>` - List of discussions for the course
#[tauri::command]
#[instrument(skip(discussion_repo), err)]
pub async fn get_discussions(
    course_id: String,
    discussion_repo: State<'_, Arc<dyn DiscussionRepository + Send + Sync>>
) -> Result<Vec<Discussion>, String> {
    info!(event = "api_call", endpoint = "get_discussions", course_id = %course_id);
    
    match discussion_repo.get_discussions_by_course(&course_id).await {
        Ok(discussions) => {
            info!(
                event = "api_success", 
                endpoint = "get_discussions",
                course_id = %course_id,
                count = discussions.len()
            );
            Ok(discussions)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_discussions", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Gets a specific discussion by ID
///
/// # Arguments
/// * `discussion_id` - ID of the discussion to retrieve
///
/// # Returns
/// * `Discussion` - The requested discussion
#[tauri::command]
#[instrument(skip(discussion_repo), err)]
pub async fn get_discussion(
    discussion_id: String,
    discussion_repo: State<'_, Arc<dyn DiscussionRepository + Send + Sync>>
) -> Result<Discussion, String> {
    info!(event = "api_call", endpoint = "get_discussion", discussion_id = %discussion_id);
    
    match discussion_repo.get_discussion_by_id(&discussion_id).await {
        Ok(Some(discussion)) => {
            info!(event = "api_success", endpoint = "get_discussion", discussion_id = %discussion_id);
            Ok(discussion)
        },
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "get_discussion", discussion_id = %discussion_id);
            Err(format!("Discussion not found with ID: {}", discussion_id))
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "get_discussion", error = %e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Creates a new discussion
///
/// # Arguments
/// * `discussion_create` - Discussion creation data
///
/// # Returns
/// * `Discussion` - The created discussion
#[tauri::command]
#[instrument(skip(discussion_repo), err)]
pub async fn create_discussion(
    discussion_create: DiscussionCreate,
    discussion_repo: State<'_, Arc<dyn DiscussionRepository + Send + Sync>>
) -> Result<Discussion, String> {
    info!(
        event = "api_call", 
        endpoint = "create_discussion",
        course_id = %discussion_create.course_id,
        title = %discussion_create.title
    );
    
    // Generate ID and create full discussion object
    let discussion = Discussion {
        id: Uuid::new_v4().to_string(),
        course_id: discussion_create.course_id,
        title: discussion_create.title,
        content: discussion_create.content,
        topic_id: discussion_create.topic_id,
        status: discussion_create.status.unwrap_or(DiscussionStatus::Open),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    match discussion_repo.create_discussion(discussion).await {
        Ok(created) => {
            info!(event = "api_success", endpoint = "create_discussion", discussion_id = %created.id);
            
            // If there's a mapping between course and Discourse category, sync this discussion
            // This would typically trigger a background task
            if let Some(topic_id) = &created.topic_id {
                info!(
                    event = "discussion_sync", 
                    discussion_id = %created.id, 
                    discourse_topic_id = %topic_id
                );
                // Actual sync would happen here or be queued
            }
            
            Ok(created)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "create_discussion", error = %e);
            Err(format!("Failed to create discussion: {}", e))
        }
    }
}

/// Updates a discussion
///
/// # Arguments
/// * `discussion` - Updated discussion data
///
/// # Returns
/// * `Discussion` - The updated discussion
#[tauri::command]
#[instrument(skip(discussion_repo), fields(discussion_id = %discussion.id), err)]
pub async fn update_discussion(
    discussion: Discussion,
    discussion_repo: State<'_, Arc<dyn DiscussionRepository + Send + Sync>>
) -> Result<Discussion, String> {
    info!(event = "api_call", endpoint = "update_discussion", discussion_id = %discussion.id);
    
    // Update the timestamp
    let mut updated_discussion = discussion;
    updated_discussion.updated_at = chrono::Utc::now().to_rfc3339();
    
    match discussion_repo.update_discussion(updated_discussion).await {
        Ok(updated) => {
            info!(event = "api_success", endpoint = "update_discussion", discussion_id = %updated.id);
            
            // If the discussion is linked to a Discourse topic, sync changes
            if let Some(topic_id) = &updated.topic_id {
                info!(
                    event = "discussion_sync_update", 
                    discussion_id = %updated.id, 
                    discourse_topic_id = %topic_id
                );
                // Actual sync would happen here or be queued
            }
            
            Ok(updated)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "update_discussion", error = %e);
            Err(format!("Failed to update discussion: {}", e))
        }
    }
}

/// Syncs a discussion with Discourse
///
/// # Arguments
/// * `discussion_id` - ID of the discussion to sync
///
/// # Returns
/// * `Discussion` - The synced discussion with updated topic_id
#[tauri::command]
#[instrument(skip(discussion_repo, sync_service), err)]
pub async fn sync_discussion(
    discussion_id: String,
    discussion_repo: State<'_, Arc<dyn DiscussionRepository + Send + Sync>>,
    sync_service: State<'_, crate::services::sync::SyncService>
) -> Result<Discussion, String> {
    info!(event = "api_call", endpoint = "sync_discussion", discussion_id = %discussion_id);
    
    // Get the discussion
    let discussion = match discussion_repo.get_discussion_by_id(&discussion_id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            warn!(event = "api_not_found", endpoint = "sync_discussion", discussion_id = %discussion_id);
            return Err(format!("Discussion not found with ID: {}", discussion_id));
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "sync_discussion", error = %e);
            return Err(format!("Database error: {}", e));
        }
    };
    
    // Find course-category mapping for this discussion's course
    let mapping = match sync_service.find_mapping_for_course(&discussion.course_id).await {
        Ok(Some(m)) => m,
        Ok(None) => {
            warn!(
                event = "api_no_mapping", 
                endpoint = "sync_discussion", 
                discussion_id = %discussion_id,
                course_id = %discussion.course_id
            );
            return Err("No category mapping found for this course".to_string());
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "sync_discussion", error = %e);
            return Err(format!("Error finding course mapping: {}", e));
        }
    };
    
    // Perform sync with Discourse
    let updated_discussion = match sync_service.sync_discussion_to_discourse(&discussion, &mapping).await {
        Ok(updated) => updated,
        Err(e) => {
            error!(
                event = "api_sync_error", 
                endpoint = "sync_discussion", 
                discussion_id = %discussion_id,
                error = %e
            );
            return Err(format!("Failed to sync discussion: {}", e));
        }
    };
    
    // Update the discussion with the new topic_id
    match discussion_repo.update_discussion(updated_discussion.clone()).await {
        Ok(saved) => {
            info!(
                event = "api_success", 
                endpoint = "sync_discussion", 
                discussion_id = %saved.id,
                topic_id = ?saved.topic_id
            );
            Ok(saved)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "sync_discussion", 
                discussion_id = %discussion_id,
                error = %e
            );
            Err(format!("Failed to update discussion with sync results: {}", e))
        }
    }
}

/// Deletes a discussion
///
/// # Arguments
/// * `discussion_id` - ID of the discussion to delete
///
/// # Returns
/// * `bool` - Whether the deletion was successful
#[tauri::command]
#[instrument(skip(discussion_repo), err)]
pub async fn delete_discussion(
    discussion_id: String,
    discussion_repo: State<'_, Arc<dyn DiscussionRepository + Send + Sync>>
) -> Result<bool, String> {
    info!(event = "api_call", endpoint = "delete_discussion", discussion_id = %discussion_id);
    
    // Get the discussion first to check if it has a topic_id (for sync deletion)
    let discussion = match discussion_repo.get_discussion_by_id(&discussion_id).await {
        Ok(Some(d)) => Some(d),
        Ok(None) => None,
        Err(e) => {
            error!(event = "api_error", endpoint = "delete_discussion", error = %e);
            return Err(format!("Database error: {}", e));
        }
    };
    
    // If discussion has a topic_id, we might want to handle Discourse side deletion
    if let Some(disc) = &discussion {
        if let Some(topic_id) = &disc.topic_id {
            info!(
                event = "discourse_topic_linked", 
                endpoint = "delete_discussion",
                topic_id = %topic_id
            );
            // Note: actual deletion in Discourse would be handled here or queued
            // This is typically a soft delete or archive in Discourse
        }
    }
    
    // Now delete from our database
    match discussion_repo.delete_discussion(&discussion_id).await {
        Ok(deleted) => {
            if deleted {
                info!(event = "api_success", endpoint = "delete_discussion", discussion_id = %discussion_id);
            } else {
                warn!(event = "api_not_found", endpoint = "delete_discussion", discussion_id = %discussion_id);
            }
            Ok(deleted)
        },
        Err(e) => {
            error!(event = "api_error", endpoint = "delete_discussion", error = %e);
            Err(format!("Failed to delete discussion: {}", e))
        }
    }
}