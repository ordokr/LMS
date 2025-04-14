use leptos::*;
use leptos_router::*;
use crate::models::integration::{IntegrationStatus, SyncHistoryEntry, SyncConflict};
use crate::services::integration_service::IntegrationService;
use crate::components::shared::ErrorAlert;
use crate::components::dashboard::{SyncStatusWidget, SyncHistoryWidget, ConflictsWidget};
use crate::utils::style_manager::use_stylesheet;

#[component]
pub fn IntegrationDashboard() -> impl IntoView {
    // Load stylesheet
    use_stylesheet("integration.css");

    // State
    let (loading, set_loading) = create_signal(false);
    let (canvas_status, set_canvas_status) = create_signal(IntegrationStatus::default());
    let (discourse_status, set_discourse_status) = create_signal(IntegrationStatus::default());
    let (error, set_error) = create_signal(None::<String>);
    let (sync_stats, set_sync_stats) = create_signal(serde_json::json!({
        "total": 0,
        "success": 0,
        "failure": 0,
        "by_type": {}
    }));
    let (sync_history, set_sync_history) = create_signal(Vec::<SyncHistoryEntry>::new());
    let (conflicts, set_conflicts) = create_signal(Vec::<SyncConflict>::new());

    // Load data function
    let load_data = move || {
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            // Load Canvas integration status
            match IntegrationService::get_canvas_integration_status().await {
                Ok(status) => {
                    set_canvas_status.set(status);
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to load Canvas integration status: {}", err)));
                }
            }

            // Load Discourse integration status
            match IntegrationService::get_discourse_integration_status().await {
                Ok(status) => {
                    set_discourse_status.set(status);
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to load Discourse integration status: {}", err)));
                }
            }

            // Load sync history stats
            match IntegrationService::get_sync_history_stats().await {
                Ok(stats) => {
                    set_sync_stats.set(stats);
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to load sync history stats: {}", err)));
                }
            }

            // Load sync history
            match IntegrationService::get_sync_history().await {
                Ok(history) => {
                    set_sync_history.set(history);
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to load sync history: {}", err)));
                }
            }

            // Load sync conflicts
            match IntegrationService::get_sync_conflicts().await {
                Ok(conflict_list) => {
                    set_conflicts.set(conflict_list);
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to load sync conflicts: {}", err)));
                }
            }

            set_loading.set(false);
        });
    };

    // Sync all pending items
    let sync_all_pending = move |_| {
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match IntegrationService::sync_all_pending().await {
                Ok(_) => {
                    // Reload data after sync
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to sync pending items: {}", err)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Clear sync errors
    let clear_sync_errors = move |_| {
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match IntegrationService::clear_sync_errors().await {
                Ok(_) => {
                    // Reload data after clearing errors
                    load_data();
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to clear sync errors: {}", err)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Load data on component mount
    create_effect(move |_| {
        load_data();
    });

    view! {
        <div class="integration-page">
            <div class="page-header">
                <h1 class="page-title">
                    <i class="icon-integration"></i>
                    "Integration Dashboard"
                </h1>

                <div class="page-actions">
                    <button
                        class="btn btn-primary"
                        on:click=sync_all_pending
                        disabled=move || loading.get() || (!canvas_status.get().connected && !discourse_status.get().connected)
                    >
                        <i class="icon-sync"></i>
                        "Sync All Pending"
                    </button>

                    <button
                        class="btn btn-secondary"
                        on:click=clear_sync_errors
                        disabled=move || loading.get() || (!canvas_status.get().connected && !discourse_status.get().connected)
                    >
                        <i class="icon-clear"></i>
                        "Clear Errors"
                    </button>
                </div>
            </div>

            {move || if let Some(err) = error.get() {
                view! { <ErrorAlert message=err /> }
            } else {
                view! { <></> }
            }}

            <div class="dashboard-grid">
                <div class="dashboard-full-width">
                    <SyncStatusWidget
                        canvas_status=canvas_status
                        discourse_status=discourse_status
                        on_sync_all=sync_all_pending
                        loading=loading
                    />
                </div>

                <div>
                    <div class="integration-card canvas-card">
                        <div class="card-header">
                            <h2 class="card-title">
                                <i class="icon-canvas"></i>
                                "Canvas Integration"
                            </h2>

                            <A href="/integrations/canvas" class="btn btn-sm btn-secondary">
                                "Manage"
                            </A>
                        </div>

                        <div class="card-content">
                            <div class="status-item">
                                <span class="status-label">"Connection Status"</span>
                                <span class=if canvas_status.get().connected { "status-connected" } else { "status-disconnected" }>
                                    {if canvas_status.get().connected { "Connected" } else { "Disconnected" }}
                                </span>
                            </div>

                            <div class="status-item">
                                <span class="status-label">"Last Sync"</span>
                                <span class="status-value">
                                    {move || canvas_status.get().last_sync.unwrap_or_else(|| "Never".to_string())}
                                </span>
                            </div>

                            <div class="status-item">
                                <span class="status-label">"Pending Syncs"</span>
                                <span class="status-value">{move || canvas_status.get().pending_syncs}</span>
                            </div>

                            <div class="status-item">
                                <span class="status-label">"Sync Errors"</span>
                                <span class="status-value">{move || canvas_status.get().sync_errors}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div>
                    <div class="integration-card discourse-card">
                        <div class="card-header">
                            <h2 class="card-title">
                                <i class="icon-discourse"></i>
                                "Discourse Integration"
                            </h2>

                            <A href="/integrations/discourse" class="btn btn-sm btn-secondary">
                                "Manage"
                            </A>
                        </div>

                        <div class="card-content">
                            <div class="status-item">
                                <span class="status-label">"Connection Status"</span>
                                <span class=if discourse_status.get().connected { "status-connected" } else { "status-disconnected" }>
                                    {if discourse_status.get().connected { "Connected" } else { "Disconnected" }}
                                </span>
                            </div>

                            <div class="status-item">
                                <span class="status-label">"Last Sync"</span>
                                <span class="status-value">
                                    {move || discourse_status.get().last_sync.unwrap_or_else(|| "Never".to_string())}
                                </span>
                            </div>

                            <div class="status-item">
                                <span class="status-label">"Pending Syncs"</span>
                                <span class="status-value">{move || discourse_status.get().pending_syncs}</span>
                            </div>

                            <div class="status-item">
                                <span class="status-label">"Sync Errors"</span>
                                <span class="status-value">{move || discourse_status.get().sync_errors}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="dashboard-full-width">
                    <div class="sync-stats-card">
                        <h2 class="card-title">"Sync Statistics"</h2>

                        <div class="sync-stats-content">
                            <div class="stats-overview">
                                <div class="stat-item">
                                    <span class="stat-value">{move || sync_stats.get()["total"].as_u64().unwrap_or(0)}</span>
                                    <span class="stat-label">"Total Syncs"</span>
                                </div>

                                <div class="stat-item success-stat">
                                    <span class="stat-value">{move || sync_stats.get()["success"].as_u64().unwrap_or(0)}</span>
                                    <span class="stat-label">"Successful"</span>
                                </div>

                                <div class="stat-item error-stat">
                                    <span class="stat-value">{move || sync_stats.get()["failure"].as_u64().unwrap_or(0)}</span>
                                    <span class="stat-label">"Failed"</span>
                                </div>

                                <div class="stat-item">
                                    <span class="stat-value">{move || format!("{:.1} ms", sync_stats.get()["avg_duration_ms"].as_f64().unwrap_or(0.0))}</span>
                                    <span class="stat-label">"Avg. Duration"</span>
                                </div>
                            </div>

                            <div class="sync-types-chart">
                                <h3 class="chart-title">"Sync Operations by Type"</h3>

                                <div class="chart-container">
                                    {move || {
                                        let by_type = &sync_stats.get()["by_type"];

                                        if by_type.is_object() && !by_type.as_object().unwrap().is_empty() {
                                            view! {
                                                <div class="chart-bars">
                                                    {by_type.as_object().unwrap().iter().map(|(key, value)| {
                                                        let count = value.as_u64().unwrap_or(0);
                                                        let total = sync_stats.get()["total"].as_u64().unwrap_or(1);
                                                        let percentage = (count as f64 / total as f64) * 100.0;

                                                        view! {
                                                            <div class="chart-bar-item">
                                                                <div class="chart-bar-label">{key}</div>
                                                                <div class="chart-bar-container">
                                                                    <div class="chart-bar" style=format!("width: {}%;", percentage)></div>
                                                                </div>
                                                                <div class="chart-bar-value">{count}</div>
                                                            </div>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="empty-chart">
                                                    <p>"No sync data available yet."</p>
                                                </div>
                                            }
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div>
                    <SyncHistoryWidget
                        history=sync_history
                        max_entries=5
                    />
                </div>

                <div>
                    <ConflictsWidget
                        conflicts=conflicts
                        on_resolve=move |conflict| {
                            // Navigate to conflict resolution page
                            let navigate = use_navigate();
                            navigate(&format!("/integrations/conflicts/{}", conflict.id), Default::default());
                        }
                        max_entries=5
                    />
                </div>
            </div>

            <div class="quick-actions">
                <h2 class="section-title">"Quick Actions"</h2>

                <div class="actions-grid">
                    <A href="/integrations/canvas" class="action-card">
                        <div class="action-icon">
                            <i class="icon-canvas"></i>
                        </div>
                        <div class="action-content">
                            <h3 class="action-title">"Manage Canvas"</h3>
                            <p class="action-description">"Sync courses, assignments, and manage Canvas integration settings."</p>
                        </div>
                    </A>

                    <A href="/integrations/discourse" class="action-card">
                        <div class="action-icon">
                            <i class="icon-discourse"></i>
                        </div>
                        <div class="action-content">
                            <h3 class="action-title">"Manage Discourse"</h3>
                            <p class="action-description">"Sync topics, categories, and manage Discourse integration settings."</p>
                        </div>
                    </A>

                    <A href="/integrations/settings" class="action-card">
                        <div class="action-icon">
                            <i class="icon-settings"></i>
                        </div>
                        <div class="action-content">
                            <h3 class="action-title">"Integration Settings"</h3>
                            <p class="action-description">"Configure API connections and synchronization settings."</p>
                        </div>
                    </A>

                    <button class="action-card" on:click=sync_all_pending disabled=loading.get()>
                        <div class="action-icon">
                            <i class="icon-sync"></i>
                        </div>
                        <div class="action-content">
                            <h3 class="action-title">"Sync All Pending"</h3>
                            <p class="action-description">"Process all pending synchronization tasks for both platforms."</p>
                        </div>
                    </button>

                    <button class="action-card" on:click=clear_sync_errors disabled=loading.get()>
                        <div class="action-icon">
                            <i class="icon-clear"></i>
                        </div>
                        <div class="action-content">
                            <h3 class="action-title">"Clear Sync Errors"</h3>
                            <p class="action-description">"Reset error status for failed synchronization tasks."</p>
                        </div>
                    </button>
                </div>
            </div>
        </div>
    }
}
