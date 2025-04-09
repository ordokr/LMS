use leptos::*;
use crate::models::user::UserSummary;
use crate::components::error_alert::ErrorAlert;

#[component]
pub fn FollowsList(
    user_id: String,
    #[prop(default = false)] followers: bool,
) -> impl IntoView {
    // State
    let (users, set_users) = create_signal(Vec::<UserSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    let per_page = 20;
    
    // Load users
    let load_users = move || {
        set_loading.set(true);
        set_error.set(None);
        
        let command = if followers { "get_followers" } else { "get_following" };
        
        spawn_local(async move {
            match invoke::<_, Vec<UserSummary>>(
                command, 
                &(user_id.clone(), Some(current_page.get()), Some(per_page))
            ).await {
                Ok(fetched_users) => {
                    set_users.set(fetched_users);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load users: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load users on mount and when page changes
    create_effect(move |_| {
        load_users();
    });
    
    // Handle pagination
    let next_page = move |_| {
        set_current_page.update(|p| *p += 1);
        load_users();
    };
    
    let prev_page = move |_| {
        if current_page.get() > 1 {
            set_current_page.update(|p| *p -= 1);
            load_users();
        }
    };
    
    // Get title based on type
    let title = if followers { "Followers" } else { "Following" };

    view! {
        <div class="follows-list">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            <h2 class="section-title">{title}</h2>
            
            {move || {
                if loading.get() && users.get().is_empty() {
                    view! { <div class="loading-state">"Loading users..."</div> }
                } else if users.get().is_empty() {
                    view! { <div class="empty-state">
                        {if followers { "No followers yet" } else { "Not following anyone yet" }}
                    </div> }
                } else {
                    view! {
                        <div class="users-grid">
                            {users.get().into_iter().map(|user| {
                                view! {
                                    <a href={format!("/users/{}", user.username)} class="user-card">
                                        <div class="user-avatar-container">
                                            {match user.avatar_url {
                                                Some(url) => view! {
                                                    <img src={url} alt="User avatar" class="user-avatar" />
                                                },
                                                None => view! {
                                                    <div class="user-avatar default-avatar">
                                                        {user.display_name.chars().next().unwrap_or('?')}
                                                    </div>
                                                }
                                            }}
                                        </div>
                                        <div class="user-info">
                                            <div class="user-display-name">{user.display_name}</div>
                                            <div class="user-username">{"@"}{user.username}</div>
                                        </div>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                        
                        <div class="pagination-controls">
                            <button 
                                class="pagination-button" 
                                on:click=prev_page 
                                disabled=move || current_page.get() <= 1
                            >
                                "Previous"
                            </button>
                            <span class="page-indicator">{"Page "}{current_page}</span>
                            <button 
                                class="pagination-button" 
                                on:click=next_page 
                                disabled=move || users.get().len() < per_page
                            >
                                "Next"
                            </button>
                        </div>
                    }
                }
            }}
        </div>
    }
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: serde::Serialize + ?Sized,
    R: for<'de> serde::de::DeserializeOwned,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}