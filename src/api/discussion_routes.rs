use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use crate::services::discussion_service::DiscussionService;
use crate::models::discussion::DiscussionMapping;
use crate::auth::middleware::auth_middleware;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateDiscussionMappingRequest {
    pub canvas_discussion_id: String,
    pub discourse_topic_id: String,
    pub course_category_id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDiscussionMappingRequest {
    pub title: Option<String>,
    pub sync_enabled: Option<bool>,
    pub sync_posts: Option<bool>,
}

pub fn discussion_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/discussions", 
            get(get_all_discussions)
            .post(create_discussion_mapping)
        )
        .route("/api/discussions/:id", 
            get(get_discussion_mapping)
            .put(update_discussion_mapping)
            .delete(delete_discussion_mapping)
        )
        .route("/api/discussions/:id/sync", post(sync_discussion_mapping))
        .route("/api/courses/:course_id/discussions", 
            get(get_discussions_for_course)
            .post(auto_create_for_course)
        )
        .route("/api/courses/:course_id/discussions/sync", post(sync_all_for_course))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(), 
            auth_middleware
        ))
        .with_state(state)
}

async fn get_all_discussions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DiscussionMapping>>, (StatusCode, String)> {
    match state.db.get_all_discussion_mappings().await {
        Ok(mappings) => Ok(Json(mappings)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_discussion_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<DiscussionMapping>, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match discussion_service.get_discussion_mapping(&id).await {
        Ok(mapping) => Ok(Json(mapping)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn get_discussions_for_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<DiscussionMapping>>, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match discussion_service.get_mappings_for_course(&course_id).await {
        Ok(mappings) => Ok(Json(mappings)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn create_discussion_mapping(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateDiscussionMappingRequest>,
) -> Result<Json<DiscussionMapping>, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match discussion_service.create_discussion_mapping(
        &request.canvas_discussion_id,
        &request.discourse_topic_id,
        &request.course_category_id,
        &request.title,
    ).await {
        Ok(mapping) => Ok(Json(mapping)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn auto_create_for_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<DiscussionMapping>>, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match discussion_service.auto_create_mappings_for_course(&course_id).await {
        Ok(mappings) => Ok(Json(mappings)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn update_discussion_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateDiscussionMappingRequest>,
) -> Result<Json<DiscussionMapping>, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    let title_ref = request.title.as_deref();
    
    match discussion_service.update_discussion_mapping(
        &id,
        title_ref,
        request.sync_enabled,
        request.sync_posts,
    ).await {
        Ok(mapping) => Ok(Json(mapping)),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn delete_discussion_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match discussion_service.delete_discussion_mapping(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn sync_discussion_mapping(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match discussion_service.sync_discussion_mapping(&id).await {
        Ok(summary) => Ok(Json(serde_json::json!({
            "success": true,
            "mapping_id": id,
            "summary": summary
        }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn sync_all_for_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let discussion_service = DiscussionService::new(
        state.db.clone(),
        state.canvas_client.clone(),
        state.discourse_client.clone(),
    );
    
    match discussion_service.sync_all_for_course(&course_id).await {
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