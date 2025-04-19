use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::app_state::AppState;
use crate::models::quiz::{
    Quiz, QuizSummary, CreateQuizRequest, UpdateQuizRequest,
    Question, QuestionWithAnswers, CreateQuestionRequest, UpdateQuestionRequest,
    AnswerOption, CreateAnswerOptionRequest, UpdateAnswerOptionRequest,
    QuizAttempt, StartAttemptRequest, CompleteAttemptRequest, AbandonAttemptRequest, AttemptStatus,
    QuizSettings, CreateQuizSettingsRequest, UpdateQuizSettingsRequest,
};
use crate::middleware::auth::verify_auth;

/// Create quiz routes
pub fn quiz_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Quiz routes
        .route("/", get(get_quizzes).post(create_quiz))
        .route("/:id", get(get_quiz).put(update_quiz).delete(delete_quiz))
        .route("/:id/questions", get(get_quiz_questions).post(create_question))
        .route("/:quiz_id/questions/:question_id", get(get_question).put(update_question).delete(delete_question))
        .route("/:quiz_id/questions/:question_id/answers", get(get_question_answers).post(create_answer))
        .route("/:quiz_id/questions/:question_id/answers/:answer_id", get(get_answer).put(update_answer).delete(delete_answer))

        // Quiz attempt routes
        .route("/:id/attempts", get(get_quiz_attempts).post(start_quiz_attempt))
        .route("/:quiz_id/attempts/:attempt_id", get(get_quiz_attempt).put(update_quiz_attempt).delete(abandon_quiz_attempt))
        .route("/:quiz_id/attempts/:attempt_id/complete", post(complete_quiz_attempt))

        // Quiz settings routes
        .route("/:id/settings", get(get_quiz_settings).post(create_quiz_settings).put(update_quiz_settings))

        // Quiz analytics routes
        .route("/:id/analytics", get(get_quiz_analytics))
        .route("/:id/analytics/user/:user_id", get(get_user_quiz_analytics))
}

/// Query parameters for listing quizzes
#[derive(Debug, Deserialize)]
pub struct QuizListParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub search: Option<String>,
    pub course_id: Option<String>,
    pub author_id: Option<String>,
    pub visibility: Option<String>,
    pub study_mode: Option<String>,
}

/// Get all quizzes
async fn get_quizzes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<QuizListParams>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get quizzes
    match repository.get_quiz_summaries().await {
        Ok(quizzes) => {
            (StatusCode::OK, Json(quizzes)).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quizzes",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get a quiz by ID
async fn get_quiz(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get quiz
    match repository.get_quiz_by_id(&id).await {
        Ok(quiz) => {
            (StatusCode::OK, Json(quiz)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Quiz not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Create a new quiz
async fn create_quiz(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateQuizRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Create quiz
    match quiz_service.create_quiz(request).await {
        Ok(quiz) => {
            (StatusCode::CREATED, Json(quiz)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to create quiz",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Update a quiz
async fn update_quiz(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateQuizRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Update quiz
    match quiz_service.update_quiz(&id, request).await {
        Ok(quiz) => {
            (StatusCode::OK, Json(quiz)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to update quiz",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Delete a quiz
async fn delete_quiz(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Delete quiz
    match quiz_service.delete_quiz(&id).await {
        Ok(_) => {
            (StatusCode::NO_CONTENT, Json(serde_json::json!({}))).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to delete quiz",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get questions for a quiz
async fn get_quiz_questions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get questions
    match repository.get_questions_by_quiz_id(&id).await {
        Ok(questions) => {
            (StatusCode::OK, Json(questions)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Questions not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Create a new question
async fn create_question(
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
    Json(request): Json<CreateQuestionRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Create question
    match quiz_service.create_question(&quiz_id, request).await {
        Ok(question) => {
            (StatusCode::CREATED, Json(question)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to create question",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get a question by ID
async fn get_question(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get question
    match repository.get_question_by_id(&question_id).await {
        Ok(question) => {
            // Verify question belongs to quiz
            if question.quiz_id != quiz_id {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": "Question not found in this quiz"
                }))).into_response();
            }

            (StatusCode::OK, Json(question)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Question not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Update a question
async fn update_question(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id)): Path<(String, String)>,
    Json(request): Json<UpdateQuestionRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Update question
    match quiz_service.update_question(&quiz_id, &question_id, request).await {
        Ok(question) => {
            (StatusCode::OK, Json(question)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to update question",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Delete a question
async fn delete_question(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Delete question
    match quiz_service.delete_question(&quiz_id, &question_id).await {
        Ok(_) => {
            (StatusCode::NO_CONTENT, Json(serde_json::json!({}))).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to delete question",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get answers for a question
async fn get_question_answers(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get answers
    match repository.get_answers_by_question_id(&question_id).await {
        Ok(answers) => {
            (StatusCode::OK, Json(answers)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Answers not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Create a new answer
async fn create_answer(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id)): Path<(String, String)>,
    Json(request): Json<CreateAnswerOptionRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Create answer
    match quiz_service.create_answer(&quiz_id, &question_id, request).await {
        Ok(answer) => {
            (StatusCode::CREATED, Json(answer)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to create answer",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get an answer by ID
async fn get_answer(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id, answer_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get answer
    match repository.get_answer_by_id(&answer_id).await {
        Ok(answer) => {
            // Verify answer belongs to question
            if answer.question_id != question_id {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": "Answer not found in this question"
                }))).into_response();
            }

            (StatusCode::OK, Json(answer)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Answer not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Update an answer
async fn update_answer(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id, answer_id)): Path<(String, String, String)>,
    Json(request): Json<UpdateAnswerOptionRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Update answer
    match quiz_service.update_answer(&quiz_id, &question_id, &answer_id, request).await {
        Ok(answer) => {
            (StatusCode::OK, Json(answer)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to update answer",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Delete an answer
async fn delete_answer(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, question_id, answer_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Delete answer
    match quiz_service.delete_answer(&quiz_id, &question_id, &answer_id).await {
        Ok(_) => {
            (StatusCode::NO_CONTENT, Json(serde_json::json!({}))).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to delete answer",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get quiz attempts
async fn get_quiz_attempts(
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get attempts
    match repository.get_attempts_by_quiz_id(&quiz_id).await {
        Ok(attempts) => {
            (StatusCode::OK, Json(attempts)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Attempts not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Start a quiz attempt
async fn start_quiz_attempt(
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
    Json(request): Json<StartAttemptRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Start attempt
    match quiz_service.start_quiz_attempt(&request.user_id, &quiz_id, request).await {
        Ok(attempt) => {
            (StatusCode::CREATED, Json(attempt)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to start quiz attempt",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get a quiz attempt
async fn get_quiz_attempt(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, attempt_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get attempt
    match repository.get_attempt_by_id(&attempt_id).await {
        Ok(attempt) => {
            // Verify attempt belongs to quiz
            if attempt.quiz_id != quiz_id {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({
                    "error": "Attempt not found for this quiz"
                }))).into_response();
            }

            (StatusCode::OK, Json(attempt)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Attempt not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Update a quiz attempt (submit answers)
async fn update_quiz_attempt(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, attempt_id)): Path<(String, String)>,
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Update attempt (submit answers)
    match quiz_service.submit_answers(&quiz_id, &attempt_id, request).await {
        Ok(attempt) => {
            (StatusCode::OK, Json(attempt)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to update quiz attempt",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Complete a quiz attempt
async fn complete_quiz_attempt(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, attempt_id)): Path<(String, String)>,
    Json(request): Json<CompleteAttemptRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Complete attempt
    match quiz_service.complete_quiz_attempt(&attempt_id, request).await {
        Ok(attempt) => {
            (StatusCode::OK, Json(attempt)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to complete quiz attempt",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Abandon a quiz attempt
async fn abandon_quiz_attempt(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, attempt_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Create abandon request
    let request = AbandonAttemptRequest {
        reason: Some("User abandoned attempt".to_string()),
    };

    // Abandon attempt
    match quiz_service.abandon_quiz_attempt(&attempt_id, request).await {
        Ok(_) => {
            (StatusCode::NO_CONTENT, Json(serde_json::json!({}))).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to abandon quiz attempt",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get quiz settings
async fn get_quiz_settings(
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
) -> impl IntoResponse {
    // Get quiz repository
    let repository = match state.get_quiz_repository() {
        Ok(repo) => repo,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz repository",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get settings
    match repository.get_quiz_settings(&quiz_id).await {
        Ok(settings) => {
            (StatusCode::OK, Json(settings)).into_response()
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(serde_json::json!({
                "error": "Settings not found",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Create quiz settings
async fn create_quiz_settings(
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
    Json(request): Json<CreateQuizSettingsRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Create settings
    match quiz_service.create_quiz_settings(&quiz_id, request).await {
        Ok(settings) => {
            (StatusCode::CREATED, Json(settings)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to create quiz settings",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Update quiz settings
async fn update_quiz_settings(
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
    Json(request): Json<UpdateQuizSettingsRequest>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Update settings
    match quiz_service.update_quiz_settings(&quiz_id, request).await {
        Ok(settings) => {
            (StatusCode::OK, Json(settings)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to update quiz settings",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get quiz analytics
async fn get_quiz_analytics(
    State(state): State<Arc<AppState>>,
    Path(quiz_id): Path<String>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get analytics
    match quiz_service.get_quiz_analytics(&quiz_id).await {
        Ok(analytics) => {
            (StatusCode::OK, Json(analytics)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to get quiz analytics",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

/// Get user quiz analytics
async fn get_user_quiz_analytics(
    State(state): State<Arc<AppState>>,
    Path((quiz_id, user_id)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get quiz service
    let quiz_service = match state.get_quiz_service() {
        Ok(service) => service,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to get quiz service",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // Get user analytics
    match quiz_service.get_user_quiz_analytics(&quiz_id, &user_id).await {
        Ok(analytics) => {
            (StatusCode::OK, Json(analytics)).into_response()
        }
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Failed to get user quiz analytics",
                "details": e.to_string()
            }))).into_response()
        }
    }
}
