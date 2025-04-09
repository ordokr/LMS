use leptos::*;
use crate::components::user::UserProfileView;
use crate::components::error_alert::ErrorAlert;

#[component]
pub fn UserPage() -> impl IntoView {
    // Get username from URL
    let username = use_params_map()
        .with(|params| params.get("username").cloned().unwrap_or_default());
    
    if username.is_empty() {
        return view! { <ErrorAlert message="User not found" /> };
    }
    
    view! {
        <div class="container user-page-container">
            <UserProfileView username={username} />
        </div>
    }
}