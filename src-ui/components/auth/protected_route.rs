use leptos::*;
use leptos_router::{use_navigate, use_location};
use crate::hooks::use_auth::use_auth;

#[component]
pub fn ProtectedRoute(
    #[prop(optional)] required_role: Option<String>,
    children: Children,
) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let location = use_location();
    
    create_effect(move |_| {
        if !auth.is_authenticated.get() {
            let current_path = location.pathname.get();
            navigate(&format!("/login?redirect={}", current_path), None, None);
        } else if let Some(role) = &required_role {
            if let Some(user) = auth.user.get() {
                if &user.role != role {
                    navigate("/unauthorized", None, None);
                }
            }
        }
    });
    
    view! {
        <Show
            when=move || auth.is_authenticated.get() && match &required_role {
                Some(role) => auth.user.get().map(|u| u.role == *role).unwrap_or(false),
                None => true,
            }
            fallback=|_| view! {
                <div class="flex justify-center items-center h-screen">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
                </div>
            }
        >
            {children()}
        </Show>
    }
}