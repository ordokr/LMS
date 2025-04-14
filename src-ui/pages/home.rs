use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <main>
            <h1>"Welcome to the Forum"</h1>
            <A href="/categories">"Browse Categories"</A>
        </main>
    }
}
