use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::error_alert::ErrorAlert;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SyncStatus {
    pub course_id: String,
    pub canvas_course_id: Option<String>,
    pub modules_synced: usize,
    pub modules_local_only: usize,
    pub modules_canvas_only: usize,
    pub items_synced: usize,
    pub items_local_only: usize,
    pub items_canvas_only: usize,
    pub last_sync: Option<String>,
}

#[component]
pub fn CanvasSyncStatus(
    course_id: String,
) -> impl IntoView {
    // State for sync status
    let (status, set_status) = create_signal(None::<SyncStatus>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Load sync status on mount
    create_effect(move |_| {
        load_sync_status();
    });
    
    // Function to load sync status
    let load_sync_status = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, SyncStatus>("get_canvas_sync_status", &course_id).await {
                Ok(sync_status) => {
                    set_status.set(Some(sync_status));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load sync status: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Function to sync from Canvas
    let sync_from_canvas = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, String>("sync_canvas_modules", &course_id).await {
                Ok(message) => {
                    window().alert_with_message(&message).ok();
                    load_sync_status();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to sync from Canvas: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Function to push all to Canvas
    let push_all_to_canvas = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, String>("push_all_modules_to_canvas", &course_id).await {
                Ok(message) => {
                    window().alert_with_message(&message).ok();
                    load_sync_status();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to push to Canvas: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Function to connect course to Canvas
    let connect_to_canvas = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        let canvas_course_id = window()
            .prompt_with_message("Enter Canvas Course ID:")
            .unwrap_or_else(|| Some("".to_string()))
            .filter(|s| !s.is_empty());
        
        if let Some(canvas_id) = canvas_course_id {
            spawn_local(async move {
                match invoke::<_, String>(
                    "connect_course_to_canvas",
                    &(course_id.clone(), canvas_id)
                ).await {
                    Ok(message) => {
                        window().alert_with_message(&message).ok();
                        load_sync_status();
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to connect to Canvas: {}", e)));
                        set_loading.set(false);
                    }
                }
            });
        } else {
            set_loading.set(false);
        }
    };

    view! {
        <div class="canvas-sync-status">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            {move || {
                if loading.get() && status.get().is_none() {
                    view! { <div class="loading-spinner">"Loading sync status..."</div> }
                } else if let Some(sync_status) = status.get() {
                    view! {
                        <div class="sync-status-panel">
                            <h2>"Canvas Sync Status"</h2>
                            
                            <div class="connection-status">
                                {move || {
                                    if let Some(canvas_id) = &sync_status.canvas_course_id {
                                        view! {
                                            <div class="connected-status">
                                                <span class="status-badge connected">"Connected"</span>
                                                <span class="canvas-course-id">"Canvas Course ID: " {canvas_id}</span>
                                            </div>
                                        }
                                    } else {
                                        view! {
                                            <div class="disconnected-status">
                                                <span class="status-badge disconnected">"Not Connected"</span>
                                                <button 
                                                    on:click=connect_to_canvas
                                                    class="btn btn-primary"
                                                    disabled=loading
                                                >
                                                    "Connect to Canvas Course"
                                                </button>
                                            </div>
                                        }
                                    }
                                }}
                            </div>
                            
                            {move || {
                                if sync_status.canvas_course_id.is_some() {
                                    view! {
                                        <>
                                            <div class="sync-stats">
                                                <div class="stats-section">
                                                    <h3>"Modules"</h3>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"Synced:"</span>
                                                        <span class="stat-value">{sync_status.modules_synced}</span>
                                                    </div>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"Local Only:"</span>
                                                        <span class="stat-value">{sync_status.modules_local_only}</span>
                                                    </div>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"Canvas Only:"</span>
                                                        <span class="stat-value">{sync_status.modules_canvas_only}</span>
                                                    </div>
                                                </div>
                                                
                                                <div class="stats-section">
                                                    <h3>"Module Items"</h3>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"Synced:"</span>
                                                        <span class="stat-value">{sync_status.items_synced}</span>
                                                    </div>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"Local Only:"</span>
                                                        <span class="stat-value">{sync_status.items_local_only}</span>
                                                    </div>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"Canvas Only:"</span>
                                                        <span class="stat-value">{sync_status.items_canvas_only}</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div class="sync-actions">
                                                <button 
                                                    on:click=sync_from_canvas
                                                    class="btn btn-primary"
                                                    disabled=loading
                                                >
                                                    "Pull from Canvas"
                                                </button>
                                                
                                                <button 
                                                    on:click=push_all_to_canvas
                                                    class="btn btn-secondary"
                                                    disabled=loading || sync_status.modules_local_only == 0
                                                >
                                                    "Push to Canvas"
                                                </button>
                                            </div>
                                            
                                            <div class="last-sync">
                                                {move || {
                                                    if let Some(last_sync) = &sync_status.last_sync {
                                                        view! {
                                                            <div>"Last synchronized: " {last_sync}</div>
                                                        }
                                                    } else {
                                                        view! {
                                                            <div>"Never synchronized"</div>
                                                        }
                                                    }
                                                }}
                                            </div>
                                        </>
                                    }
                                } else {
                                    view! { <></> }
                                }
                            }}
                        </div>
                    }
                } else {
                    view! { <div class="empty-state">"No sync information available"</div> }
                }
            }}
        </div>
    }
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: Serialize + ?Sized,
    R: for<'de> Deserialize<'de>,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}

// Wrapper for window interactions
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}