use tauri::{AppHandle, Manager, Runtime, Window};
use url::Url;
use std::sync::Arc;

/// Handler for custom tauri:// protocol
pub fn handle_tauri_protocol<R: Runtime>(
    app_handle: &AppHandle<R>,
    request: &tauri::http::Request,
) -> Result<tauri::http::Response, Box<dyn std::error::Error>> {
    // Parse the URL
    let url = request.uri().to_string();
    let parsed_url = Url::parse(&url)?;
    
    // Get the path segments
    let path = parsed_url.path();
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    
    if segments.is_empty() {
        return Err("Invalid URL".into());
    }
    
    // Get the current window
    let window = app_handle.get_window("main")
        .or_else(|| app_handle.get_window("quiz-taking"))
        .or_else(|| app_handle.get_window("quiz-results"))
        .ok_or("No window found")?;
    
    // Handle different commands
    match segments[0] {
        "select-option" => {
            if segments.len() >= 3 {
                let option_id = segments[1].to_string();
                let question_index = segments[2].parse::<usize>()?;
                
                // Call the Rust function to handle option selection
                tauri::async_runtime::block_on(async {
                    let state = app_handle.state::<crate::app_state::AppState>();
                    crate::quiz::taking_controller::select_quiz_option(state, option_id, question_index).await
                        .map_err(|e| e.into())
                })?;
            } else {
                return Err("Invalid select-option URL".into());
            }
        },
        "navigate-question" => {
            if segments.len() >= 3 {
                let direction = segments[1].to_string();
                let current_index = segments[2].parse::<usize>()?;
                
                // Call the Rust function to handle navigation
                tauri::async_runtime::block_on(async {
                    let state = app_handle.state::<crate::app_state::AppState>();
                    crate::quiz::taking_controller::navigate_quiz_question(state, direction, current_index).await
                        .map_err(|e| e.into())
                })?;
            } else {
                return Err("Invalid navigate-question URL".into());
            }
        },
        "complete-quiz" => {
            // Call the Rust function to complete the quiz
            tauri::async_runtime::block_on(async {
                let state = app_handle.state::<crate::app_state::AppState>();
                crate::quiz::taking_controller::complete_quiz_taking(state).await
                    .map_err(|e| e.into())
            })?;
        },
        "toggle-question-details" => {
            // Toggle question details visibility
            window.eval(
                "document.querySelectorAll('.question-item-details').forEach(details => {\
                    details.style.display = details.style.display === 'none' ? 'block' : 'none';\
                });\
                const reviewButton = document.getElementById('review-button');\
                reviewButton.textContent = reviewButton.textContent === 'Review All Questions' ? 'Hide Details' : 'Review All Questions';"
            )?;
        },
        "finish-quiz" => {
            // Navigate back to the main window
            window.eval("window.location.href = 'index.html';")?;
        },
        _ => return Err(format!("Unknown command: {}", segments[0]).into()),
    }
    
    // Return a success response
    Ok(tauri::http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body("Command processed successfully".into())?)
}
