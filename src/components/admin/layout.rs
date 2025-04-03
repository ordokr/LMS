use leptos::*;
use leptos_router::*;

#[component]
pub fn AdminSidebar(cx: Scope, current_page: String) -> impl IntoView {
    view! { cx,
        <div class="admin-sidebar">
            <h3>"Administration"</h3>
            <ul class="admin-menu">
                <li>
                    <A href="/admin" 
                       class={if current_page == "dashboard" { "active" } else { "" }}>
                        "Dashboard"
                    </A>
                </li>
                <li>
                    <A href="/admin/categories" 
                       class={if current_page == "categories" { "active" } else { "" }}>
                        "Categories"
                    </A>
                </li>
                <li>
                    <A href="/admin/users" 
                       class={if current_page == "users" { "active" } else { "" }}>
                        "User Management"
                    </A>
                </li>
                <li>
                    <A href="/admin/moderation" 
                       class={if current_page == "moderation" { "active" } else { "" }}>
                        "Content Moderation"
                    </A>
                </li>
                <li>
                    <A href="/admin/settings" 
                       class={if current_page == "settings" { "active" } else { "" }}>
                        "Forum Settings"
                    </A>
                </li>
                <li>
                    <A href="/">"Back to Forum"</A>
                </li>
            </ul>
        </div>
    }
}

#[component]
pub fn AdminLayout(cx: Scope, children: Children, current_page: String) -> impl IntoView {
    view! { cx,
        <div class="admin-content">
            <AdminSidebar current_page={current_page}/>
            <div class="admin-main">
                {children(cx)}
            </div>
        </div>
    }
}