use leptos::*;
use crate::models::integration::SyncHistoryEntry;

#[component]
pub fn SyncHistoryWidget(
    #[prop(into)] history: Signal<Vec<SyncHistoryEntry>>,
    #[prop(default = 5)] max_entries: usize,
) -> impl IntoView {
    view! {
        <div class="widget sync-history-widget">
            <div class="widget-header">
                <h3 class="widget-title">"Recent Sync Activity"</h3>
                
                <a href="/integrations/history" class="btn btn-sm btn-link">
                    "View All"
                </a>
            </div>
            
            <div class="widget-content">
                {move || {
                    let history_entries = history.get();
                    
                    if history_entries.is_empty() {
                        view! {
                            <div class="empty-widget">
                                <p>"No sync history available yet."</p>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="sync-history-list">
                                <table class="history-table">
                                    <thead>
                                        <tr>
                                            <th>"Type"</th>
                                            <th>"Content"</th>
                                            <th>"Time"</th>
                                            <th>"Status"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {history_entries.iter().take(max_entries).map(|entry| {
                                            let status_class = if entry.success {
                                                "status-success"
                                            } else {
                                                "status-error"
                                            };
                                            
                                            view! {
                                                <tr>
                                                    <td>{&entry.sync_type}</td>
                                                    <td>{&entry.content_type}</td>
                                                    <td>{format_time(&entry.sync_time)}</td>
                                                    <td>
                                                        <span class=format!("status-badge {}", status_class)>
                                                            {if entry.success { "Success" } else { "Failed" }}
                                                        </span>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
                        }
                    }
                }}
            </div>
        </div>
    }
}

// Helper function to format time
fn format_time(time_str: &str) -> String {
    // Simple formatting for now
    if time_str.len() > 16 {
        time_str[0..16].replace('T', " ")
    } else {
        time_str.to_string()
    }
}
