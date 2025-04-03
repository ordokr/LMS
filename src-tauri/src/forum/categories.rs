use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::core::errors::AppError;
use crate::database::repositories::forum::ForumCategoryRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub course_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub color: Option<String>,
}

pub async fn create_category(
    State(repo): State<Arc<ForumCategoryRepository>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Extract user_id from auth context in a real implementation
    // Verify that the user has permissions to create categories
    
    tracing::info!("Creating forum category: {}", payload.name);
    
    let category_id = repo.create_category(
        &payload.name,
        payload.description.as_deref(),
        payload.course_id,
        payload.parent_id,
        payload.color.as_deref(),
    ).await?;
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "category_id": category_id,
        "message": "Category created successfully"
    }))))
}

pub async fn get_categories(
    State(repo): State<Arc<ForumCategoryRepository>>,
) -> Result<impl IntoResponse, AppError> {
    let categories = repo.get_all_categories().await?;
    Ok((StatusCode::OK, Json(categories)))
}