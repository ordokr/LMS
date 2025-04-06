use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use crate::services::mapping_service::MappingService;
use crate::models::mapping::{CourseCategory, SyncDirection};
use crate::auth::middleware::auth_middleware;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateMappingRequest {
    pub canvas_course_id: String,
    pub discourse_category_id: String,
    pub name: String,
    pub sync_direction: Option<SyncDirection>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMappingRequest {
    pub name: Option<String>,
    pub sync_enabled: Option<bool>,
    pub sync_direction: Option<SyncDirection>,
}

pub fn mapping_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/mappings", 
            get(get_all_mappings)
            .post(create_mapping)
        )
        .route("/api/mappings/:id", 
            get(get_mapping)
            .put(update_mapping)
            .delete(delete_mapping)
        )
        .route("/api/mappings/:id/sync", post(sync_mapping))
        .route("/api/mappings/sync-all", post(sync_all_mappings))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(), 
            auth_middleware
        ))
        .with_state(state)
}

async fn get_all_mappings(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CourseCategory>>, (StatusCode, String)> {
    let mapping_service = MappingService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match state.db.get_all_course_categories().await {
        Ok(mappings) => Ok(Json(mappings)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<CourseCategory>, (StatusCode, String)> {
    let mapping_service = MappingService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match mapping_service.get_mapping(&id).await {
        Ok(mapping) => Ok(Json(mapping)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn create_mapping(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateMappingRequest>,
) -> Result<Json<CourseCategory>, (StatusCode, String)> {
    let mapping_service = MappingService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match mapping_service.create_mapping(
        &request.canvas_course_id,
        &request.discourse_category_id,
        &request.name,
        request.sync_direction,
    ).await {
        Ok(mapping) => Ok(Json(mapping)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn update_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateMappingRequest>,
) -> Result<Json<CourseCategory>, (StatusCode, String)> {
    let mapping_service = MappingService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    let name_ref = request.name.as_deref();
    
    match mapping_service.update_mapping(
        &id,
        name_ref,
        request.sync_enabled,
        request.sync_direction,
    ).await {
        Ok(mapping) => Ok(Json(mapping)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn delete_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mapping_service = MappingService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match mapping_service.delete_mapping(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn sync_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mapping_service = MappingService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match mapping_service.sync_mapping(&id).await {
        Ok(summary) => Ok(Json(serde_json::json!({
            "success": true,
            "mapping_id": id,
            "summary": summary
        }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn sync_all_mappings(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let mapping_service = MappingService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match mapping_service.sync_all_mappings().await {
        Ok(results) => {
            let summary = results.into_iter()
                .map(|(id, result)| {
                    match result {
                        Ok(summary) => serde_json::json!({
                            "mapping_id": id,
                            "success": true,
                            "summary": summary
                        }),
                        Err(e) => serde_json::json!({
                            "mapping_id": id,
                            "success": false,
                            "error": e.to_string()
                        }),
                    }
                })
                .collect::<Vec<_>>();
                
            Ok(Json(serde_json::json!({
                "sync_results": summary
            })))
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}