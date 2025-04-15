# Discussions API Reference

This document describes the Tauri command API for discussions in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `get_discussions` | `get_discussions(course_id: string)` | Retrieves discussions for a course | Implemented |
| `get_discussion` | `get_discussion(discussion_id: string)` | Retrieves a specific discussion by ID | Implemented |
| `create_discussion` | `create_discussion(discussion_create: DiscussionCreate)` | Creates a new discussion | Implemented |
| `update_discussion` | `update_discussion(discussion: Discussion)` | Updates a discussion | Implemented |
| `sync_discussion` | `sync_discussion(discussion_id: string)` | Syncs a discussion with Discourse | Implemented |
| `delete_discussion` | `delete_discussion(discussion_id: string)` | Deletes a discussion | Implemented |

## Data Types

### Discussion

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discussion {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub content: String,
    pub topic_id: Option<String>,
    pub status: DiscussionStatus,
    pub created_at: String,
    pub updated_at: String,
}
```

### DiscussionCreate

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionCreate {
    pub course_id: String,
    pub title: String,
    pub content: String,
    pub topic_id: Option<String>,
    pub status: Option<DiscussionStatus>,
}
```

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscussionStatus {
    Open,
    Locked,
    Archived,
    Pinned,
}

// In your Leptos component
use crate::models::discussion::{Discussion, DiscussionCreate, DiscussionStatus};
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn CourseDiscussions(course_id: String) -> impl IntoView {
    // Fetch discussions for this course
    let discussions = create_resource(
        || course_id.clone(),
        |course_id| async move {
            invoke::<_, Vec<Discussion>>("get_discussions", &serde_json::json!({
                "course_id": course_id
            })).await.ok()
        }
    );
    
    // State for new discussion form
    let (title, set_title) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    
    // Create discussion action
    let create_discussion = create_action(move |_| {
        let discussion_create = DiscussionCreate {
            course_id: course_id.clone(),
            title: title.get(),
            content: content.get(),
            topic_id: None,
            status: Some(DiscussionStatus::Open),
        };
        
        async move {
            match invoke::<_, Discussion>("create_discussion", &discussion_create).await {
                Ok(discussion) => {
                    // Clear form
                    set_title.set(String::new());
                    set_content.set(String::new());
                    
                    // Manually syncing with Discourse
                    if let Err(e) = invoke::<_, Discussion>("sync_discussion", &serde_json::json!({
                        "discussion_id": discussion.id
                    })).await {
                        log::error!("Failed to sync discussion: {}", e);
                    }
                    
                    // Refresh discussions list
                    discussions.refetch();
                    
                    Ok(discussion)
                },
                Err(e) => Err(e.to_string())
            }
        }
    });
    
    view! {
        <div>
            <h2>"Course Discussions"</h2>
            
            {/* New discussion form */}
            <div class="discussion-form">
                <h3>"Start a New Discussion"</h3>
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    create_discussion.dispatch(());
                }>
                    <div>
                        <label>"Title"</label>
                        <input
                            type="text"
                            on:input=move |ev| {
                                set_title.set(event_target_value(&ev));
                            }
                            value=title
                            required
                        />
                    </div>
                    
                    <div>
                        <label>"Message"</label>
                        <textarea
                            on:input=move |ev| {
                                set_content.set(event_target_value(&ev));
                            }
                            value=content
                            required
                        />
                    </div>
                    
                    <button type="submit">"Post Discussion"</button>
                </form>
                
                {move || create_discussion.value().map(|result| match result {
                    Ok(_) => view! { <div class="success">"Discussion created!"</div> },
                    Err(e) => view! { <div class="error">{format!("Error: {}", e)}</div> }
                })}
            </div>
            
            {/* Discussions list */}
            <Suspense fallback=move || view! { <p>"Loading discussions..."</p> }>
                {move || discussions.get().map(|maybe_discussions| {
                    match maybe_discussions {
                        Some(list) if !list.is_empty() => {
                            view! {
                                <ul class="discussions-list">
                                    {list.into_iter().map(|discussion| {
                                        let id = discussion.id.clone();
                                        view! {
                                            <li>
                                                <h3>{&discussion.title}</h3>
                                                <p>{&discussion.content}</p>
                                                <div class="discussion-meta">
                                                    <span>{discussion.created_at}</span>
                                                    <span class={"status " + &discussion.status.to_string().to_lowercase()}>
                                                        {format!("Status: {:?}", discussion.status)}
                                                    </span>
                                                </div>
                                                {move || discussion.topic_id.clone().map(|topic_id| {
                                                    view! {
                                                        <a 
                                                            href=format!("https://discourse.example.com/t/{}", topic_id)
                                                            target="_blank"
                                                        >
                                                            "View in Discourse"
                                                        </a>
                                                    }
                                                })}
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            }
                        },
                        _ => view! { <p>"No discussions found for this course."</p> }
                    }
                })}
            </Suspense>
        </div>
    }
}

.invoke_handler(tauri::generate_handler![
    // ...other commands
    api::discussions::get_discussions,
    api::discussions::get_discussion,
    api::discussions::create_discussion,
    api::discussions::update_discussion,
    api::discussions::sync_discussion,
    api::discussions::delete_discussion,
])