use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::auth::AuthData;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardStats {
    pub total_users: i32,
    pub new_users_today: i32,
    pub total_topics: i32,
    pub new_topics_today: i32,
    pub total_posts: i32,
    pub new_posts_today: i32,
    pub flagged_content: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub action_type: String,
    pub entity_type: String,
    pub entity_id: i64,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: String,
    pub database_status: String,
    pub disk_space: String,
    pub memory_usage: String,
    pub uptime: String,
}

#[component]
pub fn AdminDashboard(cx: Scope) -> impl IntoView {
    // Auth check to ensure admin access
    let auth_data = use_context::<Signal<Option<AuthData>>>(cx)
        .expect("Auth data not provided");
    
    // Create signals for dashboard stats
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);
    let (stats, set_stats) = create_signal(cx, DashboardStats::default());
    let (recent_activity, set_recent_activity) = create_signal(cx, Vec::<ActivityItem>::new());
    let (system_health, set_system_health) = create_signal(cx, None::<SystemHealth>);
    
    // Fetch dashboard data
    create_effect(cx, move |_| {
        // Make sure user is admin
        if let Some(data) = auth_data.get() {
            if !data.user.is_admin {
                // Redirect non-admins
                let window = web_sys::window().unwrap();
                let _ = window.location().set_href("/");
                return;
            }
            
            // Fetch dashboard stats
            spawn_local(async move {
                set_loading.set(true);
                set_error.set(None);
                
                // Fetch stats from API
                let client = reqwest::Client::new();
                let stats_response = client.get("http://localhost:3030/admin/stats")
                    .header("Authorization", format!("Bearer {}", data.token))
                    .send()
                    .await;
                
                match stats_response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            if let Ok(dashboard_stats) = resp.json::<DashboardStats>().await {
                                set_stats.set(dashboard_stats);
                            } else {
                                set_error.set(Some("Failed to parse dashboard statistics".to_string()));
                            }
                        } else {
                            set_error.set(Some(format!("Failed to fetch stats: {}", resp.status())));
                        }
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Request error: {}", e)));
                    }
                }
                
                // Fetch activity log
                let activity_response = client.get("http://localhost:3030/admin/activity")
                    .header("Authorization", format!("Bearer {}", data.token))
                    .send()
                    .await;
                
                match activity_response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            if let Ok(activities) = resp.json::<Vec<ActivityItem>>().await {
                                set_recent_activity.set(activities);
                            } else {
                                set_error.set(Some("Failed to parse activity data".to_string()));
                            }
                        } else {
                            set_error.set(Some(format!("Failed to fetch activity: {}", resp.status())));
                        }
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Request error: {}", e)));
                    }
                }
                
                // Fetch system health
                let health_response = client.get("http://localhost:3030/admin/system/health")
                    .header("Authorization", format!("Bearer {}", data.token))
                    .send()
                    .await;
                
                match health_response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            if let Ok(health) = resp.json::<SystemHealth>().await {
                                set_system_health.set(Some(health));
                            } else {
                                set_error.set(Some("Failed to parse system health data".to_string()));
                            }
                        } else {
                            set_error.set(Some(format!("Failed to fetch system health: {}", resp.status())));
                        }
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Request error: {}", e)));
                    }
                }
                
                set_loading.set(false);
            });
        } else {
            // User not logged in, redirect to login
            let window = web_sys::window().unwrap();
            let _ = window.location().set_href("/login");
        }
    });
    
    view! { cx,
        <div class="admin-dashboard">
            <h1>"Admin Dashboard"</h1>
            
            {move || if let Some(err) = error.get() {
                view! { cx, <div class="error-message">{err}</div> }.into_view(cx)
            } else { view! {}.into_view(cx) }}
            
            {move || if loading.get() {
                view! { cx, <div class="loading">"Loading dashboard data..."</div> }.into_view(cx)
            } else {
                view! { cx,
                    <div class="dashboard-content">
                        <section class="stat-cards">
                            <div class="stat-card">
                                <h3>"Total Users"</h3>
                                <div class="stat-number">{stats.get().total_users}</div>
                                <div class="stat-info">
                                    {stats.get().new_users_today} " new today"
                                </div>
                            </div>
                            
                            <div class="stat-card">
                                <h3>"Total Topics"</h3>
                                <div class="stat-number">{stats.get().total_topics}</div>
                                <div class="stat-info">
                                    {stats.get().new_topics_today} " new today"
                                </div>
                            </div>
                            
                            <div class="stat-card">
                                <h3>"Total Posts"</h3>
                                <div class="stat-number">{stats.get().total_posts}</div>
                                <div class="stat-info">
                                    {stats.get().new_posts_today} " new today"
                                </div>
                            </div>
                            
                            <div class="stat-card">
                                <h3>"Flagged Content"</h3>
                                <div class="stat-number">{stats.get().flagged_content}</div>
                                {move || if stats.get().flagged_content > 0 {
                                    view! { cx, 
                                        <div class="stat-info flag-alert">
                                            "Needs attention!"
                                        </div>
                                    }.into_view(cx)
                                } else {
                                    view! { cx, <div class="stat-info">"All clear"</div> }.into_view(cx)
                                }}
                            </div>
                        </section>
                        
                        <section class="admin-panels">
                            <div class="admin-panel">
                                <h3>"Recent Activity"</h3>
                                <div class="activity-log">
                                    {move || {
                                        let activities = recent_activity.get();
                                        if activities.is_empty() {
                                            view! { cx, <p>"No recent activity"</p> }.into_view(cx)
                                        } else {
                                            activities.into_iter().map(|item| {
                                                view! { cx,
                                                    <div class="activity-item">
                                                        <div class="activity-time">
                                                            {item.created_at.split('T').next().unwrap_or("").to_string()}
                                                        </div>
                                                        <div class="activity-content">
                                                            <span class="activity-type">{item.action_type}</span>
                                                            " by "
                                                            <a href={format!("/admin/users/{}", item.user_id)} class="activity-user">
                                                                {item.username}
                                                            </a>
                                                            " - "
                                                            {item.description}
                                                        </div>
                                                    </div>
                                                }
                                            }).collect_view(cx)
                                        }
                                    }}
                                </div>
                            </div>
                            
                            <div class="admin-panel">
                                <h3>"System Health"</h3>
                                {move || {
                                    if let Some(health) = system_health.get() {
                                        view! { cx,
                                            <div class="system-health">
                                                <div class="health-item">
                                                    <div class="health-label">"Status"</div>
                                                    <div class={"health-status " + get_status_class(&health.status)}>
                                                        {health.status}
                                                    </div>
                                                </div>
                                                <div class="health-item">
                                                    <div class="health-label">"Database"</div>
                                                    <div class={"health-status " + get_status_class(&health.database_status)}>
                                                        {health.database_status}
                                                    </div>
                                                </div>
                                                <div class="health-item">
                                                    <div class="health-label">"Disk Space"</div>
                                                    <div class="health-value">
                                                        {health.disk_space}
                                                    </div>
                                                </div>
                                                <div class="health-item">
                                                    <div class="health-label">"Memory Usage"</div>
                                                    <div class="health-value">
                                                        {health.memory_usage}
                                                    </div>
                                                </div>
                                                <div class="health-item">
                                                    <div class="health-label">"Uptime"</div>
                                                    <div class="health-value">
                                                        {health.uptime}
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_view(cx)
                                    } else {
                                        view! { cx, <p>"System health data unavailable"</p> }.into_view(cx)
                                    }
                                }}
                            </div>
                        </section>
                    </div>
                }.into_view(cx)
            }}
        </div>
    }
}

fn get_status_class(status: &str) -> &'static str {
    match status.to_lowercase().as_str() {
        "good" | "healthy" | "ok" => "status-good",
        "warning" => "status-warning",
        "critical" | "error" | "unhealthy" => "status-critical",
        _ => "",
    }
}