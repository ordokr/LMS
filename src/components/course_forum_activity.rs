use leptos::*;
use leptos_router::*;
use crate::models::forum::Topic;
use crate::services::integration_service::IntegrationService;
use crate::components::loading::Loading;
use crate::components::error_message::ErrorMessage;

#[component]
pub fn CourseForumActivity(
    cx: Scope,
    #[prop()] course_id: i64,
    #[prop(optional)] limit: Option<usize>,
) -> impl IntoView {
    let integration_service = use_context::<IntegrationService>(cx)
        .expect("IntegrationService should be provided");
    
    let limit = limit.unwrap_or(5);
    
    // Get recent forum activity for this course
    let forum_activity = create_resource(
        cx,
        move || (course_id, limit),
        move |(course_id, limit)| async move {
            integration_service.get_course_forum_activity(course_id, limit).await
        }
    );
    
    // Create a new general discussion for the course
    let create_discussion = create_action(cx, move |_: &()| {
        let c_id = course_id;
        async move {
            integration_service.create_general_discussion(c_id).await
        }
    });
    
    view! { cx,
        <div class="course-forum-activity">
            <div class="section-header">
                <h3>"Course Discussions"</h3>
                <button
                    class="new-discussion btn btn-sm btn-outline"
                    on:click=move |_| create_discussion.dispatch(())
                    disabled=create_discussion.pending()
                >
                    {if create_discussion.pending() {
                        "Creating..."
                    } else {
                        "New Discussion"
                    }}
                </button>
            </div>
            
            {move || match forum_activity.read(cx) {
                None => view! { cx, <Loading /> },
                Some(Ok(topics)) => {
                    if topics.is_empty() {
                        view! { cx,
                            <div class="no-activity">
                                <p>"No forum activity yet for this course."</p>
                            </div>
                        }
                    } else {
                        view! { cx,
                            <ul class="topic-list">
                                {topics.into_iter().map(|topic| {
                                    view! { cx,
                                        <li class="topic-item">
                                            <A href={format!("/forum/topics/{}", topic.id)} class="topic-link">
                                                <span class="topic-title">{topic.title}</span>
                                                <div class="topic-meta">
                                                    <span class="reply-count">{topic.reply_count} " replies"</span>
                                                    {match topic.last_post_at {
                                                        Some(date) => view! { cx,
                                                            <span class="last-post">{format_timestamp(date)}</span>
                                                        },
                                                        None => view! { cx, <></> }
                                                    }}
                                                </div>
                                            </A>
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        }
                    }
                },
                Some(Err(e)) => view! { cx, <ErrorMessage message={e} /> }
            }}
            
            {move || {
                if let Some(Err(e)) = create_discussion.value().get() {
                    view! { cx, <ErrorMessage message={e} /> }
                } else {
                    view! { cx, <></> }
                }
            }}
            
            <div class="forum-footer">
                <A href={format!("/courses/{}/forum", course_id)} class="view-all-btn">
                    "View All Discussions"
                </A>
            </div>
        </div>
    }
}

fn format_timestamp(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(date);
    
    if diff.num_days() > 30 {
        date.format("%b %d, %Y").to_string()
    } else if diff.num_days() > 0 {
        format!("{} days ago", diff.num_days())
    } else if diff.num_hours() > 0 {
        format!("{} hours ago", diff.num_hours())
    } else if diff.num_minutes() > 0 {
        format!("{} minutes ago", diff.num_minutes())
    } else {
        "Just now".to_string()
    }
}