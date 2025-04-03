use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="footer">
            <div class="footer-content">
                <p>"LMS Platform © 2025. All rights reserved."</p>
                <nav class="footer-links">
                    <a href="/help">"Help"</a>
                    <a href="/privacy">"Privacy Policy"</a>
                    <a href="/terms">"Terms of Service"</a>
                    <a href="/contact">"Contact Us"</a>
                </nav>
            </div>
        </footer>
    }
}