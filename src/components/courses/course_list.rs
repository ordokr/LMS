use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Course {
    id: i64,
    title: String,
    description: Option<String>,
    instructor: String,
    image_url: Option<String>,
}

#[component]
pub fn CourseList(cx: Scope) -> impl IntoView {
    let (courses, set_courses) = create_signal(cx, Vec::<Course>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, String::new());

    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match reqwasm::http::Request::get("/api/courses")
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status() == 200 {
                        match response.json::<Vec<Course>>().await {
                            Ok(data) => {
                                set_courses.set(data);
                            }
                            Err(e) => {
                                set_error.set(format!("Failed to parse courses: {}", e));
                            }
                        }
                    } else {
                        set_error.set("Failed to fetch courses".to_string());
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
        <div class="courses-container">
            <h1>"My Courses"</h1>

            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading courses..."</div> }
                } else if !error.get().is_empty() {
                    view! { cx, <div class="error">{error.get()}</div> }
                } else if courses.get().is_empty() {
                    view! { cx, <div class="empty-state">"No courses found."</div> }
                } else {
                    view! { cx,
                        <div class="course-grid">
                            {move || courses.get().into_iter().map(|course| {
                                view! { cx,
                                    <div class="course-card">
                                        {if let Some(image_url) = &course.image_url {
                                            view! { cx, <img src={image_url.clone()} alt="Course thumbnail" class="course-image"/> }
                                        } else {
                                            view! { cx, <div class="course-image-placeholder"></div> }
                                        }}
                                        <div class="course-content">
                                            <h3 class="course-title">{course.title}</h3>
                                            <p class="course-instructor">"Instructor: " {course.instructor}</p>
                                            {if let Some(description) = &course.description {
                                                view! { cx, <p class="course-description">{description.clone()}</p> }
                                            } else {
                                                view! { cx, <></> }
                                            }}
                                            <a href={format!("/courses/{}", course.id)} class="course-link">"View Course"</a>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }
                }
            }}
        </div>
    }
}