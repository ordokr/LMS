use leptos::*;
use crate::models::forum::post::PostSummary;
use crate::components::error_alert::ErrorAlert;
use crate::utils::date_utils::format_date_for_display;

#[component]
pub fn UserPostsList(
    user_id: String,
) -> impl IntoView {
    // State
    let (posts, set_posts) = create_signal(Vec::<PostSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    let per_page = 10;
    
    // Load posts
    let load_posts = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, Vec<PostSummary>>(
                "get_user_posts", 
                &(user_id.clone(), Some(current_page.get()), Some(per_page))
            ).await {
                Ok(fetched_posts) => {
                    set_posts.set(fetched_posts);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load posts: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load posts on mount and when page changes
    create_effect(move |_| {
        load_posts();
    });
    
    // Handle pagination
    let next_page = move |_| {
        set_current_page.update(|p| *p += 1);
        load_posts();
    };
    
    let prev_page = move |_| {
        if current_page.get() > 1 {
            set_current_page.update(|p| *p -= 1);
            load_posts();
        }
    };

    view! {
        <div class="user-posts">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            <h2 class="section-title">"Replies"</h2>
            
            {move || {
                if loading.get() && posts.get().is_empty() {
                    view! { <div class="loading-state">"Loading replies..."</div> }
                } else if posts.get().is_empty() {
                    view! { <div class="empty-state">"No replies found"</div> }
                } else {
                    view! {
                        <div class="posts-container">
                            <ul class="posts-list-full">
                                {posts.get().into_iter().map(|post| {
                                    view! {
                                        <li class="post-item-full">
                                            <div class="post-header">
                                                <a href={format!("/topics/{}", post.topic_id)} class="post-topic-title">
                                                    {post.topic_title}
                                                </a>
                                                <span class="post-date">
                                                    {format_date_for_display(Some(&post.created_at))}
                                                </span>
                                            </div>
                                            <div class="post-content">
                                                {post.raw}
                                            </div>
                                            <div class="post-footer">
                                                <a href={format!("/topics/{}?post={}", post.topic_id, post.post_number)} class="view-in-context">
                                                    "View in context"
                                                </a>
                                                <span class="post-likes">
                                                    {"♥ "}{post.like_count}
                                                </span>
                                            </div>
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                            
                            <div class="pagination-controls">
                                <button 
                                    class="pagination-button" 
                                    on:click=prev_page 
                                    disabled=move || current_page.get() <= 1
                                >
                                    "Previous"
                                </button>
                                <span class="page-indicator">{"Page "}{current_page}</span>
                                <button 
                                    class="pagination-button" 
                                    on:click=next_page 
                                    disabled=move || posts.get().len() < per_page
                                >
                                    "Next"
                                </button>
                            </div>
                        </div>
                    }
                }
            }}
        </div>
    }
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: serde::Serialize + ?Sized,
    R: for<'de> serde::de::DeserializeOwned,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}