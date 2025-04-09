use leptos::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::components::error_alert::ErrorAlert;
use crate::utils::date_utils::{serialize_optional_date, deserialize_optional_date};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CourseIntegrationSettings {
    pub course_id: String,
    pub canvas_course_id: Option<String>,
    pub auto_sync_enabled: bool,
    pub sync_frequency_hours: Option<i32>,
    pub sync_modules: bool,
    pub sync_assignments: bool,
    pub sync_discussions: bool,
    pub sync_files: bool,
    pub sync_announcements: bool,
    
    #[serde(
        serialize_with = "serialize_optional_date",
        deserialize_with = "deserialize_optional_date"
    )]
    pub last_sync: Option<DateTime<Utc>>,
}

#[component]
pub fn CourseIntegrationSettings(
    course_id: String,
) -> impl IntoView {
    // State for settings
    let (settings, set_settings) = create_signal(None::<CourseIntegrationSettings>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    let (sync_in_progress, set_sync_in_progress) = create_signal(false);
    
    // Load settings on mount
    create_effect(move |_| {
        load_settings();
    });
    
    // Function to load integration settings
    let load_settings = move || {
        set_loading.set(true);
        set_error.set(None);
        set_success.set(None);
        
        spawn_local(async move {
            match invoke::<_, CourseIntegrationSettings>("get_course_integration_settings", &course_id).await {
                Ok(loaded_settings) => {
                    set_settings.set(Some(loaded_settings));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load integration settings: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Function to save integration settings
    let save_settings = move |_| {
        if let Some(current_settings) = settings.get() {
            set_loading.set(true);
            set_error.set(None);
            set_success.set(None);
            
            let settings_to_save = current_settings.clone();
            
            spawn_local(async move {
                match invoke::<_, CourseIntegrationSettings>("update_course_integration_settings", &settings_to_save).await {
                    Ok(updated_settings) => {
                        set_settings.set(Some(updated_settings));
                        set_success.set(Some("Settings saved successfully".to_string()));
                        set_loading.set(false);
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to save settings: {}", e)));
                        set_loading.set(false);
                    }
                }
            });
        }
    };
    
    // Function to connect to Canvas
    let connect_to_canvas = move |_| {
        set_loading.set(true);
        set_error.set(None);
        set_success.set(None);
        
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
                        // Reload settings after connecting
                        load_settings();
                        set_success.set(Some(message));
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
    
    // Function to disconnect from Canvas
    let disconnect_from_canvas = move |_| {
        if !window().confirm_with_message("Are you sure you want to disconnect from Canvas? This will not delete any data, but will stop synchronization.").unwrap_or(false) {
            return;
        }
        
        set_loading.set(true);
        set_error.set(None);
        set_success.set(None);
        
        spawn_local(async move {
            match invoke::<_, String>("disconnect_course_from_canvas", &course_id).await {
                Ok(message) => {
                    // Reload settings after disconnecting
                    load_settings();
                    set_success.set(Some(message));
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to disconnect from Canvas: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Function to trigger manual sync
    let trigger_sync = move |_| {
        set_sync_in_progress.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let sync_modules = settings.get().map(|s| s.sync_modules).unwrap_or(false);
        let sync_assignments = settings.get().map(|s| s.sync_assignments).unwrap_or(false);
        let sync_discussions = settings.get().map(|s| s.sync_discussions).unwrap_or(false);
        let sync_files = settings.get().map(|s| s.sync_files).unwrap_or(false);
        let sync_announcements = settings.get().map(|s| s.sync_announcements).unwrap_or(false);
        
        spawn_local(async move {
            match invoke::<_, String>("sync_course_with_canvas", &(
                course_id.clone(),
                sync_modules,
                sync_assignments,
                sync_discussions,
                sync_files,
                sync_announcements
            )).await {
                Ok(message) => {
                    // Reload settings to get updated last_sync time
                    load_settings();
                    set_success.set(Some(message));
                    set_sync_in_progress.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Sync failed: {}", e)));
                    set_sync_in_progress.set(false);
                }
            }
        });
    };
    
    // Function to format timestamp
    let format_datetime = |timestamp: Option<String>| -> String {
        timestamp
            .filter(|s| !s.is_empty())
            .map(|s| {
                // Simple format, could be improved with a proper date formatting library
                s.replace("T", " ").replace("Z", "")
            })
            .unwrap_or_else(|| "Never".to_string())
    };

    view! {
        <div class="course-integration-settings">
            <h2>"Integration Settings"</h2>
            
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            {move || success.get().map(|msg| view! { <div class="success-alert">{msg}</div> })}
            
            {move || {
                if loading.get() && settings.get().is_none() {
                    view! { <div class="loading-spinner">"Loading settings..."</div> }
                } else if let Some(s) = settings.get() {
                    view! {
                        <div class="settings-form">
                            <div class="form-section">
                                <h3>"Canvas Integration"</h3>
                                
                                <div class="connection-status">
                                    <span>"Connection Status: "</span>
                                    {if s.canvas_course_id.is_some() {
                                        view! { 
                                            <span class="connected-badge">"Connected ✓"</span>
                                        }
                                    } else {
                                        view! { 
                                            <span class="disconnected-badge">"Not Connected"</span>
                                        }
                                    }}
                                </div>
                                
                                {move || {
                                    if let Some(canvas_id) = &s.canvas_course_id {
                                        view! {
                                            <div class="connected-info">
                                                <p>"Connected to Canvas Course ID: "{canvas_id}</p>
                                                <p>"Last Synchronized: "{format_datetime(s.last_sync.clone())}</p>
                                                
                                                <div class="action-buttons">
                                                    <button
                                                        class="btn btn-primary"
                                                        on:click=trigger_sync
                                                        disabled=sync_in_progress.get() || loading.get()
                                                    >
                                                        {if sync_in_progress.get() {
                                                            "Syncing..."
                                                        } else {
                                                            "Sync Now"
                                                        }}
                                                    </button>
                                                    
                                                    <button
                                                        class="btn btn-danger"
                                                        on:click=disconnect_from_canvas
                                                        disabled=loading.get()
                                                    >
                                                        "Disconnect from Canvas"
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        view! {
                                            <div class="connect-prompt">
                                                <p>"Connect this course to Canvas to enable synchronization and integration features."</p>
                                                <button
                                                    class="btn btn-primary"
                                                    on:click=connect_to_canvas
                                                    disabled=loading.get()
                                                >
                                                    "Connect to Canvas"
                                                </button>
                                            </div>
                                        }
                                    }
                                }}
                            </div>
                            
                            {move || {
                                if s.canvas_course_id.is_some() {
                                    view! {
                                        <div class="form-section">
                                            <h3>"Synchronization Settings"</h3>
                                            
                                            <div class="form-group">
                                                <div class="checkbox-wrapper">
                                                    <input
                                                        type="checkbox"
                                                        id="auto-sync"
                                                        checked=s.auto_sync_enabled
                                                        on:change=move |ev| {
                                                            set_settings.update(|s| {
                                                                if let Some(settings) = s {
                                                                    settings.auto_sync_enabled = event_target_checked(&ev);
                                                                }
                                                            })
                                                        }
                                                    />
                                                    <label for="auto-sync">"Enable automatic synchronization"</label>
                                                </div>
                                            </div>
                                            
                                            {move || {
                                                if s.auto_sync_enabled {
                                                    view! {
                                                        <div class="form-group">
                                                            <label for="sync-frequency">"Sync frequency (hours):"</label>
                                                            <select
                                                                id="sync-frequency"
                                                                on:change=move |ev| {
                                                                    let value = event_target_value(&ev);
                                                                    let hours = value.parse::<i32>().ok();
                                                                    
                                                                    set_settings.update(|s| {
                                                                        if let Some(settings) = s {
                                                                            settings.sync_frequency_hours = hours;
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                <option 
                                                                    value="1" 
                                                                    selected=s.sync_frequency_hours == Some(1)
                                                                >"Every hour"</option>
                                                                <option 
                                                                    value="3" 
                                                                    selected=s.sync_frequency_hours == Some(3)
                                                                >"Every 3 hours"</option>
                                                                <option 
                                                                    value="6" 
                                                                    selected=s.sync_frequency_hours == Some(6)
                                                                >"Every 6 hours"</option>
                                                                <option 
                                                                    value="12" 
                                                                    selected=s.sync_frequency_hours == Some(12)
                                                                >"Every 12 hours"</option>
                                                                <option 
                                                                    value="24" 
                                                                    selected=s.sync_frequency_hours == Some(24) || s.sync_frequency_hours.is_none()
                                                                >"Daily"</option>
                                                            </select>
                                                        </div>
                                                    }
                                                } else {
                                                    view! { <></> }
                                                }
                                            }}
                                            
                                            <h4>"What to Synchronize"</h4>
                                            
                                            <div class="form-group">
                                                <div class="checkbox-wrapper">
                                                    <input
                                                        type="checkbox"
                                                        id="sync-modules"
                                                        checked=s.sync_modules
                                                        on:change=move |ev| {
                                                            set_settings.update(|s| {
                                                                if let Some(settings) = s {
                                                                    settings.sync_modules = event_target_checked(&ev);
                                                                }
                                                            })
                                                        }
                                                    />
                                                    <label for="sync-modules">"Modules and Content Structure"</label>
                                                </div>
                                            </div>
                                            
                                            <div class="form-group">
                                                <div class="checkbox-wrapper">
                                                    <input
                                                        type="checkbox"
                                                        id="sync-assignments"
                                                        checked=s.sync_assignments
                                                        on:change=move |ev| {
                                                            set_settings.update(|s| {
                                                                if let Some(settings) = s {
                                                                    settings.sync_assignments = event_target_checked(&ev);
                                                                }
                                                            })
                                                        }
                                                    />
                                                    <label for="sync-assignments">"Assignments and Quizzes"</label>
                                                </div>
                                            </div>
                                            
                                            <div class="form-group">
                                                <div class="checkbox-wrapper">
                                                    <input
                                                        type="checkbox"
                                                        id="sync-discussions"
                                                        checked=s.sync_discussions
                                                        on:change=move |ev| {
                                                            set_settings.update(|s| {
                                                                if let Some(settings) = s {
                                                                    settings.sync_discussions = event_target_checked(&ev);
                                                                }
                                                            })
                                                        }
                                                    />
                                                    <label for="sync-discussions">"Discussions"</label>
                                                </div>
                                            </div>
                                            
                                            <div class="form-group">
                                                <div class="checkbox-wrapper">
                                                    <input
                                                        type="checkbox"
                                                        id="sync-files"
                                                        checked=s.sync_files
                                                        on:change=move |ev| {
                                                            set_settings.update(|s| {
                                                                if let Some(settings) = s {
                                                                    settings.sync_files = event_target_checked(&ev);
                                                                }
                                                            })
                                                        }
                                                    />
                                                    <label for="sync-files">"Files and Resources"</label>
                                                </div>
                                            </div>
                                            
                                            <div class="form-group">
                                                <div class="checkbox-wrapper">
                                                    <input
                                                        type="checkbox"
                                                        id="sync-announcements"
                                                        checked=s.sync_announcements
                                                        on:change=move |ev| {
                                                            set_settings.update(|s| {
                                                                if let Some(settings) = s {
                                                                    settings.sync_announcements = event_target_checked(&ev);
                                                                }
                                                            })
                                                        }
                                                    />
                                                    <label for="sync-announcements">"Announcements"</label>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    view! { <></> }
                                }
                            }}
                            
                            {move || {
                                if s.canvas_course_id.is_some() {
                                    view! {
                                        <div class="form-actions">
                                            <button
                                                class="btn btn-primary"
                                                on:click=save_settings
                                                disabled=loading.get()
                                            >
                                                "Save Settings"
                                            </button>
                                        </div>
                                    }
                                } else {
                                    view! { <></> }
                                }
                            }}
                        </div>
                    }
                } else {
                    view! { <div class="error-state">"Failed to load settings"</div> }
                }
            }}
        </div>
    }
}

// Helper function to get event target checked state
fn event_target_checked(ev: &Event) -> bool {
    let target: web_sys::HtmlInputElement = ev.target_dyn_into().unwrap();
    target.checked()
}

// Helper function to get event target value
fn event_target_value(ev: &Event) -> String {
    let target: web_sys::HtmlInputElement = ev.target_dyn_into().unwrap();
    target.value()
}

// Wrapper for window interactions
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: serde::Serialize + ?Sized,
    R: for<'de> serde::de::DeserializeOwned,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}