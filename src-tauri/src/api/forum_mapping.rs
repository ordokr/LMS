use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::AppState;
use crate::models::mapping::CourseCategoryMapping;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMappingRequest {
    pub course_id: i64,
    pub category_id: i64,
}

#[derive(Debug, Serialize)]
pub struct MappingResponse {
    pub mapping: CourseCategoryMapping,
}

pub async fn create_mapping(
    State(state): State<AppState>,
    Json(request): Json<CreateMappingRequest>,
) -> Result<Json<MappingResponse>, (StatusCode, String)> {
    let repo = &state.course_category_repo;
    
    let mapping = repo
        .create(request.course_id, request.category_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(MappingResponse { mapping }))
}

pub async fn get_mapping_by_course(
    State(state): State<AppState>,
    Path(course_id): Path<i64>,
) -> Result<Json<MappingResponse>, (StatusCode, String)> {
    let repo = &state.course_category_repo;
    
    let mapping = repo
        .get_by_course_id(course_id)
        .await
        .map_err(|e| {
            if e.to_string().contains("no rows") {
                (StatusCode::NOT_FOUND, format!("No mapping found for course ID {}", course_id))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        })?;
    
    Ok(Json(MappingResponse { mapping }))
}