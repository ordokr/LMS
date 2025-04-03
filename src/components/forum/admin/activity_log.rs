use leptos::*;
use crate::services::admin::AdminService;
use crate::models::admin::{ActivityLog, ActivityType};

#[component]
pub fn ActivityLog() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin() || s.is_moderator()).unwrap_or(false);
    
    // State signals
    let (logs, set_logs) = create_signal(Vec::<ActivityLog>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Pagination
    let (current_page, set_current_page) = create_signal(1);
    let (total_pages, set_total_pages) = create_signal(1);
    
    // Filtering
    let (filter_type, set_filter_type) = create_signal(None::<ActivityType>);
    let (filter_user, set_filter_user) = create_signal(String::new());
    let (date_from, set_date_from) = create_signal(String::new());
    let (date_to, set_date_to) = create_signal(String::new());
    
    // Load activity logs
    let load_logs = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match AdminService::get_activity_logs(
                current_page(),
                20,
                filter_type(),
                if filter_user().is_empty() { None } else { Some(filter_user()) },
                if date_from().is_empty() { None } else { Some(date_from()) },
                if date_to().is_empty() { None } else { Some(date_to()) },
            ).await {
                Ok(page) => {
                    set_logs.set(page.logs);
                    set_total_pages.set(page.total_pages);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load activity logs: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initial load
    create_effect(move |_| {
        if is_admin() {
            load_logs();
        } else {
            set_loading.set(false);
        }
    });
    
    // Apply filters
    let apply_filters = move |_| {
        set_current_page.set(1);
        load_logs();
    };
    
    // Reset filters
    let reset_filters = move |_| {
        set_filter_type.set(None);
        set_filter_user.set(String::new());
        set_date_from.set(String::new());
        set_date_to.set(String::new());
        set_current_page.set(1);
        load_logs();
    };
    
    // Go to page
    let go_to_page = move |page: usize| {
        set_current_page.set(page);
        load_logs();
    };
    
    // Format activity description
    let format_activity = move |log: &ActivityLog| -> String {
        match log.activity_type {
            ActivityType::UserLogin => format!("User logged in"),
            ActivityType::UserLogout => format!("User logged out"),
            ActivityType::UserRegistration => format!("User registered"),
            ActivityType::UserProfileUpdate => format!("Updated profile"),
            ActivityType::TopicCreated => format!("Created topic: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::TopicUpdated => format!("Updated topic: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::TopicDeleted => format!("Deleted topic: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::PostCreated => format!("Created post in: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::PostUpdated => format!("Updated post in: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::PostDeleted => format!("Deleted post in: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::CategoryCreated => format!("Created category: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::CategoryUpdated => format!("Updated category: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::CategoryDeleted => format!("Deleted category: {}", log.target_name.clone().unwrap_or_default()),
            ActivityType::ModeratorAction => format!("Moderator action: {}", log.details.clone().unwrap_or_default()),
            ActivityType::AdminAction => format!("Admin action: {}", log.details.clone().unwrap_or_default()),
            ActivityType::SystemEvent => format!("System event: {}", log.details.clone().unwrap_or_default()),
            ActivityType::Other => log.details.clone().unwrap_or_else(|| "Unknown action".to_string()),
        }
    };

    view! {
        <div class="activity-log">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <div>
                        <h1 class="mb-4">"Activity Log"</h1>
                        
                        {move || error().map(|err| view! {
                            <div class="alert alert-danger">{err}</div>
                        })}
                        
                        <div class="card mb-4">
                            <div class="card-header">
                                <h5 class="mb-0">"Filters"</h5>
                            </div>
                            <div class="card-body">
                                <div class="row g-3">
                                    <div class="col-md-6 col-lg-3">
                                        <label for="filterType" class="form-label">"Activity Type"</label>
                                        <select
                                            id="filterType"
                                            class="form-select"
                                            on:change=move |ev| {
                                                let value = event_target_value(&ev);
                                                set_filter_type.set(
                                                    if value.is_empty() {
                                                        None
                                                    } else {
                                                        Some(match value.as_str() {
                                                            "UserLogin" => ActivityType::UserLogin,
                                                            "UserLogout" => ActivityType::UserLogout,
                                                            "UserRegistration" => ActivityType::UserRegistration,
                                                            "UserProfileUpdate" => ActivityType::UserProfileUpdate,
                                                            "TopicCreated" => ActivityType::TopicCreated,
                                                            "TopicUpdated" => ActivityType::TopicUpdated,
                                                            "TopicDeleted" => ActivityType::TopicDeleted,
                                                            "PostCreated" => ActivityType::PostCreated,
                                                            "PostUpdated" => ActivityType::PostUpdated,
                                                            "PostDeleted" => ActivityType::PostDeleted,
                                                            "CategoryCreated" => ActivityType::CategoryCreated,
                                                            "CategoryUpdated" => ActivityType::CategoryUpdated,
                                                            "CategoryDeleted" => ActivityType::CategoryDeleted,
                                                            "ModeratorAction" => ActivityType::ModeratorAction,
                                                            "AdminAction" => ActivityType::AdminAction,
                                                            "SystemEvent" => ActivityType::SystemEvent,
                                                            _ => ActivityType::Other
                                                        })
                                                    }
                                                );
                                            }
                                        >
                                            <option value="">"All Types"</option>
                                            <optgroup label="User Activities">
                                                <option value="UserLogin">"User Login"</option>
                                                <option value="UserLogout">"User Logout"</option>
                                                <option value="UserRegistration">"User Registration"</option>
                                                <option value="UserProfileUpdate">"Profile Update"</option>
                                            </optgroup>
                                            <optgroup label="Content Activities">
                                                <option value="TopicCreated">"Topic Created"</option>
                                                <option value="TopicUpdated">"Topic Updated"</option>
                                                <option value="TopicDeleted">"Topic Deleted"</option>
                                                <option value="PostCreated">"Post Created"</option>
                                                <option value="PostUpdated">"Post Updated"</option>
                                                <option value="PostDeleted">"Post Deleted"</option>
                                            </optgroup>
                                            <optgroup label="Admin Activities">
                                                <option value="CategoryCreated">"Category Created"</option>
                                                <option value="CategoryUpdated">"Category Updated"</option>
                                                <option value="CategoryDeleted">"Category Deleted"</option>
                                                <option value="ModeratorAction">"Moderator Action"</option>
                                                <option value="AdminAction">"Admin Action"</option>
                                            </optgroup>
                                            <option value="SystemEvent">"System Event"</option>
                                            <option value="Other">"Other"</option>
                                        </select>
                                    </div>
                                    
                                    <div class="col-md-6 col-lg-3">
                                        <label for="filterUser" class="form-label">"User"</label>
                                        <input
                                            id="filterUser"
                                            type="text"
                                            class="form-control"
                                            placeholder="Username or ID"
                                            prop:value=move || filter_user()
                                            on:input=move |ev| set_filter_user.set(event_target_value(&ev))
                                        />
                                    </div>
                                    
                                    <div class="col-md-6 col-lg-3">
                                        <label for="dateFrom" class="form-label">"From Date"</label>
                                        <input
                                            id="dateFrom"
                                            type="date"
                                            class="form-control"
                                            prop:value=move || date_from()
                                            on:input=move |ev| set_date_from.set(event_target_value(&ev))
                                        />
                                    </div>
                                    
                                    <div class="col-md-6 col-lg-3">
                                        <label for="dateTo" class="form-label">"To Date"</label>
                                        <input
                                            id="dateTo"
                                            type="date"
                                            class="form-control"
                                            prop:value=move || date_to()
                                            on:input=move |ev| set_date_to.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                                
                                <div class="mt-3 d-flex gap-2">
                                    <button class="btn btn-primary" on:click=apply_filters>
                                        <i class="bi bi-filter me-1"></i>
                                        "Apply Filters"
                                    </button>
                                    <button class="btn btn-outline-secondary" on:click=reset_filters>
                                        "Reset Filters"
                                    </button>
                                </div>
                            </div>
                        </div>
                        
                        {move || if loading() {
                            view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                        } else if logs().is_empty() {
                            view! {
                                <div class="text-center p-5">
                                    <i class="bi bi-journal mb-3 d-block" style="font-size: 3rem;"></i>
                                    <h3>"No activity logs found"</h3>
                                    <p class="text-muted">"No activities match your search criteria."</p>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="table-responsive">
                                    <table class="table table-striped table-hover">
                                        <thead>
                                            <tr>
                                                <th>"Time"</th>
                                                <th>"User"</th>
                                                <th>"IP Address"</th>
                                                <th>"Activity"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {logs().into_iter().map(|log| {
                                                let activity_description = format_activity(&log);
                                                
                                                view! {
                                                    <tr>
                                                        <td>{format_datetime(log.created_at)}</td>
                                                        <td>
                                                            {if log.user_id > 0 {
                                                                view! {
                                                                    <a href={format!("/users/{}", log.user_id)}>
                                                                        {log.username.unwrap_or_else(|| format!("User #{}", log.user_id))}
                                                                    </a>
                                                                }
                                                            } else {
                                                                view! { <span class="text-muted">"System"</span> }
                                                            }}
                                                        </td>
                                                        <td>
                                                            <code>{log.ip_address.unwrap_or_else(|| "N/A".to_string())}</code>
                                                        </td>
                                                        <td>
                                                            <div>{activity_description}</div>
                                                            {log.target_id.map(|id| {
                                                                if id > 0 {
                                                                    view! { <small class="text-muted">{format!("Target ID: {}", id)}</small> }
                                                                } else {
                                                                    view! {}
                                                                }
                                                            })}
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>
                                </div>
                                
                                {move || if total_pages() > 1 {
                                    view! {
                                        <nav aria-label="Activity log pagination" class="mt-4">
                                            <ul class="pagination justify-content-center">
                                                <li class=format!("page-item {}", if current_page() <= 1 { "disabled" } else { "" })>
                                                    <button class="page-link" on:click=move |_| go_to_page(current_page() - 1)>
                                                        "Previous"
                                                    </button>
                                                </li>
                                                
                                                {(1..=total_pages()).map(|page| {
                                                    view! {
                                                        <li class=format!("page-item {}", if page == current_page() { "active" } else { "" })>
                                                            <button class="page-link" on:click=move |_| go_to_page(page)>
                                                                {page}
                                                            </button>
                                                        </li>
                                                    }
                                                }).collect::<Vec<_>>()}
                                                
                                                <li class=format!("page-item {}", if current_page() >= total_pages() { "disabled" } else { "" })>
                                                    <button class="page-link" on:click=move |_| go_to_page(current_page() + 1)>
                                                        "Next"
                                                    </button>
                                                </li>
                                            </ul>
                                        </nav>
                                    }
                                } else {
                                    view! {}
                                }}
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
    date.format("%b %d, %Y %H:%M:%S").to_string()
}