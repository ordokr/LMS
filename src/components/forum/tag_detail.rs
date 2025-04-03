use leptos::*;
use leptos_router::use_params_map;
use crate::models::forum::tag::{Tag, TagWithTopics};
use crate::services::forum::ForumService;

#[component]
pub fn TagDetail() -> impl IntoView {
    // Get tag slug from route params
    let params = use_params_map();
    let tag_slug = move || params.with(|p| p.get("slug").cloned().unwrap_or_default());
    
    // State signals
    let (tag, set_tag) = create_signal(None::<Tag>);
    let (topics, set_topics) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    let (total_pages, set_total_pages) = create_signal(1);
    let (total_topics, set_total_topics) = create_signal(0);
    
    // Load tag and its topics
    create_effect(move |_| {
        let slug = tag_slug();
        
        set_loading.set(true);
        
        spawn_local(async move {
            // Get tag details
            match ForumService::get_tag_by_slug(&slug).await {
                Ok(tag_data) => {
                    set_tag.set(Some(tag_data));
                    
                    // Get topics for this tag
                    match ForumService::get_topics_by_tag(&slug, current_page(), 20).await {
                        Ok(paginated) => {
                            set_topics.set(paginated.topics);
                            set_total_pages.set(paginated.total_pages);
                            set_current_page.set(paginated.page);
                            set_total_topics.set(paginated.total);
                            set_loading.set(false);
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to load topics: {}", e)));
                            set_loading.set(false);
                        }
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load tag: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Navigate to page
    let go_to_page = move |page: usize| {
        if page != current_page() && page > 0 && page <= total_pages() {
            set_current_page.set(page);
        }
    };
    
    view! {
        <div class="tag-detail">
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else if let Some(err) = error() {
                view! { <div class="alert alert-danger">{err}</div> }
            } else if let Some(t) = tag() {
                view! {
                    <div>
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <div>
                                <h1 class="mb-1 d-flex align-items-center">
                                    <span class="tag-indicator me-2" style={format!("background-color: {}", t.color.unwrap_or_else(|| "#0d6efd".to_string()))}></span>
                                    {format!("Topics tagged with \"{}\"", t.name)}
                                </h1>
                                {t.description.map(|desc| {
                                    view! { <p class="text-muted">{desc}</p> }
                                })}
                            </div>
                            
                            <div>
                                <a href="/forum/tags" class="btn btn-outline-secondary">
                                    "All Tags"
                                </a>
                            </div>
                        </div>
                        
                        <div class="mb-3">
                            <span class="badge bg-secondary">
                                {format!("{} topics", total_topics())}
                            </span>
                        </div>
                        
                        {if topics().is_empty() {
                            view! {
                                <div class="text-center p-5">
                                    <i class="bi bi-collection mb-3 d-block" style="font-size: 3rem;"></i>
                                    <h3>"No topics found"</h3>
                                    <p class="text-muted">"There are no topics with this tag yet."</p>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="list-group mb-4">
                                    {topics().into_iter().map(|topic| {
                                        view! {
                                            <a href={format!("/forum/topics/{}", topic.id)} class="list-group-item list-group-item-action">
                                                <div class="d-flex justify-content-between align-items-center">
                                                    <h5 class="mb-1">{topic.title}</h5>
                                                    <small>{format_date(topic.created_at)}</small>
                                                </div>
                                                {topic.excerpt.map(|excerpt| {
                                                    view! { <p class="mb-1">{excerpt}</p> }
                                                })}
                                                <div class="d-flex justify-content-between align-items-center">
                                                    <small class="text-muted">
                                                        "by " 
                                                        <a href={format!("/users/{}", topic.author_id)}>
                                                            {topic.author_name.unwrap_or_else(|| "Unknown".to_string())}
                                                        </a>
                                                        " in "
                                                        <a href={format!("/forum/categories/{}", topic.category_id)}>
                                                            {topic.category_name.unwrap_or_else(|| "Unknown".to_string())}
                                                        </a>
                                                    </small>
                                                    <div>
                                                        <span class="badge bg-primary rounded-pill me-2">
                                                            {format!("{} replies", topic.reply_count.unwrap_or(0))}
                                                        </span>
                                                        <span class="badge bg-secondary rounded-pill">
                                                            {format!("{} views", topic.view_count.unwrap_or(0))}
                                                        </span>
                                                    </div>
                                                </div>
                                                {topic.tags.map(|tags| {
                                                    if !tags.is_empty() {
                                                        view! {
                                                            <div class="mt-2">
                                                                {tags.iter().filter(|t| t != &&t.name).map(|tag| {
                                                                    view! {
                                                                        <a href={format!("/forum/tags/{}", tag)} 
                                                                           class="badge rounded-pill bg-light text-dark me-1">
                                                                            {tag}
                                                                        </a>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </div>
                                                        }
                                                    } else {
                                                        view! {}
                                                    }
                                                })}
                                            </a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                                
                                {if total_pages() > 1 {
                                    view! {
                                        <nav aria-label="Topics pagination">
                                            <ul class="pagination justify-content-center">
                                                <li class=format!("page-item {}", if current_page() <= 1 { "disabled" } else { "" })>
                                                    <button class="page-link" on:click=move |_| go_to_page(current_page() - 1)>
                                                        "Previous"
                                                    </button>
                                                </li>
                                                
                                                {(1..=total_pages()).map(|page| {
                                                    view! {
                                                        <li class=format!("page-item {}", if page == current_page() { "active" } else { "" })>
                                                            <button class="page-link" on:click=move |_| go_to_page(page)>
                                                                {page}
                                                            </button>
                                                        </li>
                                                    }
                                                }).collect::<Vec<_>>()}
                                                
                                                <li class=format!("page-item {}", if current_page() >= total_pages() { "disabled" } else { "" })>
                                                    <button class="page-link" on:click=move |_| go_to_page(current_page() + 1)>
                                                        "Next"
                                                    </button>
                                                </li>
                                            </ul>
                                        </nav>
                                    }
                                } else {
                                    view! {}
                                }}
                            }
                        }}
                    </div>
                }
            } else {
                view! {
                    <div class="alert alert-warning">
                        "Tag not found."
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to format date
fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%b %d, %Y").to_string()
}