use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tauri::{State, Window};
use uuid::Uuid;

use crate::app_state::AppState;

/// Represents a quiz attempt in progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: AttemptStatus,
    pub score: Option<f64>,
    pub answers: Vec<QuestionAnswer>,
    pub time_spent: i64, // in seconds
}

/// Represents the status of a quiz attempt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttemptStatus {
    InProgress,
    Completed,
    Abandoned,
}

/// Represents an answer to a question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer {
    pub question_id: String,
    pub answer_id: Option<String>,
    pub is_correct: Option<bool>,
    pub time_spent: i64, // in seconds
}

/// Represents a quiz question with its options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: String,
    pub quiz_id: String,
    pub question_text: String,
    pub question_type: String,
    pub points: i32,
    pub position: i32,
    pub options: Vec<AnswerOption>,
}

/// Represents an answer option for a question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerOption {
    pub id: String,
    pub question_id: String,
    pub option_text: String,
    pub is_correct: bool,
    pub position: i32,
}

/// Represents the quiz taking state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizTakingState {
    pub quiz_id: String,
    pub quiz_title: String,
    pub quiz_description: String,
    pub time_limit: i64, // in seconds
    pub passing_score: f64,
    pub shuffle_questions: bool,
    pub current_question_index: usize,
    pub questions: Vec<QuizQuestion>,
    pub attempt: QuizAttempt,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub time_remaining: i64, // in seconds
}

/// Start a new quiz attempt
#[tauri::command]
pub async fn start_quiz(
    state: State<'_, AppState>,
    window: Window,
    quiz_id: String,
) -> Result<QuizTakingState, String> {
    // Get quiz from database
    let quiz = match sqlx::query!(
        "SELECT * FROM quizzes WHERE id = ?",
        quiz_id
    )
    .fetch_one(&state.db_pool)
    .await {
        Ok(row) => row,
        Err(e) => return Err(format!("Failed to fetch quiz: {}", e)),
    };
    
    // Get questions for this quiz
    let questions = match get_quiz_questions_with_options(&state, &quiz_id).await {
        Ok(q) => q,
        Err(e) => return Err(e),
    };
    
    // Create a new attempt
    let user_id = "test_user".to_string(); // In a real app, get from authenticated user
    let attempt_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let attempt = QuizAttempt {
        id: attempt_id,
        quiz_id: quiz_id.clone(),
        user_id,
        start_time: now,
        end_time: None,
        status: AttemptStatus::InProgress,
        score: None,
        answers: vec![],
        time_spent: 0,
    };
    
    // Save attempt to database
    match sqlx::query!(
        "INSERT INTO quiz_attempts (id, quiz_id, user_id, start_time, status, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        attempt.id,
        attempt.quiz_id,
        attempt.user_id,
        attempt.start_time,
        "in_progress",
        now,
        now
    )
    .execute(&state.db_pool)
    .await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to save attempt: {}", e)),
    };
    
    // Create quiz taking state
    let quiz_state = QuizTakingState {
        quiz_id: quiz_id.clone(),
        quiz_title: quiz.title,
        quiz_description: quiz.description.unwrap_or_default(),
        time_limit: quiz.time_limit as i64,
        passing_score: quiz.passing_score as f64,
        shuffle_questions: quiz.shuffle_questions,
        current_question_index: 0,
        questions,
        attempt,
        start_time: now,
        time_remaining: quiz.time_limit as i64,
    };
    
    // Start timer
    let window_clone = window.clone();
    let time_limit = quiz.time_limit as i64;
    std::thread::spawn(move || {
        let start = Instant::now();
        let mut remaining = time_limit;
        
        while remaining > 0 {
            std::thread::sleep(Duration::from_secs(1));
            let elapsed = start.elapsed().as_secs() as i64;
            remaining = time_limit - elapsed;
            
            // Update time remaining
            let _ = window_clone.emit("quiz-timer-update", remaining);
            
            // Check if time is up
            if remaining <= 0 {
                let _ = window_clone.emit("quiz-time-up", ());
                break;
            }
        }
    });
    
    Ok(quiz_state)
}

/// Get the current question
#[tauri::command]
pub async fn get_current_question(
    state: State<'_, AppState>,
    quiz_state: QuizTakingState,
) -> Result<QuizQuestion, String> {
    if quiz_state.current_question_index >= quiz_state.questions.len() {
        return Err("No more questions".to_string());
    }
    
    Ok(quiz_state.questions[quiz_state.current_question_index].clone())
}

/// Submit an answer for the current question
#[tauri::command]
pub async fn submit_answer(
    state: State<'_, AppState>,
    mut quiz_state: QuizTakingState,
    answer_id: Option<String>,
) -> Result<QuizTakingState, String> {
    if quiz_state.current_question_index >= quiz_state.questions.len() {
        return Err("No more questions".to_string());
    }
    
    let current_question = &quiz_state.questions[quiz_state.current_question_index];
    
    // Check if answer is correct
    let is_correct = if let Some(answer_id) = &answer_id {
        current_question.options.iter()
            .find(|option| &option.id == answer_id)
            .map(|option| option.is_correct)
            .unwrap_or(false)
    } else {
        false
    };
    
    // Create answer record
    let answer = QuestionAnswer {
        question_id: current_question.id.clone(),
        answer_id,
        is_correct: Some(is_correct),
        time_spent: 0, // In a real app, track time spent on each question
    };
    
    // Add answer to attempt
    quiz_state.attempt.answers.push(answer.clone());
    
    // Save answer to database
    let now = chrono::Utc::now();
    match sqlx::query!(
        "INSERT INTO quiz_attempt_answers (attempt_id, question_id, answer_id, is_correct, time_spent, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        quiz_state.attempt.id,
        answer.question_id,
        answer.answer_id,
        is_correct,
        answer.time_spent,
        now,
        now
    )
    .execute(&state.db_pool)
    .await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to save answer: {}", e)),
    };
    
    // Move to next question
    quiz_state.current_question_index += 1;
    
    Ok(quiz_state)
}

/// Navigate to a specific question
#[tauri::command]
pub async fn navigate_to_question(
    state: State<'_, AppState>,
    mut quiz_state: QuizTakingState,
    question_index: usize,
) -> Result<QuizTakingState, String> {
    if question_index >= quiz_state.questions.len() {
        return Err("Invalid question index".to_string());
    }
    
    quiz_state.current_question_index = question_index;
    
    Ok(quiz_state)
}

/// Complete the quiz attempt
#[tauri::command]
pub async fn complete_quiz(
    state: State<'_, AppState>,
    mut quiz_state: QuizTakingState,
) -> Result<QuizAttempt, String> {
    let now = chrono::Utc::now();
    
    // Calculate score
    let total_questions = quiz_state.questions.len() as f64;
    let correct_answers = quiz_state.attempt.answers.iter()
        .filter(|answer| answer.is_correct.unwrap_or(false))
        .count() as f64;
    
    let score = if total_questions > 0.0 {
        (correct_answers / total_questions) * 100.0
    } else {
        0.0
    };
    
    // Update attempt
    quiz_state.attempt.end_time = Some(now);
    quiz_state.attempt.status = AttemptStatus::Completed;
    quiz_state.attempt.score = Some(score);
    quiz_state.attempt.time_spent = (now - quiz_state.attempt.start_time).num_seconds();
    
    // Save to database
    match sqlx::query!(
        "UPDATE quiz_attempts SET end_time = ?, status = ?, score = ?, time_spent = ?, updated_at = ? WHERE id = ?",
        now,
        "completed",
        score,
        quiz_state.attempt.time_spent,
        now,
        quiz_state.attempt.id
    )
    .execute(&state.db_pool)
    .await {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to update attempt: {}", e)),
    };
    
    Ok(quiz_state.attempt)
}

/// Get quiz questions with answer options
async fn get_quiz_questions_with_options(
    state: &State<'_, AppState>,
    quiz_id: &str,
) -> Result<Vec<QuizQuestion>, String> {
    // Get questions
    let questions = match sqlx::query!(
        "SELECT * FROM questions WHERE quiz_id = ? ORDER BY position ASC",
        quiz_id
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(rows) => rows,
        Err(e) => return Err(format!("Failed to fetch questions: {}", e)),
    };
    
    let mut quiz_questions = Vec::new();
    
    for question in questions {
        // Get options for this question
        let options = match sqlx::query!(
            "SELECT * FROM answers WHERE question_id = ? ORDER BY position ASC",
            question.id
        )
        .fetch_all(&state.db_pool)
        .await {
            Ok(rows) => rows,
            Err(e) => return Err(format!("Failed to fetch answer options: {}", e)),
        };
        
        let answer_options = options.into_iter()
            .map(|option| AnswerOption {
                id: option.id,
                question_id: option.question_id,
                option_text: option.option_text,
                is_correct: option.is_correct,
                position: option.position,
            })
            .collect();
        
        quiz_questions.push(QuizQuestion {
            id: question.id,
            quiz_id: question.quiz_id,
            question_text: question.question_text,
            question_type: question.question_type,
            points: question.points,
            position: question.position,
            options: answer_options,
        });
    }
    
    Ok(quiz_questions)
}

/// Get quiz results
#[tauri::command]
pub async fn get_quiz_results(
    state: State<'_, AppState>,
    attempt_id: String,
) -> Result<serde_json::Value, String> {
    // Get attempt
    let attempt = match sqlx::query!(
        "SELECT * FROM quiz_attempts WHERE id = ?",
        attempt_id
    )
    .fetch_one(&state.db_pool)
    .await {
        Ok(row) => row,
        Err(e) => return Err(format!("Failed to fetch attempt: {}", e)),
    };
    
    // Get quiz
    let quiz = match sqlx::query!(
        "SELECT * FROM quizzes WHERE id = ?",
        attempt.quiz_id
    )
    .fetch_one(&state.db_pool)
    .await {
        Ok(row) => row,
        Err(e) => return Err(format!("Failed to fetch quiz: {}", e)),
    };
    
    // Get answers
    let answers = match sqlx::query!(
        "SELECT * FROM quiz_attempt_answers WHERE attempt_id = ?",
        attempt_id
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(rows) => rows,
        Err(e) => return Err(format!("Failed to fetch answers: {}", e)),
    };
    
    // Get questions
    let questions = match sqlx::query!(
        "SELECT * FROM questions WHERE quiz_id = ? ORDER BY position ASC",
        attempt.quiz_id
    )
    .fetch_all(&state.db_pool)
    .await {
        Ok(rows) => rows,
        Err(e) => return Err(format!("Failed to fetch questions: {}", e)),
    };
    
    // Build question details
    let mut question_details = Vec::new();
    
    for question in questions {
        // Get options for this question
        let options = match sqlx::query!(
            "SELECT * FROM answers WHERE question_id = ? ORDER BY position ASC",
            question.id
        )
        .fetch_all(&state.db_pool)
        .await {
            Ok(rows) => rows,
            Err(e) => return Err(format!("Failed to fetch answer options: {}", e)),
        };
        
        // Find user's answer for this question
        let user_answer = answers.iter().find(|a| a.question_id == question.id);
        
        // Find correct answer
        let correct_option = options.iter().find(|o| o.is_correct);
        
        // Build question detail
        let question_detail = serde_json::json!({
            "id": question.id,
            "text": question.question_text,
            "position": question.position,
            "points": question.points,
            "userAnswer": user_answer.map(|a| {
                let option = options.iter().find(|o| Some(&o.id) == a.answer_id.as_ref());
                serde_json::json!({
                    "id": a.answer_id,
                    "text": option.map(|o| &o.option_text),
                    "isCorrect": a.is_correct
                })
            }),
            "correctAnswer": correct_option.map(|o| {
                serde_json::json!({
                    "id": o.id,
                    "text": o.option_text
                })
            })
        });
        
        question_details.push(question_detail);
    }
    
    // Calculate time taken
    let time_taken = attempt.time_spent.unwrap_or(0);
    let minutes = time_taken / 60;
    let seconds = time_taken % 60;
    
    // Build results
    let results = serde_json::json!({
        "quiz": {
            "id": quiz.id,
            "title": quiz.title,
            "description": quiz.description,
            "passingScore": quiz.passing_score
        },
        "attempt": {
            "id": attempt.id,
            "startTime": attempt.start_time,
            "endTime": attempt.end_time,
            "status": attempt.status,
            "score": attempt.score,
            "timeTaken": format!("{}:{:02}", minutes, seconds)
        },
        "stats": {
            "totalQuestions": questions.len(),
            "correctAnswers": answers.iter().filter(|a| a.is_correct.unwrap_or(false)).count(),
            "incorrectAnswers": answers.iter().filter(|a| !a.is_correct.unwrap_or(true)).count(),
            "unanswered": questions.len() - answers.len()
        },
        "questions": question_details,
        "passed": attempt.score.unwrap_or(0.0) >= quiz.passing_score as f64
    });
    
    Ok(results)
}
