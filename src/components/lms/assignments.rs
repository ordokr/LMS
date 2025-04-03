use leptos::*;
use leptos_router::*;
use web_sys::MouseEvent;

use crate::models::lms::Assignment;
use crate::services::lms_service::LmsService;
use crate::utils::errors::ApiError;

#[component]
pub fn AssignmentsList(cx: Scope, course_id: i64) -> impl IntoView {
    let (assignments, set_assignments) = create_signal(cx, Vec::<Assignment>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);

    // Load assignments when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            match LmsService::get_assignments(course_id).await {
                Ok(data) => {
                    set_assignments.set(data);
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_loading.set(false);
                }
            }
        });
    });

    let handle_delete = move |assignment_id: i64| {
        spawn_local(async move {
            match LmsService::delete_assignment(course_id, assignment_id).await {
                Ok(_) => {
                    // Remove the deleted assignment from the list
                    set_assignments.update(|list| {
                        list.retain(|a| a.id != Some(assignment_id));
                    });
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to delete: {}", err)));
                }
            }
        });
    };

    view! { cx,
        <div class="assignments-list">
            <h2>"Assignments"</h2>
            
            <A href=format!("/courses/{}/assignments/new", course_id) class="button primary">
                "Create Assignment"
            </A>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading assignments..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <div class="error">{err}</div> }
                } else if assignments.get().is_empty() {
                    view! { cx, <div class="empty-state">"No assignments found. Create one to get started."</div> }
                } else {
                    view! { cx,
                        <ul class="assignments">
                            {assignments.get()
                                .into_iter()
                                .map(|assignment| {
                                    let id = assignment.id.unwrap_or(0);
                                    let assignment_clone = assignment.clone();
                                    view! { cx,
                                        <li class="assignment-item" class:published={assignment.published}>
                                            <div class="assignment-header">
                                                <h3>{assignment.title}</h3>
                                                <div class="assignment-status">
                                                    {if assignment.published {
                                                        view! { cx, <span class="published">"Published"</span> }
                                                    } else {
                                                        view! { cx, <span class="unpublished">"Draft"</span> }
                                                    }}
                                                </div>
                                            </div>
                                            
                                            <div class="assignment-details">
                                                <p class="points">{"Points: "}
                                                    {assignment.points_possible.map_or("Not specified".to_string(), |p| p.to_string())}
                                                </p>
                                                
                                                <p class="due-date">
                                                    {if let Some(date) = assignment.due_date {
                                                        format!("Due: {}", date)
                                                    } else {
                                                        "No due date".to_string()
                                                    }}
                                                </p>
                                            </div>
                                            
                                            <div class="assignment-actions">
                                                <A href=format!("/courses/{}/assignments/{}", course_id, id) class="button">
                                                    "View"
                                                </A>
                                                <A href=format!("/courses/{}/assignments/{}/edit", course_id, id) class="button">
                                                    "Edit"
                                                </A>
                                                <button 
                                                    class="button danger"
                                                    on:click=move |_| handle_delete(id)
                                                >
                                                    "Delete"
                                                </button>
                                            </div>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    }
                }
            }}
        </div>
    }
}

#[component]
pub fn AssignmentDetail(cx: Scope, course_id: i64, assignment_id: i64) -> impl IntoView {
    let (assignment, set_assignment) = create_signal(cx, None::<Assignment>);
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);

    // Load assignment when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            match LmsService::get_assignment(course_id, assignment_id).await {
                Ok(data) => {
                    set_assignment.set(Some(data));
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
        <div class="assignment-detail">
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading assignment..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <div class="error">{err}</div> }
                } else if let Some(assignment) = assignment.get() {
                    view! { cx,
                        <div>
                            <div class="assignment-header">
                                <h2>{assignment.title}</h2>
                                <div class="assignment-status">
                                    {if assignment.published {
                                        view! { cx, <span class="published">"Published"</span> }
                                    } else {
                                        view! { cx, <span class="unpublished">"Draft"</span> }
                                    }}
                                </div>
                            </div>
                            
                            <div class="assignment-meta">
                                <div class="points">
                                    <strong>"Points: "</strong>
                                    {assignment.points_possible.map_or("Not specified".to_string(), |p| p.to_string())}
                                </div>
                                
                                <div class="dates">
                                    {if let Some(date) = assignment.due_date {
                                        view! { cx, 
                                            <div class="due-date">
                                                <strong>"Due Date: "</strong>
                                                {date}
                                            </div>
                                        }
                                    } else { view! { cx, <></> } }}
                                    
                                    {if let Some(date) = assignment.available_from {
                                        view! { cx, 
                                            <div class="available-from">
                                                <strong>"Available From: "</strong>
                                                {date}
                                            </div>
                                        }
                                    } else { view! { cx, <></> } }}
                                    
                                    {if let Some(date) = assignment.available_until {
                                        view! { cx, 
                                            <div class="available-until">
                                                <strong>"Available Until: "</strong>
                                                {date}
                                            </div>
                                        }
                                    } else { view! { cx, <></> } }}
                                </div>
                                
                                <div class="submission-types">
                                    <strong>"Submission Types: "</strong>
                                    {if assignment.submission_types.is_empty() {
                                        "No submission required".to_string()
                                    } else {
                                        assignment.submission_types.join(", ")
                                    }}
                                </div>
                            </div>
                            
                            <div class="assignment-description">
                                <h3>"Instructions"</h3>
                                <div class="html-content" inner_html={assignment.description}></div>
                            </div>
                            
                            <div class="assignment-actions">
                                <A href=format!("/courses/{}", course_id) class="button">
                                    "Back to Course"
                                </A>
                                <A href=format!("/courses/{}/assignments/{}/edit", course_id, assignment_id) class="button">
                                    "Edit"
                                </A>
                                <A href=format!("/courses/{}/assignments/{}/submit", course_id, assignment_id) class="button primary">
                                    "Submit Assignment"
                                </A>
                            </div>
                        </div>
                    }
                } else {
                    view! { cx, <div class="error">"Assignment not found"</div> }
                }
            }}
        </div>
    }
}

#[component]
pub fn AssignmentForm(cx: Scope, course_id: i64, assignment_id: Option<i64>) -> impl IntoView {
    let is_edit = assignment_id.is_some();
    let title = create_rw_signal(cx, String::new());
    let description = create_rw_signal(cx, String::new());
    let due_date = create_rw_signal(cx, String::new());
    let available_from = create_rw_signal(cx, String::new());
    let available_until = create_rw_signal(cx, String::new());
    let points_possible = create_rw_signal(cx, String::new());
    let published = create_rw_signal(cx, false);
    
    // Default submission types
    let submission_options = vec![
        "text_entry", "url", "file_upload", "media_recording", "none"
    ];
    let submission_types = create_rw_signal(cx, vec!["none".to_string()]);
    
    let (loading, set_loading) = create_signal(cx, false);
    let (saving, set_saving) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, None::<String>);
    
    // Load assignment data if in edit mode
    create_effect(cx, move |_| {
        if let Some(id) = assignment_id {
            spawn_local(async move {
                set_loading.set(true);
                set_error.set(None);
                
                match LmsService::get_assignment(course_id, id).await {
                    Ok(data) => {
                        title.set(data.title);
                        description.set(data.description);
                        due_date.set(data.due_date.unwrap_or_default());
                        available_from.set(data.available_from.unwrap_or_default());
                        available_until.set(data.available_until.unwrap_or_default());
                        points_possible.set(data.points_possible.map(|p| p.to_string()).unwrap_or_default());
                        published.set(data.published);
                        submission_types.set(data.submission_types);
                        set_loading.set(false);
                    },
                    Err(err) => {
                        set_error.set(Some(err.to_string()));
                        set_loading.set(false);
                    }
                }
            });
        }
    });
    
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let points = points_possible.get().parse::<f64>().ok();
        
        let assignment = Assignment {
            id: assignment_id,
            course_id,
            title: title.get(),
            description: description.get(),
            due_date: if due_date.get().is_empty() { None } else { Some(due_date.get()) },
            available_from: if available_from.get().is_empty() { None } else { Some(available_from.get()) },
            available_until: if available_until.get().is_empty() { None } else { Some(available_until.get()) },
            points_possible: points,
            submission_types: submission_types.get(),
            published: published.get(),
            created_at: None,
            updated_at: None,
        };
        
        set_saving.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            let result = if is_edit {
                LmsService::update_assignment(course_id, assignment_id.unwrap(), assignment).await
            } else {
                LmsService::create_assignment(course_id, assignment).await
            };
            
            match result {
                Ok(_) => {
                    // Redirect to assignments list
                    let navigate = use_navigate(cx);
                    navigate(&format!("/courses/{}/assignments", course_id), Default::default());
                },
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_saving.set(false);
                }
            }
        });
    };
    
    let handle_submission_type_change = move |submission_type: &str, checked: bool| {
        submission_types.update(|types| {
            if checked {
                // Add the type if not already present
                if !types.contains(&submission_type.to_string()) {
                    types.push(submission_type.to_string());
                }
            } else {
                // Remove the type
                types.retain(|t| t != submission_type);
            }
        });
    };
    
    view! { cx,
        <div class="assignment-form">
            <h2>
                {if is_edit { "Edit Assignment" } else { "Create Assignment" }}
            </h2>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading assignment data..."</div> }
                } else {
                    view! { cx,
                        <form on:submit=handle_submit>
                            {move || {
                                if let Some(err) = error.get() {
                                    view! { cx, <div class="error">{err}</div> }
                                } else {
                                    view! { cx, <></> }
                                }
                            }}
                            
                            <div class="form-group">
                                <label for="title">"Title"</label>
                                <input 
                                    type="text"
                                    id="title"
                                    prop:value=title
                                    on:input=move |ev| {
                                        title.set(event_target_value(&ev));
                                    }
                                    required
                                />
                            </div>
                            
                            <div class="form-group">
                                <label for="description">"Instructions"</label>
                                <textarea 
                                    id="description"
                                    prop:value=description
                                    on:input=move |ev| {
                                        description.set(event_target_value(&ev));
                                    }
                                    rows="10"
                                    required
                                ></textarea>
                            </div>
                            
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="points">"Points Possible"</label>
                                    <input 
                                        type="number"
                                        id="points"
                                        prop:value=points_possible
                                        on:input=move |ev| {
                                            points_possible.set(event_target_value(&ev));
                                        }
                                        min="0"
                                        step="0.01"
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label for="due_date">"Due Date"</label>
                                    <input 
                                        type="datetime-local"
                                        id="due_date"
                                        prop:value=due_date
                                        on:input=move |ev| {
                                            due_date.set(event_target_value(&ev));
                                        }
                                    />
                                </div>
                            </div>
                            
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="available_from">"Available From"</label>
                                    <input 
                                        type="datetime-local"
                                        id="available_from"
                                        prop:value=available_from
                                        on:input=move |ev| {
                                            available_from.set(event_target_value(&ev));
                                        }
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label for="available_until">"Available Until"</label>
                                    <input 
                                        type="datetime-local"
                                        id="available_until"
                                        prop:value=available_until
                                        on:input=move |ev| {
                                            available_until.set(event_target_value(&ev));
                                        }
                                    />
                                </div>
                            </div>
                            
                            <div class="form-group">
                                <label>"Submission Types"</label>
                                <div class="checkbox-group">
                                    {submission_options.iter().map(|option| {
                                        let option_clone = option.to_string();
                                        let display_name = match *option {
                                            "text_entry" => "Text Entry",
                                            "url" => "Website URL",
                                            "file_upload" => "File Upload",
                                            "media_recording" => "Media Recording",
                                            "none" => "No Submission",
                                            _ => option,
                                        };
                                        
                                        let checked = create_memo(cx, move || {
                                            submission_types.get().contains(&option_clone)
                                        });
                                        
                                        let option_for_handler = option.to_string();
                                        
                                        view! { cx,
                                            <div class="checkbox-item">
                                                <input 
                                                    type="checkbox"
                                                    id=format!("submission_{}", option)
                                                    checked=move || checked.get()
                                                    on:change=move |ev| {
                                                        let checked = event_target_checked(&ev);
                                                        handle_submission_type_change(&option_for_handler, checked);
                                                    }
                                                />
                                                <label for=format!("submission_{}", option)>
                                                    {display_name}
                                                </label>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                            
                            <div class="form-group">
                                <div class="checkbox-item">
                                    <input 
                                        type="checkbox"
                                        id="published"
                                        checked=move || published.get()
                                        on:change=move |ev| {
                                            published.set(event_target_checked(&ev));
                                        }
                                    />
                                    <label for="published">"Publish"</label>
                                </div>
                            </div>
                            
                            <div class="form-actions">
                                <A href=format!("/courses/{}/assignments", course_id) class="button">
                                    "Cancel"
                                </A>
                                <button type="submit" class="button primary" disabled=saving>
                                    {if saving.get() { "Saving..." } else { if is_edit { "Update Assignment" } else { "Create Assignment" } }}
                                </button>
                            </div>
                        </form>
                    }
                }
            }}
        </div>
    }
}