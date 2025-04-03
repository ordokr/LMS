use leptos::*;
use leptos_router::*;
use web_sys::SubmitEvent;
use std::collections::HashMap;

use crate::models::lms::{ModuleItem, Assignment, Page};
use crate::services::lms_service::LmsService;
use crate::utils::errors::ApiError;
use crate::components::shared::ErrorDisplay;

#[component]
pub fn ModuleItemForm(cx: Scope, course_id: i64, module_id: i64, item_id: Option<i64>) -> impl IntoView {
    let is_edit = item_id.is_some();
    
    let title = create_rw_signal(cx, String::new());
    let content_type = create_rw_signal(cx, String::from("page")); // Default to page
    let content_id = create_rw_signal(cx, String::new());
    let position = create_rw_signal(cx, String::new());
    let published = create_rw_signal(cx, false);
    
    // Available content for selection
    let (assignments, set_assignments) = create_signal(cx, Vec::<Assignment>::new());
    let (pages, set_pages) = create_signal(cx, Vec::<Page>::new());
    // We could add other content types as needed: files, quizzes, etc.
    
    let (loading, set_loading) = create_signal(cx, true);
    let (content_loading, set_content_loading) = create_signal(cx, true);
    let (saving, set_saving) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, None::<ApiError>);
    
    // Load content options
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_content_loading.set(true);
            
            // Load assignments
            match LmsService::get_assignments(course_id).await {
                Ok(data) => {
                    set_assignments.set(data);
                },
                Err(err) => {
                    log!("Error loading assignments: {}", err);
                    // Non-critical error, don't show to user
                }
            }
            
            // Load pages
            match LmsService::get_pages(course_id).await {
                Ok(data) => {
                    set_pages.set(data);
                },
                Err(err) => {
                    log!("Error loading pages: {}", err);
                    // Non-critical error, don't show to user
                }
            }
            
            set_content_loading.set(false);
            
            // If editing, load existing module item data
            if let Some(id) = item_id {
                match LmsService::get_module_item(course_id, module_id, id).await {
                    Ok(item) => {
                        title.set(item.title.unwrap_or_default());
                        content_type.set(item.content_type.unwrap_or_default());
                        content_id.set(item.content_id.map(|id| id.to_string()).unwrap_or_default());
                        position.set(item.position.map(|pos| pos.to_string()).unwrap_or_default());
                        published.set(item.published);
                    },
                    Err(err) => {
                        set_error.set(Some(err));
                    }
                }
            }
            
            set_loading.set(false);
        });
    });
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        // Parse content_id and position
        let c_id = content_id.get().parse::<i64>().ok();
        let pos = position.get().parse::<i32>().ok();
        
        let item = ModuleItem {
            id: item_id,
            module_id,
            title: Some(title.get()),
            content_type: Some(content_type.get()),
            content_id: c_id,
            position: pos,
            published: published.get(),
            created_at: None,
            updated_at: None,
        };
        
        set_saving.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            let result = if is_edit {
                LmsService::update_module_item(course_id, module_id, item_id.unwrap(), item).await
            } else {
                LmsService::add_module_item(course_id, module_id, item).await
            };
            
            match result {
                Ok(_) => {
                    // Navigate back to module detail
                    let navigate = use_navigate(cx);
                    navigate(&format!("/courses/{}/modules/{}", course_id, module_id), Default::default());
                },
                Err(err) => {
                    set_error.set(Some(err));
                    set_saving.set(false);
                }
            }
        });
    };
    
    view! { cx,
        <div class="module-item-form">
            <h2>
                {if is_edit { "Edit Module Item" } else { "Add Module Item" }}
            </h2>
            
            {move || {
                if loading.get() || content_loading.get() {
                    view! { cx, <div class="loading">"Loading..."</div> }
                } else {
                    view! { cx,
                        <form on:submit=handle_submit>
                            {move || {
                                if let Some(err) = error.get() {
                                    view! { cx, <ErrorDisplay error=err /> }
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
                                <label for="content-type">"Content Type"</label>
                                <select 
                                    id="content-type"
                                    prop:value=content_type
                                    on:change=move |ev| {
                                        content_type.set(event_target_value(&ev));
                                        // Reset content_id when type changes
                                        content_id.set(String::new());
                                    }
                                >
                                    <option value="page">"Page"</option>
                                    <option value="assignment">"Assignment"</option>
                                    <option value="file">"File"</option>
                                    <option value="discussion">"Discussion"</option>
                                    <option value="quiz">"Quiz"</option>
                                    <option value="url">"External URL"</option>
                                </select>
                            </div>
                            
                            {move || {
                                match content_type.get().as_str() {
                                    "page" => view! { cx,
                                        <div class="form-group">
                                            <label for="content-id">"Select Page"</label>
                                            <select 
                                                id="content-id"
                                                prop:value=content_id
                                                on:change=move |ev| {
                                                    content_id.set(event_target_value(&ev));
                                                }
                                                required
                                            >
                                                <option value="" selected=move || content_id.get().is_empty()>
                                                    "-- Select a Page --"
                                                </option>
                                                {pages.get().into_iter().map(|page| {
                                                    let id = page.id.unwrap_or(0).to_string();
                                                    view! { cx,
                                                        <option 
                                                            value={id.clone()}
                                                            selected=move || content_id.get() == id
                                                        >
                                                            {page.title}
                                                        </option>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </select>
                                            <div class="form-actions">
                                                <a href=format!("/courses/{}/pages/new?return_to=module_item", course_id) class="button small">
                                                    "Create New Page"
                                                </a>
                                            </div>
                                        </div>
                                    },
                                    "assignment" => view! { cx,
                                        <div class="form-group">
                                            <label for="content-id">"Select Assignment"</label>
                                            <select 
                                                id="content-id"
                                                prop:value=content_id
                                                on:change=move |ev| {
                                                    content_id.set(event_target_value(&ev));
                                                }
                                                required
                                            >
                                                <option value="" selected=move || content_id.get().is_empty()>
                                                    "-- Select an Assignment --"
                                                </option>
                                                {assignments.get().into_iter().map(|assignment| {
                                                    let id = assignment.id.unwrap_or(0).to_string();
                                                    view! { cx,
                                                        <option 
                                                            value={id.clone()}
                                                            selected=move || content_id.get() == id
                                                        >
                                                            {assignment.title}
                                                        </option>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </select>
                                            <div class="form-actions">
                                                <a href=format!("/courses/{}/assignments/new?return_to=module_item", course_id) class="button small">
                                                    "Create New Assignment"
                                                </a>
                                            </div>
                                        </div>
                                    },
                                    "url" => view! { cx,
                                        <div class="form-group">
                                            <label for="url">"External URL"</label>
                                            <input 
                                                type="url"
                                                id="url"
                                                prop:value=content_id
                                                on:input=move |ev| {
                                                    content_id.set(event_target_value(&ev));
                                                }
                                                placeholder="https://example.com"
                                                required
                                            />
                                        </div>
                                    },
                                    _ => view! { cx,
                                        <div class="notice">
                                            "This content type is not fully implemented yet. Please select Page, Assignment, or URL."
                                        </div>
                                    }
                                }
                            }}
                            
                            <div class="form-group">
                                <label for="position">"Position (Optional)"</label>
                                <input 
                                    type="number"
                                    id="position"
                                    prop:value=position
                                    on:input=move |ev| {
                                        position.set(event_target_value(&ev));
                                    }
                                    min="0"
                                    step="1"
                                />
                                <div class="field-hint">"Order in which this item appears in the module"</div>
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
                                <A href=format!("/courses/{}/modules/{}", course_id, module_id) class="button">
                                    "Cancel"
                                </A>
                                <button type="submit" class="button primary" disabled=saving>
                                    {if saving.get() { "Saving..." } else { if is_edit { "Update Item" } else { "Add Item" } }}
                                </button>
                            </div>
                        </form>
                    }
                }
            }}
        </div>
    }
}