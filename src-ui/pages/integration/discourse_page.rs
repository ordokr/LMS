use leptos::*;
use crate::components::integration::DiscourseIntegration;

#[component]
pub fn DiscourseIntegrationPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <DiscourseIntegration />
        </div>
    }
}
