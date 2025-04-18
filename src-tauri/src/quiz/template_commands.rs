use super::QuizEngine;
use super::templates::{QuizTemplate, TemplateCategory, TemplateRating};
use super::models::{StudyMode, QuizVisibility, AnswerType};
use uuid::Uuid;
use tauri::State;
use serde_json;

#[tauri::command]
pub async fn create_template(
    title: String,
    description: Option<String>,
    author_id: Option<String>,
    category: String,
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
    
    let category = match category.as_str() {
        "Education" => TemplateCategory::Education,
        "Business" => TemplateCategory::Business,
        "Science" => TemplateCategory::Science,
        "Technology" => TemplateCategory::Technology,
        "Language" => TemplateCategory::Language,
        "Arts" => TemplateCategory::Arts,
        "Health" => TemplateCategory::Health,
        _ => TemplateCategory::Custom,
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
    
    let template = engine.create_template(
        title,
        description,
        author_uuid,
        category,
        tags,
        study_mode,
        visibility,
        is_public,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(template).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_template_from_quiz(
    quiz_id: String,
    title: String,
    description: Option<String>,
    author_id: Option<String>,
    category: String,
    tags: Vec<String>,
    is_public: bool,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let quiz_uuid = Uuid::parse_str(&quiz_id).map_err(|e| e.to_string())?;
    
    let author_uuid = if let Some(id) = author_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };
    
    let category = match category.as_str() {
        "Education" => TemplateCategory::Education,
        "Business" => TemplateCategory::Business,
        "Science" => TemplateCategory::Science,
        "Technology" => TemplateCategory::Technology,
        "Language" => TemplateCategory::Language,
        "Arts" => TemplateCategory::Arts,
        "Health" => TemplateCategory::Health,
        _ => TemplateCategory::Custom,
    };
    
    let template = engine.create_template_from_quiz(
        quiz_uuid,
        title,
        description,
        author_uuid,
        category,
        tags,
        is_public,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(template).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_question_template(
    template_id: String,
    text: String,
    description: Option<String>,
    answer_type: String,
    placeholder_text: Option<String>,
    example_answers: Vec<String>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    
    let answer_type = match answer_type.as_str() {
        "multiple_choice" => AnswerType::MultipleChoice,
        "true_false" => AnswerType::TrueFalse,
        "short_answer" => AnswerType::ShortAnswer,
        "essay" => AnswerType::Essay,
        "matching" => AnswerType::Matching,
        "ordering" => AnswerType::Ordering,
        _ => AnswerType::MultipleChoice,
    };
    
    let question_template = engine.add_question_template(
        template_uuid,
        text,
        description,
        answer_type,
        placeholder_text,
        example_answers,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(question_template).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_template(
    template_id: String,
    title: Option<String>,
    description: Option<String>,
    category: Option<String>,
    tags: Option<Vec<String>>,
    default_study_mode: Option<String>,
    default_visibility: Option<String>,
    is_public: Option<bool>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    
    let category = if let Some(cat) = category {
        Some(match cat.as_str() {
            "Education" => TemplateCategory::Education,
            "Business" => TemplateCategory::Business,
            "Science" => TemplateCategory::Science,
            "Technology" => TemplateCategory::Technology,
            "Language" => TemplateCategory::Language,
            "Arts" => TemplateCategory::Arts,
            "Health" => TemplateCategory::Health,
            _ => TemplateCategory::Custom,
        })
    } else {
        None
    };
    
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
    
    let template = engine.update_template(
        template_uuid,
        title,
        description,
        category,
        tags,
        study_mode,
        visibility,
        is_public,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    serde_json::to_value(template).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_template(
    template_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    
    engine.delete_template(template_uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_template(
    template_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    
    let template = engine.get_template(template_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(template).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_public_templates(
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let templates = engine.get_public_templates(limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = templates.into_iter()
        .map(|t| serde_json::to_value(t))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn search_templates(
    query: String,
    category: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let category = if let Some(cat) = category {
        Some(match cat.as_str() {
            "Education" => TemplateCategory::Education,
            "Business" => TemplateCategory::Business,
            "Science" => TemplateCategory::Science,
            "Technology" => TemplateCategory::Technology,
            "Language" => TemplateCategory::Language,
            "Arts" => TemplateCategory::Arts,
            "Health" => TemplateCategory::Health,
            _ => TemplateCategory::Custom,
        })
    } else {
        None
    };
    
    let templates = engine.search_templates(&query, category, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = templates.into_iter()
        .map(|t| serde_json::to_value(t))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn get_templates_by_author(
    author_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let author_uuid = Uuid::parse_str(&author_id).map_err(|e| e.to_string())?;
    
    let templates = engine.get_templates_by_author(author_uuid, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = templates.into_iter()
        .map(|t| serde_json::to_value(t))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn create_quiz_from_template(
    template_id: String,
    title: String,
    description: Option<String>,
    author_id: Option<String>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    
    let author_uuid = if let Some(id) = author_id {
        Some(Uuid::parse_str(&id).map_err(|e| e.to_string())?)
    } else {
        None
    };
    
    let quiz = engine.create_quiz_from_template(template_uuid, title, description, author_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(quiz).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rate_template(
    template_id: String,
    user_id: String,
    rating: f32,
    comment: Option<String>,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let template_rating = engine.rate_template(template_uuid, user_uuid, rating, comment)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(template_rating).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user_template_rating(
    template_id: String,
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<serde_json::Value, String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    let rating = engine.get_user_template_rating(template_uuid, user_uuid)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(rating).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_template_ratings(
    template_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
    engine: State<'_, QuizEngine>,
) -> Result<Vec<serde_json::Value>, String> {
    let template_uuid = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    
    let ratings = engine.get_template_ratings(template_uuid, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = ratings.into_iter()
        .map(|r| serde_json::to_value(r))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub async fn delete_template_rating(
    rating_id: String,
    user_id: String,
    engine: State<'_, QuizEngine>,
) -> Result<(), String> {
    let rating_uuid = Uuid::parse_str(&rating_id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    
    engine.delete_template_rating(rating_uuid, user_uuid)
        .await
        .map_err(|e| e.to_string())
}
