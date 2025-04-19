use leptos::*;
use crate::components::quiz::Cmi5Manager;

#[component]
pub fn Cmi5Route() -> impl IntoView {
    view! {
        <div class="page-container">
            <h1 class="page-title">cmi5 Integration</h1>
            <Cmi5Manager />
        </div>
    }
}
