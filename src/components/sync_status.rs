use leptos::*;
use web_sys::console;
use crate::sync::SyncManager;

#[component]
pub fn SyncStatus(cx: Scope) -> impl IntoView {
    let sync_manager = use_context::<SyncManager>(cx)
        .expect("SyncManager should be provided");
    
    let (is_syncing, last_sync, error) = create_memo(cx, move |_| {
        sync_manager.sync_status()
    });
    
    let has_pending = create_memo(cx, move |_| {
        sync_manager.has_pending_changes()
    });
    
    let trigger_sync = move |_| {
        sync_manager.start_sync();
    };
    
    // Periodically check for changes when online
    use_effect(cx, (), |_| {
        let interval_id = window().set_interval_with_callback_and_timeout_and_arguments(
            &Closure::wrap(Box::new(move || {
                if is_online() && sync_manager.has_pending_changes() {
                    console::log_1(&"Auto-syncing pending changes...".into());
                    sync_manager.start_sync();
                }
            }) as Box<dyn FnMut()>).into_js_value(),
            60000, // Check every minute
            &js_sys::Array::new(),
        ).expect("Failed to set interval");
        
        move || {
            window().clear_interval_with_handle(interval_id);
        }
    });

    view! { cx,
        <div class="sync-status">
            <div class="sync-indicator">
                {move || {
                    let syncing = is_syncing();
                    let pending = has_pending();
                    
                    if syncing {
                        view! { cx, <span class="syncing">"Syncing..."</span> }
                    } else if pending {
                        view! { cx, <span class="pending">"Changes pending"</span> }
                    } else {
                        view! { cx, <span class="synced">"All changes synced"</span> }
                    }
                }}
            </div>
            
            <div class="last-sync">
                {move || {
                    match last_sync() {
                        Some(time) => view! { cx, 
                            <span>"Last synced: " {format_time(time)}</span>
                        },
                        None => view! { cx, <span>"Never synced"</span> }
                    }
                }}
            </div>
            
            {move || {
                if let Some(err) = error() {
                    view! { cx, <div class="sync-error">"Sync error: " {err}</div> }
                } else {
                    view! { cx, <></> }
                }
            }}
            
            <button 
                class="sync-button"
                on:click=trigger_sync
                disabled=move || is_syncing()
            >
                {move || {
                    if is_syncing() {
                        "Syncing..."
                    } else if has_pending() {
                        "Sync Now"
                    } else {
                        "Check for Updates"
                    }
                }}
            </button>
        </div>
    }
}

fn format_time(time: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(time);
    
    if diff.num_days() > 0 {
        time.format("%b %d, %Y at %H:%M").to_string()
    } else if diff.num_hours() > 0 {
        format!("{} hours ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{} minutes ago", diff.num_minutes())
    } else {
        "just now".to_string()
    }
}

fn is_online() -> bool {
    web_sys::window()
        .and_then(|window| Some(window.navigator().on_line()))
        .unwrap_or(false)
}

fn window() -> web_sys::Window {
    web_sys::window().expect("Window object not available")
}