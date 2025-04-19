// This module provides API endpoints for mapping and tracking ported entities between Canvas/Discourse and Ordo at the code/schema level.
// It does NOT support or perform data migration, user import, or live system integration. 
// This is strictly for SOURCE CODE MAPPING between original and ported code structures.

use axum::{Router, routing::{get, post}, extract::{State, Path, Json}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteMigrationRepository as SqliteCodeMappingRepository, MigrationRepository as CodeMappingRepository, MigrationRow as CodeMappingRow};

#[derive(Deserialize, Serialize)]
/// Represents a mapping between source and target entities at the code/schema level.
/// This is for tracking how source code entities are ported to the new system.
pub struct CodeMappingRequest {
    pub source_id: String,
    pub target_id: String,
    pub entity_type: String,
}

#[derive(Serialize)]
/// Response for a code/schema mapping operation.
/// This tracks how source code entities are ported to the new system.
pub struct CodeMappingResponse {
    pub id: i64,
    pub source_id: String,
    pub target_id: String,
    pub entity_type: String,
}

/// Create a new code/schema mapping between source and target entities.
/// This is used during the source code porting process, not for data migration.
async fn create_code_mapping(
    Json(req): Json<CodeMappingRequest>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<CodeMappingResponse>, StatusCode> {
    let repo = SqliteCodeMappingRepository { pool: &pool };
    match repo.create_migration(&req.source_id, &req.target_id, &req.entity_type).await {
        Ok(id) => Ok(Json(CodeMappingResponse { id, source_id: req.source_id, target_id: req.target_id, entity_type: req.entity_type })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get a code/schema mapping by ID.
/// This retrieves information about how source code entities are ported to the new system.
async fn get_code_mapping(
    Path(id): Path<i64>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<CodeMappingResponse>, StatusCode> {
    let repo = SqliteCodeMappingRepository { pool: &pool };
    match repo.get_migration(id).await {
        Ok(Some(row)) => Ok(Json(CodeMappingResponse { id: row.id, source_id: row.source_id, target_id: row.target_id, entity_type: row.entity_type })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn code_mapping_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/code-mapping", post(create_code_mapping))
        .route("/code-mapping/:id", get(get_code_mapping))
        .with_state(pool)
}
