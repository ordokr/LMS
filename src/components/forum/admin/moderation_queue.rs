use leptos::*;
use crate::models::forum::Report;
use crate::services::admin::AdminService;

#[component]
pub fn ModerationQueue() -> impl IntoView {
    // Check if user is admin or moderator
    let auth_state = use_context::<AuthState>();
    let is_mod_or_admin = move || {
        auth_state.map(|s| s.is_admin() || s.is_moderator()).unwrap_or(false)
    };
    
    // State signals
    let (reports, set_reports) = create_signal(Vec::<Report>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    let (filter, set_filter) = create_signal("pending".to_string());
    
    // Load reports
    let load_reports = move || {
        set_loading.set(true);
        
        spawn_local(async move {
            match AdminService::get_reports(&filter()).await {
                Ok(report_list) => {
                    set_reports.set(report_list);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load reports: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initial load
    create_effect(move |_| {
        load_reports();
    });
    
    // Change filter
    let change_filter = move |new_filter: String| {
        set_filter.set(new_filter);
        load_reports();
    };
    
    // Approve report (remove content)
    let approve_report = move |report_id: i64, content_type: String, content_id: i64| {
        spawn_local(async move {
            match AdminService::approve_report(report_id, &content_type, content_id).await {
                Ok(_) => {
                    set_success.set(Some("Content removed and report approved.".to_string()));
                    load_reports();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to approve report: {}", e)));
                }
            }
        });
    };
    
    // Dismiss report (keep content)
    let dismiss_report = move |report_id: i64| {
        spawn_local(async move {
            match AdminService::dismiss_report(report_id).await {
                Ok(_) => {
                    set_success.set(Some("Report dismissed.".to_string()));
                    load_reports();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to dismiss report: {}", e)));
                }
            }
        });
    };
    
    // Format report status badge
    let status_badge = |status: &str| {
        match status {
            "pending" => view! { <span class="badge bg-warning">"Pending"</span> },
            "approved" => view! { <span class="badge bg-success">"Approved"</span> },
            "dismissed" => view! { <span class="badge bg-secondary">"Dismissed"</span> },
            _ => view! { <span class="badge bg-primary">{status}</span> }
        }
    };

    view! {
        <div class="moderation-queue">
            {move || if !is_mod_or_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <h1 class="mb-4">"Moderation Queue"</h1>
                    
                    {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                    {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                    
                    <div class="card mb-4">
                        <div class="card-header">
                            <ul class="nav nav-tabs card-header-tabs">
                                <li class="nav-item">
                                    <button 
                                        class=format!("nav-link {}", if filter() == "pending" { "active" } else { "" })
                                        on:click=move |_| change_filter("pending".to_string())
                                    >
                                        "Pending"
                                        {move || {
                                            let count = reports().iter().filter(|r| r.status == "pending").count();
                                            if count > 0 {
                                                view! { <span class="badge bg-danger ms-1">{count}</span> }
                                            } else {
                                                view! {}
                                            }
                                        }}
                                    </button>
                                </li>
                                <li class="nav-item">
                                    <button 
                                        class=format!("nav-link {}", if filter() == "approved" { "active" } else { "" })
                                        on:click=move |_| change_filter("approved".to_string())
                                    >
                                        "Approved"
                                    </button>
                                </li>
                                <li class="nav-item">
                                    <button 
                                        class=format!("nav-link {}", if filter() == "dismissed" { "active" } else { "" })
                                        on:click=move |_| change_filter("dismissed".to_string())
                                    >
                                        "Dismissed"
                                    </button>
                                </li>
                                <li class="nav-item">
                                    <button 
                                        class=format!("nav-link {}", if filter() == "all" { "active" } else { "" })
                                        on:click=move |_| change_filter("all".to_string())
                                    >
                                        "All"
                                    </button>
                                </li>
                            </ul>
                        </div>
                        <div class="card-body">
                            {move || if loading() {
                                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                            } else if reports().is_empty() {
                                view! {
                                    <div class="text-center p-5">
                                        <i class="bi bi-shield-check mb-3 d-block" style="font-size: 3rem;"></i>
                                        <h3>"No reports found"</h3>
                                        {if filter() == "pending" {
                                            view! { <p class="text-muted">"There are no pending reports to review."</p> }
                                        } else if filter() == "all" {
                                            view! { <p class="text-muted">"There are no reports in the system."</p> }
                                        } else {
                                            view! { <p class="text-muted">{format!("There are no {} reports.", filter())}</p> }
                                        }}
                                    </div>
                                }
                            } else {
                                view! {
                                    <div class="report-list">
                                        {reports().into_iter().map(|report| {
                                            let report_id = report.id;
                                            let content_type = report.content_type.clone();
                                            let content_id = report.content_id;
                                            let is_pending = report.status == "pending";
                                            
                                            view! {
                                                <div class="card mb-3 report-card">
                                                    <div class="card-header d-flex justify-content-between align-items-center">
                                                        <div>
                                                            <span class="badge bg-primary me-2">{format!("#{}", report.id)}</span>
                                                            <span class="me-2">
                                                                {match report.content_type.as_str() {
                                                                    "topic" => "Topic Report",
                                                                    "post" => "Post Report",
                                                                    "user" => "User Report",
                                                                    _ => "Report"
                                                                }}
                                                            </span>
                                                            {status_badge(&report.status)}
                                                        </div>
                                                        <small class="text-muted">
                                                            {format_date_time(report.created_at)}
                                                        </small>
                                                    </div>
                                                    <div class="card-body">
                                                        <div class="mb-3">
                                                            <h5 class="card-title">
                                                                "Reason: " {report.reason}
                                                            </h5>
                                                            <p class="mb-0 text-muted">
                                                                "Reported by: "
                                                                <a href={format!("/users/{}", report.reporter_id)}>
                                                                    {report.reporter_name.unwrap_or_else(|| "Unknown".to_string())}
                                                                </a>
                                                            </p>
                                                        </div>
                                                        
                                                        <div class="card mb-3 bg-light">
                                                            <div class="card-header">
                                                                <strong>"Reported Content"</strong>
                                                            </div>
                                                            <div class="card-body">
                                                                <h6>
                                                                    {match report.content_type.as_str() {
                                                                        "topic" => view! {
                                                                            <span>
                                                                                "Topic by "
                                                                                <a href={format!("/users/{}", report.content_author_id.unwrap_or(0))}>
                                                                                    {report.content_author_name.clone().unwrap_or_else(|| "Unknown".to_string())}
                                                                                </a>
                                                                            </span>
                                                                        },
                                                                        "post" => view! {
                                                                            <span>
                                                                                "Post by "
                                                                                <a href={format!("/users/{}", report.content_author_id.unwrap_or(0))}>
                                                                                    {report.content_author_name.clone().unwrap_or_else(|| "Unknown".to_string())}
                                                                                </a>
                                                                                " in "
                                                                                <a href={format!("/forum/topics/{}", report.topic_id.unwrap_or(0))}>
                                                                                    "topic"
                                                                                </a>
                                                                            </span>
                                                                        },
                                                                        "user" => view! {
                                                                            <span>
                                                                                "User: "
                                                                                <a href={format!("/users/{}", report.content_id)}>
                                                                                    {report.content_author_name.clone().unwrap_or_else(|| "Unknown".to_string())}
                                                                                </a>
                                                                            </span>
                                                                        },
                                                                        _ => view! { <span>"Unknown content type"</span> }
                                                                    }}
                                                                </h6>
                                                                <div class="content-preview mt-2 border-top pt-2">
                                                                    <p>{report.content_preview.unwrap_or_else(|| "No preview available".to_string())}</p>
                                                                </div>
                                                                <div class="mt-3">
                                                                    <a href={report.content_url.clone().unwrap_or_default()} class="btn btn-sm btn-outline-primary" target="_blank">
                                                                        "View Original Content"
                                                                    </a>
                                                                </div>
                                                            </div>
                                                        </div>
                                                        
                                                        {if is_pending {
                                                            view! {
                                                                <div class="d-flex justify-content-end gap-2">
                                                                    <button 
                                                                        class="btn btn-outline-success" 
                                                                        on:click=move |_| dismiss_report(report_id)
                                                                    >
                                                                        <i class="bi bi-check-lg me-1"></i>
                                                                        "Dismiss (Keep Content)"
                                                                    </button>
                                                                    <button 
                                                                        class="btn btn-outline-danger" 
                                                                        on:click=move |_| approve_report(report_id, content_type.clone(), content_id)
                                                                    >
                                                                        <i class="bi bi-trash me-1"></i>
                                                                        "Approve (Remove Content)"
                                                                    </button>
                                                                </div>
                                                            }
                                                        } else {
                                                            view! {
                                                                <div class="text-muted">
                                                                    <strong>"Resolution: "</strong>
                                                                    {if report.status == "approved" {
                                                                        "Content was removed"
                                                                    } else {
                                                                        "Report was dismissed"
                                                                    }}
                                                                    {report.resolved_by_name.map(|name| {
                                                                        view! {
                                                                            <span>
                                                                                " by "
                                                                                <a href={format!("/users/{}", report.resolved_by.unwrap_or(0))}>
                                                                                    {name}
                                                                                </a>
                                                                            </span>
                                                                        }
                                                                    })}
                                                                    {report.resolved_at.map(|date| {
                                                                        view! {
                                                                            <span>
                                                                                " on " {format_date_time(date)}
                                                                            </span>
                                                                        }
                                                                    })}
                                                                </div>
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to format date and time
fn format_date_time(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%b %d, %Y at %H:%M").to_string()
}