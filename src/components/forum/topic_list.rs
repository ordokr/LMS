use leptos::*;
use web_sys::IntersectionObserver;
use super::models::Topic;
use crate::api::forum::{Topic, get_topics_by_category};
use crate::models::forum::topic::TopicSummary;
use crate::components::error_alert::ErrorAlert;
use crate::utils::date_utils::format_date_for_display;

// Add a custom hook for virtualized list
#[hook]
fn use_virtualized_list<T, F>(
    items: Signal<Vec<T>>,
    render_item: F,
    buffer_size: usize,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn(T, usize) -> View + Copy + 'static,
{
    let (visible_range, set_visible_range) = create_signal((0, 20));
    let (container_ref, set_container_ref) = create_signal::<Option<web_sys::Element>>(None);
    
    // Calculate visible items based on viewport
    create_effect(move |_| {
        if let Some(container) = container_ref.get() {
            let observer = IntersectionObserver::new(
                Closure::wrap(Box::new(move |entries: js_sys::Array, _observer: IntersectionObserver| {
                    if let Some(entry) = entries.get(0).dyn_into::<web_sys::IntersectionObserverEntry>().ok() {
                        let bounds = entry.client_bounds();
                        let height = bounds.height() as usize;
                        let scroll_top = entry.button_x() as usize;
                        
                        // Assuming each item is approximately 60px tall
                        let start = (scroll_top / 60).saturated_sub(buffer_size).max(0);
                        let visible_count = (height / 60) + buffer_size * 2;
                        let end = (start + visible_count).min(items.get().len());
                        
                        set_visible_range.set((start, end));
                    }
                }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>)
            ).unwrap();
            
            observer.observe(&container);
        }
    });
    
    // Render only visible items for performance
    let visible_items = create_memo(move |_| {
        let (start, end) = visible_range.get();
        let all_items = items.get();
        let total_height = all_items.len() * 60; // Assuming 60px per item
        
        // Items with their index
        let visible = all_items.into_iter()
            .enumerate()
            .filter(|(i, _)| *i >= start && *i < end)
            .collect::<Vec<_>>();
        
        (visible, start, total_height)
    });
    
    view! {
        <div 
            class="virtualized-list-container"
            style="position: relative; height: 100%; overflow: auto;"
            node_ref=move |el| set_container_ref.set(Some(el))
        >
            <div 
                class="virtualized-list-spacer"
                style=move || format!("height: {}px;", visible_items.get().2)
            >
                <div 
                    class="virtualized-list-items"
                    style=move || {
                        let (_, start, _) = visible_items.get();
                        format!("position: absolute; top: {}px; width: 100%;", start * 60)
                    }
                >
                    {move || {
                        let (items, _, _) = visible_items.get();
                        items.into_iter().map(|(i, item)| render_item(item, i)).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}

// Optimize the topic list component with virtualization
#[component]
pub fn TopicList(
    category_id: Option<String>,
) -> impl IntoView {
    let (topics, set_topics) = create_signal(Vec::<TopicSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    
    // Load topics
    let load_topics = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            let result = match category_id.as_ref() {
                Some(id) => {
                    invoke::<_, Vec<TopicSummary>>(
                        "list_topics_by_category",
                        &(id, Some(current_page.get()), Some(20))
                    ).await
                },
                None => {
                    invoke::<_, Vec<TopicSummary>>(
                        "list_latest_topics",
                        &(Some(current_page.get()), Some(20))
                    ).await
                }
            };
            
            match result {
                Ok(fetched_topics) => {
                    set_topics.set(fetched_topics);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load topics: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load topics on mount and when category_id changes
    create_effect(move |_| {
        // Reset to first page when category changes
        set_current_page.set(1);
        load_topics();
    });
    
    // Handle pagination
    let next_page = move |_| {
        set_current_page.update(|p| *p += 1);
        load_topics();
    };
    
    let prev_page = move |_| {
        if current_page.get() > 1 {
            set_current_page.update(|p| *p -= 1);
            load_topics();
        }
    };

    view! {
        <div class="topic-list">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            {move || {
                if loading.get() && topics.get().is_empty() {
                    view! { <div class="loading-state">"Loading topics..."</div> }
                } else if topics.get().is_empty() {
                    view! { <div class="empty-state">"No topics found"</div> }
                } else {
                    view! {
                        <div class="topics-table">
                            <div class="topics-header">
                                <div class="topic-title-header">"Topic"</div>
                                <div class="topic-category-header">"Category"</div>
                                <div class="topic-stats-header">"Replies"</div>
                                <div class="topic-stats-header">"Views"</div>
                                <div class="topic-activity-header">"Activity"</div>
                            </div>
                            
                            {topics.get().into_iter().map(|topic| {
                                let post_count_label = if topic.posts_count <= 1 {
                                    "No replies".to_string()
                                } else {
                                    format!("{} replies", topic.posts_count - 1)
                                };
                                
                                let activity_date = topic.last_posted_at.unwrap_or(topic.created_at);
                                let formatted_date = format_date_for_display(Some(&activity_date));
                                
                                view! {
                                    <div class="topic-row">
                                        <div class="topic-title">
                                            <a href={format!("/topics/{}", topic.id)} class="topic-link