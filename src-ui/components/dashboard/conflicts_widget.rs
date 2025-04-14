use leptos::*;
use crate::models::integration::SyncConflict;

#[component]
pub fn ConflictsWidget(
    #[prop(into)] conflicts: Signal<Vec<SyncConflict>>,
    #[prop(into)] on_resolve: Callback<SyncConflict>,
    #[prop(default = 5)] max_entries: usize,
) -> impl IntoView {
    view! {
        <div class="widget conflicts-widget">
            <div class="widget-header">
                <h3 class="widget-title">"Sync Conflicts"</h3>
                
                <a href="/integrations/conflicts" class="btn btn-sm btn-link">
                    "View All"
                </a>
            </div>
            
            <div class="widget-content">
                {move || {
                    let conflict_list = conflicts.get();
                    
                    if conflict_list.is_empty() {
                        view! {
                            <div class="empty-widget">
                                <p>"No conflicts detected. All systems are in sync."</p>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="conflicts-list">
                                <table class="conflicts-table">
                                    <thead>
                                        <tr>
                                            <th>"Entity Type"</th>
                                            <th>"Title"</th>
                                            <th>"Detected At"</th>
                                            <th>"Actions"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {conflict_list.iter().take(max_entries).map(|conflict| {
                                            let conflict_clone = conflict.clone();
                                            let conflict_for_resolve = conflict.clone();
                                            
                                            view! {
                                                <tr>
                                                    <td>{&conflict.entity_type}</td>
                                                    <td class="conflict-title">{&conflict.title}</td>
                                                    <td>{format_datetime(&conflict.detected_at)}</td>
                                                    <td>
                                                        <button 
                                                            class="btn btn-sm btn-primary"
                                                            on:click=move |_| on_resolve.call(conflict_for_resolve.clone())
                                                        >
                                                            "Resolve"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                                
                                {move || {
                                    let total = conflict_list.len();
                                    if total > max_entries {
                                        view! {
                                            <div class="view-more">
                                                <a href="/integrations/conflicts" class="btn btn-link">
                                                    {"View all "}{total}{" conflicts"}
                                                </a>
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

// Helper function to format datetime
fn format_datetime(datetime_str: &str) -> String {
    // Simple formatting for now
    if datetime_str.contains('T') {
        datetime_str.replace('T', " ").replace('Z', "")
    } else {
        datetime_str.to_string()
    }
}
