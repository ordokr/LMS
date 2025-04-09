use leptos::*;
use std::rc::Rc;
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry};
use wasm_bindgen::{prelude::*, JsCast};

#[component]
pub fn VirtualList<T, F, V>(
    #[prop(into)] items: Signal<Vec<T>>,
    #[prop(into)] render_fn: F,
    #[prop(default = 25)] buffer_size: usize,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> V + Copy + 'static,
    V: IntoView,
{
    let (visible_range, set_visible_range) = create_signal((0, buffer_size));
    let container_ref = create_node_ref::<html::Div>();
    
    // Track total height for accurate scrollbar
    let estimated_item_height = 150.0; // pixels, can be refined based on actual data
    
    // Calculate viewport items
    let visible_items = create_memo(move |_| {
        let all_items = items.get();
        let (start, end) = visible_range.get();
        
        // Clamp end to actual item count
        let end = end.min(all_items.len());
        
        // Return visible slice
        all_items[start..end].to_vec()
    });
    
    // Total height for scrollbar accuracy
    let total_height = create_memo(move |_| {
        items.get().len() as f64 * estimated_item_height
    });
    
    // Set up scroll tracking
    let on_scroll = move |_| {
        let Some(container) = container_ref.get() else { return };
        
        let scroll_top = container.scroll_top() as f64;
        let client_height = container.client_height() as f64;
        
        // Calculate visible range
        let start_approx = (scroll_top / estimated_item_height).floor() as usize;
        let start = start_approx.saturating_sub(buffer_size / 2);
        let visible_count = (client_height / estimated_item_height).ceil() as usize + buffer_size;
        let end = start + visible_count;
        
        set_visible_range.set((start, end));
    };
    
    // Set up intersection observer for lazy loading
    create_effect(move |_| {
        if let Some(container) = container_ref.get() {
            // Create intersection observer for lazy loading items at edges
            let callback = Closure::wrap(Box::new(move |entries: Vec<IntersectionObserverEntry>, _observer: IntersectionObserver| {
                for entry in entries {
                    if entry.is_intersecting() {
                        // Load more items if we're near the end
                        let (start, end) = visible_range.get();
                        let items_count = items.get().len();
                        
                        if end > items_count - 10 {
                            // Near end, load more items if needed
                            // You can call your load_more function here
                        }
                    }
                }
            }) as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);
            
            let options = js_sys::Object::new();
            js_sys::Reflect::set(&options, &"root".into(), &container).unwrap();
            js_sys::Reflect::set(&options, &"rootMargin".into(), &"100px".into()).unwrap();
            js_sys::Reflect::set(&options, &"threshold".into(), &0.1.into()).unwrap();
            
            let observer = IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options).unwrap();
            
            // Observe the container itself
            observer.observe(&container);
            
            callback.forget();
        }
    });

    view! {
        <div 
            ref=container_ref
            on:scroll=on_scroll
            class="virtual-list-container"
            style="height: 100%; overflow-y: auto;"
        >
            <div 
                style=move || format!("height: {}px; position: relative;", total_height.get())
            >
                <For
                    each=visible_items
                    key=|item| format!("{:?}", item)
                    children=move |item| {
                        let (start, _) = visible_range.get();
                        let index = items.get().iter().position(|i| format!("{:?}", i) == format!("{:?}", item)).unwrap_or(0);
                        let offset = index.saturating_sub(start);
                        
                        view! {
                            <div
                                style=format!("position: absolute; top: {}px; width: 100%;", 
                                            (start + offset) as f64 * estimated_item_height)
                            >
                                {render_fn(item)}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}