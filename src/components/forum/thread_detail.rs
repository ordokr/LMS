use leptos::*;
use crate::models::forum::{Topic, Post};
use crate::services::forum::ForumService;
use web_sys::SubmitEvent;
use markdown::{to_html, Options};
// Add this import for RichEditor
use crate::components::forum::rich_editor::RichEditor;

#[component]
pub fn ThreadDetail(
    #[prop()] topic_id: i64,
) -> impl IntoView {
    let (topic, set_topic) = create_signal(None::<Topic>);
    let (posts, set_posts) = create_signal(Vec::<Post>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (reply_content, set_reply_content) = create_signal(String::new());
    let (submitting, set_submitting) = create_signal(false);
    
    // Near the top of your component
    let auth_state = use_context::<AuthState>().expect("AuthState not found");
    let is_authenticated = move || auth_state.is_authenticated();
    
    // Load topic and posts
    create_effect(move |_| {
        set_loading.set(true);
        let id = topic_id;
        
        spawn_local(async move {
            match ForumService::get_topic(id).await {
                Ok(t) => {
                    set_topic.set(Some(t));
                    
                    // Also load posts
                    match ForumService::get_topic_posts(id).await {
                        Ok(p) => {
                            set_posts.set(p);
                            set_loading.set(false);
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to load posts: {}", e)));
                            set_loading.set(false);
                        }
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load topic: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Handle reply submission
    let submit_reply = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        if reply_content().trim().is_empty() {
            return;
        }
        
        let content = reply_content();
        set_submitting.set(true);
        
        spawn_local(async move {
            match ForumService::create_post(topic_id, &content).await {
                Ok(post) => {
                    // Add post to list
                    set_posts.update(|posts| {
                        posts.push(post);
                    });
                    set_reply_content.set(String::new());
                    set_submitting.set(false);

                    // Extract mentions from content
                    let mentions = extract_mentions(&content);
                    let quote_ids = extract_quotes(&content);

                    // If there are mentions or quotes, create notifications
                    if !mentions.is_empty() || !quote_ids.is_empty() {
                        spawn_local(async move {
                            for username in mentions {
                                // Send mention notification via API
                                let _ = ForumService::create_mention_notification(
                                    topic_id,
                                    post.id, // From the new post
                                    &username
                                ).await;
                            }
                            
                            for quoted_post_id in quote_ids {
                                // Send quote notification via API
                                let _ = ForumService::create_quote_notification(
                                    topic_id,
                                    post.id, // From the new post
                                    quoted_post_id
                                ).await;
                            }
                        });
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to post reply: {}", e)));
                    set_submitting.set(false);
                }
            }
        });
    };
    
    view! {
        <div class="topic-detail">
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border#" role="status"></div></div> }
            } else if let Some(t) = topic() {
                view! {
                    <div>
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <h1>{t.title}</h1>
                            <div class="d-flex gap-2">
                                <a href={format!("/forum/categories/{}", t.category_id)} 
                                   class="btn btn-sm btn-outline-secondary">
                                    "Back to Category"
                                </a>
                                {move || {
                                    if is_author() || is_moderator() {
                                        view! {
                                            <a href={format!("/forum/topics/{}/edit", topic_id)} 
                                               class="btn btn-sm btn-outline-primary">
                                                "Edit Topic"
                                            </a>
                                        }
                                    } else {
                                        view! {}
                                    }
                                }}
                            </div>
                        </div>
                        
                        <div class="d-flex justify-content-between mb-3">
                            <div>
                                {if t.pinned {
                                    view! { <span class="badge bg-info me-2">"Pinned"</span> }
                                } else {
                                    view! {}
                                }}
                                {if t.locked {
                                    view! { <span class="badge bg-secondary me-2">"Locked"</span> }
                                } else {
                                    view! {}
                                }}
                                <span class="text-muted">
                                    "Started by " {t.author_name.clone().unwrap_or_else(|| "Anonymous".to_string())} 
                                    " · " {format_date(t.created_at)}
                                </span>
                            </div>
                            <span class="badge bg-primary">
                                {t.reply_count.unwrap_or(0)} " replies"
                            </span>
                        </div>

                        {if let Some(tags) = &t.tags {
                            if !tags.is_empty() {
                                view! {
                                    <div class="topic-tags mt-2">
                                        {tags.iter().map(|tag| {
                                            let tag_object = t.tag_objects.as_ref()
                                                .and_then(|objects| objects.iter().find(|t| t.name == *tag))
                                                .cloned();
                                            
                                            let tag_color = tag_object
                                                .and_then(|t| t.color)
                                                .unwrap_or_else(|| "#0d6efd".to_string());
                                            
                                            view! {
                                                <a href={format!("/forum/tags/{}", tag.to_lowercase().replace(" ", "-"))}
                                                class="badge rounded-pill me-1"
                                                style={format!("background-color: {}; color: white;", tag_color)}>
                                                    {tag}
                                                </a>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }
                            } else {
                                view! {}
                            }
                        } else {
                            view! {}
                        }}
                        
                        <div class="posts-list mb-4">
                            {move || {
                                posts().into_iter().enumerate().map(|(index, post)| {
                                    let is_first = index == 0;
                                    view! {
                                        <div class="card mb-3" id={format!("post-{}", post.id)}>
                                            <div class="card-header d-flex justify-content-between align-items-center#">
                                                <div>
                                                    <strong>{post.author_name.clone().unwrap_or_else(|| "Anonymous".to_string())}</strong>
                                                    {if let Some(role) = post.author_role.clone() {
                                                        view! { <span class="badge bg-secondary ms-2">{role}</span> }
                                                    } else {
                                                        view! {}
                                                    }}
                                                    {if is_first {
                                                        view! { <span class="badge bg-info ms-2">"Original Post"</span> }
                                                    } else {
                                                        view! {}
                                                    }}
                                                </div>
                                                <small>{format_date(post.created_at)}</small>
                                            </div>
                                            <div class="card-body">
                                                <div class="post-content" inner_html={to_html_with_options(&post.content, &Options::gfm())}></div>
                                            </div>
                                            <div class="card-footer d-flex justify-content-between">
                                                <div>
                                                    <button class="btn btn-sm btn-outline-primary me-2">
                                                        <i class="bi bi-hand-thumbs-up"></i>
                                                        " Like" {if let Some(count) = post.like_count {
                                                            if count > 0 {
                                                                format!(" ({})", count)
                                                            } else {
                                                                "".to_string()
                                                            }
                                                        } else {
                                                            "".to_string()
                                                        }}
                                                    </button>
                                                    <button class="btn btn-sm btn-outline-secondary">
                                                        <i class="bi bi-reply"></i>
                                                        " Quote"
                                                    </button>
                                                </div>
                                                {if post.is_solution.unwrap_or(false) {
                                                    view! { <span class="badge bg-success">"Solution"</span> }
                                                } else {
                                                    view! {}
                                                }}
                                            </div>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                        
                        {move || {
                            if !t.locked && is_authenticated() {
                                view! {
                                    <div class="reply-form card mb-4">
                                        <div class="card-header#">
                                            <h4>"Post Reply"</h4>
                                        </div>
                                        <div class="card-body">
                                            <form on:submit=submit_reply>
                                                {move || error().map(|err| view! {
                                                    <div class="alert alert-danger mb-3">{err}</div>
                                                })}
                                                
                                                <div class="mb-3">
                                                    <RichEditor
                                                        content=reply_content
                                                        set_content=set_reply_content
                                                        placeholder=Some("Write your reply here...")
                                                        rows=Some(5)
                                                    />
                                                </div>
                                                <button 
                                                    type="submit" 
                                                    class="btn btn-primary" 
                                                    disabled=move || submitting() || reply_content().trim().is_empty()
                                                >
                                                    {move || if submitting() { "Submitting..." } else { "Post Reply" }}
                                                </button>
                                            </form>
                                        </div>
                                    </div>
                                }
                            } else if !t.locked {
                                view! {
                                    <div class="alert alert-info">
                                        "Please " <a href="/login">"login"</a> " to reply to this topic."
                                    </div>
                                }
                            } else {
                                view! {
                                    <div class="alert alert-warning">
                                        "This topic is locked and cannot receive new replies."
                                    </div>
                                }
                            }
                        }}
                    </div>
                }
            } else {
                view! {
                    <div class="alert alert-danger#">
                        {move || error().unwrap_or_else(|| "Topic not found".to_string())}
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to format dates
fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    // Format the date to be more human-readable
    date.format("%b %d, %Y %H:%M").to_string()
}

// Helper functions to extract mentions and quotes from post content
fn extract_mentions(content: &str) -> Vec<String> {
    let mut mentions = Vec::new();
    let re = regex::Regex::new(r#"@(\w+)"#).unwrap();
    
    for cap in re.captures_iter(content) {
        if let Some(username) = cap.get(1) {
            mentions.push(username.as_str().to_string());
        }
    }
    
    mentions
}

fn extract_quotes(content: &str) -> Vec<i64> {
    let mut quote_ids = Vec::new();
    // This is a simplified example - in a real implementation, 
    // you'd need to parse your specific quote format
    let re = regex::Regex::new(r#"data-post-id=['"](\d+)['"]"#).unwrap();
    
    for cap in re.captures_iter(content) {
        if let Some(id_str) = cap.get(1) {
            if let Ok(id) = id_str.as_str().parse::<i64>() {
                quote_ids.push(id);
            }
        }
    }
    
    quote_ids
}


