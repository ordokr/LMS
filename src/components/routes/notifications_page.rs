use leptos::*;
use crate::components::user::NotificationsPanel;
use crate::components::error_alert::ErrorAlert;
use crate::utils::auth::use_auth;

#[component]
pub fn NotificationsPage() -> impl IntoView {
    // Get current user info from auth context
    let auth = use_auth();
    let user_id = create_memo(move |_| auth.with(|a| a.user_id.clone()));
    
    view! {
        <div class="container notifications-page-container">
            {move || {
                if let Some(id) = user_id.get() {
                    view! { <NotificationsPanel user_id={id} /> }
                } else {
                    view! { 
                        <ErrorAlert message="Please login to view notifications" /> 
                    }
                }
            }}
        </div>
    }
}