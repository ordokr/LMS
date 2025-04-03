use leptos::*;
use leptos_router::*;
use web_sys::MouseEvent;

use crate::models::lms::{Course, CourseStatus, Enrollment, EnrollmentRole};
use crate::services::lms_service::LmsService;
use crate::utils::errors::ApiError;
use crate::components::shared::ErrorDisplay;
use crate::utils::auth::get_current_user_id;

#[component]
pub fn CoursesList(cx: Scope) -> impl IntoView {
    let (courses, set_courses) = create_signal(cx, Vec::<Course>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<ApiError>);

    // Load courses when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            match LmsService::get_courses().await {
                Ok(data) => {
                    set_courses.set(data);
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(err));
                    set_loading.set(false);
                }
            }
        });
    });

    view! { cx,
        <div class="courses-list">
            <h1>"My Courses"</h1>
            
            <div class="actions">
                <A href="/courses/new" class="button primary">
                    "Create New Course"
                </A>
            </div>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading courses..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <ErrorDisplay error=err /> }
                } else if courses.get().is_empty() {
                    view! { cx, <div class="empty-state">"No courses found. Create one to get started."</div> }
                } else {
                    view! { cx,
                        <div class="courses-grid">
                            {courses.get()
                                .into_iter()
                                .map(|course| {
                                    let status_class = match course.status {
                                        CourseStatus::Active => "status-active",
                                        CourseStatus::Draft => "status-draft",
                                        CourseStatus::Archived => "status-archived",
                                    };
                                    
                                    let status_label = match course.status {
                                        CourseStatus::Active => "Active",
                                        CourseStatus::Draft => "Draft",
                                        CourseStatus::Archived => "Archived",
                                    };
                                    
                                    let course_id = course.id.unwrap_or(0);
                                    
                                    view! { cx,
                                        <div class="course-card">
                                            <div class=format!("course-status {}", status_class)>
                                                {status_label}
                                            </div>
                                            <h2 class="course-title">{course.name}</h2>
                                            <div class="course-code">{course.code}</div>
                                            
                                            {if let Some(desc) = &course.description {
                                                view! { cx, <div class="course-description">{desc}</div> }
                                            } else {
                                                view! { cx, <div class="course-description empty">"No description"</div> }
                                            }}
                                            
                                            <div class="course-dates">
                                                {if let Some(start) = &course.start_date {
                                                    view! { cx, <div class="start-date">"Starts: " {start}</div> }
                                                } else {
                                                    view! { cx, <></> }
                                                }}
                                                
                                                {if let Some(end) = &course.end_date {
                                                    view! { cx, <div class="end-date">"Ends: " {end}</div> }
                                                } else {
                                                    view! { cx, <></> }
                                                }}
                                            </div>
                                            
                                            <div class="course-actions">
                                                <A href=format!("/courses/{}", course_id) class="button">
                                                    "View Course"
                                                </A>
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
        </div>
    }
}

#[component]
pub fn CourseDetail(cx: Scope, course_id: i64) -> impl IntoView {
    let (course, set_course) = create_signal(cx, None::<Course>);
    let (enrollments, set_enrollments) = create_signal(cx, Vec::<Enrollment>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);
    let (is_instructor, set_is_instructor) = create_signal(cx, false);
    
    // User ID is needed to check permissions
    let current_user_id = get_current_user_id().unwrap_or(0);

    // Load course when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            // Load course details
            match LmsService::get_course(course_id).await {
                Ok(data) => {
                    set_course.set(Some(data.clone()));
                    // Check if the current user is the instructor
                    set_is_instructor.set(data.instructor_id == current_user_id);
                    
                    // If user is instructor, also load enrollments
                    if data.instructor_id == current_user_id {
                        match LmsService::get_course_enrollments(course_id).await {
                            Ok(enrollment_data) => set_enrollments.set(enrollment_data),
                            Err(err) => {
                                // Just log this error but don't block loading the course
                                console_log(&format!("Error loading enrollments: {}", err));
                            }
                        }
                    }
                    
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_loading.set(false);
                }
            }
        });
    });

    view! { cx,
        <div class="course-detail">
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading course..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <div class="error">{err}</div> }
                } else if let Some(course) = course.get() {
                    let status_class = match course.status {
                        CourseStatus::Active => "status-active",
                        CourseStatus::Draft => "status-draft",
                        CourseStatus::Archived => "status-archived",
                    };
                    
                    let status_label = match course.status {
                        CourseStatus::Active => "Active",
                        CourseStatus::Draft => "Draft",
                        CourseStatus::Archived => "Archived",
                    };
                    
                    view! { cx,
                        <div>
                            <div class="course-header">
                                <h1>{course.name}</h1>
                                <div class={"course-status ".to_string() + status_class}>
                                    {status_label}
                                </div>
                            </div>
                            
                            <div class="course-code">{course.code}</div>
                            
                            {if let Some(desc) = &course.description {
                                view! { cx, <div class="course-description">{desc}</div> }
                            } else {
                                view! { cx, <div class="course-description empty">"No description available"</div> }
                            }}
                            
                            <div class="course-dates">
                                {if let Some(start) = &course.start_date {
                                    view! { cx, <div class="start-date">"Start Date: "{start}</div> }
                                } else {
                                    view! { cx, <></> }
                                }}
                                
                                {if let Some(end) = &course.end_date {
                                    view! { cx, <div class="end-date">"End Date: "{end}</div> }
                                } else {
                                    view! { cx, <></> }
                                }}
                            </div>
                            
                            <div class="course-actions">
                                <A href="/courses" class="button">
                                    "Back to Courses"
                                </A>
                                
                                {if is_instructor.get() {
                                    view! { cx,
                                        <>
                                            <A href=format!("/courses/{}/edit", course_id) class="button">
                                                "Edit Course"
                                            </A>
                                            <A href=format!("/courses/{}/enrollments", course_id) class="button">
                                                "Manage Enrollments"
                                            </A>
                                        </>
                                    }
                                } else {
                                    view! { cx, <></> }
                                }}
                            </div>
                            
                            <div class="course-content">
                                <div class="content-blocks">
                                    <div class="content-block">
                                        <h2>"Modules"</h2>
                                        <p>"Access learning materials organized into modules."</p>
                                        <A href=format!("/courses/{}/modules", course_id) class="button primary">
                                            "Go to Modules"
                                        </A>
                                    </div>
                                    
                                    <div class="content-block">
                                        <h2>"Assignments"</h2>
                                        <p>"View and submit assignments for this course."</p>
                                        <A href=format!("/courses/{}/assignments", course_id) class="button primary">
                                            "Go to Assignments"
                                        </A>
                                    </div>
                                    
                                    <div class="content-block">
                                        <h2>"Discussions"</h2>
                                        <p>"Participate in course discussions."</p>
                                        <A href=format!("/courses/{}/discussions", course_id) class="button">
                                            "Go to Discussions"
                                        </A>
                                    </div>
                                </div>
                            </div>
                            
                            {if is_instructor.get() && !enrollments.get().is_empty() {
                                view! { cx,
                                    <div class="enrollments-preview">
                                        <h2>"Course Enrollments"</h2>
                                        <table class="enrollments-table">
                                            <thead>
                                                <tr>
                                                    <th>"User ID"</th>
                                                    <th>"Role"</th>
                                                    <th>"Status"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {enrollments.get()
                                                    .into_iter()
                                                    .take(5) // Show only first 5 enrollments in preview
                                                    .map(|enrollment| {
                                                        let role_label = match enrollment.role {
                                                            EnrollmentRole::Student => "Student",
                                                            EnrollmentRole::Teacher => "Teacher",
                                                            EnrollmentRole::TeachingAssistant => "TA",
                                                            EnrollmentRole::Observer => "Observer",
                                                        };
                                                        
                                                        view! { cx,
                                                            <tr>
                                                                <td>{enrollment.user_id}</td>
                                                                <td>{role_label}</td>
                                                                <td>{format!("{:?}", enrollment.status)}</td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                                }
                                            </tbody>
                                        </table>
                                        
                                        {if enrollments.get().len() > 5 {
                                            view! { cx,
                                                <div class="view-all">
                                                    <A href=format!("/courses/{}/enrollments", course_id)>
                                                        "View all "{enrollments.get().len()}" enrollments"
                                                    </A>
                                                </div>
                                            }
                                        } else {
                                            view! { cx, <></> }
                                        }}
                                    </div>
                                }
                            } else {
                                view! { cx, <></> }
                            }}
                        </div>
                    }
                } else {
                    view! { cx, <div class="error">"Course not found"</div> }
                }
            }}
        </div>
    }
}

#[component]
pub fn CourseForm(cx: Scope, course_id: Option<i64>) -> impl IntoView {
    let is_edit = course_id.is_some();
    let form_title = if is_edit { "Edit Course" } else { "Create New Course" };
    
    let code = create_rw_signal(cx, String::new());
    let name = create_rw_signal(cx, String::new());
    let description = create_rw_signal(cx, String::new());
    let start_date = create_rw_signal(cx, String::new());
    let end_date = create_rw_signal(cx, String::new());
    let status = create_rw_signal(cx, CourseStatus::Draft);
    
    let (loading, set_loading) = create_signal(cx, is_edit);
    let (saving, set_saving) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, None::<ApiError>);
    
    // If editing, load existing course data
    if let Some(id) = course_id {
        create_effect(cx, move |_| {
            spawn_local(async move {
                set_loading.set(true);
                
                match LmsService::get_course(id).await {
                    Ok(course) => {
                        code.set(course.code);
                        name.set(course.name);
                        description.set(course.description.unwrap_or_default());
                        start_date.set(course.start_date.unwrap_or_default());
                        end_date.set(course.end_date.unwrap_or_default());
                        status.set(course.status);
                        set_loading.set(false);
                    },
                    Err(err) => {
                        set_error.set(Some(err));
                        set_loading.set(false);
                    }
                }
            });
        });
    }
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let course = Course {
            id: course_id,
            code: code.get(),
            name: name.get(),
            description: if description.get().is_empty() { None } else { Some(description.get()) },
            instructor_id: 0, // This will be set by the backend
            start_date: if start_date.get().is_empty() { None } else { Some(start_date.get()) },
            end_date: if end_date.get().is_empty() { None } else { Some(end_date.get()) },
            status: status.get(),
            created_at: None,
            updated_at: None,
        };
        
        set_saving.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            let result = if is_edit {
                LmsService::update_course(course_id.unwrap(), course).await.map(|_| course_id.unwrap())
            } else {
                LmsService::create_course(course).await
            };
            
            match result {
                Ok(id) => {
                    // Navigate to course detail page
                    let navigate = use_navigate(cx);
                    navigate(&format!("/courses/{}", id), Default::default());
                },
                Err(err) => {
                    set_error.set(Some(err));
                    set_saving.set(false);
                }
            }
        });
    };
    
    view! { cx,
        <div class="course-form">
            <h1>{form_title}</h1>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading course details..."</div> }
                } else {
                    view! { cx,
                        <form on:submit=handle_submit>
                            {move || if let Some(err) = error.get() {
                                view! { cx, <ErrorDisplay error=err /> }
                            } else {
                                view! { cx, <></> }
                            }}
                            
                            <div class="form-group">
                                <label for="course-code">"Course Code"</label>
                                <input 
                                    type="text"
                                    id="course-code"
                                    prop:value=code
                                    on:input=move |ev| {
                                        code.set(event_target_value(&ev));
                                    }
                                    required
                                />
                                <div class="field-hint">"e.g., CS101, MATH250, etc."</div>
                            </div>
                            
                            <div class="form-group">
                                <label for="course-name">"Course Name"</label>
                                <input 
                                    type="text"
                                    id="course-name"
                                    prop:value=name
                                    on:input=move |ev| {
                                        name.set(event_target_value(&ev));
                                    }
                                    required
                                />
                            </div>
                            
                            <div class="form-group">
                                <label for="course-description">"Description"</label>
                                <textarea 
                                    id="course-description"
                                    prop:value=description
                                    on:input=move |ev| {
                                        description.set(event_target_value(&ev));
                                    }
                                    rows="5"
                                ></textarea>
                            </div>
                            
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="start-date">"Start Date"</label>
                                    <input 
                                        type="date"
                                        id="start-date"
                                        prop:value=start_date
                                        on:input=move |ev| {
                                            start_date.set(event_target_value(&ev));
                                        }
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label for="end-date">"End Date"</label>
                                    <input 
                                        type="date"
                                        id="end-date"
                                        prop:value=end_date
                                        on:input=move |ev| {
                                            end_date.set(event_target_value(&ev));
                                        }
                                    />
                                </div>
                            </div>
                            
                            <div class="form-group">
                                <label for="status">"Status"</label>
                                <select 
                                    id="status"
                                    on:change=move |ev| {
                                        match event_target_value(&ev).as_str() {
                                            "draft" => status.set(CourseStatus::Draft),
                                            "active" => status.set(CourseStatus::Active),
                                            "archived" => status.set(CourseStatus::Archived),
                                            _ => ()
                                        }
                                    }
                                >
                                    <option 
                                        value="draft" 
                                        selected=move || matches!(status.get(), CourseStatus::Draft)
                                    >
                                        "Draft"
                                    </option>
                                    <option 
                                        value="active"
                                        selected=move || matches!(status.get(), CourseStatus::Active)
                                    >
                                        "Active"
                                    </option>
                                    <option 
                                        value="archived"
                                        selected=move || matches!(status.get(), CourseStatus::Archived)
                                    >
                                        "Archived"
                                    </option>
                                </select>
                            </div>
                            
                            <div class="form-actions">
                                <A href="/courses" class="button">
                                    "Cancel"
                                </A>
                                <button type="submit" class="button primary" disabled=saving>
                                    {if saving.get() { "Saving..." } else { if is_edit { "Update Course" } else { "Create Course" } }}
                                </button>
                            </div>
                        </form>
                    }
                }
            }}
        </div>
    }
}

// Helper function to get window object
fn window() -> web_sys::Window {
    web_sys::window().expect("No window object available")
}

// Helper function to extract value from input events
fn event_target_value(event: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    element.value()
}