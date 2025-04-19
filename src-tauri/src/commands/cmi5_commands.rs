use crate::quiz::cmi5::{Cmi5Service, LaunchMode, Cmi5Score};
use crate::AppState;
use tauri::State;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cmi5CourseInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub assignable_units: Vec<Cmi5AuInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cmi5AuInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cmi5SessionInfo {
    pub id: String,
    pub actor_id: String,
    pub course_id: String,
    pub au_id: String,
    pub state: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub result: Option<Cmi5ResultInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cmi5ResultInfo {
    pub score: Option<f64>,
    pub success: Option<bool>,
    pub completion: Option<bool>,
}

/// Import a cmi5 course package
#[tauri::command]
pub async fn import_cmi5_course(
    state: State<'_, AppState>,
    package_path: String,
) -> Result<String, String> {
    let cmi5_service = state.cmi5_service.clone();
    let path = PathBuf::from(package_path);
    
    cmi5_service.import_course(&path).await
        .map_err(|e| e.to_string())
}

/// Get all cmi5 courses
#[tauri::command]
pub async fn get_cmi5_courses(
    state: State<'_, AppState>,
) -> Result<Vec<Cmi5CourseInfo>, String> {
    let cmi5_service = state.cmi5_service.clone();
    
    let courses = cmi5_service.get_all_courses().await
        .map_err(|e| e.to_string())?;
    
    let course_infos = courses.into_iter()
        .map(|course| {
            let assignable_units = course.assignable_units.iter()
                .map(|au| Cmi5AuInfo {
                    id: au.id.clone(),
                    title: au.title.clone(),
                    description: au.description.clone(),
                })
                .collect();
            
            Cmi5CourseInfo {
                id: course.id,
                title: course.title,
                description: course.description,
                assignable_units,
            }
        })
        .collect();
    
    Ok(course_infos)
}

/// Launch a cmi5 assignable unit
#[tauri::command]
pub async fn launch_cmi5_assignable_unit(
    state: State<'_, AppState>,
    course_id: String,
    au_id: String,
    actor_id: String,
) -> Result<String, String> {
    let cmi5_service = state.cmi5_service.clone();
    
    cmi5_service.launch_assignable_unit(
        &course_id,
        &au_id,
        &actor_id,
        LaunchMode::Normal,
    ).await
        .map_err(|e| e.to_string())
}

/// Get all cmi5 sessions for a user
#[tauri::command]
pub async fn get_cmi5_user_sessions(
    state: State<'_, AppState>,
    actor_id: String,
) -> Result<Vec<Cmi5SessionInfo>, String> {
    let cmi5_service = state.cmi5_service.clone();
    
    let sessions = cmi5_service.get_actor_sessions(&actor_id).await
        .map_err(|e| e.to_string())?;
    
    let session_infos = sessions.into_iter()
        .map(|session| {
            let result = session.result.map(|r| Cmi5ResultInfo {
                score: r.score.map(|s| s.scaled),
                success: r.success,
                completion: r.completion,
            });
            
            Cmi5SessionInfo {
                id: session.id,
                actor_id: session.actor_id,
                course_id: session.course_id,
                au_id: session.au_id,
                state: format!("{:?}", session.state),
                start_time: session.start_time.to_rfc3339(),
                end_time: session.end_time.map(|t| t.to_rfc3339()),
                result,
            }
        })
        .collect();
    
    Ok(session_infos)
}

/// Complete a cmi5 session
#[tauri::command]
pub async fn complete_cmi5_session(
    state: State<'_, AppState>,
    session_id: String,
    score: Option<f64>,
    success: Option<bool>,
) -> Result<(), String> {
    let cmi5_service = state.cmi5_service.clone();
    
    let cmi5_score = score.map(|s| Cmi5Score::percentage(s * 100.0));
    
    cmi5_service.complete_session(
        &session_id,
        cmi5_score,
        success,
    ).await
        .map_err(|e| e.to_string())
}

/// Abandon a cmi5 session
#[tauri::command]
pub async fn abandon_cmi5_session(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<(), String> {
    let cmi5_service = state.cmi5_service.clone();
    
    cmi5_service.abandon_session(&session_id).await
        .map_err(|e| e.to_string())
}

/// Waive a cmi5 session
#[tauri::command]
pub async fn waive_cmi5_session(
    state: State<'_, AppState>,
    session_id: String,
    reason: String,
) -> Result<(), String> {
    let cmi5_service = state.cmi5_service.clone();
    
    cmi5_service.waive_session(&session_id, &reason).await
        .map_err(|e| e.to_string())
}
