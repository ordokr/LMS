use tauri::{command, State};
use sqlx::SqlitePool;
use std::sync::Arc;
use crate::error::Error;
use crate::models::forum::post::{Post, PostRequest};
use crate::repositories::post_repository::PostRepository;

#[command]
pub async fn create_post(
    user_id: String,
    request: PostRequest,
    pool: State<'_, SqlitePool>
) -> Result<Post, Error> {
    let post_repo = PostRepository::new(pool.inner().clone());
    post_repo.create(&user_id, request).await
}

#[command]
pub async fn get_post(
    id: String,
    pool: State<'_, SqlitePool>
) -> Result<Post, Error> {
    let post_repo = PostRepository::new(pool.inner().clone());
    post_repo.get_by_id(&id).await
}

#[command]
pub async fn list_posts_by_topic(
    topic_id: String,
    page: Option<i64>,
    per_page: Option<i64>,
    pool: State<'_, SqlitePool>
) -> Result<Vec<Post>, Error> {
    let post_repo = PostRepository::new(pool.inner().clone());
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    post_repo.list_by_topic(&topic_id, page, per_page).await
}