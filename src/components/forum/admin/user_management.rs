use leptos::*;
use leptos_router::use_query_map;
use crate::models::user::{User, UserRole};
use crate::services::admin::AdminService;
use crate::services::user::UserService;
use web_sys::SubmitEvent;

#[component]
pub fn UserManagement() -> impl IntoView {
    // Check if user is admin
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // Get query parameters
    let query = use_query_map();
    let page_param = move || {
        query.with(|q| {
            q.get("page")
                .and_then(|p| p.parse::<usize>().ok())
                .unwrap_or(1)
        })
    };
    
    let search_param = move || {
        query.with(|q| q.get("q").cloned().unwrap_or_default())
    };
    
    // State signals
    let (users, set_users) = create_signal(Vec::<User>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    let (total_pages, set_total_pages) = create_signal(1);
    let (total_users, set_total_users) = create_signal(0);
    let (search_query, set_search_query) = create_signal(String::new());
    
    // Edit state
    let (editing_user, set_editing_user) = create_signal(None::<User>);
    let (edit_role, set_edit_role) = create_signal(UserRole::Member);
    let (edit_status, set_edit_status) = create_signal("active".to_string());
    let (saving, set_saving) = create_signal(false);
    
    // Load users with pagination
    let load_users = move |page: usize, search: &str| {
        set_loading.set(true);
        
        spawn_local(async move {
            match AdminService::get_users(page, 20, search).await {
                Ok(paginated) => {
                    set_users.set(paginated.users);
                    set_total_pages.set(paginated.total_pages);
                    set_current_page.set(paginated.page);
                    set_total_users.set(paginated.total);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load users: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initialize from URL params
    create_effect(move |_| {
        let page = page_param();
        let search = search_param();
        
        set_current_page.set(page);
        set_search_query.set(search.clone());
        load_users(page, &search);
    });
    
    // Handle search form submission
    let handle_search = move |ev: SubmitEvent| {
        ev.prevent_default();
        load_users(1, &search_query());
    };
    
    // Go to page
    let go_to_page = move |page: usize| {
        if page != current_page() && page > 0 && page <= total_pages() {
            load_users(page, &search_query());
        }
    };
    
    // Edit user
    let start_edit = move |user: User| {
        set_edit_role.set(user.role.clone().unwrap_or(UserRole::Member));
        set_edit_status.set(if user.is_banned { "banned".to_string() } else { "active".to_string() });
        set_editing_user.set(Some(user));
    };
    
    // Cancel edit
    let cancel_edit = move |_| {
        set_editing_user.set(None);
    };
    
    // Save user changes
    let save_user_changes = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        if let Some(user) = editing_user() {
            let user_id = user.id;
            let is_banned = edit_status() == "banned";
            
            set_saving.set(true);
            
            spawn_local(async move {
                // Update role
                if let Err(e) = AdminService::update_user_role(user_id, edit_role()).await {
                    set_error.set(Some(format!("Failed to update user role: {}", e)));
                    set_saving.set(false);
                    return;
                }
                
                // Update ban status
                if user.is_banned != is_banned {
                    if is_banned {
                        if let Err(e) = AdminService::ban_user(user_id).await {
                            set_error.set(Some(format!("Failed to ban user: {}", e)));
                            set_saving.set(false);
                            return;
                        }
                    } else {
                        if let Err(e) = AdminService::unban_user(user_id).await {
                            set_error.set(Some(format!("Failed to unban user: {}", e)));
                            set_saving.set(false);
                            return;
                        }
                    }
                }
                
                // Refresh user list
                load_users(current_page(), &search_query());
                set_success.set(Some(format!("User {} updated successfully", user.name)));
                set_editing_user.set(None);
                set_saving.set(false);
            });
        }
    };
    
    // Ban/unban user directly
    let toggle_user_ban = move |user: User| {
        let user_id = user.id;
        let is_currently_banned = user.is_banned;
        let user_name = user.name.clone();
        
        spawn_local(async move {
            let result = if is_currently_banned {
                AdminService::unban_user(user_id).await
            } else {
                AdminService::ban_user(user_id).await
            };
            
            match result {
                Ok(_) => {
                    let action = if is_currently_banned { "unbanned" } else { "banned" };
                    set_success.set(Some(format!("User {} {} successfully", user_name, action)));
                    load_users(current_page(), &search_query());
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update user: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="admin-user-management">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <h1 class="mb-4">"User Management"</h1>
                    
                    {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                    {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                    
                    <div class="card mb-4">
                        <div class="card-body">
                            <form class="d-flex" on:submit=handle_search>
                                <input
                                    type="text"
                                    class="form-control me-2"
                                    placeholder="Search by name or email..."
                                    prop:value=move || search_query()
                                    on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                />
                                <button type="submit" class="btn btn-primary">
                                    <i class="bi bi-search me-1"></i>
                                    "Search"
                                </button>
                            </form>
                        </div>
                    </div>
                    
                    {move || if loading() {
                        view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                    } else if users().is_empty() {
                        view! {
                            <div class="text-center p-5">
                                <i class="bi bi-people mb-3 d-block" style="font-size: 3rem;"></i>
                                {if search_query().is_empty() {
                                    view! { <p class="h4">"No users found"</p> }
                                } else {
                                    view! { <p class="h4">"No users match your search criteria"</p> }
                                }}
                            </div>
                        }
                    } else {
                        view! {
                            <div class="card mb-4">
                                <div class="card-header d-flex justify-content-between align-items-center">
                                    <h5 class="mb-0">
                                        "Users "
                                        <span class="badge bg-secondary">{total_users()}</span>
                                    </h5>
                                </div>
                                <div class="card-body p-0">
                                    <div class="table-responsive">
                                        <table class="table table-striped table-hover mb-0">
                                            <thead>
                                                <tr>
                                                    <th>"User"</th>
                                                    <th>"Email"</th>
                                                    <th>"Role"</th>
                                                    <th>"Topics"</th>
                                                    <th>"Posts"</th>
                                                    <th>"Joined"</th>
                                                    <th>"Status"</th>
                                                    <th>"Actions"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {users().into_iter().map(|user| {
                                                    let u = user.clone();
                                                    
                                                    view! {
                                                        <tr class={if user.is_banned { "table-danger" } else { "" }}>
                                                            <td>
                                                                <div class="d-flex align-items-center">
                                                                    <img 
                                                                        src={user.avatar_url.unwrap_or_else(|| format!("https://ui-avatars.com/api/?name={}&background=random", user.name))}
                                                                        class="rounded-circle me-2"
                                                                        style="width: 32px; height: 32px; object-fit: cover;"
                                                                        alt={format!("{}'s avatar", user.name)}
                                                                    />
                                                                    <a href={format!("/users/{}", user.id)}>{user.name}</a>
                                                                </div>
                                                            </td>
                                                            <td>{user.email}</td>
                                                            <td>
                                                                {match user.role.unwrap_or(UserRole::Member) {
                                                                    UserRole::Admin => view! { <span class="badge bg-danger">"Admin"</span> },
                                                                    UserRole::Moderator => view! { <span class="badge bg-warning">"Moderator"</span> },
                                                                    UserRole::Member => view! { <span class="badge bg-primary">"Member"</span> }
                                                                }}
                                                            </td>
                                                            <td>{user.topic_count.unwrap_or(0)}</td>
                                                            <td>{user.post_count.unwrap_or(0)}</td>
                                                            <td>{format_date(user.created_at)}</td>
                                                            <td>
                                                                {if user.is_banned {
                                                                    view! { <span class="badge bg-danger">"Banned"</span> }
                                                                } else {
                                                                    view! { <span class="badge bg-success">"Active"</span> }
                                                                }}
                                                            </td>
                                                            <td>
                                                                <div class="btn-group btn-group-sm">
                                                                    <button class="btn btn-outline-primary" on:click=move |_| start_edit(u.clone())>
                                                                        <i class="bi bi-pencil"></i>
                                                                    </button>
                                                                    <button 
                                                                        class={if user.is_banned { "btn btn-outline-success" } else { "btn btn-outline-danger" }}
                                                                        on:click=move |_| toggle_user_ban(user.clone())
                                                                    >
                                                                        {if user.is_banned {
                                                                            view! { <i class="bi bi-person-check"></i> }
                                                                        } else {
                                                                            view! { <i class="bi bi-person-x"></i> }
                                                                        }}
                                                                    </button>
                                                                </div>
                                                            </td>
                                                        </tr>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                                <div class="card-footer">
                                    {move || if total_pages() > 1 {
                                        view! {
                                            <nav aria-label="Users pagination">
                                                <ul class="pagination justify-content-center mb-0">
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
                                </div>
                            </div>
                        }
                    }}
                    
                    <!-- Edit User Modal -->
                    {move || if let Some(user) = editing_user() {
                        view! {
                            <div class="modal d-block" tabindex="-1" style="background-color: rgba(0,0,0,0.5);">
                                <div class="modal-dialog">
                                    <div class="modal-content">
                                        <div class="modal-header">
                                            <h5 class="modal-title">{format!("Edit User: {}", user.name)}</h5>
                                            <button type="button" class="btn-close" on:click=cancel_edit></button>
                                        </div>
                                        <form on:submit=save_user_changes>
                                            <div class="modal-body">
                                                <div class="mb-3">
                                                    <label for="userRole" class="form-label">"Role"</label>
                                                    <select 
                                                        id="userRole" 
                                                        class="form-select"
                                                        prop:value={match edit_role() {
                                                            UserRole::Admin => "admin",
                                                            UserRole::Moderator => "moderator", 
                                                            UserRole::Member => "member"
                                                        }}
                                                        on:change=move |ev| {
                                                            let role = match event_target_value(&ev).as_str() {
                                                                "admin" => UserRole::Admin,
                                                                "moderator" => UserRole::Moderator,
                                                                _ => UserRole::Member
                                                            };
                                                            set_edit_role.set(role);
                                                        }
                                                    >
                                                        <option value="member">"Member"</option>
                                                        <option value="moderator">"Moderator"</option>
                                                        <option value="admin">"Admin"</option>
                                                    </select>
                                                </div>
                                                
                                                <div class="mb-3">
                                                    <label for="userStatus" class="form-label">"Status"</label>
                                                    <select 
                                                        id="userStatus" 
                                                        class="form-select"
                                                        prop:value=move || edit_status()
                                                        on:change=move |ev| set_edit_status.set(event_target_value(&ev))
                                                    >
                                                        <option value="active">"Active"</option>
                                                        <option value="banned">"Banned"</option>
                                                    </select>
                                                </div>
                                                
                                                <div class="mb-0">
                                                    <p class="mb-1">"User Info:"</p>
                                                    <ul class="list-unstyled small">
                                                        <li>"ID: " <code>{user.id}</code></li>
                                                        <li>"Email: " {user.email}</li>
                                                        <li>"Joined: " {format_date(user.created_at)}</li>
                                                        <li>"Topics: " {user.topic_count.unwrap_or(0)}</li>
                                                        <li>"Posts: " {user.post_count.unwrap_or(0)}</li>
                                                    </ul>
                                                </div>
                                            </div>
                                            <div class="modal-footer">
                                                <button type="button" class="btn btn-secondary" on:click=cancel_edit>
                                                    "Cancel"
                                                </button>
                                                <button type="submit" class="btn btn-primary" disabled=move || saving()>
                                                    {move || if saving() {
                                                        view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Saving..." }
                                                    } else {
                                                        view! { "Save Changes" }
                                                    }}
                                                </button>
                                            </div>
                                        </form>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        view! {}
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