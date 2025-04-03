use leptos::*;
use crate::models::forum::{Category, Topic};
use crate::services::forum::ForumService;

#[component]
pub fn CategoryDetail(
    #[prop()] category_id: i64,  // Change to i64 to match your model
) -> impl IntoView {
    let (category, set_category) = create_signal(None::<Category>);
    let (topics, set_topics) = create_signal(Vec::<Topic>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Load category details
    create_effect(move |_| {
        set_loading.set(true);
        let id = category_id;
        
        // Get category data
        spawn_local(async move {
            match ForumService::get_category(id).await {
                Ok(cat) => {
                    set_category.set(Some(cat));
                    
                    // Also load topics for this category
                    match ForumService::get_topics(Some(id)).await {
                        Ok(cat_topics) => {
                            set_topics.set(cat_topics);
                            set_loading.set(false);
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to load topics: {}", e)));
                            set_loading.set(false);
                        }
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load category: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    view! {
        <div class="category-detail">
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-4"><div class="spinner-border" role="status"></div></div> }
            } else if let Some(cat) = category() {
                // Create a styled header based on category colors if available
                let header_style = if cat.color.is_some() && cat.text_color.is_some() {
                    format!(
                        "background-color: {}; color: {}", 
                        cat.color.clone().unwrap_or_else(|| "#f8f9fa".to_string()),
                        cat.text_color.clone().unwrap_or_else(|| "#212529".to_string())
                    )
                } else {
                    "".to_string()
                };
                
                view! {
                    <div>
                        <div class="card mb-4">
                            <div class="card-header" style={header_style}>
                                <div class="d-flex justify-content-between align-items-center">
                                    <h1 class="mb-0">{cat.name}</h1>
                                    <div class="btn-group">
                                        <a href={format!("/forum/categories/{}/edit", category_id)} 
                                           class="btn btn-sm btn-outline-secondary">
                                            "Edit Category"
                                        </a>
                                        <a href={format!("/forum/categories/{}/topics/new", category_id)} 
                                           class="btn btn-sm btn-primary">
                                            "New Topic"
                                        </a>
                                    </div>
                                </div>
                            </div>
                            <div class="card-body">
                                <p class="lead mb-3">{cat.description.unwrap_or_default()}</p>
                                <div class="d-flex gap-3 text-muted">
                                    <div><i class="bi bi-chat"></i> {cat.topic_count} " topics"</div>
                                    <div><i class="bi bi-reply"></i> {cat.post_count} " posts"</div>
                                </div>
                            </div>
                        </div>
                        
                        <h2 class="mb-3">"Topics"</h2>
                        
                        <div class="list-group">
                            {move || {
                                let topic_list = topics();
                                if topic_list.is_empty() {
                                    view! {
                                        <div class="text-center p-4">
                                            "No topics found in this category. Be the first to create one!"
                                        </div>
                                    }
                                } else {
                                    topic_list.into_iter().map(|topic| {
                                        view! {
                                            <a href={format!("/forum/topics/{}", topic.id)} 
                                               class="list-group-item list-group-item-action">
                                                <div class="d-flex w-100 justify-content-between">
                                                    <h5 class="mb-1">
                                                        {topic.title}
                                                        {if topic.pinned {
                                                            view! { <span class="badge bg-info ms-2">"Pinned"</span> }
                                                        } else {
                                                            view! {}
                                                        }}
                                                        {if topic.locked {
                                                            view! { <span class="badge bg-secondary ms-2">"Locked"</span> }
                                                        } else {
                                                            view! {}
                                                        }}
                                                    </h5>
                                                    <small>
                                                        {format!("{} replies", topic.reply_count)}
                                                    </small>
                                                </div>
                                                <div class="d-flex w-100 justify-content-between">
                                                    <p class="mb-1">{topic.excerpt.clone().unwrap_or_default()}</p>
                                                    <small>{format!("By {}", topic.author_name)}</small>
                                                </div>
                                                <small>{format_date(topic.updated_at)}</small>
                                            </a>
                                        }
                                    }).collect_view()
                                }
                            }}
                        </div>
                    </div>
                }
            } else {
                view! {
                    <div class="alert alert-danger">
                        {move || error().unwrap_or_else(|| "Category not found".to_string())}
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