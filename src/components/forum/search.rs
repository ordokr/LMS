use leptos::*;
use serde::{Deserialize, Serialize};
use crate::util::debounce::use_debounced;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SearchHit {
    id: i64,
    title: String,
    content: Option<String>,
    category_id: Option<i64>,
    category_name: Option<String>,
    created_at: String,
    slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SearchResponse {
    hits: Vec<SearchHit>,
    total_hits: usize,
    processing_time_ms: u64,
}

#[component]
pub fn SearchBox(
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] category_id: Option<i64>,
) -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (is_searching, set_is_searching) = create_signal(false);
    
    // Use debounced query to avoid excessive API calls
    let debounced_query = use_debounced(query, 300);
    
    // Create resource based on debounced query
    let search_results = create_resource(
        move || (debounced_query.get(), category_id),
        move |(q, cat_id)| async move {
            if q.trim().len() < 2 {
                return None;
            }
            
            set_is_searching.set(true);
            
            let result = search_topics(&q, cat_id).await;
            
            set_is_searching.set(false);
            result
        }
    );
    
    // Function to call search API
    async fn search_topics(query: &str, category_id: Option<i64>) -> Option<SearchResponse> {
        let mut url = format!("/api/search/topics?q={}", query);
        
        if let Some(cat_id) = category_id {
            url.push_str(&format!("&category_id={}", cat_id));
        }
        
        // Call API
        match reqwasm::http::Request::get(&url)
            .send()
            .await
            .and_then(|res| res.json::<SearchResponse>().await)
        {
            Ok(data) => Some(data),
            Err(e) => {
                log::error!("Search error: {:?}", e);
                None
            }
        }
    }
    
    // Format content snippet to highlight search terms
    fn format_snippet(content: &str, query: &str) -> String {
        let query_terms = query.split_whitespace().collect::<Vec<_>>();
        
        // Find first occurrence of any term
        let mut best_pos = content.len();
        for term in &query_terms {
            if let Some(pos) = content.to_lowercase().find(&term.to_lowercase()) {
                if pos < best_pos {
                    best_pos = pos;
                }
            }
        }
        
        // Extract snippet around the best position
        let start = if best_pos > 50 { best_pos - 50 } else { 0 };
        let end = std::cmp::min(start + 200, content.len());
        let mut snippet = if start > 0 { "..." } else { "" }.to_string();
        snippet.push_str(&content[start..end]);
        if end < content.len() {
            snippet.push_str("...");
        }
        
        snippet
    }
    
    view! {
        <div class="search-box">
            <div class="search-input-container">
                <input
                    type="text"
                    placeholder={placeholder.unwrap_or_else(|| "Search topics...".into())}
                    on:input=move |ev| {
                        set_query.set(event_target_value(&ev));
                    }
                    value={move || query.get()}
                />
                {move || if is_searching.get() {
                    view! { <div class="spinner"></div> }
                } else {
                    view! {}
                }}
            </div>
            
            {move || {
                if let Some(results) = search_results.get().flatten() {
                    if results.hits.is_empty() {
                        view! {
                            <div class="search-results empty">
                                <p>"No results found for: " {debounced_query.get()}</p>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="search-results">
                                <div class="results-meta">
                                    <p>{results.total_hits} " results found in " {results.processing_time_ms} "ms"</p>
                                </div>
                                <ul class="results-list">
                                    {results.hits.into_iter().map(|hit| {
                                        let title = hit.title.clone();
                                        let snippet = hit.content
                                            .as_ref()
                                            .map(|c| format_snippet(c, &debounced_query.get()))
                                            .unwrap_or_default();
                                            
                                        view! {
                                            <li class="search-result-item">
                                                <a href={format!("/forum/topic/{}", hit.id)}>
                                                    <h4>{title}</h4>
                                                </a>
                                                <p class="snippet" inner_html={snippet}></p>
                                                <div class="result-meta">
                                                    {hit.category_name.map(|name| view! {
                                                        <span class="category">{name}</span>
                                                    })}
                                                    <span class="date">{hit.created_at}</span>
                                                </div>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                        }
                    }
                } else if debounced_query.get().trim().len() >= 2 {
                    view! { <div class="search-results loading"></div> }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}

// Debounce hook implementation
#[path = "c:\\Users\\Tim\\Desktop\\LMS\\src\\util\\debounce.rs"]
mod debounce {
    use std::rc::Rc;
    use std::cell::RefCell;
    use leptos::*;
    
    #[hook]
    pub fn use_debounced<T>(signal: Signal<T>, ms: u32) -> Signal<T> 
    where 
        T: Clone + 'static + PartialEq
    {
        let (debounced_value, set_debounced_value) = create_signal(signal.get());
        let timeout_id = Rc::new(RefCell::new(None::<i32>));
        
        create_effect(move |_| {
            let current = signal.get();
            let timeout_clone = timeout_id.clone();
            
            // Clear existing timeout
            if let Some(id) = *timeout_clone.borrow() {
                clear_timeout(id);
            }
            
            // Set new timeout
            let new_timeout = set_timeout(
                move || {
                    set_debounced_value.set(current);
                },
                ms,
            );
            
            *timeout_clone.borrow_mut() = Some(new_timeout);
        });
        
        debounced_value
    }
}