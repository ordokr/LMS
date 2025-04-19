use leptos::*;
use crate::components::QuentiLauncher;

#[component]
pub fn HomeRoute() -> impl IntoView {
    view! {
        <div class="page-container">
            <h1 class="page-title">Home</h1>
            <p>Welcome to Ordo LMS</p>

            <div class="modules-section">
                <h2>Available Modules</h2>

                <div class="modules-grid">
                    <QuentiLauncher />

                    <!-- Other modules can be added here -->
                </div>
            </div>
        </div>
    }
}
