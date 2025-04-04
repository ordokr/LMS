use leptos::*;
use crate::models::forum::{Category, Topic};
use crate::services::integration_service::{IntegrationService, EntityType};

#[component]
pub fn CourseCategoryLinker() -> impl IntoView {
    // Get service from context
    let integration_service = expect_context::<IntegrationService>();
    let course_service = expect_context::<crate::services::course_service::CourseService>();
    
    // Resources to load data
    let courses = create_resource(
        || (),
        move |_| {
            let service = course_service.clone();
            async move { service.get_courses().await }
        }
    );
    
    // Selected course
    let (selected_course_id, set_selected_course_id) = create_signal(None::<i64>);
    
    // Get category for selected course
    let course_category = create_resource(
        move || selected_course_id.get(),
        move |course_id| {
            let service = integration_service.clone();
            async move {
                match course_id {
                    Some(id) => service.get_course_category(id).await.map(Some).or_else(|_| Ok(None::<Category>)),
                    None => Ok(None)
                }
            }
        }
    );
    
    // Status for UI feedback
    let (status_message, set_status_message) = create_signal(None::<(String, bool)>); // (message, is_error)
    
    // Create category for course
    let create_category = move |course_id: i64| {
        let service = integration_service.clone();
        set_status_message.set(Some(("Creating category...".to_string(), false)));
        
        spawn_local(async move {
            match service.ensure_course_category(course_id).await {
                Ok(_) => {
                    set_status_message.set(Some(("Category created successfully!".to_string(), false)));
                    // Refresh data
                    course_category.refetch();
                },
                Err(e) => {
                    set_status_message.set(Some((format!("Error: {}", e), true)));
                }
            }
        });
    };
    
    view! {
        <div class="course-forum-integration">
            <h2>"Course-Forum Integration"</h2>
            
            <div class="course-selector">
                <label for="course-select">"Select Course:"</label>
                <select 
                    id="course-select"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        if value.is_empty() {
                            set_selected_course_id.set(None);
                        } else {
                            set_selected_course_id.set(value.parse::<i64>().ok());
                        }
                    }
                >
                    <option value="">"-- Select a course --"</option>
                    {move || match courses.get() {
                        None => view! { <option disabled>"Loading courses..."</option> },
                        Some(Ok(course_list)) => {
                            course_list.into_iter().map(|course| {
                                view! { 
                                    <option value={course.id.to_string()}>{&course.name}</option>
                                }
                            }).collect::<Vec<_>>()
                        },
                        Some(Err(_)) => view! { <option disabled>"Error loading courses"</option> }
                    }}
                </select>
            </div>
            
            <div class="category-details">
                {move || match (selected_course_id.get(), course_category.get()) {
                    (Some(course_id), Some(Ok(Some(category)))) => {
                        view! {
                            <div class="category-card">
                                <h3>"Forum Category"</h3>
                                <p><strong>"Name: "</strong>{&category.name}</p>
                                <div class="actions">
                                    <button class="btn primary">"View Category"</button>
                                    <button class="btn">"View Topics"</button>
                                </div>
                            </div>
                        }
                    },
                    (Some(course_id), Some(Ok(None))) => {
                        view! {
                            <div class="no-category">
                                <p>"This course doesn't have a forum category yet."</p>
                                <button 
                                    on:click=move |_| create_category(course_id)
                                    class="btn primary"
                                >
                                    "Create Category"
                                </button>
                            </div>
                        }
                    },
                    (Some(_), Some(Err(e))) => {
                        view! { <p class="error">"Error: " {e}</p> }
                    },
                    (Some(_), None) => {
                        view! { <p>"Loading category information..."</p> }
                    },
                    _ => {
                        view! { <p class="hint">"Select a course to manage its forum integration"</p> }
                    }
                }}
            </div>
            
            {move || status_message.get().map(|(msg, is_error)| {
                view! {
                    <div class={if is_error { "status-message error" } else { "status-message success" }}>
                        {msg}
                    </div>
                }
            })}
        </div>
    }
}