use leptos::*;
use crate::models::sync::{SyncHistoryEntry, SyncEntityType};
use chrono::{DateTime, Utc};

/// Enhanced sync history widget with filtering and detailed view
#[component]
pub fn EnhancedSyncHistoryWidget(
    #[prop(into)] sync_history: Signal<Vec<SyncHistoryEntry>>,
    #[prop(into)] on_view_details: Callback<String>,
    #[prop(into)] on_refresh: Callback<()>,
    #[prop(into)] loading: Signal<bool>,
) -> impl IntoView {
    // State for filtering and pagination
    let (filter_type, set_filter_type) = create_signal(String::from("all"));
    let (filter_status, set_filter_status) = create_signal(String::from("all"));
    let (page, set_page) = create_signal(0);
    let page_size = 10;
    
    // Format timestamp to relative time
    let format_time_ago = |timestamp: &str| -> String {
        match DateTime::parse_from_rfc3339(timestamp) {
            Ok(dt) => {
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
            Err(_) => timestamp.to_string(),
        }
    };
    
    // Get icon for entity type
    let get_entity_icon = |entity_type: &str| -> &'static str {
        match entity_type {
            "Course" => "fas fa-graduation-cap",
            "Topic" => "fas fa-comment-alt",
            "Post" => "fas fa-comments",
            "Assignment" => "fas fa-tasks",
            "User" => "fas fa-user",
            _ => "fas fa-sync",
        }
    };
    
    // Get badge class for sync status
    let get_status_badge_class = |success: bool| -> &'static str {
        if success { "badge badge-success" } else { "badge badge-danger" }
    };
    
    // Get badge text for sync status
    let get_status_badge_text = |success: bool| -> &'static str {
        if success { "Success" } else { "Failed" }
    };
    
    // Filter the history entries based on current filters
    let filtered_history = move || {
        let history = sync_history.get();
        let type_filter = filter_type.get();
        let status_filter = filter_status.get();
        
        history
            .into_iter()
            .filter(|entry| {
                // Apply entity type filter
                if type_filter != "all" && entry.entity_type != type_filter {
                    return false;
                }
                
                // Apply status filter
                if status_filter != "all" {
                    let is_success = status_filter == "success";
                    if entry.success != is_success {
                        return false;
                    }
                }
                
                true
            })
            .collect::<Vec<_>>()
    };
    
    // Paginate the filtered history
    let paginated_history = move || {
        let filtered = filtered_history();
        let start = page() * page_size;
        let end = (start + page_size).min(filtered.len());
        
        if start >= filtered.len() {
            // If current page is now empty due to filtering, reset to first page
            if page() > 0 {
                set_page.set(0);
            }
            return Vec::new();
        }
        
        filtered[start..end].to_vec()
    };
    
    // Total pages based on filtered results
    let total_pages = move || {
        let filtered_count = filtered_history().len();
        (filtered_count + page_size - 1) / page_size
    };
    
    view! {
        <div class="widget sync-history-widget enhanced">
            <div class="widget-header">
                <h3 class="widget-title">"Sync History"</h3>
                
                <button 
                    class="btn btn-sm btn-outline-secondary"
                    on:click=move |_| on_refresh.call(())
                    disabled=loading.get()
                >
                    <i class="fas fa-sync"></i>
                    " Refresh"
                </button>
            </div>
            
            <div class="widget-content">
                <div class="filter-controls">
                    <div class="filter-group">
                        <label for="filter-type">"Entity Type:"</label>
                        <select 
                            id="filter-type"
                            on:change=move |ev| {
                                set_filter_type.set(event_target_value(&ev));
                                set_page.set(0); // Reset to first page on filter change
                            }
                        >
                            <option value="all" selected=move || filter_type() == "all">"All Types"</option>
                            <option value="Course" selected=move || filter_type() == "Course">"Courses"</option>
                            <option value="Topic" selected=move || filter_type() == "Topic">"Topics"</option>
                            <option value="Post" selected=move || filter_type() == "Post">"Posts"</option>
                            <option value="Assignment" selected=move || filter_type() == "Assignment">"Assignments"</option>
                            <option value="User" selected=move || filter_type() == "User">"Users"</option>
                        </select>
                    </div>
                    
                    <div class="filter-group">
                        <label for="filter-status">"Status:"</label>
                        <select 
                            id="filter-status"
                            on:change=move |ev| {
                                set_filter_status.set(event_target_value(&ev));
                                set_page.set(0); // Reset to first page on filter change
                            }
                        >
                            <option value="all" selected=move || filter_status() == "all">"All Statuses"</option>
                            <option value="success" selected=move || filter_status() == "success">"Success"</option>
                            <option value="failed" selected=move || filter_status() == "failed">"Failed"</option>
                        </select>
                    </div>
                </div>
                
                <div class="history-list">
                    {move || {
                        let entries = paginated_history();
                        
                        if entries.is_empty() {
                            view! {
                                <div class="empty-history">
                                    <i class="fas fa-info-circle"></i>
                                    <p>"No sync history entries found with the current filters."</p>
                                </div>
                            }
                        } else {
                            view! {
                                <table class="history-table">
                                    <thead>
                                        <tr>
                                            <th>"Type"</th>
                                            <th>"Operation"</th>
                                            <th>"Status"</th>
                                            <th>"Time"</th>
                                            <th>"Duration"</th>
                                            <th>"Actions"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {entries.into_iter().map(|entry| {
                                            let entry_id = entry.id.clone();
                                            view! {
                                                <tr class=if entry.success { "" } else { "error-row" }>
                                                    <td class="entity-type">
                                                        <i class={get_entity_icon(&entry.entity_type)}></i>
                                                        <span>{&entry.entity_type}</span>
                                                    </td>
                                                    <td>{&entry.operation}</td>
                                                    <td>
                                                        <span class={get_status_badge_class(entry.success)}>
                                                            {get_status_badge_text(entry.success)}
                                                        </span>
                                                    </td>
                                                    <td class="timestamp">{format_time_ago(&entry.timestamp)}</td>
                                                    <td>{format!("{} ms", entry.duration_ms)}</td>
                                                    <td class="actions">
                                                        <button 
                                                            class="btn btn-sm btn-link"
                                                            on:click=move |_| on_view_details.call(entry_id.clone())
                                                        >
                                                            <i class="fas fa-info-circle"></i>
                                                            " Details"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            }
                        }
                    }}
                </div>
                
                <div class="pagination-controls">
                    <div class="pagination-info">
                        {move || {
                            let filtered = filtered_history();
                            let start = if filtered.is_empty() { 0 } else { page() * page_size + 1 };
                            let end = ((page() + 1) * page_size).min(filtered.len());
                            
                            format!("Showing {} to {} of {} entries", start, end, filtered.len())
                        }}
                    </div>
                    
                    <div class="pagination-buttons">
                        <button 
                            class="btn btn-sm"
                            disabled=move || page() == 0
                            on:click=move |_| set_page.update(|p| *p = p.saturating_sub(1))
                        >
                            <i class="fas fa-chevron-left"></i>
                            " Previous"
                        </button>
                        
                        <span class="page-indicator">
                            {move || format!("Page {} of {}", page() + 1, total_pages().max(1))}
                        </span>
                        
                        <button 
                            class="btn btn-sm"
                            disabled=move || page() + 1 >= total_pages() || filtered_history().is_empty()
                            on:click=move |_| set_page.update(|p| *p = (*p + 1).min(total_pages() - 1))
                        >
                            "Next "
                            <i class="fas fa-chevron-right"></i>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Helper to get value from an event
fn event_target_value(ev: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}