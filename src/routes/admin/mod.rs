use leptos::*;
use leptos_router::*;
use crate::components::admin::performance_dashboard::PerformanceDashboard;

#[component]
pub fn AdminRoutes() -> impl IntoView {
    view! {
        <Route path="/" view=AdminDashboard/>
        <Route path="users" view=AdminUsers/>
        <Route path="courses" view=AdminCourses/>
        <Route path="forum" view=AdminForum/>
        <Route path="performance" view=AdminPerformance/>
    }
}

#[component]
fn AdminDashboard() -> impl IntoView {
    view! {
        <div class="admin-dashboard">
            <h1>"Admin Dashboard"</h1>
            <div class="admin-cards">
                <AdminCard title="Users" count=create_signal(0).0 route="/admin/users" />
                <AdminCard title="Courses" count=create_signal(0).0 route="/admin/courses" />
                <AdminCard title="Forum Topics" count=create_signal(0).0 route="/admin/forum" />
                <AdminCard title="Performance" icon="chart-line" route="/admin/performance" />
            </div>
        </div>
    }
}

// Performance monitoring dashboard page
#[component]
fn AdminPerformance() -> impl IntoView {
    view! {
        <div class="admin-performance">
            <h1>"Performance Monitoring"</h1>
            <p class="admin-description">
                "Track and optimize your forum's performance metrics to ensure the best user experience."
            </p>
            
            <PerformanceDashboard />
        </div>
    }
}

// Other admin components (placeholders)
#[component]
fn AdminUsers() -> impl IntoView {
    view! { <h1>"User Management"</h1> }
}

#[component]
fn AdminCourses() -> impl IntoView {
    view! { <h1>"Course Management"</h1> }
}

#[component]
fn AdminForum() -> impl IntoView {
    view! { <h1>"Forum Management"</h1> }
}

// Admin card component
#[component]
fn AdminCard(
    #[prop(into)] title: String,
    #[prop(optional)] count: Option<Signal<i32>>,
    #[prop(into)] route: String,
    #[prop(optional)] icon: Option<&'static str>,
) -> impl IntoView {
    view! {
        <A href=route class="admin-card">
            <div class="card-content">
                <h2>{title}</h2>
                {move || if let Some(count) = count {
                    view! { <div class="card-count">{count.get()}</div> }
                } else {
                    view! { <></> }
                }}
                {move || if let Some(icon_name) = icon {
                    view! { <div class=format!("card-icon icon-{}", icon_name)></div> }
                } else {
                    view! { <></> }
                }}
            </div>
        </A>
    }
}