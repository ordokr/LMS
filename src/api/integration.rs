use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
    routing::{get, post, delete, put},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::services::integration_service::IntegrationService;
use crate::AppState;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct CreateMappingRequest {
    pub course_id: i64,
    pub category_id: i64,
}

#[derive(Debug, Serialize)]
pub struct MappingResponse {
    pub id: i64,
    pub course_id: i64,
    pub category_id: i64,
    pub sync_enabled: bool,
    pub sync_topics: bool,
    pub sync_users: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_synced_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMappingRequest {
    pub sync_enabled: bool,
    pub sync_topics: bool,
    pub sync_users: bool,
}

#[derive(Debug, Deserialize)]
pub struct GenerateSSOTokenRequest {
    pub user_id: String,
    pub role: String,
    pub canvas_id: String,
    pub discourse_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

// Helper function to handle errors
fn handle_error<E: std::fmt::Display>(err: E) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", err),
    )
        .into_response()
}

async fn create_mapping(
    State(state): State<AppState>,
    Json(req): Json<CreateMappingRequest>,
) -> Result<impl IntoResponse, Response> {
    let integration_service = &state.integration_service;
    
    let mapping = integration_service
        .create_course_category_mapping(req.course_id, req.category_id)
        .await
        .map_err(handle_error)?;
    
    let response = MappingResponse {
        id: mapping.id,
        course_id: mapping.course_id,
        category_id: mapping.category_id,
        sync_enabled: mapping.sync_enabled,
        sync_topics: mapping.sync_topics,
        sync_users: mapping.sync_users,
        created_at: mapping.created_at.to_rfc3339(),
        updated_at: mapping.updated_at.to_rfc3339(),
        last_synced_at: mapping.last_synced_at.map(|dt| dt.to_rfc3339()),
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

async fn get_mapping_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.course_category_repo.get_by_id(id)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Mapping with ID {} not found", id)).into_response()
        })?;
    
    let response = MappingResponse {
        id: mapping.id,
        course_id: mapping.course_id,
        category_id: mapping.category_id,
        sync_enabled: mapping.sync_enabled,
        sync_topics: mapping.sync_topics,
        sync_users: mapping.sync_users,
        created_at: mapping.created_at.to_rfc3339(),
        updated_at: mapping.updated_at.to_rfc3339(),
        last_synced_at: mapping.last_synced_at.map(|dt| dt.to_rfc3339()),
    };
    
    Ok((StatusCode::OK, Json(response)))
}

async fn get_mapping_by_course(
    State(state): State<AppState>,
    Path(course_id): Path<i64>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.integration_service.get_mapping_by_course(course_id)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Mapping for course ID {} not found", course_id)).into_response()
        })?;
    
    let response = MappingResponse {
        id: mapping.id,
        course_id: mapping.course_id,
        category_id: mapping.category_id,
        sync_enabled: mapping.sync_enabled,
        sync_topics: mapping.sync_topics,
        sync_users: mapping.sync_users,
        created_at: mapping.created_at.to_rfc3339(),
        updated_at: mapping.updated_at.to_rfc3339(),
        last_synced_at: mapping.last_synced_at.map(|dt| dt.to_rfc3339()),
    };
    
    Ok((StatusCode::OK, Json(response)))
}

async fn list_all_mappings(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Response> {
    let mappings = state.integration_service.list_all_mappings()
        .await
        .map_err(handle_error)?;
    
    let response: Vec<MappingResponse> = mappings
        .into_iter()
        .map(|mapping| MappingResponse {
            id: mapping.id,
            course_id: mapping.course_id,
            category_id: mapping.category_id,
            sync_enabled: mapping.sync_enabled,
            sync_topics: mapping.sync_topics,
            sync_users: mapping.sync_users,
            created_at: mapping.created_at.to_rfc3339(),
            updated_at: mapping.updated_at.to_rfc3339(),
            last_synced_at: mapping.last_synced_at.map(|dt| dt.to_rfc3339()),
        })
        .collect();
    
    Ok((StatusCode::OK, Json(response)))
}

async fn update_mapping(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateMappingRequest>,
) -> Result<impl IntoResponse, Response> {
    let mapping = state.integration_service
        .update_mapping(id, req.sync_enabled, req.sync_topics, req.sync_users)
        .await
        .map_err(|_| {
            (StatusCode::NOT_FOUND, format!("Mapping with ID {} not found", id)).into_response()
        })?;
    
    let response = MappingResponse {
        id: mapping.id,
        course_id: mapping.course_id,
        category_id: mapping.category_id,
        sync_enabled: mapping.sync_enabled,
        sync_topics: mapping.sync_topics,
        sync_users: mapping.sync_users,
        created_at: mapping.created_at.to_rfc3339(),
        updated_at: mapping.updated_at.to_rfc3339(),
        last_synced_at: mapping.last_synced_at.map(|dt| dt.to_rfc3339()),
    };
    
    Ok((StatusCode::OK, Json(response)))
}

async fn delete_mapping(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Response> {
    state.integration_service
        .delete_mapping(id)
        .await
        .map_err(handle_error)?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn generate_sso_token(
    State(state): State<AppState>,
    Json(req): Json<GenerateSSOTokenRequest>,
) -> Result<impl IntoResponse, Response> {
    let token = state.integration_service
        .generate_sso_token(
            &req.user_id,
            &req.role,
            &req.canvas_id,
            req.discourse_id.as_deref(),
        )
        .map_err(handle_error)?;
    
    Ok((StatusCode::OK, Json(TokenResponse { token })))
}

pub fn integration_routes() -> Router<AppState> {
    Router::new()
        .route("/api/integration/mappings", post(create_mapping))
        .route("/api/integration/mappings", get(list_all_mappings))
        .route("/api/integration/mappings/:id", get(get_mapping_by_id))
        .route("/api/integration/mappings/:id", put(update_mapping))
        .route("/api/integration/mappings/:id", delete(delete_mapping))
        .route("/api/integration/mappings/course/:course_id", get(get_mapping_by_course))
        .route("/api/integration/auth/token", post(generate_sso_token))
}