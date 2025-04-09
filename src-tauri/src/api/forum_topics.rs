use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::DB;
use crate::error::Error;
use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::user::user::User;

#[derive(Debug, Deserialize)]
pub struct TopicQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub course_id: Option<String>,
    pub category_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub content: String,
    pub category_id: Option<String>,
    pub course_id: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub async fn get_topics(
    State(db): State<DB>,
    Query(params): Query<TopicQuery>,
) -> Result<Json<Vec<Topic>>, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    
    let topics = match (params.course_id, params.category_id) {
        (Some(course_id), _) => {
            let course_uuid = Uuid::parse_str(&course_id)
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid course ID".to_string()))?;
            
            Topic::find_by_course_id(&db, course_uuid)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        },
        (_, Some(category_id)) => {
            let category_uuid = Uuid::parse_str(&category_id)
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid category ID".to_string()))?;
            
            Topic::find_by_category_id(&db, category_uuid)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        },
        _ => {
            // Get all topics (paginated)
            let topics = sqlx::query_as::<_, Topic>(
                "SELECT * FROM topics ORDER BY last_post_at DESC LIMIT ? OFFSET ?"
            )
            .bind(per_page as i64)
            .bind(((page - 1) * per_page) as i64)
            .fetch_all(&db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            topics
        }
    };
    
    Ok(Json(topics))
}

pub async fn get_topic(
    State(db): State<DB>,
    Path(id): Path<String>,
) -> Result<Json<Topic>, (StatusCode, String)> {
    let topic_id = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid topic ID".to_string()))?;
    
    let topic = Topic::find(&db, topic_id)
        .await
        .map_err(|e| match e {
            Error::NotFound => (StatusCode::NOT_FOUND, "Topic not found".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })?;
    
    Ok(Json(topic))
}

pub async fn create_topic(
    State(db): State<DB>,
    // In a real app, extract the current user from auth token
    Json(payload): Json<CreateTopicRequest>,
) -> Result<Json<Topic>, (StatusCode, String)> {
    // Get the current user (mock for now - in production this would come from auth middleware)
    let current_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001")
        .expect("Invalid user ID");
    
    // Create new topic
    let mut topic = Topic::new(
        payload.title,
        current_user_id,
        payload.content,
    );
    
    // Set optional fields
    if let Some(category_id) = payload.category_id {
        topic.category_id = Some(
            Uuid::parse_str(&category_id)
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid category ID".to_string()))?
        );
    }
    
    if let Some(course_id) = payload.course_id {
        topic.course_id = Some(
            Uuid::parse_str(&course_id)
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid course ID".to_string()))?
        );
    }
    
    if let Some(tags) = payload.tags {
        topic.tags = tags;
    }
    
    // Save the topic
    topic.create(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(topic))
}