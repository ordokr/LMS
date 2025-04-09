use tauri::{command, State};
use sqlx::SqlitePool;
use std::sync::Arc;
use crate::error::Error;
use crate::models::forum::category::{Category, CategoryRequest};
use crate::repositories::category_repository::CategoryRepository;

#[command]
pub async fn create_category(
    request: CategoryRequest,
    pool: State<'_, SqlitePool>
) -> Result<Category, Error> {
    let category_repo = CategoryRepository::new(pool.inner().clone());
    category_repo.create(request).await
}

#[command]
pub async fn get_category(
    id: String,
    pool: State<'_, SqlitePool>
) -> Result<Category, Error> {
    let category_repo = CategoryRepository::new(pool.inner().clone());
    category_repo.get_by_id(&id).await
}

#[command]
pub async fn list_categories_by_course(
    course_id: String,
    pool: State<'_, SqlitePool>
) -> Result<Vec<Category>, Error> {
    let category_repo = CategoryRepository::new(pool.inner().clone());
    category_repo.list_by_course(&course_id).await
}

#[command]
pub async fn list_root_categories(
    pool: State<'_, SqlitePool>
) -> Result<Vec<Category>, Error> {
    let category_repo = CategoryRepository::new(pool.inner().clone());
    category_repo.list_root_categories().await
}