// This module provides API endpoints for mapping and tracking ported assignment entities between Canvas and Ordo at the code/schema level.
// It does NOT support or perform data migration, user import, or live system integration. All references to 'migration' refer to code/schema/feature mapping only.

use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::app_state::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct CanvasAssignment {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub description: Option<String>,
    pub points: u32,
}

#[derive(Serialize, Deserialize, Clone)]
/// Represents a mapping between Canvas and Ordo assignments at the code/schema level (not data migration).
pub struct AssignmentMigration {
    pub canvas_assignment_id: String,
    pub ordo_assignment_id: String,
}

static ASSIGNMENT_MAPPINGS: once_cell::sync::Lazy<std::sync::Mutex<Vec<AssignmentMigration>>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

/// Map a Canvas assignment to an Ordo assignment (code/schema mapping, not data migration).
async fn map_assignment(
    Json(mapping): Json<AssignmentMigration>,
    _state: Arc<AppState>,
) -> (StatusCode, Json<AssignmentMigration>) {
    let mut mappings = ASSIGNMENT_MAPPINGS.lock().unwrap();
    mappings.push(mapping.clone());
    (StatusCode::CREATED, Json(mapping))
}

/// Get a code/schema assignment mapping by Canvas assignment ID (not data migration).
async fn get_assignment_mapping(
    Path(canvas_assignment_id): Path<String>,
    _state: Arc<AppState>,
) -> Json<Option<AssignmentMigration>> {
    let mappings = ASSIGNMENT_MAPPINGS.lock().unwrap();
    let found = mappings.iter().find(|m| m.canvas_assignment_id == canvas_assignment_id).cloned();
    Json(found)
}

pub fn assignment_migration_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/integration/map_assignment", post(map_assignment))
        .route("/integration/assignment/:canvas_assignment_id", get(get_assignment_mapping))
        .with_state(state)
}
