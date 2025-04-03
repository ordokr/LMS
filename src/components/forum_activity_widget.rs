use leptos::*;
use crate::models::forum::Topic;
use crate::services::forum_service::ForumService;

#[component]
pub fn ForumActivityWidget(
    cx: Scope,
    #[prop(optional)] limit: Option<usize>,
) -> impl IntoView {
    let forum_service = use_context::<ForumService>(cx)
        .expect("ForumService should be provided");
    
    let limit = limit.unwrap_or(5);
    
    let recent_topics = create_resource(
        cx,
        || (),
        move |_| async move {
            forum_service.get_recent_activity(limit).await
        }
    );

    view! { cx,
        <div class="dashboard-widget forum-activity-widget">
            <div class="widget-header">
                <h3>"Recent Forum Activity"</h3>
                <a href="/forum" class="view-all">
                    "View All"
                </a>
            </div>
            
            <div class="widget-content">
                {move || match recent_topics.read(cx) {
                    None => view! { cx, <p>"Loading forum activity..."</p> },
                    Some(Ok(topics)) => {
                        if topics.is_empty() {
                            view! { cx, <p class="no-activity">"No recent forum activity."</p> }
                        } else {
                            view! { cx,
                                <ul class="activity-list">
                                    {topics.into_iter().map(|topic| view! { cx,
                                        <li class="activity-item">
                                            <div class="activity-info">
                                                <a href={format!("/forum/t/{}", topic.id)} class="topic-link">
                                                    {topic.title}
                                                </a>
                                                <span class="topic-category">
                                                    "in "
                                                    <a href={format!("/forum/c/{}", topic.category_id)}>
                                                        {topic.category_name}
                                                    </a>
                                                </span>
                                            </div>
                                            <div class="activity-meta">
                                                <span class="activity-stats">
                                                    {topic.reply_count} " replies"
                                                </span>
                                                <span class="activity-time">
                                                    {format_date(topic.last_post_at.unwrap_or(topic.created_at))}
                                                </span>
                                            </div>
                                        </li>
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }
                        }
                    },
                    Some(Err(e)) => view! { cx, <p class="error">"Error: " {e}</p> }
                }}
            </div>
        </div>
    }
}

fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 0 {
        format!("{} days ago", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{} hours ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{} minutes ago", diff.num_minutes())
    } else {
        "just now".to_string()
    }
}