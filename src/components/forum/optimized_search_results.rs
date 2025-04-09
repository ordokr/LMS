use leptos::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, HtmlElement, IntersectionObserver, IntersectionObserverEntry};

#[component]
pub fn OptimizedSearchResults(
    #[prop()] results: Vec<SearchHit>,
    #[prop()] total_hits: usize,
    #[prop(default = false)] loading: bool,
    #[prop(default = false)] show_category: bool,
) -> impl IntoView {
    // Format date helper
    let format_date = |date_str: &str| {
        match chrono::DateTime::parse_from_rfc3339(date_str) {
            Ok(dt) => dt.format("%b %d, %Y").to_string(),
            Err(_) => date_str.to_string(),
        }
    };
    
    // Implementation of intersection observer for lazy loading images and content
    let setup_intersection_observer = move |element: HtmlElement| {
        let options = web_sys::IntersectionObserverInit::new();
        options.root_margin("100px");
        options.threshold(&js_sys::Array::from(&wasm_bindgen::JsValue::from(0.1)));
        
        // Closure to handle intersection
        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: IntersectionObserver| {
            for i in 0..entries.length() {
                let entry = entries.get(i).unchecked_into::<IntersectionObserverEntry>();
                let target = entry.target().unchecked_into::<Element>();
                
                if entry.is_intersecting() {
                    // Mark as visible
                    target.class_list().remove_1("lazy-load").unwrap_or_default();
                    target.set_attribute("data-loaded", "true").unwrap_or_default();
                }
            }
        }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);
        
        // Create observer
        let observer = IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
            .unwrap_throw();
            
        // Observe all lazy-load elements
        let lazy_elements = element.query_selector_all(".lazy-load").unwrap_throw();
        for i in 0..lazy_elements.length() {
            observer.observe(&lazy_elements.get(i).unwrap());
        }
        
        callback.forget();
    };
    
    // Create a reference to the results container
    let results_ref = create_node_ref::<html::Div>();
    
    // Set up the observer after component mounts
    create_effect(move |_| {
        if let Some(container) = results_ref.get() {
            setup_intersection_observer(container);
        }
    });
    
    // Create virtualized results for better performance
    let chunk_size = 10;
    let chunks = (0..results.len()).step_by(chunk_size)
        .map(|start| {
            let end = (start + chunk_size).min(results.len());
            &results[start..end]
        })
        .collect::<Vec<_>>();
    
    view! {
        <div class="search-results-container">
            {move || if loading {
                view! { <div class="search-loading-indicator">Loading results...</div> }
            } else if results.is_empty() {
                view! { <div class="no-results">No results found</div> }
            } else {
                view! {
                    <div>
                        <div class="results-count">
                            {format!("Found {} results", total_hits)}
                        </div>
                        
                        <div class="search-results-list" node_ref=results_ref>
                            {chunks.into_iter().enumerate().map(|(chunk_index, chunk)| {
                                // Determine if this chunk should start as lazy or eager
                                let is_initial = chunk_index < 2; // First 2 chunks load immediately
                                
                                view! {
                                    <div
                                        class={if is_initial { "result-chunk" } else { "result-chunk lazy-load" }}
                                        data-chunk={chunk_index}
                                    >
                                        {chunk.iter().map(|hit| {
                                            view! {
                                                <div class="search-result-item">
                                                    <a href={format!("/forum/topic/{}", hit.id)} class="result-title">
                                                        <h3>{hit.title.clone()}</h3>
                                                    </a>
                                                    
                                                    <div class={if is_initial { "result-snippet" } else { "result-snippet lazy-load" }}>
                                                        {hit.content_snippet.clone()}
                                                    </div>
                                                    
                                                    <div class="result-meta">
                                                        {show_category.then(|| view! {
                                                            <span class="result-category">
                                                                {hit.category_name.clone().unwrap_or_default()}
                                                            </span>
                                                        })}
                                                        <span class="result-date">{format_date(&hit.created_at)}</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

// SearchHit definition for clarity
#[derive(Clone, Debug)]
struct SearchHit {
    id: i64,
    title: String,
    content_snippet: String,
    category_name: Option<String>,
    created_at: String,
}