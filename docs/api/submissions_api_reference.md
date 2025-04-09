# Submissions API Reference

This document describes the Tauri command API for submissions in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `get_submissions` | `get_submissions(assignment_id: string)` | Retrieves submissions for an assignment | Implemented |
| `get_submission` | `get_submission(submission_id: string)` | Retrieves a specific submission by ID | Implemented |
| `create_submission` | `create_submission(submission_create: SubmissionCreate)` | Creates a new submission | Implemented |
| `update_submission` | `update_submission(submission: Submission)` | Updates a submission (typically for grading) | Implemented |

## Data Types

### Submission

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub assignment_id: String,
    pub user_id: String,
    pub content: String,
    pub attachments: Vec<String>,
    pub status: SubmissionStatus,
    pub score: Option<f64>,
    pub feedback: Option<String>,
    pub submitted_at: String,
    pub graded_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionCreate {
    pub assignment_id: String,
    pub user_id: String,
    pub content: String,
    pub attachments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmissionStatus {
    Draft,
    Submitted,
    Graded,
    Resubmitted,
    Late,
}
```

## Leptos Components

### SubmitAssignment Component

```rust
use crate::models::submission::{Submission, SubmissionCreate};
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn SubmitAssignment(assignment_id: String, user_id: String) -> impl IntoView {
    // Form state
    let (content, set_content) = create_signal(String::new());
    let (attachments, set_attachments) = create_signal::<Vec<String>>(vec![]);
    
    // Create submission action
    let submit_assignment = create_action(move |_| {
        let submission_create = SubmissionCreate {
            assignment_id: assignment_id.clone(),
            user_id: user_id.clone(),
            content: content.get(),
            attachments: attachments.get(),
        };
        
        async move {
            match invoke::<_, Submission>("create_submission", &submission_create).await {
                Ok(submission) => {
                    log::info!("Submission created with ID: {}", submission.id);
                    Ok(submission)
                }
                Err(e) => {
                    log::error!("Failed to create submission: {}", e);
                    Err(e.to_string())
                }
            }
        }
    });
    
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            submit_assignment.dispatch(());
        }>
            <div>
                <label>"Your Answer"</label>
                <textarea
                    on:input=move |ev| {
                        set_content.set(event_target_value(&ev));
                    }
                    required
                />
            </div>
            
            {/* Attachments handling would typically involve file uploads */}
            
            <button type="submit">"Submit Assignment"</button>
        </form>
        
        {move || {
            submit_assignment.value().map(|result| {
                match result {
                    Ok(_) => view! {
                        <div class="success">
                            <p>"Assignment submitted successfully!"</p>
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

### GradeSubmission Component

```rust
use crate::models::submission::{Submission, SubmissionStatus};
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn GradeSubmission(submission_id: String) -> impl IntoView {
    let submission = create_resource(
        || submission_id.clone(), 
        move |id| {
            async move {
                invoke::<_, Submission>("get_submission", &serde_json::json!({ "submission_id": id })).await.ok()
            }
        }
    );
    
    let (score, set_score) = create_signal(0.0);
    let (feedback, set_feedback) = create_signal(String::new());
    
    let grade_submission = create_action(move |_| {
        async move {
            if let Some(Some(mut sub)) = submission.get() {
                sub.score = Some(score.get());
                sub.feedback = Some(feedback.get());
                sub.status = crate::models::submission::SubmissionStatus::Graded;
                sub.graded_at = Some(chrono::Utc::now().to_rfc3339());
                
                match invoke::<_, Submission>("update_submission", &sub).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string())
                }
            } else {
                Err("Submission not available for grading".to_string())
            }
        }
    });
    
    view! {
        <Suspense fallback=move || view! { <p>"Loading submission..."</p> }>
            {move || submission.get().map(|maybe_submission| match maybe_submission {
                Some(sub) => view! {
                    <div class="submission">
                        <h3>"Submission Details"</h3>
                        <div class="submission-content">
                            <p>{&sub.content}</p>
                        </div>
                        
                        <form on:submit=move |ev| {
                            ev.prevent_default();
                            grade_submission.dispatch(());
                        }>
                            <div>
                                <label>"Score"</label>
                                <input
                                    type="number"
                                    step="0.1"
                                    min="0"
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                            set_score.set(val);
                                        }
                                    }
                                />
                            </div>
                            
                            <div>
                                <label>"Feedback"</label>
                                <textarea
                                    on:input=move |ev| {
                                        set_feedback.set(event_target_value(&ev));
                                    }
                                />
                            </div>
                            
                            <button type="submit">"Submit Grade"</button>
                        </form>
                        
                        {move || {
                            grade_submission.value().map(|result| {
                                match result {
                                    Ok(_) => view! {
                                        <div class="success">
                                            <p>"Submission graded successfully!"</p>
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
                    </div>
                },
                None => view! { <p>"Submission not found."</p> }
            })}
        </Suspense>
    }
}
```

.invoke_handler(tauri::generate_handler![
    // ...other commands
    api::submissions::get_submissions,
    api::submissions::get_submission,
    api::submissions::create_submission,
    api::submissions::update_submission,
])