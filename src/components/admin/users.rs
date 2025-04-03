use leptos::*;
use crate::components::auth::AuthData;

#[component]
pub fn AdminUsers(cx: Scope) -> impl IntoView {
    // Get auth data from context
    let auth_data = use_context::<Signal<Option<AuthData>>>(cx)
        .expect("Auth data not provided");
    
    // Redirect non-admin users
    create_effect(cx, move |_| {
        match auth_data.get() {
            Some(data) if data.user.is_admin => {
                // User is an admin, do nothing
            },
            _ => {
                // Not admin or not logged in, redirect
                let window = web_sys::window().unwrap();
                let _ = window.location().set_href("/");
            }
        }
    });
    
    // Signals for user management
    let (search_query, set_search_query) = create_signal(cx, String::new());
    let (filter_role, set_filter_role) = create_signal(cx, String::new());
    let (filter_trust_level, set_filter_trust_level) = create_signal(cx, String::new());
    let (error, set_error) = create_signal(cx, None::<String>);
    let (success, set_success) = create_signal(cx, None::<String>);
    let (editing_user_id, set_editing_user_id) = create_signal(cx, None::<i64>);
    let (page, set_page) = create_signal(cx, 1);
    let per_page = 20;
    
    // Create a resource to fetch users with search & filters
    let users_resource = create_resource(
        cx,
        move || (search_query.get(), filter_role.get(), filter_trust_level.get(), page.get()),
        |(query, role, trust_level, page)| async move {
            // Get auth token
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let token = storage.get_item("auth_token").unwrap_or(None);
            
            if let Some(token) = token {
                // Build query string
                let mut params = vec![format!("page={}", page), format!("per_page={}", per_page)];
                
                if !query.trim().is_empty() {
                    params.push(format!("q={}", query));
                }
                
                if !role.is_empty() {
                    params.push(format!("role={}", role));
                }
                
                if !trust_level.is_empty() {
                    params.push(format!("trust_level={}", trust_level));
                }
                
                let query_string = if !params.is_empty() {
                    format!("?{}", params.join("&"))
                } else {
                    String::new()
                };
                
                // Fetch users
                let client = reqwest::Client::new();
                let response = client.get(&format!("http://localhost:3030/admin/users{}", query_string))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await;
                    
                match response {
                    Ok(resp) if resp.status().is_success() => {
                        resp.json::<UserListResponse>().await.ok()
                    },
                    _ => None,
                }
            } else {
                None
            }
        },
    );
    
    // User actions
    let suspend_user = create_action(cx, move |payload: &SuspendUserPayload| {
        let user_id = payload.user_id;
        let days = payload.days;
        let reason = payload.reason.clone();
        
        async move {
            // Get auth token
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let token = match storage.get_item("auth_token").unwrap_or(None) {
                Some(t) => t,
                None => {
                    set_error.set(Some("Authentication token not found".to_string()));
                    return false;
                }
            };
            
            // Send suspend request
            let client = reqwest::Client::new();
            let response = client.post(&format!("http://localhost:3030/admin/users/{}/suspend", user_id))
                .header("Authorization", format!("Bearer {}", token))
                .json(&serde_json::json!({
                    "days": days,
                    "reason": reason,
                }))
                .send()
                .await;
                
            match response {
                Ok(resp) if resp.status().is_success() => {
                    set_success.set(Some(format!("User suspended for {} days", days)));
                    users_resource.refetch();
                    true
                },
                Ok(resp) => {
                    let error_message = resp.text().await
                        .unwrap_or_else(|_| "Server error".to_string());
                    set_error.set(Some(error_message));
                    false
                },
                Err(e) => {
                    set_error.set(Some(format!("Request error: {}", e)));
                    false
                }
            }
        }
    });
    
    let unsuspend_user = create_action(cx, move |user_id: &i64| {
        let id = *user_id;
        
        async move {
            // Get auth token
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let token = match storage.get_item("auth_token").unwrap_or(None) {
                Some(t) => t,
                None => {
                    set_error.set(Some("Authentication token not found".to_string()));
                    return false;
                }
            };
            
            // Send unsuspend request
            let client = reqwest::Client::new();
            let response = client.post(&format!("http://localhost:3030/admin/users/{}/unsuspend", id))
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await;
                
            match response {
                Ok(resp) if resp.status().is_success() => {
                    set_success.set(Some("User unsuspended successfully".to_string()));
                    users_resource.refetch();
                    true
                },
                Ok(resp) => {
                    let error_message = resp.text().await
                        .unwrap_or_else(|_| "Server error".to_string());
                    set_error.set(Some(error_message));
                    false
                },
                Err(e) => {
                    set_error.set(Some(format!("Request error: {}", e)));
                    false
                }
            }
        }
    });
    
    let update_user_role = create_action(cx, move |payload: &UpdateRolePayload| {
        let user_id = payload.user_id;
        let is_admin = payload.is_admin;
        
        async move {
            // Get auth token
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let token = match storage.get_item("auth_token").unwrap_or(None) {
                Some(t) => t,
                None => {
                    set_error.set(Some("Authentication token not found".to_string()));
                    return false;
                }
            };
            
            // Send update role request
            let client = reqwest::Client::new();
            let response = client.put(&format!("http://localhost:3030/admin/users/{}/role", user_id))
                .header("Authorization", format!("Bearer {}", token))
                .json(&serde_json::json!({
                    "is_admin": is_admin
                }))
                .send()
                .await;
                
            match response {
                Ok(resp) if resp.status().is_success() => {
                    set_success.set(Some(format!("User role updated to {}", 
                        if is_admin { "Administrator" } else { "Regular User" }
                    )));
                    users_resource.refetch();
                    true
                },
                Ok(resp) => {
                    let error_message = resp.text().await
                        .unwrap_or_else(|_| "Server error".to_string());
                    set_error.set(Some(error_message));
                    false
                },
                Err(e) => {
                    set_error.set(Some(format!("Request error: {}", e)));
                    false
                }
            }
        }
    });
    
    // Search handler
    let on_search = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_page.set(1); // Reset to first page on new search
        users_resource.refetch();
    };
    
    // Function to handle suspending a user
    let handle_suspend = move |user_id: i64| {
        // In a real app, use a modal dialog
        let days_str = web_sys::window()
            .unwrap()
            .prompt_with_message("Number of days to suspend (1-365):")
            .unwrap_or_else(|| Some("7".to_string()))
            .unwrap_or_else(|| "7".to_string());
            
        let days = days_str.parse::<i32>().unwrap_or(7);
        
        let reason = web_sys::window()
            .unwrap()
            .prompt_with_message("Reason for suspension:")
            .unwrap_or_else(|| Some("Violation of community guidelines".to_string()))
            .unwrap_or_else(|| "Violation of community guidelines".to_string());
            
        suspend_user.dispatch(SuspendUserPayload {
            user_id,
            days,
            reason,
        });
    };
    
    // Function to make a user admin or remove admin status
    let toggle_admin_role = move |user_id: i64, current_is_admin: bool| {
        let change_to = !current_is_admin;
        
        let message = if change_to {
            "Make this user an administrator? This will grant full access to all admin features."
        } else {
            "Remove administrator privileges from this user?"
        };
        
        if web_sys::window()
            .unwrap()
            .confirm_with_message(message)
            .unwrap_or(false) 
        {
            update_user_role.dispatch(UpdateRolePayload {
                user_id,
                is_admin: change_to,
            });
        }
    };
    
    view! { cx,
        <div class="admin-users">
            <h1>"User Management"</h1>
            
            {move || match auth_data.get() {
                Some(data) if data.user.is_admin => {
                    view! { cx,
                        <div class="admin-content">
                            <div class="admin-sidebar">
                                <h3>"Administration"</h3>
                                <ul class="admin-menu">
                                    <li>
                                        <a href="/admin">"Dashboard"</a>
                                    </li>
                                    <li>
                                        <a href="/admin/categories">"Categories"</a>
                                    </li>
                                    <li>
                                        <a href="/admin/users" class="active">"Users"</a>
                                    </li>
                                    <li>
                                        <a href="/admin/content">"Content Moderation"</a>
                                    </li>
                                    <li>
                                        <a href="/admin/settings">"Settings"</a>
                                    </li>
                                </ul>
                            </div>
                            
                            <div class="admin-main">
                                {move || error.get().map(|err| view! { cx, 
                                    <div class="alert alert-error">{err}</div> 
                                })}
                                
                                {move || success.get().map(|msg| view! { cx, 
                                    <div class="alert alert-success">{msg}</div> 
                                })}
                                
                                <div class="user-filters">
                                    <form on:submit=on_search>
                                        <div class="filter-row">
                                            <div class="search-box">
                                                <input 
                                                    type="text" 
                                                    placeholder="Search by username or email..." 
                                                    class="search-input" 
                                                    value=move || search_query.get()
                                                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                                />
                                                <button type="submit" class="search-button">🔍</button>
                                            </div>
                                            
                                            <div class="filter-group">
                                                <label for="role-filter">"Role:"</label>
                                                <select 
                                                    id="role-filter"
                                                    on:change=move |ev| {
                                                        set_filter_role.set(event_target_value(&ev));
                                                        set_page.set(1);
                                                        users_resource.refetch();
                                                    }
                                                >
                                                    <option value="">"All Roles"</option>
                                                    <option value="admin">"Administrators"</option>
                                                    <option value="regular">"Regular Users"</option>
                                                </select>
                                            </div>
                                            
                                            <div class="filter-group">
                                                <label for="trust-filter">"Trust Level:"</label>
                                                <select 
                                                    id="trust-filter"
                                                    on:change=move |ev| {
                                                        set_filter_trust_level.set(event_target_value(&ev));
                                                        set_page.set(1);
                                                        users_resource.refetch();
                                                    }
                                                >
                                                    <option value="">"All Levels"</option>
                                                    <option value="0">"Level 0 (New)"</option>
                                                    <option value="1">"Level 1 (Basic)"</option>
                                                    <option value="2">"Level 2 (Member)"</option>
                                                    <option value="3">"Level 3 (Regular)"</option>
                                                    <option value="4">"Level 4 (Leader)"</option>
                                                </select>
                                            </div>
                                        </div>
                                    </form>
                                </div>
                                
                                <div class="users-list">
                                    <table class="admin-table">
                                        <thead>
                                            <tr>
                                                <th>"User"</th>
                                                <th>"Email"</th>
                                                <th>"Role"</th>
                                                <th>"Trust Level"</th>
                                                <th>"Joined"</th>
                                                <th>"Status"</th>
                                                <th>"Actions"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {move || match users_resource.read(cx) {
                                                None => view! { cx, 
                                                    <tr>
                                                        <td colspan="7">"Loading users..."</td>
                                                    </tr>
                                                }.into_view(cx),
                                                Some(None) => view! { cx, 
                                                    <tr>
                                                        <td colspan="7">"Failed to load users"</td>
                                                    </tr>
                                                }.into_view(cx),
                                                Some(Some(response)) => {
                                                    if response.users.is_empty() {
                                                        view! { cx, 
                                                            <tr>
                                                                <td colspan="7">"No users found"</td>
                                                            </tr>
                                                        }.into_view(cx)
                                                    } else {
                                                        response.users.iter().map(|user| {
                                                            let user_id = user.id;
                                                            let is_suspended = user.is_suspended;
                                                            let is_admin = user.is_admin;
                                                            let current_user_is_admin = match auth_data.get() {
                                                                Some(data) => data.user.id == user_id,
                                                                None => false
                                                            };
                                                            
                                                            view! { cx,
                                                                <tr>
                                                                    <td>
                                                                        <div class="user-info">
                                                                            <div class="user-avatar">
                                                                                {match &user.avatar_url {
                                                                                    Some(url) => view! { cx, 
                                                                                        <img src={url.clone()} alt="User avatar" />
                                                                                    }.into_view(cx),
                                                                                    None => {
                                                                                        let initial = user.display_name.chars().next()
                                                                                            .unwrap_or('U').to_string();
                                                                                        view! { cx, 
                                                                                            <span class="avatar-initial">{initial}</span>
                                                                                        }.into_view(cx)
                                                                                    }
                                                                                }}
                                                                            </div>
                                                                            <div>
                                                                                <a href={format!("/profile/{}", user_id)} class="user-name">
                                                                                    {&user.display_name}
                                                                                </a>
                                                                                <div class="username">{"@"}{&user.username}</div>
                                                                            </div>
                                                                        </div>
                                                                    </td>
                                                                    <td>{&user.email}</td>
                                                                    <td>
                                                                        <span class={if user.is_admin { "badge admin" } else { "badge user" }}>
                                                                            {if user.is_admin { "Admin" } else { "User" }}
                                                                        </span>
                                                                    </td>
                                                                    <td>
                                                                        <span class="badge">{format!("Level {}", user.trust_level)}</span>
                                                                    </td>
                                                                    <td>{&user.created_at}</td>
                                                                    <td>
                                                                        {if user.is_suspended {
                                                                            view! { cx,
                                                                                <span class="badge suspended">
                                                                                    "Suspended"
                                                                                    {if let Some(until) = &user.suspended_until {
                                                                                        view! { cx, <span>{format!(" until {}", until)}</span> }.into_view(cx)
                                                                                    } else { view! {}.into_view(cx) }}
                                                                                </span>
                                                                            }.into_view(cx)
                                                                        } else {
                                                                            view! { cx, <span class="badge active">"Active"</span> }.into_view(cx)
                                                                        }}
                                                                    </td>
                                                                    <td>
                                                                        <div class="table-actions">
                                                                            <div class="dropdown">
                                                                                <button class="button sm secondary dropdown-toggle">
                                                                                    "Actions ▾"
                                                                                </button>
                                                                                <div class="dropdown-menu">
                                                                                    <a href={format!("/profile/{}", user_id)} class="dropdown-item">
                                                                                        "View Profile"
                                                                                    </a>
                                                                                    
                                                                                    {if !current_user_is_admin {
                                                                                        view! { cx,
                                                                                            <button class="dropdown-item"
                                                                                              on:click=move |_| toggle_admin_role(user_id, is_admin)>
                                                                                                {if is_admin { "Remove Admin" } else { "Make Admin" }}
                                                                                            </button>
                                                                                        }.into_view(cx)
                                                                                    } else { view! {}.into_view(cx) }}
                                                                                    
                                                                                    {if is_suspended {
                                                                                        view! { cx,
                                                                                            <button class="dropdown-item"
                                                                                              on:click=move |_| unsuspend_user.dispatch(user_id)>
                                                                                                "Unsuspend"
                                                                                            </button>
                                                                                        }.into_view(cx)
                                                                                    } else {
                                                                                        view! { cx,
                                                                                            <button class="dropdown-item"
                                                                                              on:click=move |_| handle_suspend(user_id)>
                                                                                                "Suspend"
                                                                                            </button>
                                                                                        }.into_view(cx)
                                                                                    }}
                                                                                    
                                                                                    <a href={format!("/admin/users/{}/activity", user_id)} 
                                                                                       class="dropdown-item">
                                                                                        "View Activity"
                                                                                    </a>
                                                                                </div>
                                                                            </div>
                                                                        </div>
                                                                    </td>
                                                                </tr>
                                                            }
                                                        }).collect_view(cx)
                                                    }
                                                }
                                            }}
                                        </tbody>
                                    </table>
                                    
                                    {move || match users_resource.read(cx) {
                                        Some(Some(response)) if response.total_pages > 1 => {
                                            view! { cx,
                                                <div class="pagination">
                                                    <button 
                                                        class="button sm" 
                                                        disabled=move || page.get() <= 1
                                                        on:click=move |_| {
                                                            if page.get() > 1 {
                                                                set_page.update(|p| *p -= 1);
                                                            }
                                                        }
                                                    >
                                                        "← Previous"
                                                    </button>
                                                    
                                                    <span class="page-info">
                                                        {"Page "}{page.get()}{" of "}{response.total_pages}
                                                        {" (Total: "}{response.total_users}{" users)"}
                                                    </span>
                                                    
                                                    <button 
                                                        class="button sm" 
                                                        disabled=move || page.get() >= response.total_pages
                                                        on:click=move |_| {
                                                            if page.get() < response.total_pages {
                                                                set_page.update(|p| *p += 1);
                                                            }
                                                        }
                                                    >
                                                        "Next →"
                                                    </button>
                                                </div>
                                            }.into_view(cx)
                                        },
                                        _ => view! {}.into_view(cx)
                                    }}
                                </div>
                            </div>
                        </div>
                    }.into_view(cx)
                },
                _ => {
                    view! { cx,
                        <div class="not-authorized">
                            <h2>"Access Denied"</h2>
                            <p>"You must be an administrator to access this page."</p>
                            <a href="/" class="button primary">"Go to Home"</a>
                        </div>
                    }.into_view(cx)
                }
            }}
        </div>
    }
}

// Models for user management
#[derive(Clone, Debug, serde::Deserialize)]
pub struct UserListResponse {
    pub users: Vec<AdminUserView>,
    pub total_users: i32,
    pub total_pages: i32,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct AdminUserView {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub trust_level: i32,
    pub created_at: String,
    pub last_seen_at: Option<String>,
    pub is_suspended: bool,
    pub suspended_until: Option<String>,
    pub topics_count: i32,
    pub posts_count: i32,
}

// Payloads for user actions
#[derive(Clone, Debug)]
pub struct SuspendUserPayload {
    pub user_id: i64,
    pub days: i32,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct UpdateRolePayload {
    pub user_id: i64,
    pub is_admin: bool,
}