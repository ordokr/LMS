use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    models::{category::Category, course::Course},
    services::course_category_mapper::CourseCategoryMapper,
};

#[derive(Serialize)]
pub struct CategoryResponse {
    category: Category,
    course: Option<Course>,
}

pub async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<String>,
) -> Result<Json<CategoryResponse>, StatusCode> {
    let category_id = Uuid::parse_str(&category_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mapper = CourseCategoryMapper::new(
        state.course_repo.clone(),
        state.category_repo.clone(),
    );
    
    let (category, course) = mapper.get_category_with_course(&category_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(CategoryResponse { category, course }))
}

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    let categories = state.category_repo.list_categories()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(categories))
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    name: String,
    description: Option<String>,
    parent_id: Option<String>,
    course_id: Option<String>,
    position: Option<i32>,
}

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, StatusCode> {
    let parent_id = match payload.parent_id {
        Some(ref id) => Some(Uuid::parse_str(id).map_err(|_| StatusCode::BAD_REQUEST)?),
        None =