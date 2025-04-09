use leptos::*;
use std::collections::HashMap;
use std::rc::Rc;

/// A general-purpose virtualized list component with efficient rendering
#[component]
pub fn VirtualizedList<T, F, V>(
    #[prop(default = 500)] viewport_height: u32,
    #[prop(default = 50)] item_height: u32,
    #[prop(default = 10)] buffer_items: usize,
    #[prop(into)] items: MaybeSignal<Vec<T>>,
    #[prop(into)] render_item: F,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> V + 'static,
    V: IntoView,
{
    let (scroll_top, set_scroll_top) = create_signal(0);
    let (viewport_ref, set_viewport_ref) = create_signal::<Option<web_sys::Element>>(None);
    
    // Calculate which items should be visible based on scroll position
    let visible_range = create_memo(move |_| {
        let items_count = items.get().len();
        if items_count == 0 {
            return (0, 0);
        }
        
        let scroll = scroll_top.get();
        let start_idx = (scroll / item_height) as usize;
        
        // Apply buffer for smoother scrolling
        let start = if start_idx > buffer_items {
            start_idx - buffer_items
        } else {
            0
        };
        
        let visible_count = (viewport_height / item_height) as usize + buffer_items * 2;
        let end = (start + visible_count).min(items_count);
        
        (start, end)
    });
    
    // Handle scroll events
    let on_scroll = move |_| {
        if let Some(el) = viewport_ref.get() {
            set_scroll_top.set(el.scroll_top() as u32);
        }
    };
    
    // Get the part of the list that should be rendered
    let visible_items = create_memo(move |_| {
        let (start, end) = visible_range.get();
        let all_items = items.get();
        
        // Only render items in the visible range
        all_items[start..end].to_vec()
    });
    
    // Calculate spacers for proper scrolling
    let top_spacer_height = create_memo(move |_| {
        let (start, _) = visible_range.get();
        (start as u32) * item_height
    });
    
    let bottom_spacer_height = create_memo(move |_| {
        let (_, end) = visible_range.get();
        let total = items.get().len();
        ((total - end) as u32) * item_height
    });
    
    // Material UI-style list virtualization with efficient rendering
    view! {
        <div 
            class="virtualized-list-container"
            style=move || format!("height: {}px; overflow-y: auto; position: relative;", viewport_height)
            on:scroll=on_scroll
            node_ref=move |el| set_viewport_ref.set(Some(el))
        >
            // Top spacer maintains scroll position
            <div
                class="virtualized-list-spacer top-spacer"
                style=move || format!("height: {}px;", top_spacer_height.get())
            ></div>
            
            // Only render visible items
            <div class="virtualized-list-items">
                {move || {
                    visible_items.get()
                        .into_iter()
                        .map(|item| {
                            let rendered = render_item(item);
                            view! {
                                <div 
                                    class="virtualized-list-item"
                                    style=format!("height: {}px;", item_height)
                                >
                                    {rendered}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
            
            // Bottom spacer maintains proper scrollbar size
            <div
                class="virtualized-list-spacer bottom-spacer"
                style=move || format!("height: {}px;", bottom_spacer_height.get())
            ></div>
        </div>
    }
}