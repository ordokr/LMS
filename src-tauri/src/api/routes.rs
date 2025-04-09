use axum::{
    Router,
    routing::{get, post},
};
use crate::AppState;
use super::forum_mapping::{create_mapping, get_mapping_by_course};

pub fn forum_routes() -> Router<AppState> {
    Router::new()
        .route("/api/forum/mappings", post(create_mapping))
        .route("/api/forum/mappings/course/:course_id", get(get_mapping_by_course))
}