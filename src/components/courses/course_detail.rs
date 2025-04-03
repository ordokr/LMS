use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CourseDetail {
    id: i64,
    title: String,
    description: String,
    instructor: String,
    image_url: Option<String>,
    modules: Vec<Module>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module {
    id: i64,
    title: String,
    position: i32,
    content_type: String,
    is_completed: bool,
}

#[component]
pub fn CourseDetail(cx: Scope, course_id: i64) -> impl IntoView {
    let (course, set_course) = create_signal(cx, None::<CourseDetail>);
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, String::new());

    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match reqwasm::http::Request::get(&format!("/api/courses/{}", course_id))
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status() == 200 {
                        match response.json::<CourseDetail>().await {
                            Ok(data) => {
                                set_course.set(Some(data));
                            }
                            Err(e) => {
                                set_error.set(format!("Failed to parse course: {}", e));
                            }
                        }
                    } else if response.status() == 404 {
                        set_error.set("Course not found".to_string());
                    } else {
                        set_error.set("Failed to fetch course".to_string());
                    }
                }
                Err(e) => {
                    set_error.set(format!("Network error: {}", e));
                }
            }
            set_loading.set(false);
        });
    });

    view! { cx,
        <div class="course-detail-container">
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading course..."</div> }
                } else if !error.get().is_empty() {
                    view! { cx, <div class="error">{error.get()}</div> }
                } else if let Some(course_data) = course.get() {
                    view! { cx,
                        <div class="course-header">
                            {if let Some(image_url) = &course_data.image_url {
                                view! { cx, <img src={image_url.clone()} alt="Course image" class="course-banner-image"/> }
                            } else {
                                view! { cx, <div class="course-banner-placeholder"></div> }
                            }}
                            <h1 class="course-title">{course_data.title}</h1>
                            <p class="course-instructor">"Instructor: " {course_data.instructor}</p>
                            <div class="course-description">{course_data.description}</div>
                        </div>

                        <div class="course-modules">
                            <h2>"Course Content"</h2>
                            {if course_data.modules.is_empty() {
                                view! { cx, <p>"No content available yet."</p> }
                            } else {
                                view! { cx,
                                    <ul class="module-list">
                                        {course_data.modules.iter().map(|module| {
                                            view! { cx,
                                                <li class="module-item">
                                                    <div class=format!("module-status {}", if module.is_completed { "completed" } else { "incomplete" })>
                                                        {if module.is_completed {
                                                            view! { cx, <span class="check-icon">"✓"</span> }
                                                        } else {
                                                            view! { cx, <span class="circle-icon"></span> }
                                                        }}
                                                    </div>
                                                    <div class="module-content">
                                                        <h3 class="module-title">{&module.title}</h3>
                                                        <span class="module-type">{&module.content_type}</span>
                                                    </div>
                                                    <a href={format!("/courses/{}/modules/{}", course_id, module.id)} class="module-link">"View"</a>
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                }
                            }}
                        </div>

                        <div class="course-forums">
                            <h2>"Course Discussions"</h2>
                            <a href={format!("/forum/courses/{}", course_id)} class="forum-link">"Go to Forum"</a>
                        </div>
                    }
                } else {
                    view! { cx, <div class="empty-state">"Course not found."</div> }
                }
            }}
        </div>
    }
}