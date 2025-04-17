use leptos::*;
use crate::models::integration::CanvasCourse;

/// Badge style for sync status
#[derive(Clone, Debug)]
pub enum SyncStatusBadge {
    Success,
    Warning,
    Error,
    Info,
    Default,
}

impl SyncStatusBadge {
    /// Get the CSS class for the badge
    fn css_class(&self) -> &'static str {
        match self {
            SyncStatusBadge::Success => "status-badge status-success",
            SyncStatusBadge::Warning => "status-badge status-warning",
            SyncStatusBadge::Error => "status-badge status-error",
            SyncStatusBadge::Info => "status-badge status-info",
            SyncStatusBadge::Default => "status-badge status-default",
        }
    }
    
    /// Get the badge type for a sync status
    fn from_status(status: &str) -> Self {
        match status {
            "Synced" => SyncStatusBadge::Success,
            "PendingToCanvas" | "PendingToDiscourse" | "PendingSync" => SyncStatusBadge::Warning,
            "Conflict" => SyncStatusBadge::Error,
            "Error" => SyncStatusBadge::Error,
            "Processing" => SyncStatusBadge::Info,
            _ => SyncStatusBadge::Default,
        }
    }
}

#[component]
pub fn CoursesList(
    #[prop(into)] courses: Signal<Vec<CanvasCourse>>,
    #[prop(into)] sync_course: Callback<String>,
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
        <div class="courses-list-container">
            {move || {
                let courses_list = courses.get();
                
                if courses_list.is_empty() {
                    view! {
                        <div class="empty-list">
                            <p>"No courses found. Courses will appear here once they are synchronized with Canvas."</p>
                        </div>
                    }
                } else {
                    view! {
                        <div class="table-responsive">
                            <table class="table">
                                <thead>
                                    <tr>
                                        <th>"Course Name"</th>
                                        <th>"Code"</th>
                                        <th>"Term"</th>
                                        <th>"Status"</th>
                                        <th class="text-right">"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {move || {
                                        let start = page() * rows_per_page();
                                        let end = start + rows_per_page();
                                        
                                        courses_list
                                            .iter()
                                            .skip(start)
                                            .take(rows_per_page())
                                            .map(|course| {
                                                let course_id = course.id.clone();
                                                let badge = SyncStatusBadge::from_status(&course.sync_status);
                                                
                                                view! {
                                                    <tr>
                                                        <td class="course-name">{&course.name}</td>
                                                        <td>{&course.course_code}</td>
                                                        <td>{course.term.clone().unwrap_or_else(|| "N/A".to_string())}</td>
                                                        <td>
                                                            <span class={badge.css_class()}>
                                                                <i class=get_status_icon(&course.sync_status)></i>
                                                                {&course.sync_status}
                                                            </span>
                                                        </td>
                                                        <td class="text-right">
                                                            <div class="action-buttons">
                                                                <button 
                                                                    class="btn btn-sm btn-primary"
                                                                    on:click=move |_| sync_course.call(course_id.clone())
                                                                    disabled=loading.get() || get_sync_disabled_status(&course.sync_status)
                                                                >
                                                                    {get_sync_button_text(&course.sync_status)}
                                                                </button>
                                                                
                                                                <a 
                                                                    href=format!("/courses/{}", course.id)
                                                                    class="btn btn-sm btn-secondary"
                                                                >
                                                                    "View"
                                                                </a>

                                                                {if course.sync_status == "Conflict" {
                                                                    view! {
                                                                        <a 
                                                                            href=format!("/conflicts/{}", course.id)
                                                                            class="btn btn-sm btn-warning"
                                                                        >
                                                                            "Resolve Conflict"
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    view! {}
                                                                }}
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
                                    {"Page "}{page() + 1}{" of "}{(courses_list.len() + rows_per_page() - 1) / rows_per_page()}
                                </span>
                                
                                <button
                                    class="btn btn-sm"
                                    disabled={page() >= (courses_list.len() - 1) / rows_per_page()}
                                    on:click=move |_| {
                                        if page() < (courses_list.len() - 1) / rows_per_page() {
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

/// Get appropriate icon class for a sync status
fn get_status_icon(status: &str) -> &'static str {
    match status {
        "Synced" => "fas fa-check-circle",
        "PendingToCanvas" | "PendingToDiscourse" | "PendingSync" => "fas fa-sync",
        "Conflict" => "fas fa-exclamation-triangle",
        "Error" => "fas fa-times-circle",
        "Processing" => "fas fa-spinner fa-spin",
        _ => "fas fa-question-circle",
    }
}

/// Get sync button text based on sync status
fn get_sync_button_text(status: &str) -> &'static str {
    match status {
        "Synced" => "Sync Again",
        "PendingToCanvas" => "Sync to Canvas",
        "PendingToDiscourse" => "Sync to Discourse",
        "PendingSync" => "Sync Now",
        "Conflict" => "Sync After Resolve",
        "Error" => "Retry Sync",
        "Processing" => "Syncing...",
        _ => "Sync",
    }
}

/// Determine if sync button should be disabled
fn get_sync_disabled_status(status: &str) -> bool {
    matches!(status, "Processing" | "Conflict")
}

// Helper to get value from an event
fn event_target_value(ev: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}
