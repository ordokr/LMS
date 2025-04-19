use tauri::{State, command};
use std::sync::Arc;
use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::app_state::AppState;
use crate::models::quiz::{ActivityType, QuizActivitySummary, QuizActivityStats};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityResponse {
    pub success: bool,
    pub message: String,
    pub activity_id: Option<String>,
}

/// Track quiz started activity
#[command]
pub async fn track_quiz_started(
    user_id: String,
    quiz_id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<ActivityResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    match quiz_service.track_quiz_started(&user_id, &quiz_id).await {
        Ok(activity_id) => Ok(ActivityResponse {
            success: true,
            message: "Quiz started activity tracked".to_string(),
            activity_id: Some(activity_id),
        }),
        Err(e) => Ok(ActivityResponse {
            success: false,
            message: format!("Failed to track quiz started activity: {}", e),
            activity_id: None,
        }),
    }
}

/// Track quiz completed activity
#[command]
pub async fn track_quiz_completed(
    user_id: String,
    quiz_id: String,
    score: f64,
    duration_ms: i64,
    app_state: State<'_, Arc<AppState>>
) -> Result<ActivityResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    match quiz_service.track_quiz_completed(&user_id, &quiz_id, score, duration_ms).await {
        Ok(activity_id) => Ok(ActivityResponse {
            success: true,
            message: "Quiz completed activity tracked".to_string(),
            activity_id: Some(activity_id),
        }),
        Err(e) => Ok(ActivityResponse {
            success: false,
            message: format!("Failed to track quiz completed activity: {}", e),
            activity_id: None,
        }),
    }
}

/// Track quiz abandoned activity
#[command]
pub async fn track_quiz_abandoned(
    user_id: String,
    quiz_id: String,
    duration_ms: i64,
    app_state: State<'_, Arc<AppState>>
) -> Result<ActivityResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    match quiz_service.track_quiz_abandoned(&user_id, &quiz_id, duration_ms).await {
        Ok(activity_id) => Ok(ActivityResponse {
            success: true,
            message: "Quiz abandoned activity tracked".to_string(),
            activity_id: Some(activity_id),
        }),
        Err(e) => Ok(ActivityResponse {
            success: false,
            message: format!("Failed to track quiz abandoned activity: {}", e),
            activity_id: None,
        }),
    }
}

/// Track question answered activity
#[command]
pub async fn track_question_answered(
    user_id: String,
    quiz_id: String,
    question_id: String,
    is_correct: bool,
    duration_ms: i64,
    app_state: State<'_, Arc<AppState>>
) -> Result<ActivityResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    match quiz_service.track_question_answered(&user_id, &quiz_id, &question_id, is_correct, duration_ms).await {
        Ok(activity_id) => Ok(ActivityResponse {
            success: true,
            message: "Question answered activity tracked".to_string(),
            activity_id: Some(activity_id),
        }),
        Err(e) => Ok(ActivityResponse {
            success: false,
            message: format!("Failed to track question answered activity: {}", e),
            activity_id: None,
        }),
    }
}

/// Track flashcard activity
#[command]
pub async fn track_flashcard_activity(
    user_id: String,
    quiz_id: String,
    question_id: String,
    activity_type: String,
    rating: Option<i32>,
    app_state: State<'_, Arc<AppState>>
) -> Result<ActivityResponse, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    // Convert string to ActivityType
    let activity_type = match activity_type.as_str() {
        "flashcard_viewed" => ActivityType::FlashcardViewed,
        "flashcard_flipped" => ActivityType::FlashcardFlipped,
        "flashcard_rated" => ActivityType::FlashcardRated,
        _ => return Ok(ActivityResponse {
            success: false,
            message: format!("Invalid flashcard activity type: {}", activity_type),
            activity_id: None,
        }),
    };
    
    match quiz_service.track_flashcard_activity(&user_id, &quiz_id, &question_id, activity_type, rating).await {
        Ok(activity_id) => Ok(ActivityResponse {
            success: true,
            message: "Flashcard activity tracked".to_string(),
            activity_id: Some(activity_id),
        }),
        Err(e) => Ok(ActivityResponse {
            success: false,
            message: format!("Failed to track flashcard activity: {}", e),
            activity_id: None,
        }),
    }
}

/// Get user activity summary
#[command]
pub async fn get_user_activity_summary(
    user_id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuizActivitySummary, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    quiz_service.get_user_activity_summary(&user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get quiz activity summary
#[command]
pub async fn get_quiz_activity_summary(
    quiz_id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuizActivitySummary, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    quiz_service.get_quiz_activity_summary(&quiz_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get activity stats
#[command]
pub async fn get_activity_stats(
    user_id: Option<String>,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuizActivityStats, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    quiz_service.get_activity_stats(user_id.as_deref())
        .await
        .map_err(|e| e.to_string())
}
