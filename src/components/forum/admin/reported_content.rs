use leptos::*;
use crate::services::admin::AdminService;
use crate::models::admin::{ReportedContent, ReportStatus, ReportDecision};

#[component]
pub fn ReportedContent() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin() || s.is_moderator()).unwrap_or(false);
    
    // State signals
    let (reports, set_reports) = create_signal(Vec::<ReportedContent>::new());
    let (loading, set_loading) = create_signal(true);
    let (processing, set_processing) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    let (filter, set_filter) = create_signal(ReportStatus::Pending);
    
    // Load reported content
    let load_reports = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match AdminService::get_reported_content(filter()).await {
                Ok(report_list) => {
                    set_reports.set(report_list);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load reported content: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initial load
    create_effect(move |_| {
        if is_admin() {
            load_reports();
        } else {
            set_loading.set(false);
        }
    });
    
    // Handle filter change
    let handle_filter_change = move |status: ReportStatus| {
        set_filter.set(status);
        load_reports();
    };
    
    // Handle report action
    let handle_action = move |report_id: i64, decision: ReportDecision, note: String| {
        set_processing.set(true);
        set_error.set(None);
        set_success.set(None);
        
        spawn_local(async move {
            match AdminService::process_report(report_id, decision, note).await {
                Ok(_) => {
                    set_success.set(Some("Report processed successfully".to_string()));
                    load_reports();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to process report: {}", e)));
                    set_processing.set(false);
                }
            }
        });
    };
    
    // Create a report action component
    let report_action = move |report: ReportedContent| {
        let report_id = report.id;
        let (note, set_note) = create_signal(String::new());
        
        view! {
            <div class="report-actions">
                <div class="mb-3">
                    <label class="form-label">"Moderator Note"</label>
                    <textarea
                        class="form-control"
                        prop:value=move || note()
                        on:input=move |ev| set_note.set(event_target_value(&ev))
                        rows="2"
                        placeholder="Add a note explaining your decision (optional)"
                    ></textarea>
                </div>
                
                <div class="d-flex gap-2">
                    <button
                        class="btn btn-danger"
                        on:click=move |_| handle_action(report_id, ReportDecision::Remove, note())
                        disabled=move || processing()
                    >
                        <i class="bi bi-trash me-1"></i>
                        "Remove Content"
                    </button>
                    
                    <button
                        class="btn btn-warning"
                        on:click=move |_| handle_action(report_id, ReportDecision::Warn, note())
                        disabled=move || processing()
                    >
                        <i class="bi bi-exclamation-triangle me-1"></i>
                        "Warn User"
                    </button>
                    
                    <button
                        class="btn btn-success"
                        on:click=move |_| handle_action(report_id, ReportDecision::Approve, note())
                        disabled=move || processing()
                    >
                        <i class="bi bi-check-circle me-1"></i>
                        "Approve Content"
                    </button>
                </div>
            </div>
        }
    };
    
    view! {
        <div class="reported-content">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"Reported Content"</h1>
                        
                        {move || error().map(|err| view! {
                            <div class="alert alert-danger">{err}</div>
                        })}
                        
                        {move || success().map(|msg| view! {
                            <div class="alert alert-success">{msg}</div>
                        })}
                        
                        <div class="mb-4">
                            <ul class="nav nav-tabs">
                                <li class="nav-item">
                                    <button 
                                        class={format!("nav-link {}", if filter() == ReportStatus::Pending { "active" } else { "" })}
                                        on:click=move |_| handle_filter_change(ReportStatus::Pending)
                                    >
                                        "Pending" 
                                        <span class="badge bg-danger ms-1">
                                            {move || reports().iter().filter(|r| r.status == ReportStatus::Pending).count()}
                                        </span>
                                    </button>
                                </li>
                                <li class="nav-item">
                                    <button 
                                        class={format!("nav-link {}", if filter() == ReportStatus::Resolved { "active" } else { "" })}
                                        on:click=move |_| handle_filter_change(ReportStatus::Resolved)
                                    >
                                        "Resolved"
                                    </button>
                                </li>
                                <li class="nav-item">
                                    <button 
                                        class={format!("nav-link {}", if filter() == ReportStatus::Dismissed { "active" } else { "" })}
                                        on:click=move |_| handle_filter_change(ReportStatus::Dismissed)
                                    >
                                        "Dismissed"
                                    </button>
                                </li>
                            </ul>
                        </div>
                        
                        {move || if loading() {
                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                        } else if reports().is_empty() {
                            view! {
                                <div class="text-center p-5">
                                    <i class="bi bi-shield-check mb-3 d-block" style="font-size: 3rem;"></i>
                                    <h3>"No reports found"</h3>
                                    <p class="text-muted">
                                        {match filter() {
                                            ReportStatus::Pending => "There are no pending reports to review.",
                                            ReportStatus::Resolved => "There are no resolved reports.",
                                            ReportStatus::Dismissed => "There are no dismissed reports."
                                        }}
                                    </p>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="report-list">
                                    {reports().into_iter().map(|report| {
                                        let report_clone = report.clone();
                                        
                                        view! {
                                            <div class="card mb-4">
                                                <div class="card-header d-flex justify-content-between align-items-center">
                                                    <h5 class="mb-0">
                                                        {format!("Report #{}: {}", report.id, report.reason)}
                                                    </h5>
                                                    <span class={format!("badge {}", match report.status {
                                                        ReportStatus::Pending => "bg-danger",
                                                        ReportStatus::Resolved => "bg-success",
                                                        ReportStatus::Dismissed => "bg-secondary"
                                                    })}>
                                                        {format!("{:?}", report.status)}
                                                    </span>
                                                </div>
                                                <div class="card-body">
                                                    <div class="mb-3">
                                                        <h6>"Report Details"</h6>
                                                        <ul class="list-group list-group-flush">
                                                            <li class="list-group-item d-flex justify-content-between">
                                                                <span>"Reported By"</span>
                                                                <a href={format!("/users/{}", report.reporter_id)}>
                                                                    {report.reporter_name}
                                                                </a>
                                                            </li>
                                                            <li class="list-group-item d-flex justify-content-between">
                                                                <span>"Reported At"</span>
                                                                <span>{format_datetime(report.created_at)}</span>
                                                            </li>
                                                            <li class="list-group-item d-flex justify-content-between">
                                                                <span>"Content Type"</span>
                                                                <span>{report.content_type}</span>
                                                            </li>
                                                        </ul>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <h6>"Report Reason"</h6>
                                                        <p class="p-3 bg-light rounded">{report.reason}</p>
                                                        
                                                        {report.details.map(|details| {
                                                            if !details.is_empty() {
                                                                view! {
                                                                    <div>
                                                                        <h6>"Additional Details"</h6>
                                                                        <p class="p-3 bg-light rounded">{details}</p>
                                                                    </div>
                                                                }
                                                            } else {
                                                                view! {}
                                                            }
                                                        })}
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <h6>"Reported Content"</h6>
                                                        <div class="border p-3 rounded">
                                                            {match report.content_type.as_str() {
                                                                "Topic" => {
                                                                    view! {
                                                                        <div>
                                                                            <h5>
                                                                                <a href={format!("/forum/topics/{}", report.content_id)} target="_blank">
                                                                                    {report.content_title.unwrap_or_else(|| "View Topic".to_string())}
                                                                                </a>
                                                                            </h5>
                                                                            <div class="reported-content-preview">
                                                                                {report.content_excerpt.map(|excerpt| {
                                                                                    view! { <div class="text-muted">{excerpt}</div> }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                },
                                                                "Post" => {
                                                                    view! {
                                                                        <div>
                                                                            <h5>
                                                                                <a href={format!("/forum/topics/{}#post-{}", 
                                                                                                report.parent_id.unwrap_or(0), report.content_id)} 
                                                                                   target="_blank">
                                                                                    {"Post in: "}{report.content_title.unwrap_or_else(|| "Unknown Topic".to_string())}
                                                                                </a>
                                                                            </h5>
                                                                            <div class="reported-content-preview">
                                                                                {report.content_excerpt.map(|excerpt| {
                                                                                    view! { <div class="text-muted">{excerpt}</div> }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                },
                                                                "User" => {
                                                                    view! {
                                                                        <div>
                                                                            <h5>
                                                                                <a href={format!("/users/{}", report.content_id)} target="_blank">
                                                                                    {report.content_title.unwrap_or_else(|| "User Profile".to_string())}
                                                                                </a>
                                                                            </h5>
                                                                            <div class="reported-content-preview">
                                                                                {report.content_excerpt.map(|excerpt| {
                                                                                    view! { <div class="text-muted">{excerpt}</div> }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                },
                                                                _ => {
                                                                    view! {
                                                                        <div class="text-muted">
                                                                            "Unknown content type"
                                                                        </div>
                                                                    }
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                    
                                                    {match report.status {
                                                        ReportStatus::Pending => {
                                                            report_action(report_clone)
                                                        },
                                                        _ => {
                                                            view! {
                                                                <div class="report-resolution">
                                                                    <h6>"Resolution"</h6>
                                                                    <ul class="list-group list-group-flush">
                                                                        <li class="list-group-item d-flex justify-content-between">
                                                                            <span>"Resolved By"</span>
                                                                            <a href={format!("/users/{}", report.resolved_by.unwrap_or(0))}>
                                                                                {report.resolver_name.unwrap_or_else(|| "Unknown".to_string())}
                                                                            </a>
                                                                        </li>
                                                                        <li class="list-group-item d-flex justify-content-between">
                                                                            <span>"Resolved At"</span>
                                                                            <span>
                                                                                {report.resolved_at.map(|date| format_datetime(date))
                                                                                        .unwrap_or_else(|| "N/A".to_string())}
                                                                            </span>
                                                                        </li>
                                                                        <li class="list-group-item d-flex justify-content-between">
                                                                            <span>"Decision"</span>
                                                                            <span class={format!("badge {}", match report.decision {
                                                                                Some(ReportDecision::Remove) => "bg-danger",
                                                                                Some(ReportDecision::Warn) => "bg-warning",
                                                                                Some(ReportDecision::Approve) => "bg-success",
                                                                                None => "bg-secondary"
                                                                            })}>
                                                                                {report.decision.map(|d| format!("{:?}", d))
                                                                                        .unwrap_or_else(|| "None".to_string())}
                                                                            </span>
                                                                        </li>
                                                                    </ul>
                                                                    
                                                                    {report.resolution_note.map(|note| {
                                                                        if !note.is_empty() {
                                                                            view! {
                                                                                <div class="mt-3">
                                                                                    <h6>"Moderator Note"</h6>
                                                                                    <p class="p-3 bg-light rounded">{note}</p>
                                                                                </div>
                                                                            }
                                                                        } else {
                                                                            view! {}
                                                                        }
                                                                    })}
                                                                </div>
                                                            }
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
                }
            }}
        </div>
    }
}

// Helper function to format date and time
fn format_datetime(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%b %d, %Y %H:%M").to_string()
}