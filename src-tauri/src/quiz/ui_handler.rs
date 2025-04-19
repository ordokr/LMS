use tauri::{AppHandle, Manager, Window};
use std::path::Path;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::app_state::AppState;
use crate::quiz::taking::{QuizTakingState, QuizQuestion, QuestionAnswer, AttemptStatus};

/// Handles navigation between different UI pages
#[tauri::command]
pub async fn navigate_to_page(
    app_handle: AppHandle,
    page: &str,
    params: Option<serde_json::Value>,
) -> Result<(), String> {
    match page {
        "quiz-taking" => {
            if let Some(params) = params {
                if let Some(quiz_id) = params.get("quiz_id").and_then(|v| v.as_str()) {
                    // Open the quiz taking page
                    let quiz_taking_path = Path::new("quiz-taking.html");
                    let url = app_handle.path_resolver()
                        .resolve_resource(quiz_taking_path)
                        .ok_or_else(|| "Failed to resolve quiz-taking.html path".to_string())?
                        .to_string_lossy()
                        .to_string();
                    
                    // Create a new window for the quiz taking page
                    let window = tauri::WindowBuilder::new(
                        &app_handle,
                        "quiz-taking",
                        tauri::WindowUrl::External(url.parse().unwrap())
                    )
                    .title("Ordo Quiz - Take Quiz")
                    .inner_size(800.0, 600.0)
                    .build()
                    .map_err(|e| format!("Failed to create quiz taking window: {}", e))?;
                    
                    // Store the quiz ID in the window's state
                    window.set_title(&format!("Ordo Quiz - Take Quiz (ID: {})", quiz_id))?;
                    
                    // Emit an event to load the quiz
                    window.emit("load-quiz", quiz_id)
                        .map_err(|e| format!("Failed to emit load-quiz event: {}", e))?;
                    
                    return Ok(());
                }
            }
            return Err("Missing quiz_id parameter".to_string());
        },
        "quiz-results" => {
            if let Some(params) = params {
                if let Some(attempt_id) = params.get("attempt_id").and_then(|v| v.as_str()) {
                    // Open the quiz results page
                    let quiz_results_path = Path::new("quiz-results.html");
                    let url = app_handle.path_resolver()
                        .resolve_resource(quiz_results_path)
                        .ok_or_else(|| "Failed to resolve quiz-results.html path".to_string())?
                        .to_string_lossy()
                        .to_string();
                    
                    // Create a new window for the quiz results page
                    let window = tauri::WindowBuilder::new(
                        &app_handle,
                        "quiz-results",
                        tauri::WindowUrl::External(url.parse().unwrap())
                    )
                    .title("Ordo Quiz - Results")
                    .inner_size(800.0, 600.0)
                    .build()
                    .map_err(|e| format!("Failed to create quiz results window: {}", e))?;
                    
                    // Store the attempt ID in the window's state
                    window.set_title(&format!("Ordo Quiz - Results (Attempt: {})", attempt_id))?;
                    
                    // Emit an event to load the results
                    window.emit("load-results", attempt_id)
                        .map_err(|e| format!("Failed to emit load-results event: {}", e))?;
                    
                    return Ok(());
                }
            }
            return Err("Missing attempt_id parameter".to_string());
        },
        "home" => {
            // Close the current window and return to the main window
            if let Some(window) = app_handle.get_window("quiz-taking") {
                window.close()?;
            }
            if let Some(window) = app_handle.get_window("quiz-results") {
                window.close()?;
            }
            return Ok(());
        },
        _ => return Err(format!("Unknown page: {}", page)),
    }
}

/// Starts a quiz and initializes the UI
#[tauri::command]
pub async fn start_quiz_ui(
    state: tauri::State<'_, AppState>,
    window: Window,
    quiz_id: String,
) -> Result<(), String> {
    // Call the start_quiz function to get the quiz state
    let quiz_state = crate::quiz::taking::start_quiz(state, window.clone(), quiz_id).await?;
    
    // Update the UI with the quiz information
    update_quiz_ui(&window, &quiz_state)?;
    
    // Load the first question
    load_question(&window, &state, quiz_state, 0).await?;
    
    Ok(())
}

/// Updates the quiz UI with the quiz information
fn update_quiz_ui(window: &Window, quiz_state: &QuizTakingState) -> Result<(), String> {
    // Update the quiz title and description
    window.eval(&format!(
        "document.getElementById('quiz-title').textContent = '{}';",
        escape_js_string(&quiz_state.quiz_title)
    )).map_err(|e| format!("Failed to update quiz title: {}", e))?;
    
    window.eval(&format!(
        "document.getElementById('quiz-description').textContent = '{}';",
        escape_js_string(&quiz_state.quiz_description)
    )).map_err(|e| format!("Failed to update quiz description: {}", e))?;
    
    // Update the total questions count
    window.eval(&format!(
        "document.getElementById('total-questions').textContent = '{}';",
        quiz_state.questions.len()
    )).map_err(|e| format!("Failed to update total questions: {}", e))?;
    
    Ok(())
}

/// Loads a question into the UI
async fn load_question(
    window: &Window,
    state: &tauri::State<'_, AppState>,
    mut quiz_state: QuizTakingState,
    question_index: usize,
) -> Result<(), String> {
    // Update the current question index
    quiz_state.current_question_index = question_index;
    
    // Get the current question
    let question = quiz_state.questions.get(question_index)
        .ok_or_else(|| "Question not found".to_string())?;
    
    // Update the current question number
    window.eval(&format!(
        "document.getElementById('current-question').textContent = '{}';",
        question_index + 1
    )).map_err(|e| format!("Failed to update current question: {}", e))?;
    
    // Update the progress bar
    let progress_percentage = ((question_index + 1) as f64 / quiz_state.questions.len() as f64) * 100.0;
    window.eval(&format!(
        "document.getElementById('progress-bar').style.width = '{}%';",
        progress_percentage
    )).map_err(|e| format!("Failed to update progress bar: {}", e))?;
    
    // Update the question text
    let question_html = format!(
        "<div class=\"question-text\"><h2>{}</h2></div>",
        escape_html(&question.question_text)
    );
    
    // Generate the options HTML
    let mut options_html = String::from("<div class=\"question-options\">");
    for (i, option) in question.options.iter().enumerate() {
        options_html.push_str(&format!(
            "<div class=\"option\" data-option-id=\"{}\">\
                <input type=\"radio\" id=\"option{}\" name=\"answer\" value=\"{}\">\
                <label for=\"option{}\">{}</label>\
            </div>",
            option.id,
            i + 1,
            option.id,
            i + 1,
            escape_html(&option.option_text)
        ));
    }
    options_html.push_str("</div>");
    
    // Update the question container
    window.eval(&format!(
        "document.getElementById('question-container').innerHTML = '{}';",
        escape_js_string(&(question_html + &options_html))
    )).map_err(|e| format!("Failed to update question container: {}", e))?;
    
    // Update the navigation buttons
    window.eval(&format!(
        "document.getElementById('prev-button').disabled = {};",
        question_index == 0
    )).map_err(|e| format!("Failed to update prev button: {}", e))?;
    
    let is_last_question = question_index == quiz_state.questions.len() - 1;
    window.eval(&format!(
        "document.getElementById('next-button').textContent = '{}';",
        if is_last_question { "Finish" } else { "Next" }
    )).map_err(|e| format!("Failed to update next button text: {}", e))?;
    
    // Add event listeners to the options
    window.eval(
        "document.querySelectorAll('.option').forEach(option => {\
            option.addEventListener('click', function() {\
                document.querySelectorAll('.option').forEach(opt => opt.classList.remove('selected'));\
                this.classList.add('selected');\
                const optionId = this.getAttribute('data-option-id');\
                window.__TAURI__.invoke('select_option', { optionId });\
            });\
        });"
    ).map_err(|e| format!("Failed to add option event listeners: {}", e))?;
    
    // Add event listeners to the navigation buttons
    window.eval(
        "document.getElementById('prev-button').addEventListener('click', function() {\
            window.__TAURI__.invoke('navigate_question', { direction: 'prev' });\
        });\
        document.getElementById('next-button').addEventListener('click', function() {\
            window.__TAURI__.invoke('navigate_question', { direction: 'next' });\
        });"
    ).map_err(|e| format!("Failed to add navigation event listeners: {}", e))?;
    
    Ok(())
}

/// Handles option selection
#[tauri::command]
pub async fn select_option(
    window: Window,
    option_id: String,
) -> Result<(), String> {
    // Store the selected option ID in the window's state
    window.set_title(&format!("Ordo Quiz - Take Quiz (Selected: {})", option_id))?;
    
    Ok(())
}

/// Handles question navigation
#[tauri::command]
pub async fn navigate_question(
    state: tauri::State<'_, AppState>,
    window: Window,
    direction: String,
) -> Result<(), String> {
    // Get the current quiz state from the window's title
    let title = window.title()?;
    let quiz_id = title
        .split("ID: ")
        .nth(1)
        .and_then(|s| s.split(")").next())
        .ok_or_else(|| "Failed to get quiz ID from window title".to_string())?;
    
    // Get the current quiz state
    let quiz_state = crate::quiz::taking::start_quiz(state.clone(), window.clone(), quiz_id.to_string()).await?;
    
    // Calculate the new question index
    let current_index = quiz_state.current_question_index;
    let new_index = match direction.as_str() {
        "prev" => {
            if current_index > 0 {
                current_index - 1
            } else {
                return Err("Already at the first question".to_string());
            }
        },
        "next" => {
            if current_index < quiz_state.questions.len() - 1 {
                current_index + 1
            } else {
                // This is the last question, complete the quiz
                let attempt = crate::quiz::taking::complete_quiz(state, quiz_state).await?;
                
                // Navigate to the results page
                let app_handle = window.app_handle();
                navigate_to_page(
                    app_handle,
                    "quiz-results",
                    Some(serde_json::json!({ "attempt_id": attempt.id })),
                ).await?;
                
                return Ok(());
            }
        },
        _ => return Err(format!("Invalid direction: {}", direction)),
    };
    
    // Load the new question
    load_question(&window, &state, quiz_state, new_index).await?;
    
    Ok(())
}

/// Loads quiz results into the UI
#[tauri::command]
pub async fn load_quiz_results(
    state: tauri::State<'_, AppState>,
    window: Window,
    attempt_id: String,
) -> Result<(), String> {
    // Get the quiz results
    let results = crate::quiz::taking::get_quiz_results(state, attempt_id).await?;
    
    // Update the quiz title and description
    let quiz = results.get("quiz").ok_or_else(|| "Quiz data not found in results".to_string())?;
    window.eval(&format!(
        "document.getElementById('quiz-title').textContent = '{}';",
        escape_js_string(quiz.get("title").and_then(|v| v.as_str()).unwrap_or(""))
    )).map_err(|e| format!("Failed to update quiz title: {}", e))?;
    
    window.eval(&format!(
        "document.getElementById('quiz-description').textContent = '{}';",
        escape_js_string(quiz.get("description").and_then(|v| v.as_str()).unwrap_or(""))
    )).map_err(|e| format!("Failed to update quiz description: {}", e))?;
    
    // Update the score
    let attempt = results.get("attempt").ok_or_else(|| "Attempt data not found in results".to_string())?;
    let score = attempt.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0);
    window.eval(&format!(
        "document.getElementById('score-value').textContent = '{}%';",
        score.round()
    )).map_err(|e| format!("Failed to update score: {}", e))?;
    
    // Update the stats
    let stats = results.get("stats").ok_or_else(|| "Stats data not found in results".to_string())?;
    let time_taken = attempt.get("timeTaken").and_then(|v| v.as_str()).unwrap_or("0:00");
    window.eval(&format!(
        "document.getElementById('time-taken').textContent = '{}';",
        time_taken
    )).map_err(|e| format!("Failed to update time taken: {}", e))?;
    
    let correct_answers = stats.get("correctAnswers").and_then(|v| v.as_u64()).unwrap_or(0);
    let total_questions = stats.get("totalQuestions").and_then(|v| v.as_u64()).unwrap_or(0);
    window.eval(&format!(
        "document.getElementById('correct-answers').textContent = '{}/{}';",
        correct_answers,
        total_questions
    )).map_err(|e| format!("Failed to update correct answers: {}", e))?;
    
    let passing_score = quiz.get("passingScore").and_then(|v| v.as_f64()).unwrap_or(0.0);
    window.eval(&format!(
        "document.getElementById('passing-score').textContent = '{}%';",
        passing_score
    )).map_err(|e| format!("Failed to update passing score: {}", e))?;
    
    // Update the results message
    let passed = results.get("passed").and_then(|v| v.as_bool()).unwrap_or(false);
    let message_html = if passed {
        "<h2>Congratulations!</h2><p>You've successfully passed the quiz.</p>"
    } else {
        "<h2>Not Passed</h2><p>You didn't reach the passing score for this quiz.</p>"
    };
    
    window.eval(&format!(
        "document.getElementById('results-message').innerHTML = '{}';",
        escape_js_string(message_html)
    )).map_err(|e| format!("Failed to update results message: {}", e))?;
    
    if !passed {
        window.eval(
            "document.getElementById('results-message').classList.add('failed');"
        ).map_err(|e| format!("Failed to add failed class: {}", e))?;
    }
    
    // Generate the question list HTML
    let questions = results.get("questions").and_then(|v| v.as_array()).unwrap_or(&Vec::new());
    let mut questions_html = String::new();
    
    for (i, question) in questions.iter().enumerate() {
        let question_text = question.get("text").and_then(|v| v.as_str()).unwrap_or("");
        let user_answer = question.get("userAnswer");
        let correct_answer = question.get("correctAnswer");
        
        let is_correct = user_answer.and_then(|a| a.get("isCorrect")).and_then(|v| v.as_bool()).unwrap_or(false);
        let status_class = if is_correct { "correct" } else { "incorrect" };
        let status_text = if is_correct { "Correct" } else { "Incorrect" };
        
        let user_answer_text = user_answer.and_then(|a| a.get("text")).and_then(|v| v.as_str()).unwrap_or("No answer");
        let correct_answer_text = correct_answer.and_then(|a| a.get("text")).and_then(|v| v.as_str()).unwrap_or("");
        
        questions_html.push_str(&format!(
            "<div class=\"question-item\">\
                <div class=\"question-item-header\">\
                    <div class=\"question-number\">{}</div>\
                    <div class=\"question-text\">{}</div>\
                    <div class=\"question-status {}\">{}</div>\
                </div>\
                <div class=\"question-item-details\">\
                    <div class=\"question-answer\">Your answer: <span class=\"answer-text\">{}</span></div>\
                    <div class=\"question-correct-answer\">Correct answer: <span class=\"answer-text\">{}</span></div>\
                </div>\
            </div>",
            i + 1,
            escape_html(question_text),
            status_class,
            status_text,
            escape_html(user_answer_text),
            escape_html(correct_answer_text)
        ));
    }
    
    window.eval(&format!(
        "document.getElementById('question-list').innerHTML = '{}';",
        escape_js_string(&questions_html)
    )).map_err(|e| format!("Failed to update question list: {}", e))?;
    
    // Add event listeners to the buttons
    window.eval(
        "document.getElementById('review-button').addEventListener('click', function() {\
            window.__TAURI__.invoke('toggle_question_details');\
        });\
        document.getElementById('finish-button').addEventListener('click', function() {\
            window.__TAURI__.invoke('navigate_to_page', { page: 'home' });\
        });"
    ).map_err(|e| format!("Failed to add button event listeners: {}", e))?;
    
    Ok(())
}

/// Toggles the visibility of question details
#[tauri::command]
pub async fn toggle_question_details(window: Window) -> Result<(), String> {
    window.eval(
        "document.querySelectorAll('.question-item-details').forEach(details => {\
            details.style.display = details.style.display === 'none' ? 'block' : 'none';\
        });\
        const reviewButton = document.getElementById('review-button');\
        reviewButton.textContent = reviewButton.textContent === 'Review All Questions' ? 'Hide Details' : 'Review All Questions';"
    ).map_err(|e| format!("Failed to toggle question details: {}", e))?;
    
    Ok(())
}

// Helper functions

/// Escapes a string for use in JavaScript
fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('\'', "\\'")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

/// Escapes a string for use in HTML
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}
