use tauri::{State, command};
use std::sync::Arc;
use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::app_state::AppState;
use crate::models::quiz::{
    Quiz, QuizSummary, CreateQuizRequest, UpdateQuizRequest,
    Question, QuestionWithAnswers, CreateQuestionRequest, UpdateQuestionRequest,
    AnswerOption, CreateAnswerOptionRequest, UpdateAnswerOptionRequest,
    QuizAttempt, StartAttemptRequest, CompleteAttemptRequest, AbandonAttemptRequest,
    QuizSettings, CreateQuizSettingsRequest, UpdateQuizSettingsRequest,
};

// Quiz commands

/// Get all quizzes
#[command]
pub async fn get_quizzes(
    app_state: State<'_, Arc<AppState>>
) -> Result<Vec<QuizSummary>, String> {
    let repository = app_state.get_quiz_repository();
    
    repository.get_quiz_summaries()
        .await
        .map_err(|e| e.to_string())
}

/// Get a quiz by ID
#[command]
pub async fn get_quiz(
    id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<Quiz, String> {
    let repository = app_state.get_quiz_repository();
    
    repository.get_quiz_by_id(&id)
        .await
        .map_err(|e| e.to_string())
}

/// Create a new quiz
#[command]
pub async fn create_quiz(
    user_id: String,
    quiz: CreateQuizRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<String, String> {
    let repository = app_state.get_quiz_repository();
    
    repository.create_quiz(&user_id, quiz)
        .await
        .map_err(|e| e.to_string())
}

/// Update a quiz
#[command]
pub async fn update_quiz(
    id: String,
    quiz: UpdateQuizRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<(), String> {
    let repository = app_state.get_quiz_repository();
    
    repository.update_quiz(&id, quiz)
        .await
        .map_err(|e| e.to_string())
}

/// Delete a quiz
#[command]
pub async fn delete_quiz(
    id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<(), String> {
    let repository = app_state.get_quiz_repository();
    
    repository.delete_quiz(&id)
        .await
        .map_err(|e| e.to_string())
}

// Question commands

/// Get questions for a quiz
#[command]
pub async fn get_questions(
    quiz_id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<Vec<Question>, String> {
    let repository = app_state.get_quiz_repository();
    
    repository.get_questions_by_quiz_id(&quiz_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get a question with answers
#[command]
pub async fn get_question_with_answers(
    id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuestionWithAnswers, String> {
    let repository = app_state.get_quiz_repository();
    
    repository.get_question_with_answers(&id)
        .await
        .map_err(|e| e.to_string())
}

/// Create a new question
#[command]
pub async fn create_question(
    question: CreateQuestionRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<String, String> {
    let repository = app_state.get_quiz_repository();
    
    repository.create_question(question)
        .await
        .map_err(|e| e.to_string())
}

/// Update a question
#[command]
pub async fn update_question(
    id: String,
    question: UpdateQuestionRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<(), String> {
    let repository = app_state.get_quiz_repository();
    
    repository.update_question(&id, question)
        .await
        .map_err(|e| e.to_string())
}

/// Delete a question
#[command]
pub async fn delete_question(
    id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<(), String> {
    let repository = app_state.get_quiz_repository();
    
    repository.delete_question(&id)
        .await
        .map_err(|e| e.to_string())
}

// Quiz attempt commands

/// Start a quiz attempt
#[command]
pub async fn start_quiz_attempt(
    user_id: String,
    quiz_id: String,
    request: StartAttemptRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuizAttempt, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    quiz_service.start_quiz_attempt(&user_id, &quiz_id, request)
        .await
        .map_err(|e| e.to_string())
}

/// Complete a quiz attempt
#[command]
pub async fn complete_quiz_attempt(
    attempt_id: String,
    request: CompleteAttemptRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuizAttempt, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    quiz_service.complete_quiz_attempt(&attempt_id, request)
        .await
        .map_err(|e| e.to_string())
}

/// Abandon a quiz attempt
#[command]
pub async fn abandon_quiz_attempt(
    attempt_id: String,
    request: AbandonAttemptRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuizAttempt, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    quiz_service.abandon_quiz_attempt(&attempt_id, request)
        .await
        .map_err(|e| e.to_string())
}

// Quiz settings commands

/// Get quiz settings
#[command]
pub async fn get_quiz_settings(
    quiz_id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<QuizSettings, String> {
    let repository = app_state.get_quiz_repository();
    
    repository.get_quiz_settings(&quiz_id)
        .await
        .map_err(|e| e.to_string())
}

/// Create quiz settings
#[command]
pub async fn create_quiz_settings(
    settings: CreateQuizSettingsRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<(), String> {
    let repository = app_state.get_quiz_repository();
    
    repository.create_quiz_settings(settings)
        .await
        .map_err(|e| e.to_string())
}

/// Update quiz settings
#[command]
pub async fn update_quiz_settings(
    quiz_id: String,
    settings: UpdateQuizSettingsRequest,
    app_state: State<'_, Arc<AppState>>
) -> Result<(), String> {
    let repository = app_state.get_quiz_repository();
    
    repository.update_quiz_settings(&quiz_id, settings)
        .await
        .map_err(|e| e.to_string())
}

/// Launch the quiz module
#[command]
pub async fn launch_quiz_module(
    quiz_id: String,
    user_id: String,
    app_state: State<'_, Arc<AppState>>
) -> Result<String, String> {
    let quiz_service = app_state.get_quiz_service()?;
    
    quiz_service.launch_quiz_module(&quiz_id, &user_id)
        .await
        .map_err(|e| e.to_string())
}
