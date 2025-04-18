use super::QuizEngine;
use super::adaptive_learning::{LearningPathNodeType, EdgeConditionType};
use super::models::{StudyMode, QuizVisibility};
use uuid::Uuid;
use tauri::State;
use serde_json;

#[tauri::command]
pub async fn create_learning_path(
    title: String,
    description: Option<String>,
    author_id: Option<String>,
    subject: String,
    tags: Vec<String>,
    default_study_mode: String,
    default_visibility: String,
    is_public: bool,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let author_uuid = if let Some(id) = author_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };
    
    let study_mode = match default_study_mode.as_str() {
        "flashcards" => StudyMode::Flashcards,
        "multiple_choice" => StudyMode::MultipleChoice,
        "written" => StudyMode::Written,
        "mixed" => StudyMode::Mixed,
        _ => StudyMode::MultipleChoice,
    };
    
    let visibility = match default_visibility.as_str() {
        "public" => QuizVisibility::Public,
        "private" => QuizVisibility::Private,
        "unlisted" => QuizVisibility::Unlisted,
        _ => QuizVisibility::Private,
    };
    
    let path = engine.create_learning_path(
        title,
        description,
        author_uuid,
        subject,
        tags,
        study_mode,
        visibility,
        is_public,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_learning_path_node(
    path_id: String,
    title: String,
    description: Option<String>,
    node_type: String,
    content_id: Option<String>,
    position_x: f32,
    position_y: f32,
    required_score: Option<f32>,
    custom_data: Option<serde_json::Value>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    
    let content_uuid = if let Some(id) = content_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };
    
    let node_type = match node_type.as_str() {
        "Quiz" => LearningPathNodeType::Quiz,
        "Assessment" => LearningPathNodeType::Assessment,
        "Content" => LearningPathNodeType::Content,
        "Checkpoint" => LearningPathNodeType::Checkpoint,
        "Start" => LearningPathNodeType::Start,
        "End" => LearningPathNodeType::End,
        _ => LearningPathNodeType::Custom,
    };
    
    let node = engine.add_learning_path_node(
        path_uuid,
        title,
        description,
        node_type,
        content_uuid,
        position_x,
        position_y,
        required_score,
        custom_data,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(node).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_learning_path_edge(
    path_id: String,
    source_node_id: String,
    target_node_id: String,
    condition_type: String,
    condition_value: Option<serde_json::Value>,
    label: Option<String>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    let source_node_uuid = Uuid::parse_str(&source_node_id).map_err(|e| e.to_string())?;
    let target_node_uuid = Uuid::parse_str(&target_node_id).map_err(|e| e.to_string())?;
    
    let condition_type = match condition_type.as_str() {
        "Score" => EdgeConditionType::Score,
        "Completion" => EdgeConditionType::Completion,
        "Time" => EdgeConditionType::Time,
        _ => EdgeConditionType::Custom,
    };
    
    let edge = engine.add_learning_path_edge(
        path_uuid,
        source_node_uuid,
        target_node_uuid,
        condition_type,
        condition_value,
        label,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(edge).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_learning_path(
    path_id: String,
    title: Option<String>,
    description: Option<String>,
    subject: Option<String>,
    tags: Option<Vec<String>>,
    default_study_mode: Option<String>,
    default_visibility: Option<String>,
    is_public: Option<bool>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    
    let study_mode = if let Some(mode) = default_study_mode {
        Some(match mode.as_str() {
            "flashcards" => StudyMode::Flashcards,
            "multiple_choice" => StudyMode::MultipleChoice,
            "written" => StudyMode::Written,
            "mixed" => StudyMode::Mixed,
            _ => StudyMode::MultipleChoice,
        })
    } else {
        None
    };
    
    let visibility = if let Some(vis) = default_visibility {
        Some(match vis.as_str() {
            "public" => QuizVisibility::Public,
            "private" => QuizVisibility::Private,
            "unlisted" => QuizVisibility::Unlisted,
            _ => QuizVisibility::Private,
        })
    } else {
        None
    };
    
    let path = engine.update_learning_path(
        path_uuid,
        title,
        description,
        subject,
        tags,
        study_mode,
        visibility,
        is_public,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_learning_path_node(
    node_id: String,
    title: Option<String>,
    description: Option<String>,
    node_type: Option<String>,
    content_id: Option<String>,
    position_x: Option<f32>,
    position_y: Option<f32>,
    required_score: Option<f32>,
    custom_data: Option<serde_json::Value>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let node_uuid = Uuid::parse_str(&node_id).map_err(|e| e.to_string())?;
    
    let content_uuid = if let Some(id) = content_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };
    
    let node_type = if let Some(nt) = node_type {
        Some(match nt.as_str() {
            "Quiz" => LearningPathNodeType::Quiz,
            "Assessment" => LearningPathNodeType::Assessment,
            "Content" => LearningPathNodeType::Content,
            "Checkpoint" => LearningPathNodeType::Checkpoint,
            "Start" => LearningPathNodeType::Start,
            "End" => LearningPathNodeType::End,
            _ => LearningPathNodeType::Custom,
        })
    } else {
        None
    };
    
    let node = engine.update_learning_path_node(
        node_uuid,
        title,
        description,
        node_type,
        content_uuid,
        position_x,
        position_y,
        required_score,
        custom_data,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(node).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_learning_path_edge(
    edge_id: String,
    condition_type: Option<String>,
    condition_value: Option<serde_json::Value>,
    label: Option<String>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let edge_uuid = Uuid::parse_str(&edge_id).map_err(|e| e.to_string())?;
    
    let condition_type = if let Some(ct) = condition_type {
        Some(match ct.as_str() {
            "Score" => EdgeConditionType::Score,
            "Completion" => EdgeConditionType::Completion,
            "Time" => EdgeConditionType::Time,
            _ => EdgeConditionType::Custom,
        })
    } else {
        None
    };
    
    let edge = engine.update_learning_path_edge(
        edge_uuid,
        condition_type,
        condition_value,
        label,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(edge).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_learning_path(
    path_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    
    engine.delete_learning_path(path_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_learning_path_node(
    node_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let node_uuid = Uuid::parse_str(&node_id).map_err(|e| e.to_string())?;
    
    engine.delete_learning_path_node(node_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_learning_path_edge(
    edge_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let edge_uuid = Uuid::parse_str(&edge_id).map_err(|e| e.to_string())?;
    
    engine.delete_learning_path_edge(edge_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_learning_path(
    path_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    
    let path = engine.get_learning_path(path_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_learning_paths(
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let paths = engine.get_all_learning_paths(limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = paths.into_iter()
        .map(|p| serde_json::to_value(p))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn get_public_learning_paths(
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let paths = engine.get_public_learning_paths(limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = paths.into_iter()
        .map(|p| serde_json::to_value(p))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn search_learning_paths(
    query: String,
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let paths = engine.search_learning_paths(&query, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = paths.into_iter()
        .map(|p| serde_json::to_value(p))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn get_learning_paths_by_author(
    author_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let author_uuid = Uuid::parse_str(&author_id).map_err(|e| e.to_string())?;
    
    let paths = engine.get_learning_paths_by_author(author_uuid, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = paths.into_iter()
        .map(|p| serde_json::to_value(p))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn start_learning_path(
    user_id: String,
    path_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    
    let progress = engine.start_learning_path(user_uuid, path_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(progress).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user_learning_path_progress(
    user_id: String,
    path_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    
    let progress = engine.get_user_learning_path_progress(user_uuid, path_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(progress).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_user_learning_path_progress(
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let progress_list = engine.get_all_user_learning_path_progress(user_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = progress_list.into_iter()
        .map(|p| serde_json::to_value(p))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn complete_learning_path_node(
    user_id: String,
    path_id: String,
    node_id: String,
    score: Option<f32>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    let node_uuid = Uuid::parse_str(&node_id).map_err(|e| e.to_string())?;
    
    let progress = engine.complete_learning_path_node(user_uuid, path_uuid, node_uuid, score)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(progress).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn move_to_next_learning_path_node(
    user_id: String,
    path_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    let path_uuid = Uuid::parse_str(&path_id).map_err(|e| e.to_string())?;
    
    let (progress, node) = engine.move_to_next_learning_path_node(user_uuid, path_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = serde_json::json!({
        "progress": progress,
        "node": node
    });
    
    Ok(result)
}

#[tauri::command]
pub async fn get_learning_path_recommendations(
    user_id: String,
    limit: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let recommendations = engine.get_learning_path_recommendations(user_uuid, limit)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = recommendations.into_iter()
        .map(|r| serde_json::to_value(r))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn generate_learning_path_recommendations(
    user_id: String,
    limit: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let recommendations = engine.generate_learning_path_recommendations(user_uuid, limit)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = recommendations.into_iter()
        .map(|r| serde_json::to_value(r))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}
