use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::error_alert::ErrorAlert;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncHistoryEntry {
    pub id: i64,
    pub sync_type: String,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub sync_time: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub duration_ms: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncHistoryStats {
    pub total_count: i64,
    pub success_count: i64,
    pub error_count: i64,
    pub avg_duration_ms: Option<f64>,
    pub content_type_stats: Vec<ContentTypeStats>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContentTypeStats {
    pub content_type: String,
    pub count: i64,
    pub success_count: i64,
    pub error_count: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncHistoryFilters {
    pub content_type: Option<String>,
    pub success_only: Option<bool>,
    pub error_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[component]
pub fn SyncHistory() -> impl IntoView {
    let (history, set_history) = create_signal(Vec::<SyncHistoryEntry>::new());
    let (stats, set_stats) = create_signal(None::<SyncHistoryStats>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Filtering state
    let (filters, set_filters) = create_signal(SyncHistoryFilters {
        content_type: None,
        success_only: None,
        error_only: None,
        limit: Some(50),
        offset: Some(0),
    });
    
    // Load history data
    let load_history = move || {
        set_loading.set(true);
        set_error.set(None);
        
        let current_filters = filters.get().clone();
        
        spawn_local(async move {
            match invoke::<_, Vec<SyncHistoryEntry>>("get_sync_history", &Some(current_filters)).await {
                Ok(entries) => {
                    set_history.set(entries);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load sync history: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load stats
    let load_stats = move || {
        spawn_local(async move {
            match invoke::<(), SyncHistoryStats>("get_sync_history_stats", &()).await {
                Ok(result) => {
                    set_stats.set(Some(result));
                },
                Err(e) => {
                    log::error!("Failed to load sync history stats: {}", e);
                }
            }
        });
    };
    
    // Format date helper
    let format_date = |date_str: &str| -> String {
        // Simple formatting - in a real app you'd use a proper date library
        date_str.replace("T", " ").replace("Z", "")
    };
    
    // Handle filter changes
    let set_content_type_filter = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        let content_type = if value.is_empty() { None } else { Some(value) };
        set_filters.update(|f| f.content_type = content_type);
        load_history();
    };
    
    let set_status_filter = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        
        match value.as_str() {
            "all" => {
                set_filters.update(|f| {
                    f.success_only = None;
                    f.error_only = None;
                });
            },
            "success" => {
                set_filters.update(|f| {
                    f.success_only = Some(true);
                    f.error_only = None;
                });
            },
            "error" => {
                set_filters.update(|f| {
                    f.success_only = None;
                    f.error_only = Some(true);
                });
            },
            _ => {}
        }
        
        load_history();
    };
    
    let handle_load_more = move |_| {
        set_filters.update(|f| {
            let current_limit = f.limit.unwrap_or(50);
            f.limit = Some(current_limit + 50);
        });
        
        load_history();
    };
    
    // Initial data load
    create_effect(move |_| {
        load_history();
        load_stats();
        
        // Refresh stats periodically
        let interval_handle = set_interval(
            move || {
                load_stats();
            },
            std::time::Duration::from_secs(60) // Every minute
        );
        
        on_cleanup(move || {
            clear_interval(interval_handle);
        });
    });

    view! {
        <div class="sync-history-component">
            <h2>"Synchronization History"</h2>
            
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            // Stats section
            {move || stats.get().map(|s| view! {
                <div class="sync-stats">
                    <div class="stats-card">
                        <div class="stat">
                            <span class="stat-label">"Total Syncs"</span>
                            <span class="stat-value">{s.total_count}</span>
                        </div>
                        
                        <div class="stat">
                            <span class="stat-label">"Successful"</span>
                            <span class="stat-value success">{s.success_count}</span>
                        </div>
                        
                        <div class="stat">
                            <span class="stat-label">"Failed"</span>
                            <span class="stat-value error">{s.error_count}</span>
                        </div>
                        
                        <div class="stat">
                            <span class="stat-label">"Avg Duration"</span>
                            <span class="stat-value">{
                                s.avg_duration_ms
                                    .map(|d| format!("{:.1} ms", d))
                                    .unwrap_or_else(|| "N/A".to_string())
                            }</span>
                        </div>
                    </div>
                    
                    <div class="content-type-stats">
                        <h3>"Sync By Content Type"</h3>
                        <table class="stats-table">
                            <thead>
                                <tr>
                                    <th>"Content Type"</th>
                                    <th>"Total"</th>
                                    <th>"Success"</th>
                                    <th>"Error"</th>
                                    <th>"Success Rate"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {s.content_type_stats.iter().map(|stat| {
                                    let success_rate = if stat.count > 0 {
                                        (stat.success_count as f64 / stat.count as f64) * 100.0
                                    } else {
                                        0.0
                                    };
                                    
                                    view! {
                                        <tr>
                                            <td>{&stat.content_type}</td>
                                            <td>{stat.count}</td>
                                            <td class="success">{stat.success_count}</td>
                                            <td class="error">{stat.error_count}</td>
                                            <td class=if success_rate >= 95.0 { "success" } 
                                                    else if success_rate >= 80.0 { "warning" } 
                                                    else { "error" }
                                            >
                                                {format!("{:.1}%", success_rate)}
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    </div>
                </div>
            })}
            
            // Filters
            <div class="history-filters">
                <div class="filter-group">
                    <label for="content-type">"Content Type"</label>
                    <select 
                        id="content-type"
                        on:change=set_content_type_filter
                    >
                        <option value="">"All Types"</option>
                        <option value="forum_topic">"Forum Topics"</option>
                        <option value="assignment">"Assignments"</option>
                        <option value="discussion">"Discussions"</option>
                        <option value="module">"Modules"</option>
                        <option value="file">"Files"</option>
                    </select>
                </div>
                
                <div class="filter-group">
                    <label for="status-filter">"Status"</label>
                    <select 
                        id="status-filter"
                        on:change=set_status_filter
                    >
                        <option value="all">"All"</option>
                        <option value="success">"Success Only"</option>
                        <option value="error">"Errors Only"</option>
                    </select>
                </div>
            </div>
            
            // History table
            {move || {
                if loading.get() && history.get().is_empty() {
                    view! { <div class="loading-spinner">"Loading history..."</div> }
                } else if history.get().is_empty() {
                    view! { <div class="empty-state">"No sync history found"</div> }
                } else {
                    view! {
                        <div class="history-table-container">
                            <table class="history-table">
                                <thead>
                                    <tr>
                                        <th>"Time"</th>
                                        <th>"Type"</th>
                                        <th>"Content"</th>
                                        <th>"Status"</th>
                                        <th>"Duration"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {history.get().iter().map(|entry| {
                                        view! {
                                            <tr class=if entry.success { "success-row" } else { "error-row" }>
                                                <td>{format_date(&entry.sync_time)}</td>
                                                <td>{&entry.sync_type}</td>
                                                <td>
                                                    {match (&entry.content_type, &entry.content_id) {
                                                        (Some(content_type), Some(content_id)) => {
                                                            format!("{}: {}", content_type, content_id)
                                                        }
                                                        (Some(content_type), None) => content_type.clone(),
                                                        (None, Some(content_id)) => content_id.clone(),
                                                        _ => "N/A".to_string()
                                                    }}
                                                </td>
                                                <td class=if entry.success { "success" } else { "error" }>
                                                    {if entry.success {
                                                        "Success ✓"
                                                    } else {
                                                        "Failed ✗"
                                                    }}
                                                </td>
                                                <td>
                                                    {entry.duration_ms.map(|d| format!("{} ms", d))
                                                        .unwrap_or_else(|| "N/A".to_string())}
                                                </td>
                                            </tr>
                                            {if !entry.success && entry.error_message.is_some() {
                                                view! {
                                                    <tr class="error-details">
                                                        <td colspan="5">
                                                            <div class="error-message">
                                                                {entry.error_message.as_ref().unwrap()}
                                                            </div>
                                                        </td>
                                                    </tr>
                                                }
                                            } else {
                                                view! { <></> }
                                            }}
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                            
                            <div class="load-more">
                                <button 
                                    class="btn btn-secondary"
                                    on:click=handle_load_more
                                    disabled=loading.get()
                                >
                                    {if loading.get() { "Loading..." } else { "Load More" }}
                                </button>
                            </div>
                        </div>
                    }
                }
            }}
        </div>
    }
}

// Helper functions (reuse from the SyncStatusMonitor component)
// ... include the helper functions from the SyncStatusMonitor component