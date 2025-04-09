# Assignments API Reference

This document describes the Tauri command API for assignments in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `get_assignments` | `get_assignments(course_id?: string)` | Retrieves assignments, optionally filtered by course | Implemented |
| `get_assignment` | `get_assignment(assignment_id: string)` | Retrieves a specific assignment by ID | Implemented |
| `create_assignment` | `create_assignment(assignment: Assignment)` | Creates a new assignment | Implemented |
| `update_assignment` | `update_assignment(assignment: Assignment)` | Updates an existing assignment | Implemented |
| `delete_assignment` | `delete_assignment(assignment_id: string)` | Deletes an assignment | Implemented |

## Data Types

### Assignment

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub description: String,
    pub due_date: Option<String>, // ISO date-time string
    pub points_possible: f64,
    pub status: AssignmentStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentStatus {
    Draft,
    Published,
    Archived,
}
```

## Leptos Components

### AssignmentForm

```rust
use crate::models::course::{Assignment, AssignmentStatus};
use leptos::*;
use tauri_sys::tauri::invoke;
use uuid::Uuid;

#[component]
pub fn AssignmentForm(course_id: String) -> impl IntoView {
    // Form state
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (due_date, set_due_date) = create_signal(String::new());
    let (points, set_points) = create_signal(0.0);
    
    // Create assignment action
    let create_assignment = create_action(move |_| {
        let assignment = Assignment {
            id: Uuid::new_v4().to_string(),
            course_id: course_id.clone(),
            title: title.get(),
            description: description.get(),
            due_date: if due_date.get().is_empty() { None } else { Some(due_date.get()) },
            points_possible: points.get(),
            status: AssignmentStatus::Draft,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        
        async move {
            match invoke::<_, Assignment>("create_assignment", &assignment).await {
                Ok(created) => {
                    log::info!("Assignment created with ID: {}", created.id);
                    Ok(created)
                }
                Err(e) => {
                    log::error!("Failed to create assignment: {}", e);
                    Err(e.to_string())
                }
            }
        }
    });
    
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            create_assignment.dispatch(());
        }>
            <div>
                <label>"Title"</label>
                <input
                    type="text"
                    on:input=move |ev| {
                        set_title.set(event_target_value(&ev));
                    }
                    required
                />
            </div>
            
            <div>
                <label>"Description"</label>
                <textarea
                    on:input=move |ev| {
                        set_description.set(event_target_value(&ev));
                    }
                />
            </div>
            
            <div>
                <label>"Due Date"</label>
                <input
                    type="datetime-local"
                    on:input=move |ev| {
                        set_due_date.set(event_target_value(&ev));
                    }
                />
            </div>
            
            <div>
                <label>"Points Possible"</label>
                <input
                    type="number"
                    step="0.1"
                    min="0"
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        if let Ok(p) = val.parse::<f64>() {
                            set_points.set(p);
                        }
                    }
                />
            </div>
            
            <button type="submit">"Create Assignment"</button>
        </form>
        
        {move || {
            create_assignment.value().map(|result| {
                match result {
                    Ok(assignment) => view! {
                        <div class="success">
                            <p>"Assignment created: " {&assignment.title}</p>
                        </div>
                    },
                    Err(e) => view! {
                        <div class="error">
                            <p>"Error: " {e}</p>
                        </div>
                    }
                }
            })
        }}
    }
}
```

### AssignmentList

```rust
use crate::models::course::Assignment;
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn AssignmentList(course_id: String) -> impl IntoView {
    // Assignments resource
    let assignments = create_resource(
        || (), 
        move |_| {
            let course_id = course_id.clone();
            async move {
                invoke::<_, Vec<Assignment>>("get_assignments", &serde_json::json!({
                    "course_id": course_id
                })).await.ok()
            }
        }
    );

    view! {
        <div>
            <h2>"Assignments"</h2>
            
            <Suspense fallback=move || view! { <p>"Loading assignments..."</p> }>
                {move || {
                    assignments.get().map(|maybe_assignments| {
                        match maybe_assignments {
                            Some(assignments) if !assignments.is_empty() => {
                                view! {
                                    <ul class="assignments-list">
                                        {assignments.into_iter().map(|assignment| {
                                            view! {
                                                <li>
                                                    <h3>{&assignment.title}</h3>
                                                    <p>{&assignment.description}</p>
                                                    <div class="meta">
                                                        <span>"Due: " {assignment.due_date.unwrap_or_else(|| "No due date".to_string())}</span>
                                                        <span>"Points: " {assignment.points_possible}</span>
                                                    </div>
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                }
                            }
                            _ => view! { <p>"No assignments found for this course."</p> }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
```

```rust
.invoke_handler(tauri::generate_handler![
    // ...other commands
    api::assignments::get_assignments,
    api::assignments::get_assignment,
    api::assignments::create_assignment,
    api::assignments::update_assignment,
    api::assignments::delete_assignment,
])
```