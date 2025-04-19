use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};
use std::rc::Rc;
use std::cell::RefCell;

/// Props for the VirtualizedList component
#[derive(Props, Clone)]
pub struct VirtualizedListProps<T, F, V>
where
    T: Clone + 'static,
    F: Fn(T) -> V + 'static,
    V: IntoView + 'static,
{
    /// The items to render
    pub items: Signal<Vec<T>>,
    
    /// Function to render an item
    pub render_item: F,
    
    /// Height of each item in pixels
    #[prop(default = 50)]
    pub item_height: u32,
    
    /// Number of items to render beyond the visible area (buffer)
    #[prop(default = 5)]
    pub overscan: u32,
    
    /// CSS class for the container
    #[prop(default = "".to_string())]
    pub class: String,
    
    /// CSS class for the item container
    #[prop(default = "".to_string())]
    pub item_class: String,
    
    /// Height of the container (CSS value)
    #[prop(default = "500px".to_string())]
    pub height: String,
    
    /// Width of the container (CSS value)
    #[prop(default = "100%".to_string())]
    pub width: String,
    
    /// Whether to enable smooth scrolling
    #[prop(default = true)]
    pub smooth_scroll: bool,
}

/// A component that efficiently renders large lists by only rendering items that are visible
#[component]
pub fn VirtualizedList<T, F, V>(props: VirtualizedListProps<T, F, V>) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> V + 'static,
    V: IntoView + 'static,
{
    let VirtualizedListProps {
        items,
        render_item,
        item_height,
        overscan,
        class,
        item_class,
        height,
        width,
        smooth_scroll,
    } = props;
    
    // Container reference
    let container_ref = create_node_ref::<html::Div>();
    
    // Scroll position state
    let (scroll_top, set_scroll_top) = create_signal(0);
    
    // Calculate visible range based on scroll position
    let visible_range = create_memo(move |_| {
        let items_count = items.get().len();
        let container_height = container_ref
            .get()
            .map(|el| el.client_height() as u32)
            .unwrap_or(500);
        
        let start_index = (scroll_top.get() / item_height).max(0);
        let visible_items = (container_height / item_height) + 1;
        let end_index = (start_index + visible_items + overscan).min(items_count as u32);
        let start_index = start_index.saturating_sub(overscan);
        
        (start_index as usize, end_index as usize)
    });
    
    // Handle scroll events
    let on_scroll = move |_| {
        if let Some(el) = container_ref.get() {
            set_scroll_top.set(el.scroll_top() as u32);
        }
    };
    
    // Calculate total height of all items
    let total_height = create_memo(move |_| {
        items.get().len() as u32 * item_height
    });
    
    // Calculate offset for the visible items
    let items_offset = create_memo(move |_| {
        let (start_index, _) = visible_range.get();
        start_index as u32 * item_height
    });
    
    // Get the visible items
    let visible_items = create_memo(move |_| {
        let (start_index, end_index) = visible_range.get();
        items.get()[start_index..end_index.min(items.get().len())].to_vec()
    });
    
    view! {
        <div
            class=format!("virtualized-list-container {}", class)
            style=format!("height: {}; width: {}; overflow-y: auto; position: relative;", height, width)
            on:scroll=on_scroll
            _ref=container_ref
        >
            <div
                class="virtualized-list-total-height"
                style=move || format!("height: {}px; width: 100%;", total_height.get())
            >
                <div
                    class="virtualized-list-items"
                    style=move || format!("position: absolute; top: {}px; width: 100%;", items_offset.get())
                >
                    {move || {
                        visible_items.get().into_iter().map(|item| {
                            view! {
                                <div
                                    class=format!("virtualized-list-item {}", item_class)
                                    style=format!("height: {}px;", item_height)
                                >
                                    {render_item(item)}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Props for the VirtualizedGrid component
#[derive(Props, Clone)]
pub struct VirtualizedGridProps<T, F, V>
where
    T: Clone + 'static,
    F: Fn(T) -> V + 'static,
    V: IntoView + 'static,
{
    /// The items to render
    pub items: Signal<Vec<T>>,
    
    /// Function to render an item
    pub render_item: F,
    
    /// Height of each item in pixels
    #[prop(default = 200)]
    pub item_height: u32,
    
    /// Width of each item in pixels
    #[prop(default = 200)]
    pub item_width: u32,
    
    /// Number of columns in the grid
    #[prop(default = 3)]
    pub columns: u32,
    
    /// Gap between items in pixels
    #[prop(default = 16)]
    pub gap: u32,
    
    /// Number of items to render beyond the visible area (buffer)
    #[prop(default = 5)]
    pub overscan: u32,
    
    /// CSS class for the container
    #[prop(default = "".to_string())]
    pub class: String,
    
    /// CSS class for the item container
    #[prop(default = "".to_string())]
    pub item_class: String,
    
    /// Height of the container (CSS value)
    #[prop(default = "500px".to_string())]
    pub height: String,
    
    /// Width of the container (CSS value)
    #[prop(default = "100%".to_string())]
    pub width: String,
}

/// A component that efficiently renders large grids by only rendering items that are visible
#[component]
pub fn VirtualizedGrid<T, F, V>(props: VirtualizedGridProps<T, F, V>) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T) -> V + 'static,
    V: IntoView + 'static,
{
    let VirtualizedGridProps {
        items,
        render_item,
        item_height,
        item_width,
        columns,
        gap,
        overscan,
        class,
        item_class,
        height,
        width,
    } = props;
    
    // Container reference
    let container_ref = create_node_ref::<html::Div>();
    
    // Scroll position state
    let (scroll_top, set_scroll_top) = create_signal(0);
    
    // Calculate row height including gap
    let row_height = item_height + gap;
    
    // Calculate visible range based on scroll position
    let visible_range = create_memo(move |_| {
        let items_count = items.get().len();
        let container_height = container_ref
            .get()
            .map(|el| el.client_height() as u32)
            .unwrap_or(500);
        
        let rows = (items_count as f32 / columns as f32).ceil() as u32;
        let start_row = (scroll_top.get() / row_height).max(0);
        let visible_rows = (container_height / row_height) + 1;
        let end_row = (start_row + visible_rows + overscan).min(rows);
        let start_row = start_row.saturating_sub(overscan);
        
        let start_index = start_row * columns;
        let end_index = (end_row * columns).min(items_count as u32);
        
        (start_index as usize, end_index as usize)
    });
    
    // Handle scroll events
    let on_scroll = move |_| {
        if let Some(el) = container_ref.get() {
            set_scroll_top.set(el.scroll_top() as u32);
        }
    };
    
    // Calculate total height of all items
    let total_height = create_memo(move |_| {
        let items_count = items.get().len();
        let rows = (items_count as f32 / columns as f32).ceil() as u32;
        rows * row_height
    });
    
    // Calculate offset for the visible items
    let items_offset = create_memo(move |_| {
        let (start_index, _) = visible_range.get();
        (start_index as u32 / columns) * row_height
    });
    
    // Get the visible items
    let visible_items = create_memo(move |_| {
        let (start_index, end_index) = visible_range.get();
        items.get()[start_index..end_index.min(items.get().len())].to_vec()
    });
    
    view! {
        <div
            class=format!("virtualized-grid-container {}", class)
            style=format!("height: {}; width: {}; overflow-y: auto; position: relative;", height, width)
            on:scroll=on_scroll
            _ref=container_ref
        >
            <div
                class="virtualized-grid-total-height"
                style=move || format!("height: {}px; width: 100%;", total_height.get())
            >
                <div
                    class="virtualized-grid-items"
                    style=move || format!("position: absolute; top: {}px; width: 100%; display: grid; grid-template-columns: repeat({}, {}px); gap: {}px;", 
                        items_offset.get(), columns, item_width, gap)
                >
                    {move || {
                        visible_items.get().into_iter().map(|item| {
                            view! {
                                <div
                                    class=format!("virtualized-grid-item {}", item_class)
                                    style=format!("height: {}px; width: {}px;", item_height, item_width)
                                >
                                    {render_item(item)}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}
