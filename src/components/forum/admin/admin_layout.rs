use leptos::*;
use leptos_router::*;

#[component]
pub fn AdminLayout(
    #[prop()] children: Children
) -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    let is_moderator = move || auth_state.map(|s| s.is_moderator()).unwrap_or(false);
    let show_admin_tools = move || is_admin() || is_moderator();
    
    // Get current route to highlight active menu item
    let route = use_location().pathname;
    
    view! {
        <div class="admin-layout">
            {move || if !show_admin_tools() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <div class="row">
                        <div class="col-md-3 col-lg-2">
                            <div class="card admin-sidebar">
                                <div class="card-header">
                                    <h5 class="mb-0">"Admin Panel"</h5>
                                </div>
                                <div class="list-group list-group-flush">
                                    <A 
                                        href="/admin/dashboard"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/dashboard"
                                    >
                                        <i class="bi bi-speedometer2 me-2"></i>
                                        "Dashboard"
                                    </A>
                                    
                                    {move || if is_admin() {
                                        view! {
                                            <A 
                                                href="/admin/users"
                                                class="list-group-item list-group-item-action d-flex align-items-center"
                                                class:active=move || route() == "/admin/users"
                                            >
                                                <i class="bi bi-people me-2"></i>
                                                "User Management"
                                            </A>
                                        }
                                    } else {
                                        view! {}
                                    }}
                                    
                                    <A 
                                        href="/admin/content"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/content"
                                    >
                                        <i class="bi bi-layout-text-window me-2"></i>
                                        "Content Moderation"
                                    </A>
                                    
                                    <A 
                                        href="/admin/reports"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/reports"
                                    >
                                        <i class="bi bi-flag me-2"></i>
                                        "Reported Content"
                                    </A>
                                    
                                    <A 
                                        href="/admin/tags"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/tags"
                                    >
                                        <i class="bi bi-tags me-2"></i>
                                        "Tag Management"
                                    </A>
                                    
                                    {move || if is_admin() {
                                        view! {
                                            <A 
                                                href="/admin/settings"
                                                class="list-group-item list-group-item-action d-flex align-items-center"
                                                class:active=move || route() == "/admin/settings"
                                            >
                                                <i class="bi bi-gear me-2"></i>
                                                "Forum Settings"
                                            </A>
                                        }
                                    } else {
                                        view! {}
                                    }}

                                    <A 
                                        href="/admin/customization"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/customization"
                                    >
                                        <i class="bi bi-palette-fill me-2"></i>
                                        "Site Customization"
                                    </A>
                                    
                                    <A 
                                        href="/admin/groups"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/groups"
                                    >
                                        <i class="bi bi-people-fill me-2"></i>
                                        "User Groups"
                                    </A>

                                    <A 
                                        href="/admin/logs"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/logs"
                                    >
                                        <i class="bi bi-journal-text me-2"></i>
                                        "Activity Logs"
                                    </A>

                                    <A 
                                        href="/admin/import-export"
                                        class="list-group-item list-group-item-action d-flex align-items-center"
                                        class:active=move || route() == "/admin/import-export"
                                    >
                                        <i class="bi bi-box-arrow-in-down me-2"></i>
                                        "Import & Export"
                                    </A>
                                    
                                    <A 
                                        href="/forum"
                                        class="list-group-item list-group-item-action d-flex align-items-center text-danger"
                                    >
                                        <i class="bi bi-x-circle me-2"></i>
                                        "Exit Admin Panel"
                                    </A>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-9 col-lg-10">
                            <div class="admin-content">
                                {children()}
                            </div>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}