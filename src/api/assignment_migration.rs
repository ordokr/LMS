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
pub struct AssignmentMigration {
    pub canvas_assignment_id: String,
    pub ordo_assignment_id: String,
}

static ASSIGNMENT_MAPPINGS: once_cell::sync::Lazy<std::sync::Mutex<Vec<AssignmentMigration>>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(vec![]));

// POST /api/integration/map_assignment
async fn map_assignment(
    Json(mapping): Json<AssignmentMigration>,
    _state: Arc<AppState>,
) -> (StatusCode, Json<AssignmentMigration>) {
    let mut mappings = ASSIGNMENT_MAPPINGS.lock().unwrap();
    mappings.push(mapping.clone());
    (StatusCode::CREATED, Json(mapping))
}

// GET /api/integration/assignment/:canvas_assignment_id
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
