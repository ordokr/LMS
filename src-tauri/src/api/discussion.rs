use std::sync::Arc;
use tauri::State;
use tracing::{info, error, instrument};
use crate::models::DiscussionTopic;
use crate::repositories::DiscussionTopicRepository;

/// Gets discussion topics by category ID
/// 
/// # Arguments
/// * `category_id` - ID of the category to get topics for
/// * `page` - Page number (optional, defaults to 1)
/// * `limit` - Number of topics per page (optional, defaults to 20)
/// 
/// # Returns
/// * `Vec<DiscussionTopic>` - List of discussion topics in the category
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn get_topics_by_category(
    category_id: String,
    page: Option<u32>,
    limit: Option<u32>,
    repo: State<'_, Arc<dyn DiscussionTopicRepository + Send + Sync>>
) -> Result<Vec<DiscussionTopic>, String> {
    info!(
        event = "api_call", 
        endpoint = "get_topics_by_category", 
        category_id = %category_id,
        page = ?page,
        limit = ?limit
    );
    
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    
    match repo.find_by_category_id(&category_id, limit as i64, offset as i64).await {
        Ok(topics) => {
            info!(
                event = "api_success", 
                endpoint = "get_topics_by_category", 
                count = topics.len()
            );
            Ok(topics)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "get_topics_by_category", 
                error = %e
            );
            Err(format!("Failed to get topics: {}", e))
        }
    }
}

/// Gets discussion topics by author ID
/// 
/// # Arguments
/// * `author_id` - ID of the author to get topics for
/// * `page` - Page number (optional, defaults to 1)
/// * `limit` - Number of topics per page (optional, defaults to 20)
/// 
/// # Returns
/// * `Vec<DiscussionTopic>` - List of discussion topics by the author
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn get_topics_by_author(
    author_id: String,
    page: Option<u32>,
    limit: Option<u32>,
    repo: State<'_, Arc<dyn DiscussionTopicRepository + Send + Sync>>
) -> Result<Vec<DiscussionTopic>, String> {
    info!(
        event = "api_call", 
        endpoint = "get_topics_by_author", 
        author_id = %author_id,
        page = ?page,
        limit = ?limit
    );
    
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    
    match repo.find_by_author_id(&author_id, limit as i64, offset as i64).await {
        Ok(topics) => {
            info!(
                event = "api_success", 
                endpoint = "get_topics_by_author", 
                count = topics.len()
            );
            Ok(topics)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "get_topics_by_author", 
                error = %e
            );
            Err(format!("Failed to get topics: {}", e))
        }
    }
}

/// Searches discussion topics
/// 
/// # Arguments
/// * `query` - Search query string
/// * `category_id` - Optional category ID to restrict search to
/// * `page` - Page number (optional, defaults to 1)
/// * `limit` - Number of topics per page (optional, defaults to 20)
/// 
/// # Returns
/// * `Vec<DiscussionTopic>` - List of discussion topics matching the search query
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn search_topics(
    query: String,
    category_id: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
    repo: State<'_, Arc<dyn DiscussionTopicRepository + Send + Sync>>
) -> Result<Vec<DiscussionTopic>, String> {
    info!(
        event = "api_call", 
        endpoint = "search_topics", 
        query = %query,
        category_id = ?category_id,
        page = ?page,
        limit = ?limit
    );
    
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);
    let offset = (page - 1) * limit;
    
    match repo.search(&query, category_id.as_deref(), limit as i64, offset as i64).await {
        Ok(topics) => {
            info!(
                event = "api_success", 
                endpoint = "search_topics", 
                count = topics.len()
            );
            Ok(topics)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "search_topics", 
                error = %e
            );
            Err(format!("Failed to search topics: {}", e))
        }
    }
}

/// Gets recent discussion topics across all categories
/// 
/// # Arguments
/// * `days` - Number of days to look back (optional, defaults to 7)
/// * `limit` - Number of topics to return (optional, defaults to 20)
/// 
/// # Returns
/// * `Vec<DiscussionTopic>` - List of recent discussion topics
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn get_recent_topics(
    days: Option<u32>,
    limit: Option<u32>,
    repo: State<'_, Arc<dyn DiscussionTopicRepository + Send + Sync>>
) -> Result<Vec<DiscussionTopic>, String> {
    let days = days.unwrap_or(7);
    let limit = limit.unwrap_or(20);
    
    info!(
        event = "api_call", 
        endpoint = "get_recent_topics", 
        days = days,
        limit = limit
    );
    
    // Calculate the date threshold (N days ago)
    let now = chrono::Utc::now();
    let days_ago = now - chrono::Duration::days(days as i64);
    
    match repo.find_recent(days_ago, limit as i64).await {
        Ok(topics) => {
            info!(
                event = "api_success", 
                endpoint = "get_recent_topics", 
                count = topics.len()
            );
            Ok(topics)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "get_recent_topics", 
                error = %e
            );
            Err(format!("Failed to get recent topics: {}", e))
        }
    }
}

/// Gets the most active discussion topics (by post count)
/// 
/// # Arguments
/// * `limit` - Number of topics to return (optional, defaults to 10)
/// 
/// # Returns
/// * `Vec<DiscussionTopic>` - List of most active discussion topics
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn get_active_topics(
    limit: Option<u32>,
    repo: State<'_, Arc<dyn DiscussionTopicRepository + Send + Sync>>
) -> Result<Vec<DiscussionTopic>, String> {
    let limit = limit.unwrap_or(10);
    
    info!(
        event = "api_call", 
        endpoint = "get_active_topics", 
        limit = limit
    );
    
    match repo.find_most_active(limit as i64).await {
        Ok(topics) => {
            info!(
                event = "api_success", 
                endpoint = "get_active_topics", 
                count = topics.len()
            );
            Ok(topics)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "get_active_topics", 
                error = %e
            );
            Err(format!("Failed to get active topics: {}", e))
        }
    }
}