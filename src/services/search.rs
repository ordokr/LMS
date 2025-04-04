use leptos::*;
use gloo_net::http::Request;
use crate::models::search::{SearchRequest, SearchResponse, SearchSuggestion, SearchStats};
use crate::utils::errors::AppError;

pub struct SearchService;

impl SearchService {
    pub async fn search(search_request: &SearchRequest) -> Result<SearchResponse, AppError> {
        let resp = Request::post("/api/search")
            .json(search_request)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let search_results = resp.json::<SearchResponse>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(search_results)
    }

    pub async fn get_search_suggestions(query: &str) -> Result<Vec<SearchSuggestion>, AppError> {
        if query.trim().len() < 2 {
            return Ok(vec![]);
        }

        let resp = Request::get(&format!("/api/search/suggestions?q={}", query))
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let suggestions = resp.json::<Vec<SearchSuggestion>>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(suggestions)
    }

    pub async fn get_stats() -> Result<SearchStats, AppError> {
        let resp = Request::get("/api/search/stats")
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if resp.status() != 200 {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ApiError(error_text));
        }

        let stats = resp.json::<SearchStats>().await
            .map_err(|e| AppError::DeserializationError(e.to_string()))?;
        
        Ok(stats)
    }
}