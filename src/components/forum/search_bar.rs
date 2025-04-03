use leptos::*;
use web_sys::SubmitEvent;
use leptos_router::use_navigate;

#[component]
pub fn SearchBar() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let navigate = use_navigate();
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let search_query = query().trim().to_string();
        
        if !search_query.is_empty() {
            // Navigate to search page with query
            navigate(&format!("/forum/search?q={}", urlencoding::encode(&search_query)), Default::default());
        }
    };
    
    view! {
        <form class="d-flex" on:submit=handle_submit>
            <input 
                class="form-control me-2" 
                type="search" 
                placeholder="Search forum..." 
                aria-label="Search"
                prop:value=move || query()
                on:input=move |ev| set_query.set(event_target_value(&ev))
            />
            <button class="btn btn-outline-primary" type="submit">
                <i class="bi bi-search"></i>
            </button>
        </form>
    }
}