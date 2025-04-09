use leptos::*;
use crate::components::forum::search::SearchBox;

#[component]
pub fn LazySearchBox(
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] category_id: Option<i64>,
) -> impl IntoView {
    // Track if the search component has been loaded
    let (is_loaded, set_is_loaded) = create_signal(false);
    
    // Track if the search component is visible in the viewport
    let search_ref = create_node_ref::<html::Div>();
    
    // Function to check if element is in viewport
    let check_visibility = move || {
        if is_loaded.get() {
            return;
        }
        
        if let Some(element) = search_ref.get() {
            let rect = element.get_bounding_client_rect();
            let viewport_height = window().inner_height().unwrap().as_f64().unwrap_or(0.0);
            
            // If element is in viewport or about to enter it, load the search component
            if rect.top() < viewport_height + 200.0 {
                set_is_loaded.set(true);
            }
        }
    };
    
    // Set up scroll and resize listeners
    create_effect(move |_| {
        let window = window();
        
        // Check initial visibility
        check_visibility();
        
        // Set up scroll listener
        let scroll_callback = Closure::wrap(Box::new(move || {
            check_visibility();
        }) as Box<dyn FnMut()>);
        
        window.add_event_listener_with_callback("scroll", scroll_callback.as_ref().unchecked_ref())
            .unwrap_or_else(|e| log::error!("Failed to add scroll listener: {:?}", e));
            
        // Set up resize listener
        let resize_callback = Closure::wrap(Box::new(move || {
            check_visibility();
        }) as Box<dyn FnMut()>);
        
        window.add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
            .unwrap_or_else(|e| log::error!("Failed to add resize listener: {:?}", e));
        
        // Keep callbacks alive
        scroll_callback.forget();
        resize_callback.forget();
    });
    
    view! {
        <div class="search-box-container" node_ref=search_ref>
            {move || {
                if is_loaded.get() {
                    view! {
                        <SearchBox 
                            placeholder=placeholder.clone()
                            category_id=category_id
                        />
                    }
                } else {
                    view! {
                        <div class="search-placeholder">
                            <div class="search-input-placeholder">
                                <input
                                    type="text"
                                    placeholder={placeholder.clone().unwrap_or_else(|| "Search topics...".into())}
                                    on:focus=move |_| { set_is_loaded.set(true); }
                                />
                            </div>
                        </div>
                    }
                }
            }}
        </div>
    }
}