use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_state::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscourseTopic {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscussionMigration {
    pub discourse_topic_id: String,
    pub ordo_discussion_id: String,
}

static DISCUSSION_MAPPINGS: once_cell::sync::Lazy<std::sync::Mutex<Vec<DiscussionMigration>>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

// POST /api/integration/map_discussion
async fn map_discussion(
    Json(mapping): Json<DiscussionMigration>,
    _state: Arc<AppState>,
) -> (StatusCode, Json<DiscussionMigration>) {
    let mut mappings = DISCUSSION_MAPPINGS.lock().unwrap();
    mappings.push(mapping.clone());
    (StatusCode::CREATED, Json(mapping))
}

// GET /api/integration/discussion/:discourse_topic_id
async fn get_discussion_mapping(
    Path(discourse_topic_id): Path<String>,
    _state: Arc<AppState>,
) -> Json<Option<DiscussionMigration>> {
    let mappings = DISCUSSION_MAPPINGS.lock().unwrap();
    let found = mappings.iter().find(|m| m.discourse_topic_id == discourse_topic_id).cloned();
    Json(found)
}

pub fn discussion_migration_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/integration/map_discussion", post(map_discussion))
        .route("/integration/discussion/:discourse_topic_id", get(get_discussion_mapping))
        .with_state(state)
}
