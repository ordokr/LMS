use leptos::*;

#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
    view! { cx,
        <aside class="sidebar">
            <nav class="sidebar-nav">
                <ul>
                    <li><a href="/">"Dashboard"</a></li>
                    <li><a href="/courses">"My Courses"</a></li>
                    <li><a href="/calendar">"Calendar"</a></li>
                    <li><a href="/inbox">"Inbox"</a></li>
                    <li>
                        <span class="nav-category">"Course Tools"</span>
                        <ul>
                            <li><a href="/assignments">"Assignments"</a></li>
                            <li><a href="/forum">"Discussion Forum"</a></li>
                            <li><a href="/grades">"Grades"</a></li>
                            <li><a href="/resources">"Resources"</a></li>
                        </ul>
                    </li>
                </ul>
            </nav>
        </aside>
    }
}