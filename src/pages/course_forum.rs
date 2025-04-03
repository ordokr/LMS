use leptos::*;
use leptos_router::*;
use crate::models::forum::{Category, Topic, CreateTopicRequest};
use crate::services::forum_service::ForumService;
use crate::services::integration_service::IntegrationService;
use crate::services::course_service::CourseService;
use crate::components::loading::Loading;
use crate::components::error_message::ErrorMessage;

#[component]
pub fn CourseForum(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let course_id = create_memo(cx, move |_| {
        params.with(|params| {
            params.get("id")
                .and_then(|id| id.parse::<i64>().ok())
                .unwrap_or(0)
        })
    });
    
    let course_service = use_context::<CourseService>(cx)
        .expect("CourseService should be provided");
        
    let integration_service = use_context::<IntegrationService>(cx)
        .expect("IntegrationService should be provided");
        
    let forum_service = use_context::<ForumService>(cx)
        .expect("ForumService should be provided");
    
    // Get course details
    let course = create_resource(
        cx,
        move || course_id(),
        move |id| async move {
            course_service.get_course(id).await
        }
    );
    
    // Get forum category for the course
    let category = create_resource(
        cx,
        move || course_id(),
        move |id| async move {
            // Try to get existing category, or create if it doesn't exist
            integration_service.ensure_course_category(id).await
        }
    );
    
    // Get topics in this category
    let topics = create_resource(
        cx,
        move || {
            category.read(cx).map(|result| {
                result.ok().map(|cat| cat.id)
            }).flatten()
        },
        move |category_id| async move {
            if let Some(cat_id) = category_id {
                forum_service.get_topics_by_category(cat_id).await
            } else {
                Ok(Vec::new())
            }
        }
    );
    
    // Create a new topic
    let (new_topic_title, set_new_topic_title) = create_signal(cx, String::new());
    let (new_topic_content, set_new_topic_content) = create_signal(cx, String::new());
    
    let create_topic = create_action(cx, move |_: &()| {
        let category_id = category.read(cx)
            .map(|result| result.ok().map(|cat| cat.id))
            .flatten()
            .unwrap_or(0);
            
        let request = CreateTopicRequest {
            title: new_topic_title.get(),
            category_id,
            content: new_topic_content.get(),
        };
        
        async move {
            let result = forum_service.create_topic(request).await;
            
            // Clear form on success
            if result.is_ok() {
                set_new_topic_title.set(String::new());
                set_new_topic_content.set(String::new());
            }
            
            result
        }
    });
    
    view! { cx,
        <div class="course-forum-page">
            {move || course.read(cx).map(|result| match result {
                Ok(course) => view! { cx,
                    <div class="course-forum-header">
                        <h2>"Forum: " {course.title}</h2>
                        <A href={format!("/courses/{}", course.id)} class="back-to-course">
                            "← Back to Course"
                        </A>
                    </div>
                },
                Err(e) => view! { cx, <ErrorMessage message={e} /> }
            })}
            
            <div class="forum-container">
                {move || topics.read(cx).map(|result| match result {
                    None => view! { cx, <Loading /> },
                    Some(Ok(topics)) => {
                        if topics.is_empty() {
                            view! { cx,
                                <div class="no-topics">
                                    <p>"No discussions have been created yet."</p>
                                </div>
                            }
                        } else {
                            view! { cx,
                                <div class="topic-list-container">
                                    <h3>"Discussions"</h3>
                                    <table class="topics-table">
                                        <thead>
                                            <tr>
                                                <th>"Topic"</th>
                                                <th>"Replies"</th>
                                                <th>"Created by"</th>
                                                <th>"Last Post"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {topics.into_iter().map(|topic| view! { cx,
                                                <tr class="topic-row">
                                                    <td class="topic-title-cell">
                                                        <A href={format!("/forum/topics/{}", topic.id)}>
                                                            {topic.title}
                                                            {if topic.pinned {
                                                                view! { cx, <span class="topic-pinned">📌</span> }
                                                            } else {
                                                                view! { cx, <></> }
                                                            }}
                                                        </A>
                                                    </td>
                                                    <td class="topic-replies">{topic.reply_count}</td>
                                                    <td class="topic-author">{topic.author_name}</td>
                                                    <td class="topic-last-post">
                                                        {topic.last_post_at.map(|date| format_timestamp(date))
                                                            .unwrap_or_else(|| "No replies yet".to_string())}
                                                    </td>
                                                </tr>
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>
                                </div>
                            }
                        }
                    },
                    Some(Err(e)) => view! { cx, <ErrorMessage message={e} /> }
                })}
                
                <div class="new-topic-form">
                    <h3>"Start a New Discussion"</h3>
                    <form on:submit=|ev| ev.prevent_default()>
                        <div class="form-group">
                            <label for="topic-title">"Title"</label>
                            <input
                                id="topic-title"
                                type="text"
                                value={new_topic_title}
                                on:input=move |ev| {
                                    set_new_topic_title.set(event_target_value(&ev));
                                }
                                required
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="topic-content">"Content"</label>
                            <textarea
                                id="topic-content"
                                value={new_topic_content}
                                on:input=move |ev| {
                                    set_new_topic_content.set(event_target_value(&ev));
                                }
                                required
                                rows="6"
                            ></textarea>
                        </div>
                        
                        <button
                            type="button"
                            class="create-topic-btn btn btn-primary"
                            on:click=move |_| create_topic.dispatch(())
                            disabled=create_topic.pending()
                        >
                            {if create_topic.pending() {
                                "Creating Discussion..."
                            } else {
                                "Create Discussion"
                            }}
                        </button>
                    </form>
                    
                    {move || {
                        if let Some(Err(e)) = create_topic.value().get() {
                            view! { cx, <ErrorMessage message={e} /> }
                        } else if let Some(Ok(_)) = create_topic.value().get() {
                            view! { cx,
                                <div class="success-message">
                                    <p>"Discussion created successfully!"</p>
                                </div>
                            }
                        } else {
                            view! { cx, <></> }
                        }
                    }}
                </div>
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