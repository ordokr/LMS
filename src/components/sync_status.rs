use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::error_alert::ErrorAlert;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncStatus {
    pub canvas_connected: bool,
    pub discourse_connected: bool,
    pub last_sync: Option<String>,
    pub pending_syncs: i32,
    pub sync_in_progress: bool,
    pub sync_errors: Vec<String>,
}

#[component]
pub fn SyncStatusMonitor() -> impl IntoView {
    let (status, set_status) = create_signal(None::<SyncStatus>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Function to load the status
    let load_status = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async {
            match invoke::<(), SyncStatus>("get_sync_status", &()).await {
                Ok(result) => {
                    set_status.set(Some(result));
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load sync status: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Function to trigger sync
    let trigger_sync = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async {
            match invoke::<(), String>("sync_all_pending", &()).await {
                Ok(message) => {
                    // Reload status after sync
                    load_status();
                },
                Err(e) => {
                    set_error.set(Some(format!("Sync failed: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Check connectivity
    let check_connectivity = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async {
            // Check Canvas connectivity
            match invoke::<(), bool>("test_canvas_connectivity", &()).await {
                Ok(_) => {},
                Err(e) => {
                    set_error.set(Some(format!("Canvas connectivity test failed: {}", e)));
                }
            }
            
            // Check Discourse connectivity
            match invoke::<(), bool>("test_discourse_connectivity", &()).await {
                Ok(_) => {},
                Err(e) => {
                    set_error.set(Some(format!("Discourse connectivity test failed: {}", e)));
                }
            }
            
            // Reload status
            load_status();
        });
    };
    
    // Format date helper
    let format_date = |date_str: Option<String>| -> String {
        match date_str {
            Some(date) if !date.is_empty() => {
                // Simple formatting - in a real app you'd use a proper date library
                date.replace("T", " ").replace("Z", "")
            },
            _ => "Never".to_string()
        }
    };
    
    // Load status on component mount
    create_effect(move |_| {
        load_status();
        
        // Set up polling interval
        let interval_handle = set_interval(
            move || {
                // Only reload if not already loading
                if !loading.get() {
                    load_status();
                }
            },
            std::time::Duration::from_secs(30) // Every 30 seconds
        );
        
        on_cleanup(move || {
            clear_interval(interval_handle);
        });
    });

    view! {
        <div class="sync-status-component">
            <h2>"Integration Sync Status"</h2>
            
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            {move || {
                if loading.get() && status.get().is_none() {
                    view! { <div class="loading-spinner">"Loading status..."</div> }
                } else if let Some(s) = status.get() {
                    view! {
                        <div class="sync-status-panel">
                            <div class="integration-status">
                                <div class="status-card">
                                    <h3>"Canvas LMS"</h3>
                                    <div class="status-indicator">
                                        <span class=if s.canvas_connected { "connected" } else { "disconnected" }>
                                            {if s.canvas_connected { "Connected ✓" } else { "Disconnected" }}
                                        </span>
                                    </div>
                                    <div class="status-actions">
                                        <a href="#/settings/integrations">"Manage Integration"</a>
                                    </div>
                                </div>
                                
                                <div class="status-card">
                                    <h3>"Discourse"</h3>
                                    <div class="status-indicator">
                                        <span class=if s.discourse_connected { "connected" } else { "disconnected" }>
                                            {if s.discourse_connected { "Connected ✓" } else { "Disconnected" }}
                                        </span>
                                    </div>
                                    <div class="status-actions">
                                        <a href="#/settings/integrations">"Manage Integration"</a>
                                    </div>
                                </div>
                            </div>
                            
                            <div class="sync-info">
                                <div class="info-item">
                                    <strong>"Last Sync: "</strong>
                                    <span>{format_date(s.last_sync)}</span>
                                </div>
                                
                                <div class="info-item">
                                    <strong>"Pending Items: "</strong>
                                    <span>{s.pending_syncs}</span>
                                </div>
                            </div>
                            
                            {move || {
                                if !s.sync_errors.is_empty() {
                                    view! {
                                        <div class="sync-errors">
                                            <h4>"Recent Sync Errors"</h4>
                                            <ul>
                                                {s.sync_errors.iter().map(|err| view! {
                                                    <li>{err}</li>
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        </div>
                                    }
                                } else {
                                    view! { <></> }
                                }
                            }}
                            
                            <div class="sync-actions">
                                <button 
                                    class="btn btn-primary"
                                    on:click=trigger_sync
                                    disabled=s.sync_in_progress || s.pending_syncs == 0
                                >
                                    {if s.sync_in_progress { "Sync in Progress..." } else { "Sync Now" }}
                                </button>
                                
                                <button 
                                    class="btn btn-secondary"
                                    on:click=check_connectivity
                                    disabled=loading.get()
                                >
                                    "Check Connectivity"
                                </button>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div class="error-state">"Failed to load sync status"</div> }
                }
            }}
        </div>
    }
}

// Helper function to set interval
fn set_interval<F>(f: F, duration: std::time::Duration) -> i32
where
    F: Fn() + 'static,
{
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    
    let callback = Closure::wrap(Box::new(move || {
        f();
    }) as Box<dyn Fn()>);
    
    let window = web_sys::window().expect("No global `window` exists");
    let id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            duration.as_millis() as i32,
        )
        .expect("Failed to set interval");
    
    callback.forget();
    id
}

// Helper function to clear interval
fn clear_interval(id: i32) {
    if let Some(window) = web_sys::window() {
        window.clear_interval_with_handle(id);
    }
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