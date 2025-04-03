use leptos::*;
use leptos_router::*;
use crate::models::forum::Topic;
use crate::services::forum_service::ForumService;
use crate::services::integration_service::IntegrationService;
use crate::components::loading::Loading;
use crate::components::error_message::ErrorMessage;

#[component]
pub fn AssignmentDiscussion(
    cx: Scope,
    #[prop()] course_id: i64,
    #[prop()] assignment_id: i64,
) -> impl IntoView {
    let integration_service = use_context::<IntegrationService>(cx)
        .expect("IntegrationService should be provided");
    
    // Get the discussion topic for this assignment
    let assignment_topic = create_resource(
        cx,
        move || (course_id, assignment_id),
        move |(course_id, assignment_id)| async move {
            integration_service.get_assignment_topic(assignment_id).await.or_else(|_| {
                // If topic doesn't exist, return None rather than an error
                Ok(None)
            })
        }
    );
    
    // Create a new discussion topic for this assignment
    let create_discussion = create_action(cx, move |_: &()| {
        let c_id = course_id;
        let a_id = assignment_id;
        async move {
            integration_service.create_assignment_discussion(c_id, a_id).await
        }
    });
    
    view! { cx,
        <div class="assignment-discussion">
            <h3>"Assignment Help & Discussion"</h3>
            
            {move || {
                let pending = create_discussion.pending();
                let value = create_discussion.value();
                assignment_topic.read(cx).map(|result| match result {
                    Ok(Some(topic)) => view! { cx, 
                        <div class="discussion-topic">
                            <h4>{topic.title}</h4>
                            <div class="topic-stats">
                                <span class="post-count">{topic.post_count} " posts"</span>
                                {if topic.has_solution {
                                    view! { cx, <span class="has-solution">"Has solution" ✓</span> }
                                } else {
                                    view! { cx, <></> }
                                }}
                                {match topic.last_post_at {
                                    Some(date) => view! { cx, 
                                        <span class="last-post">"Last post: " {format_timestamp(date)}</span>
                                    },
                                    None => view! { cx, <></> }
                                }}
                            </div>
                            <div class="topic-actions">
                                <A 
                                    href={format!("/forum/topics/{}", topic.id)}
                                    class="view-discussion btn btn-primary"
                                >
                                    "View Discussion"
                                </A>
                                <A 
                                    href={format!("/forum/topics/{}/new", topic.id)}
                                    class="ask-question btn btn-secondary"
                                >
                                    "Ask a Question"
                                </A>
                            </div>
                        </div>
                    },
                    Ok(None) => view! { cx,
                        <div class="no-discussion">
                            <p>"There is no discussion board for this assignment yet."</p>
                            <button
                                class="create-discussion btn btn-primary"
                                on:click=move |_| create_discussion.dispatch(())
                                disabled=pending
                            >
                                {if pending() {
                                    "Creating Discussion..."
                                } else {
                                    "Create Discussion Board"
                                }}
                            </button>
                            
                            {move || {
                                if let Some(Ok(topic)) = value.get() {
                                    view! { cx,
                                        <div class="success-message">
                                            <p>"Discussion created successfully!"</p>
                                            <A 
                                                href={format!("/forum/topics/{}", topic.id)}
                                                class="view-discussion btn btn-primary"
                                            >
                                                "View Discussion"
                                            </A>
                                        </div>
                                    }
                                } else if let Some(Err(e)) = value.get() {
                                    view! { cx, <ErrorMessage message={e} /> }
                                } else {
                                    view! { cx, <></> }
                                }
                            }}
                        </div>
                    },
                    Err(e) => view! { cx, <ErrorMessage message={e} /> }
                })
            }}
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