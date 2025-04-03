use leptos::*;
use crate::models::forum::SearchResult;
use crate::services::forum::ForumService;
use web_sys::SubmitEvent;

#[component]
pub fn ForumSearch() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (results, set_results) = create_signal(Vec::<SearchResult>::new());
    let (loading, set_loading) = create_signal(false);
    let (searched, set_searched) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    
    // Perform search when form is submitted
    let perform_search = move |ev: SubmitEvent| {
        ev.prevent_default();
        let search_query = query().trim().to_string();
        
        if search_query.is_empty() {
            return;
        }
        
        set_loading.set(true);
        set_searched.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match ForumService::search(&search_query).await {
                Ok(search_results) => {
                    set_results.set(search_results);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Search failed: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="forum-search">
            <div class="search-header mb-4">
                <h1>"Search the Forum"</h1>
                <p class="text-muted">"Find topics, posts, and users across all categories"</p>
            </div>
            
            <div class="search-form mb-4">
                <form on:submit=perform_search>
                    <div class="input-group">
                        <input 
                            type="text" 
                            class="form-control form-control-lg" 
                            placeholder="Search for topics, posts, or users..." 
                            prop:value=move || query()
                            on:input=move |ev| set_query.set(event_target_value(&ev))
                            aria-label="Search"
                        />
                        <button class="btn btn-primary" type="submit" disabled=move || loading()>
                            {move || if loading() {
                                view! { <span class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span> "Searching..." }
                            } else {
                                view! { <i class="bi bi-search me-2"></i> "Search" }
                            }}
                        </button>
                    </div>
                    
                    <div class="search-tips small text-muted mt-2">
                        <strong>"Search Tips: "</strong>
                        "Use quotes for exact phrases, + to require words, - to exclude words."
                    </div>
                </form>
            </div>
            
            {move || {
                if let Some(err) = error() {
                    view! { <div class="alert alert-danger">{err}</div> }
                } else if loading() {
                    view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                } else if searched() {
                    if results().is_empty() {
                        view! {
                            <div class="search-no-results text-center p-5">
                                <div class="mb-3">
                                    <i class="bi bi-search" style="font-size: 3rem;"></i>
                                </div>
                                <h3>"No results found"</h3>
                                <p class="text-muted">"Try different keywords or check your spelling"</p>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="search-results">
                                <h2 class="mb-3">{format!("{} Results", results().len())}</h2>
                                
                                <div class="list-group">
                                    {results().into_iter().map(|result| {
                                        match result {
                                            SearchResult::Topic(topic) => {
                                                view! {
                                                    <a href={format!("/forum/topics/{}", topic.id)} class="list-group-item list-group-item-action">
                                                        <div class="d-flex w-100 justify-content-between">
                                                            <h5 class="mb-1">
                                                                <i class="bi bi-chat-square-text me-2 text-primary"></i>
                                                                {topic.title}
                                                            </h5>
                                                            <small>{format_relative_date(topic.created_at)}</small>
                                                        </div>
                                                        <p class="mb-1 search-excerpt">{topic.excerpt.unwrap_or_default()}</p>
                                                        <div class="d-flex justify-content-between align-items-center">
                                                            <small class="text-muted">
                                                                "In " <a href={format!("/forum/categories/{}", topic.category_id)}>{topic.category_name.unwrap_or_else(|| "Unknown".to_string())}</a>
                                                                " by " <a href={format!("/users/{}", topic.author_id)}>{topic.author_name.unwrap_or_else(|| "Unknown".to_string())}</a>
                                                            </small>
                                                            <span class="badge bg-primary rounded-pill">{topic.reply_count.unwrap_or(0)} " replies"</span>
                                                        </div>
                                                    </a>
                                                }
                                            },
                                            SearchResult::Post(post) => {
                                                view! {
                                                    <a href={format!("/forum/topics/{}#post-{}", post.topic_id, post.id)} class="list-group-item list-group-item-action">
                                                        <div class="d-flex w-100 justify-content-between">
                                                            <h5 class="mb-1">
                                                                <i class="bi bi-reply me-2 text-secondary"></i>
                                                                {post.topic_title.clone().unwrap_or_else(|| "Reply".to_string())}
                                                            </h5>
                                                            <small>{format_relative_date(post.created_at)}</small>
                                                        </div>
                                                        <p class="mb-1 search-excerpt">
                                                            {post.excerpt.unwrap_or_else(|| truncate(&post.content, 200))}
                                                        </p>
                                                        <small class="text-muted">
                                                            "Reply by " <a href={format!("/users/{}", post.author_id)}>{post.author_name.unwrap_or_else(|| "Unknown".to_string())}</a>
                                                            " in " <a href={format!("/forum/topics/{}", post.topic_id)}>"topic"</a>
                                                        </small>
                                                    </a>
                                                }
                                            },
                                            SearchResult::User(user) => {
                                                view! {
                                                    <a href={format!("/users/{}", user.id)} class="list-group-item list-group-item-action">
                                                        <div class="d-flex w-100 justify-content-between">
                                                            <h5 class="mb-1">
                                                                <i class="bi bi-person me-2 text-info"></i>
                                                                {user.name}
                                                            </h5>
                                                            <small>{"Member since "}{format_date(user.created_at)}</small>
                                                        </div>
                                                        <p class="mb-1">{user.bio.unwrap_or_default()}</p>
                                                        <small class="text-muted">
                                                            {format!("{} topics", user.topic_count.unwrap_or(0))} 
                                                            {" · "}
                                                            {format!("{} posts", user.post_count.unwrap_or(0))}
                                                        </small>
                                                    </a>
                                                }
                                            }
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }
                    }
                } else {
                    view! {}
                }
            }}
        </div>
    }
}

// Helper function to format relative dates
fn format_relative_date(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 365 {
        format!("{} years ago", diff.num_days() / 365)
    } else if diff.num_days() > 30 {
        format!("{} months ago", diff.num_days() / 30)
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

// Helper function to format dates
fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%b %d, %Y").to_string()
}

// Helper function to truncate text
fn truncate(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let mut truncated = text.chars().take(max_length).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}