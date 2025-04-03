use leptos::*;
use crate::models::forum::Topic;
use crate::services::integration_service::IntegrationService;

#[component]
pub fn ModuleDiscussions(
    cx: Scope,
    #[prop()] module_id: i64,
    #[prop()] course_id: i64,
) -> impl IntoView {
    let integration_service = use_context::<IntegrationService>(cx)
        .expect("IntegrationService should be provided");
    
    // Get the topic associated with this module, if any
    let module_topic = create_resource(
        cx,
        || (module_id, course_id),
        move |(module_id, course_id)| async move {
            match integration_service.course_service.get_topic_for_module(module_id).await {
                Ok(Some(topic)) => Ok(Some(topic)),
                Ok(None) => Ok(None),
                Err(e) => Err(format!("Error: {}", e)),
            }
        }
    );
    
    let create_discussion = create_action(cx, move |_: &()| {
        let module_id = module_id;
        let course_id = course_id;
        async move {
            integration_service.create_module_discussion(module_id, course_id).await
        }
    });

    view! { cx,
        <div class="module-discussions">
            <h3>"Module Discussion"</h3>
            
            {move || match module_topic.read(cx) {
                None => view! { cx, <p>"Loading..."</p> },
                Some(Ok(None)) => view! { cx,
                    <div class="no-discussion">
                        <p>"No discussion board exists for this module yet."</p>
                        <button 
                            class="create-discussion-btn"
                            on:click=move |_| create_discussion.dispatch(())
                            disabled=create_discussion.pending()
                        >
                            "Create Discussion Board"
                        </button>
                    </div>
                },
                Some(Ok(Some(topic))) => view! { cx,
                    <div class="discussion-preview">
                        <h4>
                            <a href={format!("/forum/t/{}", topic.id)}>{topic.title}</a>
                        </h4>
                        <div class="topic-stats">
                            <span class="reply-count">{topic.reply_count} " replies"</span>
                            <span class="last-activity">
                                "Last activity: "
                                {format_date(topic.last_post_at.unwrap_or(topic.created_at))}
                            </span>
                        </div>
                        <a 
                            href={format!("/forum/t/{}", topic.id)}
                            class="view-discussion-btn"
                        >
                            "View Discussion"
                        </a>
                    </div>
                },
                Some(Err(e)) => view! { cx, <p class="error">{e}</p> }
            }}
            
            {move || {
                if let Some(Err(e)) = create_discussion.value().get() {
                    view! { cx, <p class="error">"Error creating discussion: " {e}</p> }
                } else {
                    view! { cx, <></> }
                }
            }}
        </div>
    }
}

fn format_date(date: chrono::DateTime<chrono::Utc>) -> String {
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
        "just now".to_string()
    }
}