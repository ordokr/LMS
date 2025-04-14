use leptos::*;
use crate::models::integration::IntegrationStatus;

#[component]
pub fn SyncStatusWidget(
    #[prop(into)] canvas_status: Signal<IntegrationStatus>,
    #[prop(into)] discourse_status: Signal<IntegrationStatus>,
    #[prop(into)] on_sync_all: Callback<()>,
    #[prop(into)] loading: Signal<bool>,
) -> impl IntoView {
    // Calculate total pending syncs
    let total_pending = move || {
        canvas_status.get().pending_syncs + discourse_status.get().pending_syncs
    };
    
    // Calculate total errors
    let total_errors = move || {
        canvas_status.get().sync_errors + discourse_status.get().sync_errors
    };
    
    // Determine status class
    let status_class = move || {
        if total_errors() > 0 {
            "status-error"
        } else if total_pending() > 0 {
            "status-warning"
        } else {
            "status-success"
        }
    };
    
    // Determine status text
    let status_text = move || {
        if total_errors() > 0 {
            "Issues Detected"
        } else if total_pending() > 0 {
            "Pending Syncs"
        } else {
            "All Systems Synced"
        }
    };
    
    view! {
        <div class="widget sync-status-widget">
            <div class="widget-header">
                <h3 class="widget-title">"Sync Status"</h3>
                
                <button 
                    class="btn btn-sm btn-primary"
                    on:click=move |_| on_sync_all.call(())
                    disabled=move || loading.get() || (!canvas_status.get().connected && !discourse_status.get().connected)
                >
                    <i class="icon-sync"></i>
                    "Sync All"
                </button>
            </div>
            
            <div class="widget-content">
                <div class=format!("sync-status-indicator {}", status_class())>
                    <div class="status-icon">
                        <i class=if total_errors() > 0 {
                            "icon-error"
                        } else if total_pending() > 0 {
                            "icon-warning"
                        } else {
                            "icon-success"
                        }></i>
                    </div>
                    <div class="status-text">{status_text()}</div>
                </div>
                
                <div class="sync-status-details">
                    <div class="status-item">
                        <span class="status-label">"Pending Syncs"</span>
                        <span class="status-value">{total_pending()}</span>
                    </div>
                    
                    <div class="status-item">
                        <span class="status-label">"Sync Errors"</span>
                        <span class="status-value">{total_errors()}</span>
                    </div>
                    
                    <div class="status-item">
                        <span class="status-label">"Last Canvas Sync"</span>
                        <span class="status-value">
                            {move || canvas_status.get().last_sync.unwrap_or_else(|| "Never".to_string())}
                        </span>
                    </div>
                    
                    <div class="status-item">
                        <span class="status-label">"Last Discourse Sync"</span>
                        <span class="status-value">
                            {move || discourse_status.get().last_sync.unwrap_or_else(|| "Never".to_string())}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}
