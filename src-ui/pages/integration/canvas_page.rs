use leptos::*;
use crate::components::integration::CanvasIntegration;

#[component]
pub fn CanvasIntegrationPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <CanvasIntegration />
        </div>
    }
}
