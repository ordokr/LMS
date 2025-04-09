use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::search::meilisearch::{MeiliSearchClient, SearchOptions};

pub struct AppState {
    search_client: Arc<MeiliSearchClient>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: String,
    #[serde(default)]
    limit: usize,
    #[serde(default)]
    offset: usize,
    category_id: Option<i64>,
    user_id: Option<i64>,
    sort_by: Option<String>,
    sort_dir: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse<T> {
    hits: Vec<T>,
    total_hits: usize,
    processing_time_ms: u64,
}

async fn search_topics(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse<serde_json::Value>>, String> {
    let mut options = SearchOptions {
        limit: if query.limit == 0 { 20 } else { query.limit },
        offset: query.offset,
        filter: None,
        sort: None,
    };
    
    // Build filter string
    let mut filters = Vec::new();
    if let Some(cat_id) = query.category_id {
        filters.push(format!("category_id = {}", cat_id));
    }
    
    if let Some(user_id) = query.user_id {
        filters.push(format!("user_id = {}", user_id));
    }
    
    if !filters.is_empty() {
        options.filter = Some(filters.join(" AND "));
    }
    
    // Handle sorting
    if let Some(sort_by) = query.sort_by {
        let direction = query.sort_dir.unwrap_or_else(|| "asc".to_string());
        options.sort = Some(vec![format!("{}:{}", sort_by, direction)]);
    }
    
    // Execute search
    let results = state.search_client.search_topics(&query.q, options).await?;
    
    let response = SearchResponse {
        hits: results.hits.into_iter().map(|h| serde_json::to_value(h).unwrap_or_default()).collect(),
        total_hits: results.nb_hits,
        processing_time_ms: results.processing_time_ms,
    };
    
    Ok(Json(response))
}

async fn search_categories(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse<serde_json::Value>>, String> {
    let options = SearchOptions {
        limit: if query.limit == 0 { 20 } else { query.limit },
        offset: query.offset,
        filter: None,
        sort: None,
    };
    
    // Execute search
    let results = state.search_client.search_categories(&query.q, options).await?;
    
    let response = SearchResponse {
        hits: results.hits.into_iter().map(|h| serde_json::to_value(h).unwrap_or_default()).collect(),
        total_hits: results.nb_hits,
        processing_time_ms: results.processing_time_ms,
    };
    
    Ok(Json(response))
}

// Force reindex
async fn trigger_reindex(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, String> {
    state.search_client.sync_data(true).await?;
    
    Ok(Json(serde_json::json!({ "status": "success", "message": "Reindexing completed" })))
}

// Create router
pub fn search_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/topics", get(search_topics))
        .route("/categories", get(search_categories))
        .route("/reindex", get(trigger_reindex))
}