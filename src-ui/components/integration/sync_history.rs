use leptos::*;
use crate::models::integration::SyncHistoryEntry;

#[component]
pub fn SyncHistory(
    history: Signal<Vec<SyncHistoryEntry>>,
) -> impl IntoView {
    // Pagination state
    let (page, set_page) = create_signal(0);
    let (rows_per_page, set_rows_per_page) = create_signal(10);
    
    // Handle page change
    let handle_page_change = move |new_page: usize| {
        set_page.set(new_page);
    };
    
    // Handle rows per page change
    let handle_rows_per_page_change = move |new_rows: usize| {
        set_rows_per_page.set(new_rows);
        set_page.set(0);
    };
    
    view! {
        <div class="sync-history-container">
            {move || if history().is_empty() {
                view! {
                    <div class="empty-list">
                        <p>"No synchronization history found. This will populate as sync operations are performed."</p>
                    </div>
                }
            } else {
                view! {
                    <div class="table-responsive">
                        <table class="table">
                            <thead>
                                <tr>
                                    <th>"Status"</th>
                                    <th>"Sync Type"</th>
                                    <th>"Entity ID"</th>
                                    <th>"Entity Type"</th>
                                    <th class="text-center">"Timestamp"</th>
                                    <th class="text-right">"Duration (ms)"</th>
                                    <th>"Error"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || {
                                    let start = page() * rows_per_page();
                                    let end = start + rows_per_page();
                                    
                                    history()
                                        .iter()
                                        .skip(start)
                                        .take(rows_per_page())
                                        .map(|entry| {
                                            view! {
                                                <tr>
                                                    <td>
                                                        {if entry.success {
                                                            view! {
                                                                <span class="status-badge status-success">
                                                                    <i class="icon-check"></i>
                                                                    "Success"
                                                                </span>
                                                            }
                                                        } else {
                                                            view! {
                                                                <span class="status-badge status-error">
                                                                    <i class="icon-error"></i>
                                                                    "Failed"
                                                                </span>
                                                            }
                                                        }}
                                                    </td>
                                                    <td>{format_sync_type(&entry.sync_type)}</td>
                                                    <td class="entity-id">{truncate_id(&entry.content_id)}</td>
                                                    <td>{format_entity_type(&entry.content_type)}</td>
                                                    <td class="text-center">{entry.sync_time.clone()}</td>
                                                    <td class="text-right">{entry.duration_ms}</td>
                                                    <td>
                                                        {if let Some(error) = &entry.error_message {
                                                            view! {
                                                                <div class="error-tooltip" title={error}>
                                                                    <i class="icon-info error-icon"></i>
                                                                </div>
                                                            }
                                                        } else {
                                                            view! { <></> }
                                                        }}
                                                    </td>
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }}
                            </tbody>
                        </table>
                    </div>
                    
                    <div class="pagination-controls">
                        <div class="rows-per-page">
                            <span>"Rows per page:"</span>
                            <select 
                                on:change=move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                                        handle_rows_per_page_change(value);
                                    }
                                }
                            >
                                <option value="5" selected={rows_per_page() == 5}>"5"</option>
                                <option value="10" selected={rows_per_page() == 10}>"10"</option>
                                <option value="25" selected={rows_per_page() == 25}>"25"</option>
                            </select>
                        </div>
                        
                        <div class="page-navigation">
                            <button 
                                class="btn btn-sm"
                                disabled={page() == 0}
                                on:click=move |_| {
                                    if page() > 0 {
                                        handle_page_change(page() - 1);
                                    }
                                }
                            >
                                "Previous"
                            </button>
                            
                            <span class="page-info">
                                {"Page "}{page() + 1}{" of "}{(history().len() + rows_per_page() - 1) / rows_per_page()}
                            </span>
                            
                            <button
                                class="btn btn-sm"
                                disabled={page() >= (history().len() - 1) / rows_per_page()}
                                on:click=move |_| {
                                    if page() < (history().len() - 1) / rows_per_page() {
                                        handle_page_change(page() + 1);
                                    }
                                }
                            >
                                "Next"
                            </button>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

// Helper to format sync type for display
fn format_sync_type(sync_type: &str) -> String {
    if sync_type.is_empty() {
        return "Unknown".to_string();
    }
    
    // Convert from snake_case or camelCase to Title Case with spaces
    sync_type
        .replace('_', " ")
        .replace('-', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// Helper to format entity type for display
fn format_entity_type(entity_type: &str) -> String {
    if entity_type.is_empty() {
        return "Unknown".to_string();
    }
    
    // Convert from snake_case or camelCase to Title Case with spaces
    entity_type
        .replace('_', " ")
        .replace('-', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// Helper to truncate long IDs
fn truncate_id(id: &str) -> String {
    if id.len() <= 12 {
        return id.to_string();
    }
    
    format!("{}...{}", &id[0..6], &id[id.len() - 6..])
}

// Helper to get value from an event
fn event_target_value(ev: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}
