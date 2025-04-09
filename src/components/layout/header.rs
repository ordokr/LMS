use leptos::*;
use crate::state::auth::use_auth_state; // Make sure to import your auth state
use crate::components::forum::notifications::NotificationCenter;
use crate::components::notifications::NotificationCenter;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    // Get the authentication state
    let auth_state = use_context::<AuthState>(cx);
    
    view! { cx,
        <header class="header">
            <div class="logo">
                <h1>"LMS Platform"</h1>
            </div>
            <nav class="main-nav">
                <ul>
                    <li><a href="/">"Dashboard"</a></li>
                    <li><a href="/courses">"Courses"</a></li>
                    <li><a href="/assignments">"Assignments"</a></li>
                    <li><a href="/forum">"Discussion Forum"</a></li>
                    
                    // Add admin link - only visible to admins and moderators
                    {move || {
                        if auth_state.and_then(|s| Some(s.is_admin() || s.is_moderator())).unwrap_or(false) {
                            view! { cx,
                                <li>
                                    <a href="/admin/dashboard" class="admin-link">
                                        <span class="admin-icon">"🛡️"</span>
                                        " Admin"
                                    </a>
                                </li>
                            }
                        } else {
                            view! { cx, <></> }
                        }
                    }}
                </ul>
            </nav>
            <div class="user-menu">
                <a href="/profile" class="user-profile">
                    <span class="avatar">"👤"</span>
                    <span class="username">"User"</span>
                </a>
            </div>
            <div class="d-flex align-items-center">
                // If user is logged in
                {move || if is_logged_in() {
                    view! {
                        <>
                            <div class="me-3">
                                <NotificationCenter />
                            </div>
                            // ... user dropdown menu
                        </>
                    }
                } else {
                    view! {
                        // ... login/register buttons
                    }
                }}
            </div>
            <div class="user-navigation">
                <NotificationCenter />
                // ... existing user dropdown or login button
            </div>
        </header>
    }
}