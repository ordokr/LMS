use leptos::*;
use crate::api::forum::{Topic, get_topics_by_category};
use crate::util::parallel_loader::use_parallel_resources;

/// Optimized topic list component with lazy loading and virtualization
#[component]
pub fn OptimizedTopicList(
    #[prop(optional)] category_ids: Vec<i64>,
    #[prop(default = 1)] page: i64,
    #[prop(default = 20)] per_page: i64,
) -> impl IntoView {
    // Format date helper
    let format_date = |date: chrono::DateTime<chrono::Utc>| {
        date.format("%b %d, %Y").to_string()
    };

    // Load topics for all categories in parallel
    let loaders = category_ids.iter().map(|&id| {
        (id, move |cat_id| get_topics_by_category(cat_id, page, per_page))
    }).collect::<Vec<_>>();
    
    let topics_resources = use_parallel_resources(loaders);
    
    // Create selector for active category
    let (active_tab, set_active_tab) = create_signal(0);
    
    // Track scroll position for virtualization
    let (scroll_pos, set_scroll_pos) = create_signal(0);
    
    // Handle scroll event
    let on_scroll = move |ev: leptos::ev::Event| {
        if let Some(target) = ev.target() {
            if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                set_scroll_pos.set(element.scroll_top() as i32);
            }
        }
    };
    
    view! {
        <div class="optimized-topic-list">
            // Tabs for category selection
            <div class="category-tabs">
                {move || category_ids.iter().enumerate().map(|(idx, &id)| {
                    // Get category name (would normally fetch from category API)
                    let category_name = format!("Category {}", id);
                    
                    view! {
                        <button 
                            class={if idx == active_tab.get() { "active" } else { "" }}
                            on:click=move |_| set_active_tab.set(idx)
                        >
                            {category_name}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            // Topics table with virtualization
            <div class="topic-container" on:scroll=on_scroll style="height: 500px; overflow-y: auto;">
                {move || {
                    match topics_resources.get() {
                        None => view! { <p>"Loading topics..."</p> },
                        Some(results) => {
                            let active_idx = active_tab.get();
                            if active_idx >= results.len() {
                                return view! { <p>"Invalid category selected"</p> };
                            }
                            
                            let result = &results[active_idx];
                            match &result.data {
                                Some(topics) => {
                                    if topics.is_empty() {
                                        view! { <p>"No topics found"</p> }
                                    } else {
                                        let scroll = scroll_pos.get();
                                        let row_height = 40; // Estimated height of each row
                                        let visible_count = 500 / row_height + 2; // Visible rows + buffer
                                        
                                        // Calculate which items to render
                                        let start_idx = (scroll / row_height) as usize;
                                        let end_idx = (start_idx + visible_count).min(topics.len());
                                        
                                        // Create spacers for virtualization
                                        let top_spacer = start_idx * row_height;
                                        let bottom_spacer = (topics.len() - end_idx) * row_height;
                                        
                                        let visible_topics = &topics[start_idx..end_idx];
                                        
                                        view! {
                                            <table class="topics-table" style="width: 100%;">
                                                <thead>
                                                    <tr>
                                                        <th>"Topic"</th>
                                                        <th>"Created"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    // Top spacer for virtualization
                                                    {if top_spacer > 0 {
                                                        view! {
                                                            <tr style={format!("height: {}px;", top_spacer)}>
                                                                <td colspan="2"></td>
                                                            </tr>
                                                        }
                                                    } else {
                                                        view! {}
                                                    }}
                                                    
                                                    // Only render visible rows
                                                    {visible_topics.iter().map(|topic| {
                                                        view! {
                                                            <tr style={format!("height: {}px;", row_height)}>
                                                                <td>
                                                                    <a href={format!("/forum/topic/{}", topic.id)}>
                                                                        {&topic.title}
                                                                    </a>
                                                                </td>
                                                                <td>{format_date(topic.created_at)}</td>
                                                            </tr>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                    
                                                    // Bottom spacer for virtualization
                                                    {if bottom_spacer > 0 {
                                                        view! {
                                                            <tr style={format!("height: {}px;", bottom_spacer)}>
                                                                <td colspan="2"></td>
                                                            </tr>
                                                        }
                                                    } else {
                                                        view! {}
                                                    }}
                                                </tbody>
                                            </table>
                                        }
                                    }
                                },
                                None => view! {
                                    <p class="error">"Error loading topics: " 
                                        {result.error.as_ref().map_or("Unknown error".to_string(), 
                                          |e| format!("{:?}", e))}
                                    </p>
                                }
                            }
                        }
                    }
                }}
            </div>
        </div>
    }
}