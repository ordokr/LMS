use leptos::*;
use crate::models::integration::{DiscourseTopic, SyncStatus};
use web_sys::MouseEvent;
use crate::services::shell::open_url;

#[component]
pub fn TopicsList(
    topics: Signal<Vec<DiscourseTopic>>,
    sync_topic: Callback<String>,
    loading: Signal<bool>,
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
    
    // Handle sync topic
    let handle_sync_topic = move |topic_id: String| {
        sync_topic.call(topic_id);
    };
    
    // Handle view in Discourse
    let handle_view_in_discourse = move |topic_id: String, url: String| move |_: MouseEvent| {
        spawn_local(async move {
            if let Err(e) = open_url(&url).await {
                log::error!("Failed to open Discourse topic: {}", e);
            }
        });
    };
    
    view! {
        <div class="topics-list-container">
            {move || if topics().is_empty() {
                view! {
                    <div class="empty-list">
                        <p>"No topics found. Topics will appear here once they are synchronized with Discourse."</p>
                    </div>
                }
            } else {
                view! {
                    <div class="table-responsive">
                        <table class="table">
                            <thead>
                                <tr>
                                    <th>"Title"</th>
                                    <th>"Category"</th>
                                    <th class="text-center">"Posts"</th>
                                    <th class="text-center">"Sync Status"</th>
                                    <th class="text-center">"Last Synced"</th>
                                    <th class="text-right">"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || {
                                    let start = page() * rows_per_page();
                                    let end = start + rows_per_page();
                                    
                                    topics()
                                        .iter()
                                        .skip(start)
                                        .take(rows_per_page())
                                        .map(|topic| {
                                            let topic_id = topic.id.clone();
                                            let discourse_url = topic.discourse_url.clone();
                                            let has_discourse_id = topic.discourse_topic_id.is_some();
                                            
                                            view! {
                                                <tr>
                                                    <td class="topic-title">{&topic.title}</td>
                                                    <td>{topic.category.as_deref().unwrap_or("Uncategorized")}</td>
                                                    <td class="text-center">{topic.post_count}</td>
                                                    <td class="text-center">
                                                        <span class={format!("status-badge status-{}", get_status_class(&topic.sync_status))}>
                                                            {&topic.sync_status}
                                                        </span>
                                                    </td>
                                                    <td class="text-center">
                                                        {topic.last_synced_at.as_deref().unwrap_or("Never")}
                                                    </td>
                                                    <td class="text-right">
                                                        <div class="action-buttons">
                                                            <button
                                                                class="btn btn-sm btn-outline-secondary"
                                                                disabled={!has_discourse_id}
                                                                title="View in Discourse"
                                                                on:click=handle_view_in_discourse(topic_id.clone(), discourse_url.clone().unwrap_or_default())
                                                            >
                                                                <i class="icon-external-link"></i>
                                                            </button>
                                                            
                                                            <button
                                                                class="btn btn-sm btn-primary"
                                                                disabled=loading()
                                                                title="Sync Topic"
                                                                on:click=move |_| handle_sync_topic(topic_id.clone())
                                                            >
                                                                <i class="icon-sync"></i>
                                                            </button>
                                                        </div>
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
                                {"Page "}{page() + 1}{" of "}{(topics().len() + rows_per_page() - 1) / rows_per_page()}
                            </span>
                            
                            <button
                                class="btn btn-sm"
                                disabled={page() >= (topics().len() - 1) / rows_per_page()}
                                on:click=move |_| {
                                    if page() < (topics().len() - 1) / rows_per_page() {
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

// Helper function to determine class name based on sync status
fn get_status_class(status: &str) -> &'static str {
    match status.to_lowercase().as_str() {
        "synced" => "success",
        "pending" => "warning",
        "error" => "error",
        "local only" => "default",
        _ => "default",
    }
}

// Helper to get value from an event
fn event_target_value(ev: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}
