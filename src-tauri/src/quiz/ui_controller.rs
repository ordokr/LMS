use tauri::{AppHandle, Manager, Window, WindowEvent};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::app_state::AppState;
use crate::quiz::models::{Quiz, QuizData};
use crate::models::network::{ConnectionStatus, SyncStatus};
use std::collections::HashMap;
use web_view::{Content, WebView};
use html_builder::{Html5, Node, Doctype, Buffer};
use std::fs;
use std::path::Path;

/// Represents the UI state for the quiz application
#[derive(Debug, Clone)]
pub struct UiState {
    pub is_online: bool,
    pub sync_pending_count: usize,
    pub quizzes: Vec<Quiz>,
    pub current_quiz_id: Option<String>,
    pub modal_visible: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            is_online: true,
            sync_pending_count: 0,
            quizzes: Vec::new(),
            current_quiz_id: None,
            modal_visible: false,
        }
    }
}

/// UI Controller for the quiz application
pub struct UiController {
    state: Arc<Mutex<UiState>>,
    app_handle: Option<AppHandle>,
    main_window: Option<Window>,
}

impl UiController {
    /// Create a new UI controller
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(UiState::default())),
            app_handle: None,
            main_window: None,
        }
    }

    /// Initialize the UI controller with the app handle
    pub fn initialize(&mut self, app_handle: AppHandle, window: Window) {
        self.app_handle = Some(app_handle);
        self.main_window = Some(window);
        
        // Set up event listeners
        if let Some(window) = &self.main_window {
            let state_clone = self.state.clone();
            let window_clone = window.clone();
            
            // Listen for window events
            window.listen("tauri://close-requested", move |_| {
                // Save state or perform cleanup if needed
                println!("Window close requested");
            });
            
            // Set up button click handlers
            self.setup_button_handlers(window);
        }
    }

    /// Set up button click handlers
    fn setup_button_handlers(&self, window: &Window) {
        let window_clone = window.clone();
        let state_clone = self.state.clone();
        
        // Create quiz button
        window.listen("createQuizButton", move |_| {
            let mut state = state_clone.lock().unwrap();
            state.modal_visible = true;
            window_clone.emit("update_ui", ()).unwrap();
        });
        
        // Close modal button
        let window_clone = window.clone();
        let state_clone = self.state.clone();
        window.listen("closeCreateModal", move |_| {
            let mut state = state_clone.lock().unwrap();
            state.modal_visible = false;
            window_clone.emit("update_ui", ()).unwrap();
        });
        
        // Sync button
        let window_clone = window.clone();
        let state_clone = self.state.clone();
        window.listen("syncButton", move |_| {
            let state = state_clone.lock().unwrap();
            if state.is_online {
                // Trigger sync
                window_clone.emit("sync_now", ()).unwrap();
            }
        });
        
        // Offline toggle button
        let window_clone = window.clone();
        let state_clone = self.state.clone();
        window.listen("offlineToggle", move |_| {
            let mut state = state_clone.lock().unwrap();
            state.is_online = !state.is_online;
            window_clone.emit("toggle_offline_mode", state.is_online).unwrap();
            window_clone.emit("update_ui", ()).unwrap();
        });
    }

    /// Update the UI state
    pub fn update_state(&self, update_fn: impl FnOnce(&mut UiState)) {
        if let Ok(mut state) = self.state.lock() {
            update_fn(&mut state);
            
            // Update the UI
            if let Some(window) = &self.main_window {
                window.emit("update_ui", ()).unwrap();
            }
        }
    }

    /// Load quizzes and update the UI
    pub async fn load_quizzes(&self, app_state: tauri::State<'_, AppState>) -> Result<(), String> {
        // Get quizzes from the database
        let quizzes = match sqlx::query_as!(
            Quiz,
            "SELECT * FROM quizzes ORDER BY created_at DESC"
        )
        .fetch_all(&app_state.db_pool)
        .await {
            Ok(quizzes) => quizzes,
            Err(e) => return Err(format!("Failed to load quizzes: {}", e)),
        };
        
        // Update the state
        self.update_state(|state| {
            state.quizzes = quizzes;
        });
        
        Ok(())
    }

    /// Create a new quiz
    pub async fn create_quiz(&self, app_state: tauri::State<'_, AppState>, quiz_data: QuizData) -> Result<Quiz, String> {
        // Create the quiz in the database
        let quiz = crate::quiz::commands::create_quiz(app_state, quiz_data).await?;
        
        // Update the state
        self.update_state(|state| {
            state.quizzes.push(quiz.clone());
            state.modal_visible = false;
        });
        
        Ok(quiz)
    }

    /// Start a quiz
    pub async fn start_quiz(&self, app_state: tauri::State<'_, AppState>, quiz_id: String) -> Result<(), String> {
        // Get the quiz
        let quiz = crate::quiz::commands::get_quiz_by_id(app_state.clone(), quiz_id.clone()).await?;
        
        // Update the state
        self.update_state(|state| {
            state.current_quiz_id = Some(quiz_id.clone());
        });
        
        // Open the quiz taking window
        if let Some(app_handle) = &self.app_handle {
            let quiz_taking_window = tauri::WindowBuilder::new(
                app_handle,
                "quiz-taking",
                tauri::WindowUrl::App("quiz-taking.html".into())
            )
            .title(format!("Ordo Quiz - {}", quiz.title))
            .inner_size(800.0, 600.0)
            .build()
            .map_err(|e| format!("Failed to open quiz taking window: {}", e))?;
            
            // Pass the quiz ID to the window
            quiz_taking_window.emit("quiz_id", quiz_id).unwrap();
        }
        
        Ok(())
    }

    /// Toggle offline mode
    pub async fn toggle_offline_mode(&self, app_state: tauri::State<'_, AppState>) -> Result<bool, String> {
        let is_online = {
            let state = self.state.lock().unwrap();
            !state.is_online
        };
        
        // Update the connection status
        let status = if is_online {
            ConnectionStatus::Online
        } else {
            ConnectionStatus::Offline
        };
        
        // Update the app state
        crate::quiz::commands::toggle_offline_mode(app_state, status).await?;
        
        // Update the UI state
        self.update_state(|state| {
            state.is_online = is_online;
        });
        
        Ok(is_online)
    }

    /// Sync now
    pub async fn sync_now(&self, app_state: tauri::State<'_, AppState>) -> Result<String, String> {
        let is_online = {
            let state = self.state.lock().unwrap();
            state.is_online
        };
        
        if !is_online {
            return Err("Cannot sync while offline".to_string());
        }
        
        // Trigger sync
        let result = crate::quiz::commands::sync_now(app_state.clone()).await?;
        
        // Reload quizzes
        self.load_quizzes(app_state).await?;
        
        Ok(result)
    }

    /// Update the sync status
    pub async fn update_sync_status(&self, app_state: tauri::State<'_, AppState>) -> Result<(), String> {
        // Get the sync status
        let status = crate::quiz::commands::get_sync_status(app_state).await?;
        
        // Update the UI state
        self.update_state(|state| {
            state.is_online = status.0;
            state.sync_pending_count = status.1;
        });
        
        Ok(())
    }

    /// Render the main UI
    pub fn render_main_ui(&self) -> String {
        let state = self.state.lock().unwrap();
        
        // Create HTML builder
        let mut html = Buffer::new();
        html.doctype();
        
        html.html().attr("lang", "en").inner(|html| {
            html.head().inner(|head| {
                head.meta().attr("charset", "UTF-8");
                head.meta().attr("name", "viewport").attr("content", "width=device-width, initial-scale=1.0");
                head.title().text("Ordo Quiz Module");
                
                // Include CSS
                head.style().inner(|style| {
                    style.text(include_str!("../../styles/main.css"));
                });
            });
            
            html.body().inner(|body| {
                // Header
                body.header().inner(|header| {
                    header.h1().text("Ordo Quiz Module");
                    
                    // Status bar
                    header.div().attr("class", "status-bar").inner(|div| {
                        // Status indicator
                        let status_class = if state.is_online { "status-indicator online" } else { "status-indicator offline" };
                        let status_text = if state.is_online { "Online" } else { "Offline" };
                        let status_text = if state.sync_pending_count > 0 {
                            format!("{} ({} pending sync items)", status_text, state.sync_pending_count)
                        } else {
                            status_text.to_string()
                        };
                        
                        div.div().attr("class", status_class).attr("id", "statusIndicator").text(&status_text);
                        
                        // Sync controls
                        div.div().attr("class", "sync-controls").inner(|div| {
                            let sync_disabled = if !state.is_online { " disabled" } else { "" };
                            let sync_title = if !state.is_online { "Cannot sync while offline" } else { "" };
                            
                            div.button().attr("class", "sync-button").attr("id", "syncButton")
                                .attr("disabled", sync_disabled).attr("title", sync_title)
                                .text("Sync Now");
                            
                            let toggle_class = if state.is_online { "offline-toggle" } else { "offline-toggle active" };
                            let toggle_text = if state.is_online { "Go Offline" } else { "Go Online" };
                            
                            div.button().attr("class", toggle_class).attr("id", "offlineToggle")
                                .text(toggle_text);
                        });
                    });
                });
                
                // Container
                body.div().attr("class", "container").inner(|div| {
                    // Create quiz button
                    div.div().attr("class", "create-quiz").inner(|div| {
                        div.button().attr("class", "create-button").attr("id", "createQuizButton")
                            .text("Create New Quiz");
                    });
                    
                    // Quiz list
                    div.div().attr("class", "quiz-list").attr("id", "quizList").inner(|div| {
                        // Add quiz cards
                        for quiz in &state.quizzes {
                            div.div().attr("class", "quiz-card").inner(|card| {
                                card.div().attr("class", "quiz-title").text(&quiz.title);
                                
                                if let Some(description) = &quiz.description {
                                    card.div().attr("class", "quiz-description").text(description);
                                }
                                
                                card.div().attr("class", "quiz-meta").inner(|meta| {
                                    meta.span().text(&format!("{} minutes", quiz.time_limit / 60));
                                    meta.span().text(&format!("{}% passing", quiz.passing_score));
                                });
                                
                                card.div().attr("style", "margin-top: 15px;").inner(|div| {
                                    div.button().attr("class", "quiz-button")
                                        .attr("data-quiz-id", &quiz.id.to_string())
                                        .text("Start Quiz");
                                });
                            });
                        }
                    });
                });
                
                // Create Quiz Modal
                let modal_display = if state.modal_visible { "flex" } else { "none" };
                body.div().attr("class", "quiz-modal").attr("id", "createQuizModal")
                    .attr("style", &format!("display: {}", modal_display))
                    .inner(|div| {
                        div.div().attr("class", "modal-content").inner(|content| {
                            // Modal header
                            content.div().attr("class", "modal-header").inner(|header| {
                                header.h2().text("Create New Quiz");
                                header.button().attr("class", "close-button").attr("id", "closeCreateModal")
                                    .text("×");
                            });
                            
                            // Form
                            content.form().attr("id", "createQuizForm").inner(|form| {
                                // Title
                                form.div().attr("class", "form-group").inner(|group| {
                                    group.label().attr("for", "quizTitle").text("Quiz Title");
                                    group.input().attr("type", "text").attr("id", "quizTitle")
                                        .attr("name", "title").attr("required", "");
                                });
                                
                                // Description
                                form.div().attr("class", "form-group").inner(|group| {
                                    group.label().attr("for", "quizDescription").text("Description");
                                    group.textarea().attr("id", "quizDescription").attr("name", "description")
                                        .attr("rows", "3");
                                });
                                
                                // Time limit
                                form.div().attr("class", "form-group").inner(|group| {
                                    group.label().attr("for", "timeLimit").text("Time Limit (minutes)");
                                    group.input().attr("type", "number").attr("id", "timeLimit")
                                        .attr("name", "timeLimit").attr("min", "1").attr("value", "10");
                                });
                                
                                // Passing score
                                form.div().attr("class", "form-group").inner(|group| {
                                    group.label().attr("for", "passingScore").text("Passing Score (%)");
                                    group.input().attr("type", "number").attr("id", "passingScore")
                                        .attr("name", "passingScore").attr("min", "0").attr("max", "100")
                                        .attr("value", "70");
                                });
                                
                                // Shuffle questions
                                form.div().attr("class", "form-group").inner(|group| {
                                    group.label().attr("for", "shuffleQuestions").text("Shuffle Questions");
                                    group.select().attr("id", "shuffleQuestions").attr("name", "shuffleQuestions")
                                        .inner(|select| {
                                            select.option().attr("value", "false").text("No");
                                            select.option().attr("value", "true").text("Yes");
                                        });
                                });
                                
                                // Show results
                                form.div().attr("class", "form-group").inner(|group| {
                                    group.label().attr("for", "showResults").text("Show Results After Completion");
                                    group.select().attr("id", "showResults").attr("name", "showResults")
                                        .inner(|select| {
                                            select.option().attr("value", "true").text("Yes");
                                            select.option().attr("value", "false").text("No");
                                        });
                                });
                                
                                // Submit button
                                form.button().attr("type", "submit").attr("class", "submit-button")
                                    .text("Create Quiz");
                            });
                        });
                    });
            });
        });
        
        html.finish()
    }
}

/// Tauri command to initialize the UI
#[tauri::command]
pub async fn initialize_ui(
    app_handle: AppHandle,
    window: Window,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let mut controller = ui_controller.lock().unwrap();
    
    // Initialize the controller
    controller.initialize(app_handle, window);
    
    // Load quizzes
    controller.load_quizzes(state).await?;
    
    // Update sync status
    controller.update_sync_status(state).await?;
    
    Ok(())
}

/// Tauri command to create a quiz
#[tauri::command]
pub async fn create_quiz_ui(
    state: tauri::State<'_, AppState>,
    quiz_data: QuizData,
) -> Result<Quiz, String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let controller = ui_controller.lock().unwrap();
    
    // Create the quiz
    controller.create_quiz(state, quiz_data).await
}

/// Tauri command to start a quiz
#[tauri::command]
pub async fn start_quiz_ui(
    state: tauri::State<'_, AppState>,
    quiz_id: String,
) -> Result<(), String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let controller = ui_controller.lock().unwrap();
    
    // Start the quiz
    controller.start_quiz(state, quiz_id).await
}

/// Tauri command to toggle offline mode
#[tauri::command]
pub async fn toggle_offline_mode_ui(
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let controller = ui_controller.lock().unwrap();
    
    // Toggle offline mode
    controller.toggle_offline_mode(state).await
}

/// Tauri command to sync now
#[tauri::command]
pub async fn sync_now_ui(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let controller = ui_controller.lock().unwrap();
    
    // Sync now
    controller.sync_now(state).await
}

/// Tauri command to update the UI
#[tauri::command]
pub async fn update_ui(
    state: tauri::State<'_, AppState>,
    window: Window,
) -> Result<(), String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let controller = ui_controller.lock().unwrap();
    
    // Render the UI
    let html = controller.render_main_ui();
    
    // Update the window content
    window.eval(&format!("document.documentElement.innerHTML = `{}`", html))
        .map_err(|e| format!("Failed to update UI: {}", e))?;
    
    Ok(())
}

/// Tauri command to handle button clicks
#[tauri::command]
pub async fn handle_button_click(
    state: tauri::State<'_, AppState>,
    button_id: String,
    data: Option<serde_json::Value>,
) -> Result<(), String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let controller = ui_controller.lock().unwrap();
    
    match button_id.as_str() {
        "createQuizButton" => {
            controller.update_state(|state| {
                state.modal_visible = true;
            });
        },
        "closeCreateModal" => {
            controller.update_state(|state| {
                state.modal_visible = false;
            });
        },
        "syncButton" => {
            controller.sync_now(state).await?;
        },
        "offlineToggle" => {
            controller.toggle_offline_mode(state).await?;
        },
        "quiz-button" => {
            if let Some(data) = data {
                if let Some(quiz_id) = data.get("quiz_id").and_then(|v| v.as_str()) {
                    controller.start_quiz(state, quiz_id.to_string()).await?;
                }
            }
        },
        _ => return Err(format!("Unknown button ID: {}", button_id)),
    }
    
    Ok(())
}

/// Tauri command to handle form submission
#[tauri::command]
pub async fn handle_form_submit(
    state: tauri::State<'_, AppState>,
    form_id: String,
    form_data: serde_json::Value,
) -> Result<(), String> {
    // Get the UI controller
    let ui_controller = state.ui_controller.clone();
    let controller = ui_controller.lock().unwrap();
    
    match form_id.as_str() {
        "createQuizForm" => {
            // Parse form data
            let quiz_data = serde_json::from_value::<QuizData>(form_data)
                .map_err(|e| format!("Failed to parse form data: {}", e))?;
            
            // Create the quiz
            controller.create_quiz(state, quiz_data).await?;
        },
        _ => return Err(format!("Unknown form ID: {}", form_id)),
    }
    
    Ok(())
}
