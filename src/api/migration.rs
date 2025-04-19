// This module provides API endpoints for mapping and tracking ported entities between Canvas/Discourse and Ordo at the code/schema level.
// It does NOT support or perform data migration, user import, or live system integration. All references to 'migration' refer to code/schema/feature mapping only.

use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteMigrationRepository, MigrationRepository, MigrationRow};

#[derive(Deserialize, Serialize)]
/// Represents a mapping between source and target entities at the code/schema level (not data migration).
pub struct MigrationRequest {
    pub source_id: String,
    pub target_id: String,
    pub entity_type: String,
}

#[derive(Serialize)]
/// Response for a code/schema migration mapping (not data migration).
pub struct MigrationResponse {
    pub id: i64,
    pub source_id: String,
    pub target_id: String,
    pub entity_type: String,
}

/// Create a new code/schema migration mapping (not data migration).
async fn create_migration(
    Json(req): Json<MigrationRequest>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<MigrationResponse>, StatusCode> {
    let repo = SqliteMigrationRepository { pool: &pool };
    match repo.create_migration(&req.source_id, &req.target_id, &req.entity_type).await {
        Ok(id) => Ok(Json(MigrationResponse { id, source_id: req.source_id, target_id: req.target_id, entity_type: req.entity_type })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get a code/schema migration mapping by ID (not data migration).
async fn get_migration(
    Path(id): Path<i64>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<MigrationResponse>, StatusCode> {
    let repo = SqliteMigrationRepository { pool: &pool };
    match repo.get_migration(id).await {
        Ok(Some(row)) => Ok(Json(MigrationResponse { id: row.id, source_id: row.source_id, target_id: row.target_id, entity_type: row.entity_type })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn migration_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/migration", post(create_migration))
        .route("/migration/:id", get(get_migration))
        .with_state(pool)
}
