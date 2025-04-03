use leptos::*;
use crate::services::admin::AdminService;
use crate::models::admin::{AdminStats, ActivityData};
use crate::components::charts::{LineChart, PieChart};

#[component]
pub fn AdminDashboard() -> impl IntoView {
    // Check if user is admin
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (stats, set_stats) = create_signal(None::<AdminStats>);
    let (activity_data, set_activity_data) = create_signal(None::<ActivityData>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (time_range, set_time_range) = create_signal("30days".to_string());
    
    // Load dashboard data
    let load_dashboard = move || {
        set_loading.set(true);
        
        spawn_local(async move {
            // Load admin stats
            match AdminService::get_stats().await {
                Ok(admin_stats) => {
                    set_stats.set(Some(admin_stats));
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load admin stats: {}", e)));
                }
            }
            
            // Load activity data for the selected time range
            match AdminService::get_activity_data(&time_range()).await {
                Ok(data) => {
                    set_activity_data.set(Some(data));
                },
                Err(e) => {
                    log::error!("Failed to load activity data: {}", e);
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Initial load
    create_effect(move |_| {
        load_dashboard();
    });
    
    // Handle time range change
    let handle_time_range_change = move |new_range: String| {
        set_time_range.set(new_range);
        load_dashboard();
    };

    view! {
        <div class="admin-dashboard">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <h1 class="mb-4">"Admin Dashboard"</h1>
                    
                    {move || if loading() {
                        view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                    } else if let Some(err) = error() {
                        view! { <div class="alert alert-danger mb-4">{err}</div> }
                    } else if let Some(s) = stats() {
                        view! {
                            <div>
                                <div class="row g-4 mb-4">
                                    <div class="col-md-3">
                                        <div class="card h-100 border-primary">
                                            <div class="card-body">
                                                <h5 class="card-title text-primary">"Users"</h5>
                                                <div class="d-flex align-items-center">
                                                    <div class="display-4 me-3">{s.total_users}</div>
                                                    <div class="text-success">
                                                        <i class="bi bi-graph-up"></i>
                                                        <span>{format!("+{} today", s.new_users_today)}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="col-md-3">
                                        <div class="card h-100 border-success">
                                            <div class="card-body">
                                                <h5 class="card-title text-success">"Topics"</h5>
                                                <div class="d-flex align-items-center">
                                                    <div class="display-4 me-3">{s.total_topics}</div>
                                                    <div class="text-success">
                                                        <i class="bi bi-graph-up"></i>
                                                        <span>{format!("+{} today", s.new_topics_today)}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="col-md-3">
                                        <div class="card h-100 border-info">
                                            <div class="card-body">
                                                <h5 class="card-title text-info">"Posts"</h5>
                                                <div class="d-flex align-items-center">
                                                    <div class="display-4 me-3">{s.total_posts}</div>
                                                    <div class="text-success">
                                                        <i class="bi bi-graph-up"></i>
                                                        <span>{format!("+{} today", s.new_posts_today)}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="col-md-3">
                                        <div class="card h-100 border-warning">
                                            <div class="card-body">
                                                <h5 class="card-title text-warning">"Page Views"</h5>
                                                <div class="d-flex align-items-center">
                                                    <div class="display-4 me-3">{s.total_page_views}</div>
                                                    <div class="text-success">
                                                        <i class="bi bi-graph-up"></i>
                                                        <span>{format!("+{} today", s.page_views_today)}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                
                                {move || {
                                    if let Some(activity) = activity_data() {
                                        view! {
                                            <div class="row g-4">
                                                <div class="col-md-8">
                                                    <div class="card h-100">
                                                        <div class="card-header d-flex justify-content-between align-items-center">
                                                            <h5 class="mb-0">"Activity over time"</h5>
                                                            <div class="btn-group btn-group-sm">
                                                                <button 
                                                                    type="button" 
                                                                    class=format!("btn {}", if time_range() == "7days" { "btn-primary" } else { "btn-outline-primary" })
                                                                    on:click=move |_| handle_time_range_change("7days".to_string())
                                                                >
                                                                    "7 days"
                                                                </button>
                                                                <button 
                                                                    type="button" 
                                                                    class=format!("btn {}", if time_range() == "30days" { "btn-primary" } else { "btn-outline-primary" })
                                                                    on:click=move |_| handle_time_range_change("30days".to_string())
                                                                >
                                                                    "30 days"
                                                                </button>
                                                                <button 
                                                                    type="button" 
                                                                    class=format!("btn {}", if time_range() == "90days" { "btn-primary" } else { "btn-outline-primary" })
                                                                    on:click=move |_| handle_time_range_change("90days".to_string())
                                                                >
                                                                    "90 days"
                                                                </button>
                                                            </div>
                                                        </div>
                                                        <div class="card-body">
                                                            <LineChart
                                                                data=activity.time_series
                                                                height=300
                                                                show_legend=true
                                                            />
                                                        </div>
                                                    </div>
                                                </div>
                                                
                                                <div class="col-md-4">
                                                    <div class="card h-100">
                                                        <div class="card-header">
                                                            <h5 class="mb-0">"Content Distribution"</h5>
                                                        </div>
                                                        <div class="card-body">
                                                            <PieChart
                                                                data=activity.distribution
                                                                height=200
                                                                show_labels=true
                                                            />
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        view! {}
                                    }
                                }}
                                
                                <div class="row g-4 mt-4">
                                    <div class="col-md-6">
                                        <div class="card h-100">
                                            <div class="card-header">
                                                <h5 class="mb-0">"Popular Topics"</h5>
                                            </div>
                                            <div class="card-body p-0">
                                                <div class="list-group list-group-flush">
                                                    {s.popular_topics.into_iter().map(|topic| {
                                                        view! {
                                                            <a href={format!("/forum/topics/{}", topic.id)} class="list-group-item list-group-item-action">
                                                                <div class="d-flex justify-content-between align-items-center">
                                                                    <div>
                                                                        <h6 class="mb-1">{topic.title}</h6>
                                                                        <small class="text-muted">
                                                                            "by " {topic.author_name}
                                                                            " in " {topic.category_name}
                                                                        </small>
                                                                    </div>
                                                                    <span class="badge bg-primary rounded-pill">
                                                                        {format!("{} views", topic.view_count)}
                                                                    </span>
                                                                </div>
                                                            </a>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="col-md-6">
                                        <div class="card h-100">
                                            <div class="card-header">
                                                <h5 class="mb-0">"Top Contributors"</h5>
                                            </div>
                                            <div class="card-body p-0">
                                                <div class="list-group list-group-flush">
                                                    {s.top_contributors.into_iter().map(|user| {
                                                        view! {
                                                            <a href={format!("/users/{}", user.id)} class="list-group-item list-group-item-action">
                                                                <div class="d-flex justify-content-between align-items-center">
                                                                    <div class="d-flex align-items-center">
                                                                        <img 
                                                                            src={user.avatar_url.unwrap_or_else(|| format!("https://ui-avatars.com/api/?name={}&background=random", user.name))}
                                                                            alt={format!("{}'s avatar", user.name)}
                                                                            class="rounded-circle me-2"
                                                                            style="width: 32px; height: 32px; object-fit: cover;"
                                                                        />
                                                                        <div>
                                                                            <h6 class="mb-0">{user.name}</h6>
                                                                            <small class="text-muted">
                                                                                "Member since " {format_date(user.created_at)}
                                                                            </small>
                                                                        </div>
                                                                    </div>
                                                                    <div class="text-end">
                                                                        <div class="fw-bold">{user.post_count} " posts"</div>
                                                                        <small class="text-muted">{user.topic_count} " topics"</small>
                                                                    </div>
                                                                </div>
                                                            </a>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="row g-4 mt-4">
                                    <div class="col-12">
                                        <div class="card">
                                            <div class="card-header">
                                                <h5 class="mb-0">"Quick Actions"</h5>
                                            </div>
                                            <div class="card-body">
                                                <div class="d-flex flex-wrap gap-2">
                                                    <a href="/admin/users" class="btn btn-outline-primary">
                                                        <i class="bi bi-people me-1"></i>
                                                        "Manage Users"
                                                    </a>
                                                    <a href="/admin/categories" class="btn btn-outline-secondary">
                                                        <i class="bi bi-folder me-1"></i>
                                                        "Manage Categories"
                                                    </a>
                                                    <a href="/admin/tags" class="btn btn-outline-info">
                                                        <i class="bi bi-tags me-1"></i>
                                                        "Manage Tags"
                                                    </a>
                                                    <a href="/admin/reports" class="btn btn-outline-danger">
                                                        <i class="bi bi-flag me-1"></i>
                                                        "Moderation Queue"
                                                        {s.pending_reports_count.map(|count| {
                                                            if count > 0 {
                                                                view! {
                                                                    <span class="badge bg-danger ms-1">{count}</span>
                                                                }
                                                            } else {
                                                                view! {}
                                                            }
                                                        })}
                                                    </a>
                                                    <a href="/admin/settings" class="btn btn-outline-dark">
                                                        <i class="bi bi-gear me-1"></i>
                                                        "Forum Settings"
                                                    </a>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        view! { <div class="alert alert-warning">"Failed to load admin data."</div> }
                    }}
                }
            }}
        </div>
    }
}

// Helper function to format date
fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%b %d, %Y").to_string()
}