use leptos::*;
use web_sys::SubmitEvent;
use leptos_router::use_navigate;
use crate::models::search::SearchSuggestion;
use crate::services::search::SearchService;

#[component]
pub fn SearchBar() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let navigate = use_navigate();
    
    // Add suggestions functionality
    let (suggestions, set_suggestions) = create_signal(Vec::<SearchSuggestion>::new());
    let (show_suggestions, set_show_suggestions) = create_signal(false);
    let (loading, set_loading) = create_signal(false);
    
    // Get suggestions when query changes
    create_effect(move |_| {
        let q = query();
        if q.trim().len() < 2 {
            set_suggestions.set(Vec::new());
            return;
        }
        
        set_loading.set(true);
        
        spawn_local(async move {
            match SearchService::get_search_suggestions(&q).await {
                Ok(results) => {
                    set_suggestions.set(results);
                },
                Err(_) => {
                    set_suggestions.set(Vec::new());
                }
            }
            set_loading.set(false);
        });
    });
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let search_query = query().trim().to_string();
        
        if !search_query.is_empty() {
            set_show_suggestions.set(false);
            // Navigate to search page with query
            navigate(&format!("/forum/search?q={}", urlencoding::encode(&search_query)), Default::default());
        }
    };
    
    // Handle suggestion click
    let handle_suggestion_click = move |url: String| {
        set_show_suggestions.set(false);
        navigate(&url, Default::default());
    };
    
    // Focus and blur handlers
    let handle_focus = move |_| {
        if !query().trim().is_empty() && !suggestions().is_empty() {
            set_show_suggestions.set(true);
        }
    };
    
    let handle_blur = move |_| {
        // Small delay to allow click events on suggestions to fire first
        spawn_local(async move {
            leptos::timeout(150, move || {
                set_show_suggestions.set(false);
            }).await;
        });
    };
    
    // Handle input changes
    let handle_input = move |ev| {
        let value = event_target_value(&ev);
        set_query.set(value);
        
        if query().trim().len() >= 2 {
            set_show_suggestions.set(true);
        } else {
            set_show_suggestions.set(false);
        }
    };
    
    // Get icon for suggestion type
    let get_suggestion_icon = |suggestion_type: &str| -> &'static str {
        match suggestion_type {
            "topic" => "bi-chat-text",
            "user" => "bi-person",
            "tag" => "bi-tag",
            "category" => "bi-folder",
            _ => "bi-search",
        }
    };
    
    view! {
        <div class="search-bar position-relative">
            <form class="d-flex" on:submit=handle_submit>
                <input 
                    class="form-control me-2" 
                    type="search" 
                    placeholder="Search forum..." 
                    aria-label="Search"
                    prop:value=move || query()
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                    autocomplete="off"
                />
                <button class="btn btn-outline-primary" type="submit">
                    {move || if loading() {
                        view! { <span class="spinner-border spinner-border-sm" role="status"></span> }
                    } else {
                        view! { <i class="bi bi-search"></i> }
                    }}
                </button>
            </form>
            
            {move || if show_suggestions() && !suggestions().is_empty() {
                view! {
                    <div class="search-suggestions position-absolute w-100 mt-1 shadow-sm rounded border bg-body z-3">
                        <ul class="list-group list-group-flush">
                            {suggestions().into_iter().take(10).map(|suggestion| {
                                let url = suggestion.url.clone();
                                let icon = get_suggestion_icon(&suggestion.type_);
                                
                                view! {
                                    <li class="list-group-item list-group-item-action p-2"
                                        on:mousedown=move |_| handle_suggestion_click(url.clone())
                                    >
                                        <div class="d-flex align-items-center">
                                            <div class="me-2">
                                                <i class={format!("bi {}", icon)}></i>
                                            </div>
                                            <div>
                                                <div>{&suggestion.text}</div>
                                                <small class="text-muted">{&suggestion.type_.to_uppercase()}</small>
                                            </div>
                                        </div>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                            
                            <li class="list-group-item list-group-item-action p-2 text-center">
                                <a 
                                    href=format!("/forum/search?q={}", urlencoding::encode(&query()))
                                    class="text-decoration-none"
                                    on:mousedown=move |ev| {
                                        ev.prevent_default();
                                        handle_suggestion_click(format!("/forum/search?q={}", urlencoding::encode(&query())));
                                    }
                                >
                                    <i class="bi bi-search me-1"></i>
                                    {format!("Search for \"{}\"", query())}
                                </a>
                            </li>
                        </ul>
                    </div>
                }
            } else {
                view! {}
            }}
        </div>
    }
}