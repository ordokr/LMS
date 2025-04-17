use leptos::*;
use crate::models::integration::IntegrationStatus;
use crate::models::sync::SyncErrorDetails;
use chrono::{DateTime, Utc};

/// Enhanced sync status widget with detailed metrics and error reporting
#[component]
pub fn EnhancedSyncStatusWidget(
    #[prop(into)] canvas_status: Signal<IntegrationStatus>,
    #[prop(into)] discourse_status: Signal<IntegrationStatus>,
    #[prop(into)] sync_errors: Signal<Vec<SyncErrorDetails>>,
    #[prop(into)] on_sync_all: Callback<()>,
    #[prop(into)] on_retry_failed: Callback<()>,
    #[prop(into)] on_clear_errors: Callback<()>,
    #[prop(into)] on_view_error_details: Callback<String>,
    #[prop(into)] loading: Signal<bool>,
    #[prop(default = true)] show_detailed_metrics: bool,
) -> impl IntoView {
    // State for UI
    let (show_error_panel, set_show_error_panel) = create_signal(false);
    
    // Calculate total pending syncs
    let total_pending = move || {
        canvas_status.get().pending_syncs + discourse_status.get().pending_syncs
    };
    
    // Calculate total errors
    let total_errors = move || {
        canvas_status.get().sync_errors + discourse_status.get().sync_errors
    };
    
    // Calculate sync success rate
    let sync_success_rate = move || {
        let canvas = canvas_status.get();
        let discourse = discourse_status.get();
        
        let total_syncs = canvas.total_syncs + discourse.total_syncs;
        if total_syncs == 0 {
            return 100.0; // No syncs yet
        }
        
        let total_success = total_syncs - (canvas.sync_errors + discourse.sync_errors);
        (total_success as f64 / total_syncs as f64 * 100.0).round()
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
    
    // Format datetime for display
    let format_datetime = |dt: Option<String>| -> String {
        match dt {
            Some(dt_str) => {
                // Try to parse the datetime string
                match DateTime::parse_from_rfc3339(&dt_str) {
                    Ok(dt) => {
                        // Format as a human-readable string
                        let utc_dt = dt.with_timezone(&Utc);
                        let now = Utc::now();
                        let diff = now.signed_duration_since(utc_dt);
                        
                        if diff.num_days() > 1 {
                            format!("{} days ago", diff.num_days())
                        } else if diff.num_hours() > 0 {
                            format!("{} hours ago", diff.num_hours())
                        } else if diff.num_minutes() > 0 {
                            format!("{} minutes ago", diff.num_minutes())
                        } else {
                            "Just now".to_string()
                        }
                    },
                    Err(_) => dt_str.clone(),
                }
            },
            None => "Never".to_string(),
        }
    };
    
    // Toggle error panel
    let toggle_error_panel = move |_| {
        set_show_error_panel.update(|value| *value = !*value);
    };
    
    view! {
        <div class="widget sync-status-widget enhanced">
            <div class="widget-header">
                <div class="widget-title-area">
                    <h3 class="widget-title">"Sync Status"</h3>
                    <span class=format!("status-badge {}", status_class())>{status_text()}</span>
                </div>
                
                <div class="widget-actions">
                    <button 
                        class="btn btn-sm btn-secondary"
                        on:click=toggle_error_panel
                        disabled=move || total_errors() == 0
                    >
                        <i class="fas fa-exclamation-triangle"></i>
                        " Errors "
                        <span class="badge">{total_errors}</span>
                    </button>
                    
                    <button 
                        class="btn btn-sm btn-primary"
                        on:click=move |_| on_sync_all.call(())
                        disabled=move || loading.get() || (!canvas_status.get().connected && !discourse_status.get().connected)
                    >
                        <i class="fas fa-sync"></i>
                        " Sync All"
                    </button>
                </div>
            </div>
            
            <div class="widget-content">
                {move || {
                    if show_error_panel() && total_errors() > 0 {
                        view! {
                            <div class="error-panel">
                                <div class="error-panel-header">
                                    <h4>"Sync Errors"</h4>
                                    <div class="error-actions">
                                        <button 
                                            class="btn btn-sm btn-warning"
                                            on:click=move |_| on_retry_failed.call(())
                                            disabled=loading.get()
                                        >
                                            <i class="fas fa-redo"></i>
                                            " Retry Failed"
                                        </button>
                                        
                                        <button 
                                            class="btn btn-sm btn-outline-danger"
                                            on:click=move |_| on_clear_errors.call(())
                                            disabled=loading.get()
                                        >
                                            <i class="fas fa-trash"></i>
                                            " Clear Errors"
                                        </button>
                                    </div>
                                </div>
                                
                                <div class="error-list">
                                    {move || {
                                        let errors = sync_errors.get();
                                        if errors.is_empty() {
                                            view! { <p class="no-errors">"No detailed error information available."</p> }
                                        } else {
                                            errors.into_iter().map(|error| {
                                                let error_id = error.id.clone();
                                                view! {
                                                    <div class="error-item">
                                                        <div class="error-info">
                                                            <div class="error-title">
                                                                <span class="error-type">{&error.entity_type}</span>
                                                                <span class="error-timestamp">{format_datetime(Some(error.timestamp))}</span>
                                                            </div>
                                                            <div class="error-message">{&error.message}</div>
                                                        </div>
                                                        <div class="error-actions">
                                                            <button 
                                                                class="btn btn-sm btn-link"
                                                                on:click=move |_| on_view_error_details.call(error_id.clone())
                                                            >
                                                                "View Details"
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="sync-status-overview">
                                <div class="status-metrics">
                                    <div class="metric-card">
                                        <div class="metric-icon">
                                            <i class="fas fa-clock"></i>
                                        </div>
                                        <div class="metric-content">
                                            <div class="metric-value">{total_pending}</div>
                                            <div class="metric-label">"Pending Syncs"</div>
                                        </div>
                                    </div>
                                    
                                    <div class="metric-card">
                                        <div class="metric-icon error-icon">
                                            <i class="fas fa-exclamation-circle"></i>
                                        </div>
                                        <div class="metric-content">
                                            <div class="metric-value">{total_errors}</div>
                                            <div class="metric-label">"Sync Errors"</div>
                                        </div>
                                    </div>
                                    
                                    <div class="metric-card">
                                        <div class="metric-icon success-icon">
                                            <i class="fas fa-check-circle"></i>
                                        </div>
                                        <div class="metric-content">
                                            <div class="metric-value">{move || format!("{}%", sync_success_rate())}</div>
                                            <div class="metric-label">"Success Rate"</div>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="status-details">
                                    <div class="integration-status">
                                        <div class="integration-header">
                                            <h4>"Canvas Integration"</h4>
                                            <span class=move || if canvas_status.get().connected { "connection-status connected" } else { "connection-status disconnected" }>
                                                {move || if canvas_status.get().connected { "Connected" } else { "Disconnected" }}
                                            </span>
                                        </div>
                                        <div class="integration-metrics">
                                            <div class="detail-item">
                                                <span class="detail-label">"Last Sync:"</span>
                                                <span class="detail-value">{move || format_datetime(canvas_status.get().last_sync)}</span>
                                            </div>
                                            <div class="detail-item">
                                                <span class="detail-label">"Pending:"</span>
                                                <span class="detail-value">{move || canvas_status.get().pending_syncs}</span>
                                            </div>
                                            <div class="detail-item">
                                                <span class="detail-label">"Errors:"</span>
                                                <span class="detail-value">{move || canvas_status.get().sync_errors}</span>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="integration-status">
                                        <div class="integration-header">
                                            <h4>"Discourse Integration"</h4>
                                            <span class=move || if discourse_status.get().connected { "connection-status connected" } else { "connection-status disconnected" }>
                                                {move || if discourse_status.get().connected { "Connected" } else { "Disconnected" }}
                                            </span>
                                        </div>
                                        <div class="integration-metrics">
                                            <div class="detail-item">
                                                <span class="detail-label">"Last Sync:"</span>
                                                <span class="detail-value">{move || format_datetime(discourse_status.get().last_sync)}</span>
                                            </div>
                                            <div class="detail-item">
                                                <span class="detail-label">"Pending:"</span>
                                                <span class="detail-value">{move || discourse_status.get().pending_syncs}</span>
                                            </div>
                                            <div class="detail-item">
                                                <span class="detail-label">"Errors:"</span>
                                                <span class="detail-value">{move || discourse_status.get().sync_errors}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                
                                {move || {
                                    if show_detailed_metrics {
                                        view! {
                                            <div class="detailed-metrics">
                                                <h4>"Sync Performance Metrics"</h4>
                                                <div class="metrics-grid">
                                                    <div class="metric-item">
                                                        <div class="metric-title">"Total Syncs"</div>
                                                        <div class="metric-value">{move || canvas_status.get().total_syncs + discourse_status.get().total_syncs}</div>
                                                    </div>
                                                    <div class="metric-item">
                                                        <div class="metric-title">"Average Sync Time"</div>
                                                        <div class="metric-value">{move || format!("{:.1}s", (canvas_status.get().avg_sync_time + discourse_status.get().avg_sync_time) / 2.0)}</div>
                                                    </div>
                                                    <div class="metric-item">
                                                        <div class="metric-title">"Successful Syncs"</div>
                                                        <div class="metric-value">
                                                            {move || {
                                                                let canvas = canvas_status.get();
                                                                let discourse = discourse_status.get();
                                                                (canvas.total_syncs + discourse.total_syncs) - (canvas.sync_errors + discourse.sync_errors)
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="metric-item">
                                                        <div class="metric-title">"Failed Syncs"</div>
                                                        <div class="metric-value">{total_errors}</div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        view! { <></> }
                                    }
                                }}
                            </div>
                        }
                    }
                }}
            </div>
        </div>
    }
}