use leptos::*;
use crate::models::forum::{Category, CategoryStats};
use crate::services::forum::ForumService;

#[component]
pub fn CategoriesList() -> impl IntoView {
    let (categories, set_categories) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Load categories on component mount
    create_effect(move |_| {
        spawn_local(async move {
            match ForumService::get_categories().await {
                Ok(cats) => {
                    set_categories.set(cats);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load categories: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    view! {
        <div class="forum-categories">
            <div class="d-flex justify-content-between align-items-center mb-4">
                <h1 class="mb-0">"Forum Categories"</h1>
                <a href="/forum/categories/new" class="btn btn-primary">"New Category"</a>
            </div>
            
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else if let Some(err) = error() {
                view! { <div class="alert alert-danger">{err}</div> }
            } else if categories().is_empty() {
                view! { <div class="alert alert-info">"No categories found."</div> }
            } else {
                view! {
                    <div class="table-responsive">
                        <table class="table table-hover category-list">
                            <thead>
                                <tr>
                                    <th>"Category"</th>
                                    <th class="text-center">"Topics"</th>
                                    <th class="text-center">"Posts"</th>
                                    <th>"Latest"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {categories().into_iter().map(|category| {
                                    let stats = category.stats.as_ref().unwrap_or(&CategoryStats::default());
                                    
                                    view! {
                                        <tr class="category-row">
                                            <td class="category-name">
                                                <div class="d-flex align-items-center">
                                                    <div class="category-color me-3" 
                                                         style={format!("background-color: {}", category.color.clone().unwrap_or_else(|| "#0088cc".to_string()))}>
                                                    </div>
                                                    <div>
                                                        <a href={format!("/forum/categories/{}", category.id)} class="category-title">
                                                            {category.name}
                                                        </a>
                                                        {category.description.as_ref().map(|desc| view! {
                                                            <div class="category-description small text-muted">{desc}</div>
                                                        })}
                                                        {category.subcategories.map(|subs| {
                                                            if !subs.is_empty() {
                                                                view! {
                                                                    <div class="subcategories small mt-1">
                                                                        <span class="text-muted">"Subcategories: "</span>
                                                                        {subs.into_iter().map(|sub| view! {
                                                                            <a href={format!("/forum/categories/{}", sub.id)} class="badge rounded-pill bg-light text-dark me-1">
                                                                                {sub.name}
                                                                            </a>
                                                                        }).collect::<Vec<_>>()}
                                                                    </div>
                                                                }
                                                            } else {
                                                                view! {}
                                                            }
                                                        })}
                                                    </div>
                                                </div>
                                            </td>
                                            <td class="text-center">{stats.topic_count}</td>
                                            <td class="text-center">{stats.post_count}</td>
                                            <td class="latest">
                                                {stats.latest_topic.as_ref().map(|topic| {
                                                    view! {
                                                        <div class="latest-topic">
                                                            <a href={format!("/forum/topics/{}", topic.id)} class="topic-title">
                                                                {topic.title.clone()}
                                                            </a>
                                                            <div class="small text-muted">
                                                                {format_relative_date(topic.created_at)}
                                                                " by "
                                                                <a href={format!("/users/{}", topic.author_id)}>
                                                                    {topic.author_name.clone().unwrap_or_else(|| "Unknown".to_string())}
                                                                </a>
                                                            </div>
                                                        </div>
                                                    }
                                                })}
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to format relative dates as seen in Discourse
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
