use leptos::*;
use crate::models::forum::{Post, Topic, CreatePostRequest};
use crate::services::forum_service::ForumService;
use crate::services::auth_service::AuthService;

#[component]
pub fn ThreadDetail(
    cx: Scope,
    #[prop()] topic_id: i64,
) -> impl IntoView {
    let forum_service = use_context::<ForumService>(cx)
        .expect("ForumService should be provided");
    
    let auth_service = use_context::<AuthService>(cx)
        .expect("AuthService should be provided");
    
    let topic = create_resource(
        cx,
        move || topic_id,
        move |id| async move {
            forum_service.get_topic(id).await
        }
    );
    
    let posts = create_resource(
        cx,
        move || topic_id,
        move |id| async move {
            forum_service.get_posts_by_topic(id).await
        }
    );
    
    let (new_post_content, set_new_post_content) = create_signal(cx, String::new());
    
    let create_post = create_action(cx, move |(topic_id, content): &(i64, String)| {
        let request = CreatePostRequest {
            topic_id: *topic_id,
            content: content.clone(),
            parent_id: None,
        };
        async move {
            let result = forum_service.create_post(request).await;
            if result.is_ok() {
                // Refetch posts after successful creation
                posts.refetch();
            }
            result
        }
    });
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let content = new_post_content();
        if !content.trim().is_empty() {
            create_post.dispatch((topic_id, content));
            set_new_post_content(String::new());
        }
    };

    view! { cx,
        <div class="thread-detail">
            // Topic header
            {move || match topic.read(cx) {
                None => view! { cx, <h1>"Loading topic..."</h1> },
                Some(Ok(t)) => view! { cx, 
                    <div class="topic-header">
                        <h1>{t.title}</h1>
                        <div class="topic-meta">
                            <span>
                                "in "
                                <a href={format!("/forum/c/{}", t.category_id)}>
                                    {t.category_name}
                                </a>
                            </span>
                            <span class="topic-stats">
                                {t.view_count} " views • " {t.reply_count} " replies"
                            </span>
                        </div>
                    </div>
                },
                Some(Err(e)) => view! { cx, <p class="error">"Error loading topic: " {e.to_string()}</p> }
            }}
            
            // Posts list
            <div class="posts-list">
                {move || match posts.read(cx) {
                    None => view! { cx, <p>"Loading posts..."</p> },
                    Some(Ok(post_list)) => {
                        view! { cx,
                            <div class="posts">
                                {post_list.into_iter().map(|post| view! { cx, 
                                    <div class="post" id={format!("post-{}", post.id)}>
                                        <div class="post-sidebar">
                                            <div class="post-author">
                                                <img 
                                                    src={format!("/api/users/{}/avatar", post.user_id)} 
                                                    alt="User avatar" 
                                                    class="avatar"
                                                />
                                                <div class="author-name">{post.author_name}</div>
                                                <div class="author-role">{post.author_role}</div>
                                            </div>
                                        </div>
                                        <div class="post-content">
                                            <div class="post-meta">
                                                <a href={format!("#post-{}", post.id)} class="post-date">
                                                    {format_date(post.created_at)}
                                                </a>
                                                {move || {
                                                    if post.is_solution {
                                                        view! { cx, <span class="solution-badge">"Solution"</span> }
                                                    } else {
                                                        view! { cx, <></> }
                                                    }
                                                }}
                                            </div>
                                            <div class="post-body" inner_html={post.content}></div>
                                            <div class="post-actions">
                                                <button class="like-button" on:click=move |_| {
                                                    forum_service.like_post(post.id);
                                                }>
                                                    <span class="like-icon">"♥"</span>
                                                    <span class="like-count">{post.like_count}</span>
                                                </button>
                                                <button class="quote-button">
                                                    "Quote"
                                                </button>
                                                <button class="reply-button">
                                                    "Reply"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    },
                    Some(Err(e)) => view! { cx, <p class="error">"Error loading posts: " {e.to_string()}</p> }
                }}
            </div>
            
            // Reply form
            {move || {
                if auth_service.is_logged_in() {
                    view! { cx,
                        <form class="reply-form" on:submit=handle_submit>
                            <h3>"Add Your Reply"</h3>
                            <textarea 
                                id="reply-content"
                                rows="5"
                                placeholder="Write your reply here..."
                                prop:value=new_post_content
                                on:input=move |ev| {
                                    set_new_post_content(event_target_value(&ev));
                                }
                            ></textarea>
                            <div class="form-actions">
                                <button 
                                    type="submit" 
                                    class="primary-button"
                                    disabled=move || new_post_content().trim().is_empty() || create_post.pending()
                                >
                                    {move || if create_post.pending() { "Posting..." } else { "Post Reply" }}
                                </button>
                            </div>
                            {move || {
                                if let Some(Err(e)) = create_post.value().get() {
                                    view! { cx, <div class="error">"Error: " {e.to_string()}</div> }
                                } else {
                                    view! { cx, <></> }
                                }
                            }}
                        </form>
                    }
                } else {
                    view! { cx,
                        <div class="login-prompt">
                            <p>"You need to be logged in to reply."</p>
                            <a href="/login" class="button">
                                "Log In"
                            </a>
                        </div>
                    }
                }
            }}
        </div>
    }
}

fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 30 {
        date.format("%b %d, %Y at %H:%M").to_string()
    } else if diff.num_days() > 0 {
        format!("{} days ago", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{} hours ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{} minutes ago", diff.num_minutes())
    } else {
        "just now".to_string()
    }
}

// filepath: c:\Users\Tim\Desktop\LMS\src\utils\offline.rs
use web_sys::window;

pub fn is_online() -> bool {
    let window = match window() {
        Some(win) => win,
        None => return false,
    };
    
    window.navigator().on_line()
}