// This module provides API endpoints for mapping and tracking ported post entities between Discourse and Ordo at the code/schema level.
// It does NOT support or perform data migration, user import, or live system integration. All references to 'migration' refer to code/schema/feature mapping only.

use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_state::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscoursePost {
    pub id: String,
    pub topic_id: String,
    pub user_id: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
/// Represents a mapping between Discourse and Ordo posts at the code/schema level (not data migration).
pub struct PostMigration {
    pub discourse_post_id: String,
    pub ordo_post_id: String,
}

static POST_MAPPINGS: once_cell::sync::Lazy<std::sync::Mutex<Vec<PostMigration>>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

/// Map a Discourse post to an Ordo post (code/schema mapping, not data migration).
async fn map_post(
    Json(mapping): Json<PostMigration>,
    _state: Arc<AppState>,
) -> (StatusCode, Json<PostMigration>) {
    let mut mappings = POST_MAPPINGS.lock().unwrap();
    mappings.push(mapping.clone());
    (StatusCode::CREATED, Json(mapping))
}

/// Get a code/schema post mapping by Discourse post ID (not data migration).
async fn get_post_mapping(
    Path(discourse_post_id): Path<String>,
    _state: Arc<AppState>,
) -> Json<Option<PostMigration>> {
    let mappings = POST_MAPPINGS.lock().unwrap();
    let found = mappings.iter().find(|m| m.discourse_post_id == discourse_post_id).cloned();
    Json(found)
}

pub fn post_migration_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/integration/map_post", post(map_post))
        .route("/integration/post/:discourse_post_id", get(get_post_mapping))
        .with_state(state)
}
