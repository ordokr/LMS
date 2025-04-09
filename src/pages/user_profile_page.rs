use leptos::*;
use leptos_router::*;
use crate::components::user::UserProfileView;

#[component]
pub fn UserProfilePage(
    #[prop(optional)] active_tab: Option<String>,
) -> impl IntoView {
    // Get username from route params
    let params = use_params_map();
    let username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    
    view! {
        <div class="page-container">
            <UserProfileView 
                username={username()}
                default_tab={active_tab}
            />
        </div>
    }
}