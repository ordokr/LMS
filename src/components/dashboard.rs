use leptos::*;
use leptos_router::*;

use crate::models::lms::{Course, Enrollment};
use crate::models::forum::Topic;
use crate::services::lms_service::LmsService;
use crate::services::forum_service::ForumService;
use crate::components::shared::OfflineIndicator;

#[component]
pub fn Dashboard(cx: Scope) -> impl IntoView {
    let (courses, set_courses) = create_signal(cx, Vec::<Course>::new());
    let (recent_topics, set_recent_topics) = create_signal(cx, Vec::<Topic>::new());
    let (loading_courses, set_loading_courses) = create_signal(cx, true);
    let (loading_topics, set_loading_topics) = create_signal(cx, true);
    let (course_error, set_course_error) = create_signal(cx, None::<String>);
    let (topic_error, set_topic_error) = create_signal(cx, None::<String>);

    // Load user's courses
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading_courses.set(true);
            set_course_error.set(None);

            match LmsService::get_user_courses().await {
                Ok(data) => {
                    set_courses.set(data);
                    set_loading_courses.set(false);
                }
                Err(err) => {
                    set_course_error.set(Some(err.to_string()));
                    set_loading_courses.set(false);
                }
            }
        });
    });

    // Load recent forum topics
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading_topics.set(true);
            set_topic_error.set(None);

            match ForumService::get_recent_topics().await {
                Ok(data) => {
                    set_recent_topics.set(data);
                    set_loading_topics.set(false);
                }
                Err(err) => {
                    set_topic_error.set(Some(err.to_string()));
                    set_loading_topics.set(false);
                }
            }
        });
    });

    view! { cx,
        <div class="dashboard">
            <h1>"Dashboard"</h1>
            
            <div class="dashboard-grid">
                <div class="dashboard-section courses-section">
                    <h2>"My Courses"</h2>
                    
                    {move || {
                        if loading_courses.get() {
                            view! { cx, <div class="loading">"Loading courses..."</div> }
                        } else if let Some(err) = course_error.get() {
                            view! { cx, <div class="error">{err}</div> }
                        } else if courses.get().is_empty() {
                            view! { cx, 
                                <div class="empty-state">
                                    <p>"You are not enrolled in any courses."</p>
                                    <A href="/courses" class="button">"Browse Courses"</A>
                                </div>
                            }
                        } else {
                            view! { cx,
                                <div class="course-cards">
                                    {courses.get()
                                        .into_iter()
                                        .map(|course| {
                                            let id = course.id.unwrap_or(0);
                                            view! { cx,
                                                <div class="course-card">
                                                    <h3>{course.name.clone()}</h3>
                                                    <div class="course-code">{course.code.clone()}</div>
                                                    <A href=format!("/courses/{}", id) class="button">"Go to Course"</A>
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                </div>
                            }
                        }
                    }}
                    
                    <div class="section-actions">
                        <A href="/courses" class="button">"All Courses"</A>
                    </div>
                </div>
                
                <div class="dashboard-section forum-section">
                    <h2>"Recent Forum Activity"</h2>
                    
                    {move || {
                        if loading_topics.get() {
                            view! { cx, <div class="loading">"Loading forum activity..."</div> }
                        } else if let Some(err) = topic_error.get() {
                            view! { cx, <div class="error">{err}</div> }
                        } else if recent_topics.get().is_empty() {
                            view! { cx, 
                                <div class="empty-state">
                                    <p>"No recent forum activity."</p>
                                    <A href="/forum" class="button">"Visit Forum"</A>
                                </div>
                            }
                        } else {
                            view! { cx,
                                <div class="topic-list">
                                    {recent_topics.get()
                                        .into_iter()
                                        .map(|topic| {
                                            let id = topic.id.unwrap_or(0);
                                            view! { cx,
                                                <div class="topic-item">
                                                    <A href=format!("/forum/thread/{}", id)>
                                                        {topic.title.clone()}
                                                    </A>
                                                    <div class="topic-meta">
                                                        {if let Some(date) = &topic.last_post_at {
                                                            view! { cx, <span class="last-reply">{format!("Last reply: {}", date)}</span> }
                                                        } else if let Some(date) = &topic.created_at {
                                                            view! { cx, <span class="created-at">{format!("Created: {}", date)}</span> }
                                                        } else {
                                                            view! { cx, <></> }
                                                        }}
                                                    </div>
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                </div>
                            }
                        }
                    }}
                    
                    <div class="section-actions">
                        <A href="/forum" class="button">"Visit Forum"</A>
                    </div>
                </div>
                
                <div class="dashboard-section status-section">
                    <h2>"System Status"</h2>
                    <div class="status-cards">
                        <div class="status-card">
                            <h3>"Connection Status"</h3>
                            <OfflineIndicator/>
                        </div>
                        <div class="status-card">
                            <h3>"Sync Status"</h3>
                            <div class="sync-status">
                                "Pending items: 0"
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}