use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::CustomEvent;
use crate::util::debounce::use_debounced;

// Tauri API bindings
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(CustomEvent)>);
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SearchStatus {
    state: String,
    documents_indexed: usize,
    is_available: bool,
    last_indexed: Option<String>,
}

#[component]
pub fn OptimizedSearchBox(
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] category_id: Option<i64>,
    #[prop(optional)] enable_search_button: Option<bool>,
) -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (is_searching, set_is_searching) = create_signal(false);
    let (search_status, set_search_status) = create_signal(None::<SearchStatus>);
    let (indexing_in_progress, set_indexing_in_progress) = create_signal(false);
    
    // Use debounced query to avoid excessive API calls
    let debounced_query = use_debounced(query, 300);
    
    // Start search service when component mounts
    create_effect(move |_| {
        spawn_local(async move {
            // Start search service
            let args = JsValue::NULL;
            let result = invoke("start_search", args).await;
            if let Ok(success) = serde_wasm_bindgen::from_value::<bool>(result) {
                log::info!("Search service started: {}", success);
                // Get initial status
                update_search_status().await;
            }
            
            // Setup event listeners for search sync events
            let sync_complete = Closure::wrap(Box::new(move |event: CustomEvent| {
                if let Ok(count) = event.detail().as_f64() {
                    log::info!("Search sync completed: {} documents", count as usize);
                    set_indexing_in_progress.set(false);
                    spawn_local(update_search_status());
                }
            }) as Box<dyn FnMut(CustomEvent)>);
            
            let sync_error = Closure::wrap(Box::new(move |event: CustomEvent| {
                log::error!("Search sync error: {:?}", event.detail());
                set_indexing_in_progress.set(false);
            }) as Box<dyn FnMut(CustomEvent)>);
            
            listen("search:sync-complete", &sync_complete).await;
            listen("search:sync-error", &sync_error).await;
            
            // Keep closures alive
            sync_complete.forget();
            sync_error.forget();
        });
    });
    
    // Create resource based on debounced query
    let search_results = create_resource(
        move || (debounced_query.get(), category_id, search_status.get()),
        move |(q, cat_id, status)| async move {
            if q.trim().len() < 2 {
                return None;
            }
            
            // Check if search is available
            if let Some(status) = status {
                if !status.is_available {
                    return None;
                }
            } else {
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
        // Build request arguments
        #[derive(Serialize)]
        struct SearchArgs {
            query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: Option<i64>,
        }
        
        let args = SearchArgs {
            query: query.to_string(),
            category_id,
        };
        
        // Call search API
        let args_value = serde_wasm_bindgen::to_value(&args).ok()?;
        let result = invoke("search_topics", args_value).await;
        
        // Parse response
        match serde_wasm_bindgen::from_value::<SearchResponse>(result) {
            Ok(data) => Some(data),
            Err(e) => {
                log::error!("Search error: {:?}", e);
                None
            }
        }
    }
    
    // Update search status
    async fn update_search_status() {
        let args = JsValue::NULL;
        match invoke("get_search_status", args).await {
            value => {
                if let Ok(status) = serde_wasm_bindgen::from_value::<SearchStatus>(value) {
                    set_search_status.set(Some(status));
                }
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
        
        // Highlight search terms (could be enhanced with proper HTML highlighting)
        snippet
    }
    
    // Function to trigger search sync
    let trigger_sync = move |_| {
        set_indexing_in_progress.set(true);
        spawn_local(async move {
            let args = JsValue::NULL;
            let _ = invoke("sync_search_data", args).await;
        });
    };
    
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
                    disabled={move || !search_status.get().map(|s| s.is_available).unwrap_or(false)}
                />
                
                {move || if enable_search_button.unwrap_or(false) {
                    view! {
                        <button 
                            class="search-sync-button"
                            on:click=trigger_sync
                            disabled={indexing_in_progress.get()}
                        >
                            {move || if indexing_in_progress.get() {
                                "Syncing..."
                            } else {
                                "Sync"
                            }}
                        </button>
                    }
                } else {
                    view! {}
                }}
                
                {move || if is_searching.get() {
                    view! { <div class="spinner"></div> }
                } else {
                    view! {}
                }}
            </div>
            
            // Search status indicator
            {move || match search_status.get() {
                None => view! {},
                Some(status) if !status.is_available => view! {
                    <div class="search-status warning">
                        "Search is not available. " 
                        {if enable_search_button.unwrap_or(false) {
                            view! { <button on:click=trigger_sync>"Initialize"</button> }
                        } else {
                            view! {}
                        }}
                    </div>
                },
                Some(status) => view! {
                    <div class="search-status">
                        <span class="search-indexed">{status.documents_indexed}</span>
                        " documents indexed"
                        {status.last_indexed.map(|t| view! {
                            <span class="search-timestamp">" (last updated: " {t} ")"</span>
                        })}
                    </div>
                }
            }}
            
            // Search results
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
                    let status = search_status.get();
                    if status.map(|s| s.is_available).unwrap_or(false) {
                        view! { <div class="search-results loading"></div> }
                    } else {
                        view! { <div class="search-results unavailable">
                            "Search is not available."
                        </div> }
                    }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}