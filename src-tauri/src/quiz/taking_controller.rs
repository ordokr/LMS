use tauri::{AppHandle, Manager, Window};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::app_state::AppState;
use crate::quiz::taking::{QuizTakingState, QuizQuestion, QuestionAnswer, AttemptStatus};
use std::time::{Duration, Instant};
use uuid::Uuid;
use chrono::Utc;
use std::thread;

/// Represents the state of a quiz-taking session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizTakingSessionState {
    pub quiz_id: String,
    pub attempt_id: String,
    pub current_question_index: usize,
    pub total_questions: usize,
    pub time_remaining: i64,
    pub selected_answers: Vec<Option<String>>,
    pub is_completed: bool,
}

impl Default for QuizTakingSessionState {
    fn default() -> Self {
        Self {
            quiz_id: String::new(),
            attempt_id: String::new(),
            current_question_index: 0,
            total_questions: 0,
            time_remaining: 0,
            selected_answers: Vec::new(),
            is_completed: false,
        }
    }
}

/// Controller for quiz-taking functionality
pub struct QuizTakingController {
    state: Arc<Mutex<QuizTakingSessionState>>,
    app_handle: Option<AppHandle>,
    window: Option<Window>,
    timer_handle: Option<thread::JoinHandle<()>>,
}

impl QuizTakingController {
    /// Create a new quiz-taking controller
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(QuizTakingSessionState::default())),
            app_handle: None,
            window: None,
            timer_handle: None,
        }
    }

    /// Initialize the controller with the app handle and window
    pub fn initialize(&mut self, app_handle: AppHandle, window: Window) {
        self.app_handle = Some(app_handle);
        self.window = Some(window);
    }

    /// Start a new quiz
    pub async fn start_quiz(&mut self, app_state: tauri::State<'_, AppState>, quiz_id: String) -> Result<(), String> {
        if let Some(window) = &self.window {
            // Get the quiz from the database
            let quiz = match sqlx::query!(
                "SELECT * FROM quizzes WHERE id = ?",
                quiz_id
            )
            .fetch_one(&app_state.db_pool)
            .await {
                Ok(row) => row,
                Err(e) => return Err(format!("Failed to fetch quiz: {}", e)),
            };
            
            // Create a new attempt
            let attempt_id = Uuid::new_v4().to_string();
            let now = Utc::now();
            let user_id = "test_user".to_string(); // In a real app, get from authenticated user
            
            // Save attempt to database
            match sqlx::query!(
                "INSERT INTO quiz_attempts (id, quiz_id, user_id, start_time, status, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                attempt_id,
                quiz_id,
                user_id,
                now,
                "in_progress",
                now,
                now
            )
            .execute(&app_state.db_pool)
            .await {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to save attempt: {}", e)),
            };
            
            // Get questions for this quiz
            let questions = match self.get_quiz_questions_with_options(&app_state, &quiz_id).await {
                Ok(q) => q,
                Err(e) => return Err(e),
            };
            
            // Initialize the state
            let total_questions = questions.len();
            let time_limit = quiz.time_limit;
            
            {
                let mut state = self.state.lock().unwrap();
                state.quiz_id = quiz_id.clone();
                state.attempt_id = attempt_id.clone();
                state.current_question_index = 0;
                state.total_questions = total_questions;
                state.time_remaining = time_limit as i64;
                state.selected_answers = vec![None; total_questions];
                state.is_completed = false;
            }
            
            // Update the UI
            self.update_quiz_ui(window, &quiz.title, &quiz.description.unwrap_or_default())?;
            self.load_question(&app_state, 0).await?;
            
            // Start the timer
            self.start_timer(window.clone(), time_limit as i64);
            
            Ok(())
        } else {
            Err("Window not initialized".to_string())
        }
    }

    /// Start the quiz timer
    fn start_timer(&mut self, window: Window, time_limit: i64) {
        let state_clone = self.state.clone();
        
        // Stop any existing timer
        if let Some(handle) = self.timer_handle.take() {
            handle.join().ok();
        }
        
        // Start a new timer
        let handle = thread::spawn(move || {
            let start = Instant::now();
            let mut remaining = time_limit;
            
            while remaining > 0 {
                thread::sleep(Duration::from_secs(1));
                let elapsed = start.elapsed().as_secs() as i64;
                remaining = time_limit - elapsed;
                
                // Update the state
                {
                    let mut state = state_clone.lock().unwrap();
                    state.time_remaining = remaining;
                }
                
                // Update the UI
                let minutes = remaining / 60;
                let seconds = remaining % 60;
                let time_str = format!("{:02}:{:02}", minutes, seconds);
                
                let _ = window.eval(&format!(
                    "document.getElementById('time-remaining').textContent = '{}';",
                    time_str
                ));
                
                // Check if time is up
                if remaining <= 0 {
                    // Auto-submit the quiz
                    let _ = window.eval(
                        "alert('Time is up! Your quiz will be submitted automatically.');"
                    );
                    
                    // In a real implementation, you would submit the quiz here
                    break;
                }
            }
        });
        
        self.timer_handle = Some(handle);
    }

    /// Update the quiz UI with the quiz information
    fn update_quiz_ui(&self, window: &Window, title: &str, description: &str) -> Result<(), String> {
        // Update the quiz title and description
        window.eval(&format!(
            "document.getElementById('quiz-title').textContent = '{}';",
            escape_js_string(title)
        )).map_err(|e| format!("Failed to update quiz title: {}", e))?;
        
        window.eval(&format!(
            "document.getElementById('quiz-description').textContent = '{}';",
            escape_js_string(description)
        )).map_err(|e| format!("Failed to update quiz description: {}", e))?;
        
        // Update the total questions count
        let state = self.state.lock().unwrap();
        window.eval(&format!(
            "document.getElementById('total-questions').textContent = '{}';",
            state.total_questions
        )).map_err(|e| format!("Failed to update total questions: {}", e))?;
        
        Ok(())
    }

    /// Load a question into the UI
    pub async fn load_question(&self, app_state: &tauri::State<'_, AppState>, question_index: usize) -> Result<(), String> {
        if let Some(window) = &self.window {
            let state = self.state.lock().unwrap();
            
            if question_index >= state.total_questions {
                return Err("Invalid question index".to_string());
            }
            
            // Get the question from the database
            let questions = self.get_quiz_questions_with_options(app_state, &state.quiz_id).await?;
            let question = questions.get(question_index).ok_or("Question not found")?;
            
            // Update the current question number
            window.eval(&format!(
                "document.getElementById('current-question').textContent = '{}';",
                question_index + 1
            )).map_err(|e| format!("Failed to update current question: {}", e))?;
            
            // Update the progress bar
            let progress_percentage = ((question_index + 1) as f64 / state.total_questions as f64) * 100.0;
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
                // Check if this option is selected
                let selected = state.selected_answers[question_index].as_ref()
                    .map_or(false, |selected_id| selected_id == &option.id);
                
                let selected_class = if selected { " selected" } else { "" };
                
                options_html.push_str(&format!(
                    "<div class=\"option{0}\" data-option-id=\"{1}\" onclick=\"window.location.href='tauri://select-option/{1}/{2}'\">\
                        <input type=\"radio\" id=\"option{3}\" name=\"answer\" value=\"{1}\" {4}>\
                        <label for=\"option{3}\">{5}</label>\
                    </div>",
                    selected_class,
                    option.id,
                    question_index,
                    i + 1,
                    if selected { "checked" } else { "" },
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
            
            let is_last_question = question_index == state.total_questions - 1;
            window.eval(&format!(
                "document.getElementById('next-button').textContent = '{}';",
                if is_last_question { "Finish" } else { "Next" }
            )).map_err(|e| format!("Failed to update next button text: {}", e))?;
            
            // Add event listeners to the navigation buttons
            window.eval(&format!(
                "document.getElementById('prev-button').onclick = function() {{ window.location.href = 'tauri://navigate-question/prev/{}'; }};",
                question_index
            )).map_err(|e| format!("Failed to add prev button event listener: {}", e))?;
            
            window.eval(&format!(
                "document.getElementById('next-button').onclick = function() {{ window.location.href = 'tauri://navigate-question/next/{}'; }};",
                question_index
            )).map_err(|e| format!("Failed to add next button event listener: {}", e))?;
            
            Ok(())
        } else {
            Err("Window not initialized".to_string())
        }
    }

    /// Handle option selection
    pub async fn select_option(&self, app_state: tauri::State<'_, AppState>, option_id: String, question_index: usize) -> Result<(), String> {
        {
            let mut state = self.state.lock().unwrap();
            
            if question_index >= state.total_questions {
                return Err("Invalid question index".to_string());
            }
            
            // Update the selected answer
            state.selected_answers[question_index] = Some(option_id.clone());
        }
        
        // Reload the question to update the UI
        self.load_question(&app_state, question_index).await?;
        
        Ok(())
    }

    /// Handle question navigation
    pub async fn navigate_question(&self, app_state: tauri::State<'_, AppState>, direction: &str, current_index: usize) -> Result<(), String> {
        let state = self.state.lock().unwrap();
        
        // Calculate the new question index
        let new_index = match direction {
            "prev" => {
                if current_index > 0 {
                    current_index - 1
                } else {
                    return Err("Already at the first question".to_string());
                }
            },
            "next" => {
                if current_index < state.total_questions - 1 {
                    current_index + 1
                } else {
                    // This is the last question, ask for confirmation to complete the quiz
                    if let Some(window) = &self.window {
                        window.eval(
                            "if (confirm('Are you sure you want to finish the quiz?')) { \
                                window.location.href = 'tauri://complete-quiz'; \
                            }"
                        ).map_err(|e| format!("Failed to show confirmation dialog: {}", e))?;
                    }
                    return Ok(());
                }
            },
            _ => return Err(format!("Invalid direction: {}", direction)),
        };
        
        // Load the new question
        drop(state); // Release the lock before the async call
        self.load_question(&app_state, new_index).await?;
        
        Ok(())
    }

    /// Complete the quiz
    pub async fn complete_quiz(&self, app_state: tauri::State<'_, AppState>) -> Result<(), String> {
        let state = self.state.lock().unwrap();
        
        // Get the attempt ID
        let attempt_id = state.attempt_id.clone();
        
        // Update the attempt in the database
        let now = Utc::now();
        
        // Calculate score
        let mut correct_answers = 0;
        let total_questions = state.total_questions;
        
        // Get all questions with their correct answers
        let questions = self.get_quiz_questions_with_options(&app_state, &state.quiz_id).await?;
        
        // Check each answer
        for (i, question) in questions.iter().enumerate() {
            if let Some(selected_id) = &state.selected_answers[i] {
                // Find the selected option
                let selected_option = question.options.iter()
                    .find(|option| &option.id == selected_id);
                
                // Check if it's correct
                if let Some(option) = selected_option {
                    if option.is_correct {
                        correct_answers += 1;
                    }
                    
                    // Save the answer to the database
                    match sqlx::query!(
                        "INSERT INTO quiz_attempt_answers (attempt_id, question_id, answer_id, is_correct, time_spent, created_at, updated_at)
                         VALUES (?, ?, ?, ?, ?, ?, ?)",
                        attempt_id,
                        question.id,
                        selected_id,
                        option.is_correct,
                        0, // Time spent not tracked in this implementation
                        now,
                        now
                    )
                    .execute(&app_state.db_pool)
                    .await {
                        Ok(_) => (),
                        Err(e) => return Err(format!("Failed to save answer: {}", e)),
                    };
                }
            }
        }
        
        // Calculate score percentage
        let score = if total_questions > 0 {
            (correct_answers as f64 / total_questions as f64) * 100.0
        } else {
            0.0
        };
        
        // Update the attempt
        match sqlx::query!(
            "UPDATE quiz_attempts SET end_time = ?, status = ?, score = ?, updated_at = ? WHERE id = ?",
            now,
            "completed",
            score,
            now,
            attempt_id
        )
        .execute(&app_state.db_pool)
        .await {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to update attempt: {}", e)),
        };
        
        // Navigate to the results page
        if let Some(window) = &self.window {
            window.eval(&format!(
                "window.location.href = 'quiz-results.html?attempt_id={}';",
                attempt_id
            )).map_err(|e| format!("Failed to navigate to results page: {}", e))?;
        }
        
        Ok(())
    }

    /// Get quiz questions with answer options
    async fn get_quiz_questions_with_options(
        &self,
        state: &tauri::State<'_, AppState>,
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
                .map(|option| crate::quiz::taking::AnswerOption {
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
}

/// Tauri command to start a quiz
#[tauri::command]
pub async fn start_quiz_taking(
    app_handle: AppHandle,
    window: Window,
    state: tauri::State<'_, AppState>,
    quiz_id: String,
) -> Result<(), String> {
    // Get the quiz-taking controller
    let quiz_taking_controller = state.get_quiz_taking_controller();
    let mut controller = quiz_taking_controller.lock().unwrap();
    
    // Initialize the controller
    controller.initialize(app_handle, window);
    
    // Start the quiz
    controller.start_quiz(state, quiz_id).await
}

/// Tauri command to select an option
#[tauri::command]
pub async fn select_quiz_option(
    state: tauri::State<'_, AppState>,
    option_id: String,
    question_index: usize,
) -> Result<(), String> {
    // Get the quiz-taking controller
    let quiz_taking_controller = state.get_quiz_taking_controller();
    let controller = quiz_taking_controller.lock().unwrap();
    
    // Select the option
    controller.select_option(state, option_id, question_index).await
}

/// Tauri command to navigate between questions
#[tauri::command]
pub async fn navigate_quiz_question(
    state: tauri::State<'_, AppState>,
    direction: String,
    current_index: usize,
) -> Result<(), String> {
    // Get the quiz-taking controller
    let quiz_taking_controller = state.get_quiz_taking_controller();
    let controller = quiz_taking_controller.lock().unwrap();
    
    // Navigate to the question
    controller.navigate_question(state, &direction, current_index).await
}

/// Tauri command to complete the quiz
#[tauri::command]
pub async fn complete_quiz_taking(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Get the quiz-taking controller
    let quiz_taking_controller = state.get_quiz_taking_controller();
    let controller = quiz_taking_controller.lock().unwrap();
    
    // Complete the quiz
    controller.complete_quiz(state).await
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
