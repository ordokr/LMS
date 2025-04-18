use super::QuizEngine;
use super::ai_generation::{AISourceType, AIModelType, AIGenerationStatus};
use super::models::{AnswerType, StudyMode, QuizVisibility};
use uuid::Uuid;
use tauri::State;
use serde_json;

#[tauri::command]
pub async fn create_ai_generation_request(
    title: String,
    description: Option<String>,
    user_id: Option<String>,
    source_type: String,
    source_content: String,
    model_type: String,
    model_parameters: Option<serde_json::Value>,
    num_questions: i32,
    question_types: Vec<String>,
    difficulty_level: i32,
    topic_focus: Option<String>,
    language: String,
    study_mode: String,
    visibility: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let user_uuid = if let Some(id) = user_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };
    
    let source_type = match source_type.as_str() {
        "Text" => AISourceType::Text,
        "URL" => AISourceType::URL,
        "PDF" => AISourceType::PDF,
        "Image" => AISourceType::Image,
        "Video" => AISourceType::Video,
        "Audio" => AISourceType::Audio,
        _ => AISourceType::Custom,
    };
    
    let model_type = match model_type.as_str() {
        "OpenAI" => AIModelType::OpenAI,
        "Anthropic" => AIModelType::Anthropic,
        "Gemini" => AIModelType::Gemini,
        "LocalLLM" => AIModelType::LocalLLM,
        _ => AIModelType::Custom,
    };
    
    let parsed_question_types = question_types.iter()
        .map(|qt| match qt.as_str() {
            "multiple_choice" => AnswerType::MultipleChoice,
            "true_false" => AnswerType::TrueFalse,
            "short_answer" => AnswerType::ShortAnswer,
            "essay" => AnswerType::Essay,
            "matching" => AnswerType::Matching,
            "ordering" => AnswerType::Ordering,
            _ => AnswerType::MultipleChoice,
        })
        .collect();
    
    let study_mode = match study_mode.as_str() {
        "flashcards" => StudyMode::Flashcards,
        "multiple_choice" => StudyMode::MultipleChoice,
        "written" => StudyMode::Written,
        "mixed" => StudyMode::Mixed,
        _ => StudyMode::MultipleChoice,
    };
    
    let visibility = match visibility.as_str() {
        "public" => QuizVisibility::Public,
        "private" => QuizVisibility::Private,
        "unlisted" => QuizVisibility::Unlisted,
        _ => QuizVisibility::Private,
    };
    
    let request = engine.create_ai_generation_request(
        title,
        description,
        user_uuid,
        source_type,
        source_content,
        model_type,
        model_parameters,
        num_questions,
        parsed_question_types,
        difficulty_level,
        topic_focus,
        language,
        study_mode,
        visibility,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn process_ai_generation_request(
    request_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let request_uuid = Uuid::parse_str(&request_id).map_err(|e| e.to_string())?;
    
    let result = engine.process_ai_generation_request(request_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_generation_request(
    request_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let request_uuid = Uuid::parse_str(&request_id).map_err(|e| e.to_string())?;
    
    let request = engine.get_ai_generation_request(request_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_generation_result(
    request_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let request_uuid = Uuid::parse_str(&request_id).map_err(|e| e.to_string())?;
    
    let result = engine.get_ai_generation_result(request_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_generation_requests_by_user(
    user_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let requests = engine.get_ai_generation_requests_by_user(user_uuid, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = requests.into_iter()
        .map(|r| serde_json::to_value(r))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn cancel_ai_generation_request(
    request_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let request_uuid = Uuid::parse_str(&request_id).map_err(|e| e.to_string())?;
    
    engine.cancel_ai_generation_request(request_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_ai_model_providers(
    engine: State<'_, QuizEngine>,
) -> Result<Vec<(String, String)>, String> {
    let providers = engine.get_ai_model_providers()
        .map_err(|e| e.to_string())?;
    
    let result = providers.into_iter()
        .map(|(name, model_type)| {
            let type_str = match model_type {
                AIModelType::OpenAI => "OpenAI",
                AIModelType::Anthropic => "Anthropic",
                AIModelType::Gemini => "Gemini",
                AIModelType::LocalLLM => "LocalLLM",
                AIModelType::Custom => "Custom",
            };
            (name, type_str.to_string())
        })
        .collect();
    
    Ok(result)
}
