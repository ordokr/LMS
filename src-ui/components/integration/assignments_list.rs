use leptos::*;
use crate::models::integration::CanvasAssignment;

#[component]
pub fn AssignmentsList(
    #[prop(into)] assignments: Signal<Vec<CanvasAssignment>>,
    #[prop(into)] loading: Signal<bool>,
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
        <div class="assignments-list-container">
            {move || {
                let assignments_list = assignments.get();
                
                if assignments_list.is_empty() {
                    view! {
                        <div class="empty-list">
                            <p>"No assignments found. Assignments will appear here once they are synchronized with Canvas."</p>
                        </div>
                    }
                } else {
                    view! {
                        <div class="table-responsive">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>"Assignment Name"</th>
                                        <th>"Course"</th>
                                        <th>"Due Date"</th>
                                        <th>"Points"</th>
                                        <th>"Status"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || {
                                        let start = page() * rows_per_page();
                                        let end = start + rows_per_page();
                                        
                                        assignments_list
                                            .iter()
                                            .skip(start)
                                            .take(rows_per_page())
                                            .map(|assignment| {
                                                let status_class = match assignment.sync_status.as_str() {
                                                    "Synced" => "status-success",
                                                    "PendingToCanvas" | "PendingToDiscourse" => "status-warning",
                                                    "Conflict" => "status-error",
                                                    "Error" => "status-error",
                                                    "LocalOnly" => "status-default",
                                                    _ => "status-default",
                                                };
                                                
                                                view! {
                                                    <tr>
                                                        <td class="assignment-name">{&assignment.name}</td>
                                                        <td>{&assignment.course_name}</td>
                                                        <td>{format_date(&assignment.due_at)}</td>
                                                        <td>{assignment.points_possible}</td>
                                                        <td>
                                                            <span class=format!("status-badge {}", status_class)>
                                                                {&assignment.sync_status}
                                                            </span>
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
                                    {"Page "}{page() + 1}{" of "}{(assignments_list.len() + rows_per_page() - 1) / rows_per_page()}
                                </span>
                                
                                <button
                                    class="btn btn-sm"
                                    disabled={page() >= (assignments_list.len() - 1) / rows_per_page()}
                                    on:click=move |_| {
                                        if page() < (assignments_list.len() - 1) / rows_per_page() {
                                            handle_page_change(page() + 1);
                                        }
                                    }
                                >
                                    "Next"
                                </button>
                            </div>
                        </div>
                    }
                }
            }}
        </div>
    }
}

// Helper to format date
fn format_date(date_str: &Option<String>) -> String {
    match date_str {
        Some(date) if !date.is_empty() => {
            // Simple formatting for now
            date.replace('T', " ").replace('Z', "")
        },
        _ => "No due date".to_string()
    }
}

// Helper to get value from an event
fn event_target_value(ev: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}
