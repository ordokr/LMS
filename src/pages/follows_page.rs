use leptos::*;
use leptos_router::*;
use crate::components::user::FollowsList;

#[component]
pub fn FollowingPage() -> impl IntoView {
    // Get username from route params
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    
    // Load user ID based on username
    let (user_id, set_user_id) = create_signal(None::<String>);
    
    create_effect(move |_| {
        let username_value = username();
        if !username_value.is_empty() {
            spawn_local(async move {
                if let Ok(id) = get_user_id_by_username(&username_value).await {
                    set_user_id.set(Some(id));
                }
            });
        }
    });

    view! {
        <div class="page-container">
            <div class="page-header">
                <h1 class="page-title">"Users Following "{username()}</h1>
                <a href={format!("/users/{}", username())} class="back-link">"Back to Profile"</a>
            </div>
            
            {move || match user_id.get() {
                Some(id) => view! { <FollowsList user_id={id} followers=false /> },
                None => view! { <div class="loading-state">"Loading..."</div> }
            }}
        </div>
    }
}

#[component]
pub fn FollowersPage() -> impl IntoView {
    // Get username from route params
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    
    // Load user ID based on username
    let (user_id, set_user_id) = create_signal(None::<String>);
    
    create_effect(move |_| {
        let username_value = username();
        if !username_value.is_empty() {
            spawn_local(async move {
                if let Ok(id) = get_user_id_by_username(&username_value).await {
                    set_user_id.set(Some(id));
                }
            });
        }
    });

    view! {
        <div class="page-container">
            <div class="page-header">
                <h1 class="page-title">"Followers of "{username()}</h1>
                <a href={format!("/users/{}", username())} class="back-link">"Back to Profile"</a>
            </div>
            
            {move || match user_id.get() {
                Some(id) => view! { <FollowsList user_id={id} followers=true /> },
                None => view! { <div class="loading-state">"Loading..."</div> }
            }}
        </div>
    }
}

// Helper to get user ID from username
async fn get_user_id_by_username(username: &str) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct UserIdResponse {
        id: String,
    }
    
    tauri_sys::tauri::invoke::<_, UserIdResponse>("get_user_id_by_username", &username)
        .await
        .map(|response| response.id)
        .map_err(|e| e.to_string())
}