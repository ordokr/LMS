use leptos::*;
use leptos_router::*;
use web_sys::MouseEvent;

use crate::models::forum::{Category, Topic};
use crate::services::forum_service::ForumService;
use crate::utils::errors::ApiError;

#[component]
pub fn ForumThreads(cx: Scope, category_id: i64) -> impl IntoView {
    let (category, set_category) = create_signal(cx, None::<Category>);
    let (topics, set_topics) = create_signal(cx, Vec::<Topic>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);
    
    // Form state for creating new topics
    let (show_new_topic_form, set_show_new_topic_form) = create_signal(cx, false);
    let topic_title = create_rw_signal(cx, String::new());
    let topic_content = create_rw_signal(cx, String::new());

    // Load category and topics when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            // Load category details
            match ForumService::get_category(category_id).await {
                Ok(cat_data) => {
                    set_category.set(Some(cat_data));
                }
                Err(err) => {
                    set_error.set(Some(format!("Error loading category: {}", err)));
                    set_loading.set(false);
                    return;
                }
            }

            // Load topics for this category
            match ForumService::get_topics_by_category(category_id).await {
                Ok(data) => {
                    set_topics.set(data);
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(format!("Error loading topics: {}", err)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Handle creating a new topic
    let handle_create_topic = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        let title = topic_title.get();
        let content = topic_content.get();
        
        if title.is_empty() || content.is_empty() {
            set_error.set(Some("Title and content are required".to_string()));
            return;
        }
        
        spawn_local(async move {
            match ForumService::create_topic(Topic {
                id: None,
                category_id,
                title: title.clone(),
                content: Some(content),
                created_by: 0, // This would be the current user's ID
                pinned: false,
                locked: false,
                post_count: None,
                last_post_at: None,
                created_at: None,
                updated_at: None,
            }).await {
                Ok(_) => {
                    // Reset form and refresh topics
                    topic_title.set(String::new());
                    topic_content.set(String::new());
                    set_show_new_topic_form.set(false);
                    
                    // Reload topics
                    match ForumService::get_topics_by_category(category_id).await {
                        Ok(data) => {
                            set_topics.set(data);
                        },
                        Err(err) => {
                            set_error.set(Some(format!("Error reloading topics: {}", err)));
                        }
                    }
                },
                Err(err) => {
                    set_error.set(Some(format!("Failed to create topic: {}", err)));
                }
            }
        });
    };

    view! { cx,
        <div class="forum-threads">
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <div class="error">{err}</div> }
                } else if let Some(cat) = category.get() {
                    view! { cx,
                        <div>
                            <div class="category-header">
                                <h1>{cat.name}</h1>
                                <div class="breadcrumbs">
                                    <A href="/forum">"Forum"</A>
                                    <span class="separator">"›"</span>
                                    <span class="current">{cat.name}</span>
                                </div>
                                
                                {if let Some(desc) = cat.description {
                                    view! { cx, <div class="category-description">{desc}</div> }
                                } else { view! { cx, <></> } }}
                            </div>
                            
                            <div class="actions">
                                <button 
                                    class="button primary"
                                    on:click=move |_| set_show_new_topic_form.set(!show_new_topic_form.get())
                                >
                                    {if show_new_topic_form.get() { "Cancel" } else { "New Topic" }}
                                </button>
                            </div>
                            
                            {move || {
                                if show_new_topic_form.get() {
                                    view! { cx,
                                        <div class="new-topic-form">
                                            <h3>"Create New Topic"</h3>
                                            <form on:submit=handle_create_topic>
                                                <div class="form-group">
                                                    <label for="topic-title">"Title"</label>
                                                    <input 
                                                        type="text"
                                                        id="topic-title"
                                                        prop:value=topic_title
                                                        on:input=move |ev| {
                                                            topic_title.set(event_target_value(&ev));
                                                        }
                                                        required
                                                    />
                                                </div>
                                                
                                                <div class="form-group">
                                                    <label for="topic-content">"Content"</label>
                                                    <textarea 
                                                        id="topic-content"
                                                        prop:value=topic_content
                                                        on:input=move |ev| {
                                                            topic_content.set(event_target_value(&ev));
                                                        }
                                                        rows="5"
                                                        required
                                                    ></textarea>
                                                </div>
                                                
                                                <div class="form-actions">
                                                    <button 
                                                        type="button" 
                                                        class="button"
                                                        on:click=move |_| set_show_new_topic_form.set(false)
                                                    >
                                                        "Cancel"
                                                    </button>
                                                    <button type="submit" class="button primary">
                                                        "Create Topic"
                                                    </button>
                                                </div>
                                            </form>
                                        </div>
                                    }
                                } else {
                                    view! { cx, <></> }
                                }
                            }}
                            
                            {if topics.get().is_empty() {
                                view! { cx, <div class="empty-state">"No topics in this category yet. Be the first to create one!"</div> }
                            } else {
                                view! { cx,
                                    <div class="topics-list">
                                        <table class="topics-table">
                                            <thead>
                                                <tr>
                                                    <th class="topic-title">"Topic"</th>
                                                    <th class="topic-stats">"Replies"</th>
                                                    <th class="topic-last-post">"Last Post"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {topics.get()
                                                    .into_iter()
                                                    .map(|topic| {
                                                        let id = topic.id.unwrap_or(0);
                                                        view! { cx,
                                                            <tr class="topic-row" class:pinned={topic.pinned} class:locked={topic.locked}>
                                                                <td class="topic-title">
                                                                    <A href=format!("/forum/thread/{}", id)>
                                                                        {topic.title.clone()}
                                                                    </A>
                                                                    
                                                                    <div class="topic-meta">
                                                                        {if topic.pinned {
                                                                            view! { cx, <span class="topic-pinned">"Pinned"</span> }
                                                                        } else {
                                                                            view! { cx, <></> }
                                                                        }}
                                                                        {if topic.locked {
                                                                            view! { cx, <span class="topic-locked">"Locked"</span> }
                                                                        } else {
                                                                            view! { cx, <></> }
                                                                        }}
                                                                        <span class="topic-created">
                                                                            {"Started "}
                                                                            {if let Some(date) = &topic.created_at {
                                                                                date.clone()
                                                                            } else {
                                                                                "Unknown date".to_string()
                                                                            }}
                                                                        </span>
                                                                    </div>
                                                                </td>
                                                                <td class="topic-stats">
                                                                    {topic.post_count.unwrap_or(0) - 1} // Subtract 1 for the initial post
                                                                </td>
                                                                <td class="topic-last-post">
                                                                    {if let Some(date) = &topic.last_post_at {
                                                                        date.clone()
                                                                    } else {
                                                                        "No replies yet".to_string()
                                                                    }}
                                                                </td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                                }
                                            </tbody>
                                        </table>
                                    </div>
                                }
                            }}
                        </div>
                    }
                } else {
                    view! { cx, <div class="error">"Category not found"</div> }
                }
            }}
        </div>
    }
}