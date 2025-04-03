use leptos::*;

#[component]
pub fn ErrorDisplay(cx: Scope, message: String) -> impl IntoView {
    view! { cx,
        <div class="error-display">
            <div class="error-message">
                {message}
            </div>
        </div>
    }
}