use tauri::{command, State};
use sqlx::SqlitePool;
use std::sync::Arc;
use crate::error::Error;
use crate::models::forum::topic::{Topic, TopicRequest, TopicSummary};
use crate::repositories::topic_repository::TopicRepository;

#[command]
pub async fn create_topic(
    user_id: String,
    request: TopicRequest,
    pool: State<'_, SqlitePool>
) -> Result<Topic, Error> {
    let topic_repo = TopicRepository::new(pool.inner().clone());
    topic_repo.create(&user_id, request).await
}

#[command]
pub async fn get_topic(
    id: String,
    pool: State<'_, SqlitePool>
) -> Result<Topic, Error> {
    let topic_repo = TopicRepository::new(pool.inner().clone());
    topic_repo.get_by_id(&id).await
}

#[command]
pub async fn list_topics_by_category(
    category_id: String,
    page: Option<i64>,
    per_page: Option<i64>,
    pool: State<'_, SqlitePool>
) -> Result<Vec<TopicSummary>, Error> {
    let topic_repo = TopicRepository::new(pool.inner().clone());
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    topic_repo.list_by_category(&category_id, page, per_page).await
}

#[command]
pub async fn list_latest_topics(
    page: Option<i64>,
    per_page: Option<i64>,
    pool: State<'_, SqlitePool>
) -> Result<Vec<TopicSummary>, Error> {
    let topic_repo = TopicRepository::new(pool.inner().clone());
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    topic_repo.list_latest(page, per_page).await
}