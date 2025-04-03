use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    models::Category,
    database::CategoryRepository,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateCategoryPayload {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub position: Option<i32>,
}

// List all categories
pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = CategoryRepository::new(&conn);
    
    match repo.list_all() {
        Ok(categories) => (StatusCode::OK, Json(categories)).into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve categories"})),
            )
                .into_response()
        }
    }
}

// Get a single category by ID
pub async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = CategoryRepository::new(&conn);
    
    match repo.find_by_id(id) {
        Ok(Some(category)) => (StatusCode::OK, Json(category)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Category not found"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve category"})),
            )
                .into_response()
        }
    }
}

// Create a new category
pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategoryPayload>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = CategoryRepository::new(&conn);
    
    // Create a new Category model
    let mut category = Category::new(
        payload.name,
        payload.slug,
        payload.description,
    );
    
    // Set parent ID if provided
    category.parent_id = payload.parent_id;
    
    // Save to database
    match repo.create(&category) {
        Ok(id) => {
            // Get the created category with ID
            match repo.find_by_id(id) {
                Ok(Some(created)) => (StatusCode::CREATED, Json(created)).into_response(),
                _ => (
                    StatusCode::CREATED,
                    Json(serde_json::json!({"id": id, "message": "Category created successfully"})),
                )
                    .into_response(),
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create category"})),
            )
                .into_response()
        }
    }
}

// Update an existing category
pub async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateCategoryPayload>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = CategoryRepository::new(&conn);
    
    // First get the existing category
    match repo.find_by_id(id) {
        Ok(Some(mut category)) => {
            // Update fields if provided
            if let Some(name) = payload.name {
                category.name = name;
            }
            
            if let Some(description) = payload.description {
                category.description = Some(description);
            }
            
            if let Some(color) = payload.color {
                category.color = Some(color);
            }
            
            if let Some(text_color) = payload.text_color {
                category.text_color = Some(text_color);
            }
            
            if let Some(position) = payload.position {
                category.position = position;
            }
            
            // Save the updated category
            match repo.update(&category) {
                Ok(_) => (StatusCode::OK, Json(category)).into_response(),
                Err(e) => {
                    eprintln!("Database error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "Failed to update category"})),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Category not found"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to retrieve category for update"})),
            )
                .into_response()
        }
    }
}

// Delete a category (soft delete)
pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let conn = state.conn.lock().await;
    let repo = CategoryRepository::new(&conn);
    
    match repo.delete(id) {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Category deleted successfully"})),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to delete category"})),
            )
                .into_response()
        }
    }
}