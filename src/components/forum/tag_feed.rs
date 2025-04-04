use leptos::*;
use crate::models::forum::{TopicSummary, Post};
use crate::services::forum::ForumService;
use crate::utils::formatting::format_relative_time;

#[component]
pub fn TagFeed() -> impl IntoView {
    // State signals
    let (topics, set_topics) = create_signal(Vec::<TopicSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (feed_type, set_feed_type) = create_signal("followed".to_string()); // "followed", "all", "trending"
    
    // Load topics based on feed type
    create_effect(move |_| {
        set_loading.set(true);
        let feed = feed_type();
        
        spawn_local(async move {
            let result = match feed.as_str() {
                "followed" => ForumService::get_followed_tag_topics().await,
                "trending" => ForumService::get_trending_topics().await,
                _ => ForumService::get_latest_topics().await,
            };
            
            match result {
                Ok(loaded_topics) => {
                    set_topics.set(loaded_topics);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load topics: {}", e)));
                }
            }
            
            set_loading.set(false);
        });
    });

    view! {
        <div class="tag-feed">
            <div class="card">
                <div class="card-header">
                    <ul class="nav nav-tabs card-header-tabs">
                        <li class="nav-item">
                            <a 
                                class="nav-link" 
                                class:active=move || feed_type() == "followed"
                                href="javascript:void(0)"
                                on:click=move |_| set_feed_type.set("followed".to_string())
                            >
                                <i class="bi bi-bookmark me-1"></i>
                                "My Tags"
                            </a>
                        </li>
                        <li class="nav-item">
                            <a 
                                class="nav-link" 
                                class:active=move || feed_type() == "trending"
                                href="javascript:void(0)"
                                on:click=move |_| set_feed_type.set("trending".to_string())
                            >
                                <i class="bi bi-graph-up me-1"></i>
                                "Trending"
                            </a>
                        </li>
                        <li class="nav-item">
                            <a 
                                class="nav-link" 
                                class:active=move || feed_type() == "all"
                                href="javascript:void(0)"
                                on:click=move |_| set_feed_type.set("all".to_string())
                            >
                                <i class="bi bi-clock-history me-1"></i>
                                "Latest"
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="card-body">
                    {move || if let Some(err) = error() {
                        view! { 
                            <div class="alert alert-danger">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                {err}
                            </div>
                        }
                    } else {
                        view! {}
                    }}
                    
                    <h5 class="card-title mb-4">
                        {move || match feed_type().as_str() {
                            "followed" => "Topics from Tags You Follow",
                            "trending" => "Trending Topics",
                            _ => "Latest Topics"
                        }}
                    </h5>
                    
                    {move || if loading() {
                        view! { <div class="d-flex justify-content-center p-4"><div class="spinner-border" role="status"></div></div> }
                    } else if topics().is_empty() {
                        view! {
                            <div class="alert alert-info">
                                {match feed_type().as_str() {
                                    "followed" => "You don't have any content from followed tags yet. Try following more tags or check back later.",
                                    "trending" => "No trending topics available right now.",
                                    _ => "No topics available."
                                }}
                            </div>
                        }
                    } else {
                        view! {
                            <div class="list-group">
                                {topics().into_iter().map(|topic| {
                                    view! {
                                        <a 
                                            href={format!("/forum/topics/{}", topic.id)} 
                                            class="list-group-item list-group-item-action"
                                        >
                                            <div class="d-flex justify-content-between align-items-center">
                                                <h6 class="mb-1">{&topic.title}</h6>
                                                <small class="text-muted">
                                                    {format!("{} replies", topic.reply_count)}
                                                </small>
                                            </div>
                                            
                                            <div class="mb-1">
                                                {topic.tags.as_ref().map_or_else(
                                                    || view! {},
                                                    |tags| view! {
                                                        <>
                                                            {tags.iter().map(|tag| {
                                                                view! {
                                                                    <span 
                                                                        class="badge me-1"
                                                                        style={format!(
                                                                            "background-color: {};", 
                                                                            tag.color.clone().unwrap_or_else(|| "#6c757d".to_string())
                                                                        )}
                                                                    >
                                                                        {&tag.name}
                                                                    </span>
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </>
                                                    }
                                                )}
                                            </div>
                                            
                                            <div class="d-flex justify-content-between align-items-center">
                                                <small>
                                                    <span class="text-muted">
                                                        <i class="bi bi-person me-1"></i>
                                                        {&topic.author.username}
                                                    </span>
                                                </small>
                                                <small class="text-muted">
                                                    {format_relative_time(topic.last_activity_at.unwrap_or(topic.created_at))}
                                                </small>
                                            </div>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }}
                    
                    <div class="mt-3 text-center">
                        <a href="/forum" class="btn btn-outline-primary btn-sm">
                            "View All Topics"
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}