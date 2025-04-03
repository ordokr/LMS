use leptos::*;
use crate::models::forum::Topic;
use crate::services::forum::ForumService;
use web_sys::SubmitEvent;
// Add this import for RichEditor
use crate::components::forum::rich_editor::RichEditor;
use crate::components::forum::tag_selector::TagSelector;

#[component]
pub fn TopicForm(
    #[prop(optional)] topic_id: Option<i64>,  // Change to i64 to match your model
    #[prop(optional)] category_id: Option<i64>, // Change to i64 to match your model
) -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    let (pinned, set_pinned) = create_signal(false);  // Add pinned property
    let (locked, set_locked) = create_signal(false);  // Add locked property
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (tags, set_tags) = create_signal(Vec::<String>::new()); // Add a signal for tags
    
    // Load existing topic if editing
    create_effect(move |_| {
        if let Some(id) = topic_id {
            set_saving.set(true);
            
            spawn_local(async move {
                match ForumService::get_topic(id).await {
                    Ok(topic) => {
                        set_title.set(topic.title);
                        set_pinned.set(topic.pinned);  // Set pinned status
                        set_locked.set(topic.locked);  // Set locked status
                        
                        // Get first post content
                        // If your API returns posts with the topic, use this:
                        if let Some(posts) = topic.get_posts() {
                            if let Some(first_post) = posts.first() {
                                set_content.set(first_post.content.clone());
                            }
                        } else {
                            // If posts are not included, fetch separately
                            match ForumService::get_topic_first_post(id).await {
                                Ok(post) => {
                                    set_content.set(post.content);
                                },
                                Err(e) => {
                                    set_error.set(Some(format!("Failed to load topic content: {}", e)));
                                }
                            }
                        }
                        
                        // Set tags if available
                        if let Some(topic_tags) = topic.tags {
                            set_tags.set(topic_tags);
                        }
                        
                        set_saving.set(false);
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load topic: {}", e)));
                        set_saving.set(false);
                    }
                }
            });
        }
    });
    
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_saving.set(true);
        set_error.set(None);
        
        // Generate a slug from the title
        let slug = title()
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        
        let t_id = topic_id;
        let c_id = category_id;
        let t_title = title();
        let t_content = content();
        let t_pinned = pinned();
        let t_locked = locked();
        let t_tags = tags();
        
        spawn_local(async move {
            let result = if let Some(id) = t_id {
                // Try the enhanced method first, fall back to original if not available
                match ForumService::update_topic_enhanced(id, &t_title, &slug, t_pinned, t_locked, &t_content).await {
                    Ok(topic) => Ok(topic),
                    Err(_) => {
                        // Fall back to original method if enhanced is not implemented
                        ForumService::update_topic(&id.to_string(), &t_title, &t_content).await
                    }
                }
            } else if let Some(cid) = c_id {
                // Try the enhanced method first, fall back to original if not available
                match ForumService::create_topic_enhanced(cid, &t_title, &slug, t_pinned, t_locked, &t_content).await {
                    Ok(topic) => Ok(topic),
                    Err(_) => {
                        // Fall back to original method if enhanced is not implemented
                        ForumService::create_topic(&cid.to_string(), &t_title, &t_content).await
                    }
                }
            } else {
                Err("Missing category ID for new topic".to_string())
            };
            
            match result {
                Ok(new_topic) => {
                    // Navigate to topic detail
                    let window = web_sys::window().unwrap();
                    let redirect_url = format!("/forum/topics/{}", new_topic.id);
                    let _ = window.location().set_href(&redirect_url);
                    set_saving.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Error saving topic: {}", e)));
                    set_saving.set(false);
                }
            }
        });
    };
    
    view! {
        <div class="topic-form card p-4">
            <h2 class="mb-4">{move || if topic_id.is_some() { "Edit Topic" } else { "Create New Topic" }}</h2>
            
            {move || error().map(|err| view! {
                <div class="alert alert-danger">{err}</div>
            })}
            
            <form on:submit=on_submit>
                <div class="mb-3">
                    <label for="title" class="form-label">"Topic Title"</label>
                    <input
                        type="text"
                        id="title"
                        class="form-control"
                        prop:value=move || title()
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="mb-3">
                    <label for="content" class="form-label">"Content"</label>
                    <RichEditor
                        content=content
                        set_content=set_content
                        placeholder=Some("Write your topic content here...")
                        rows=Some(10)
                    />
                </div>
                
                <div class="mb-3">
                    <div class="form-check">
                        <input 
                            class="form-check-input" 
                            type="checkbox" 
                            id="pinned"
                            prop:checked=move || pinned()
                            on:change=move |ev| set_pinned.set(event_target_checked(&ev))
                        />
                        <label class="form-check-label" for="pinned">"Pin this topic"</label>
                    </div>
                    <div class="form-check">
                        <input 
                            class="form-check-input" 
                            type="checkbox" 
                            id="locked"
                            prop:checked=move || locked()
                            on:change=move |ev| set_locked.set(event_target_checked(&ev))
                        />
                        <label class="form-check-label" for="locked">"Lock this topic"</label>
                    </div>
                </div>
                
                <div class="mb-3">
                    <label class="form-label">"Tags"</label>
                    <TagSelector
                        selected_tags=tags
                        set_selected_tags=set_tags
                        max_tags=Some(5)
                        allow_create=Some(true)
                        restricted_mode=Some(false)
                    />
                    <div class="form-text">
                        "Add up to 5 tags to categorize your topic."
                    </div>
                </div>
                
                <div class="d-flex gap-2">
                    <button 
                        type="submit" 
                        class="btn btn-primary"
                        disabled=move || saving()
                    >
                        {move || if saving() { "Saving..." } else { "Save Topic" }}
                    </button>
                    
                    {move || {
                        let back_url = if let Some(cat_id) = category_id {
                            format!("/forum/categories/{}", cat_id)
                        } else if let Some(t_id) = topic_id {
                            format!("/forum/topics/{}", t_id)
                        } else {
                            "/forum/categories".to_string()
                        };
                        
                        view! {
                            <a href={back_url} class="btn btn-outline-secondary">"Cancel"</a>
                        }
                    }}
                </div>
            </form>
        </div>
    }
}