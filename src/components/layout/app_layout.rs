use leptos::*;
use crate::components::layout::{Header, Footer, Sidebar};

#[component]
pub fn AppLayout(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="app-container">
            <Header />
            <div class="main-container">
                <Sidebar />
                <main class="content">
                    {children(cx)}
                </main>
            </div>
            <Footer />
        </div>
    }
}