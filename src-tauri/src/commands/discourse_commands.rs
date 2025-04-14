use serde::{Deserialize, Serialize};
use tauri::command;
use uuid::Uuid;

use crate::services::discourse::{DiscourseService, DiscourseError, DiscourseConfig};
use crate::models::discourse::{Topic, Post, Category, User, DiscourseConnectionStatus};

/// Represents the response for Discourse API operations
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscourseResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Creates a new successful response
fn success<T>(data: T) -> DiscourseResponse<T> {
    DiscourseResponse {
        success: true,
        data: Some(data),
        error: None,
    }
}

/// Creates a new error response
fn error<T>(message: String) -> DiscourseResponse<T> {
    DiscourseResponse {
        success: false,
        data: None,
        error: Some(message),
    }
}

/// Converts a DiscourseError to a DiscourseResponse
fn handle_error<T>(err: DiscourseError) -> DiscourseResponse<T> {
    error(format!("{}", err))
}

/// Check the connection status with the Discourse instance
#[command]
pub async fn check_discourse_connection(url: String, api_key: String) -> DiscourseResponse<DiscourseConnectionStatus> {
    let service = DiscourseService::new(&url, &api_key);
    match service.check_connection().await {
        Ok(status) => success(status),
        Err(err) => handle_error(err),
    }
}

/// Configure the Discourse integration
#[command]
pub async fn configure_discourse_integration(config: DiscourseConfig) -> DiscourseResponse<bool> {
    let service = DiscourseService::from_config(config);
    match service.save_configuration().await {
        Ok(_) => success(true),
        Err(err) => handle_error(err),
    }
}

/// Get Discourse configuration
#[command]
pub async fn get_discourse_config() -> DiscourseResponse<DiscourseConfig> {
    match DiscourseService::load_config().await {
        Ok(config) => success(config),
        Err(err) => handle_error(err),
    }
}

/// Fetch all categories from Discourse
#[command]
pub async fn get_discourse_categories() -> DiscourseResponse<Vec<Category>> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.get_categories().await {
                Ok(categories) => success(categories),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Create a new category in Discourse
#[command]
pub async fn create_discourse_category(name: String, description: String, parent_id: Option<i32>, color: Option<String>) -> DiscourseResponse<Category> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.create_category(name, description, parent_id, color).await {
                Ok(category) => success(category),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Fetch topics from a specific category
#[command]
pub async fn get_discourse_topics(category_id: i32) -> DiscourseResponse<Vec<Topic>> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.get_topics_by_category(category_id).await {
                Ok(topics) => success(topics),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Get a single topic with its posts
#[command]
pub async fn get_discourse_topic(topic_id: i32) -> DiscourseResponse<Topic> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.get_topic(topic_id).await {
                Ok(topic) => success(topic),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Create a new topic in Discourse
#[command]
pub async fn create_discourse_topic(category_id: i32, title: String, content: String) -> DiscourseResponse<Topic> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.create_topic(category_id, title, content).await {
                Ok(topic) => success(topic),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Reply to a topic
#[command]
pub async fn create_discourse_post(topic_id: i32, content: String) -> DiscourseResponse<Post> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.create_post(topic_id, content).await {
                Ok(post) => success(post),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Update an existing post
#[command]
pub async fn update_discourse_post(post_id: i32, content: String) -> DiscourseResponse<Post> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.update_post(post_id, content).await {
                Ok(post) => success(post),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Get user information
#[command]
pub async fn get_discourse_user(username: String) -> DiscourseResponse<User> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.get_user(username).await {
                Ok(user) => success(user),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Sync LMS course with Discourse category
#[command]
pub async fn sync_course_to_discourse(course_id: Uuid) -> DiscourseResponse<Category> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.sync_course(course_id).await {
                Ok(category) => success(category),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}

/// Sync LMS discussion topic with Discourse topic
#[command]
pub async fn sync_discussion_to_discourse(discussion_id: Uuid) -> DiscourseResponse<Topic> {
    match DiscourseService::get_instance().await {
        Ok(service) => {
            match service.sync_discussion(discussion_id).await {
                Ok(topic) => success(topic),
                Err(err) => handle_error(err),
            }
        },
        Err(err) => handle_error(err),
    }
}
