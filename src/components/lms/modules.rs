use leptos::*;
use leptos_router::*;
use web_sys::{SubmitEvent, DragEvent};

use crate::models::lms::{Module, ModuleItem, ModuleWithItems};
use crate::services::lms_service::LmsService;
use crate::utils::errors::ApiError;
use crate::components::shared::ErrorDisplay;

#[component]
pub fn ModulesList(cx: Scope, course_id: i64) -> impl IntoView {
    let (modules, set_modules) = create_signal(cx, Vec::<Module>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);

    // Load modules when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            match LmsService::get_modules(course_id).await {
                Ok(data) => {
                    set_modules.set(data);
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_loading.set(false);
                }
            }
        });
    });

    let handle_delete = move |module_id: i64| {
        spawn_local(async move {
            match LmsService::delete_module(course_id, module_id).await {
                Ok(_) => {
                    // Remove the deleted module from the list
                    set_modules.update(|list| {
                        list.retain(|m| m.id != Some(module_id));
                    });
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to delete: {}", err)));
                }
            }
        });
    };

    view! { cx,
        <div class="modules-list">
            <h2>"Course Modules"</h2>
            
            <div class="actions">
                <A href=format!("/courses/{}/modules/new", course_id) class="button primary">
                    "Create Module"
                </A>
                <A href=format!("/courses/{}", course_id) class="button">
                    "Back to Course"
                </A>
            </div>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading modules..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <div class="error">{err}</div> }
                } else if modules.get().is_empty() {
                    view! { cx, <div class="empty-state">"No modules found. Create one to get started."</div> }
                } else {
                    view! { cx,
                        <div class="modules-container">
                            {modules.get()
                                .into_iter()
                                .map(|module| {
                                    let id = module.id.unwrap_or(0);
                                    let module_clone = module.clone();
                                    view! { cx,
                                        <div class="module-card" class:published={module.published}>
                                            <div class="module-header">
                                                <h3>{module.title.unwrap_or_else(|| "Untitled Module".to_string())}</h3>
                                                <div class="module-status">
                                                    {if module.published {
                                                        view! { cx, <span class="published">"Published"</span> }
                                                    } else {
                                                        view! { cx, <span class="unpublished">"Draft"</span> }
                                                    }}
                                                </div>
                                            </div>
                                            
                                            {if let Some(desc) = module.description {
                                                view! { cx, <div class="module-description">{desc}</div> }
                                            } else { view! { cx, <></> } }}
                                            
                                            <div class="module-actions">
                                                <A href=format!("/courses/{}/modules/{}", course_id, id) class="button">
                                                    "View"
                                                </A>
                                                <A href=format!("/courses/{}/modules/{}/edit", course_id, id) class="button">
                                                    "Edit"
                                                </A>
                                                <button 
                                                    class="button danger"
                                                    on:click=move |_| handle_delete(id)
                                                >
                                                    "Delete"
                                                </button>
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
pub fn ModuleDetail(cx: Scope, course_id: i64, module_id: i64) -> impl IntoView {
    let (module, set_module) = create_signal(cx, None::<Module>);
    let (items, set_items) = create_signal(cx, Vec::<ModuleItem>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);

    // Load module when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            match LmsService::get_module(course_id, module_id).await {
                Ok((mod_data, items_data)) => {
                    set_module.set(Some(mod_data));
                    set_items.set(items_data);
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_loading.set(false);
                }
            }
        });
    });

    let handle_delete_item = move |item_id: i64| {
        spawn_local(async move {
            match LmsService::delete_module_item(course_id, module_id, item_id).await {
                Ok(_) => {
                    // Remove the deleted item from the list
                    set_items.update(|list| {
                        list.retain(|i| i.id != Some(item_id));
                    });
                }
                Err(err) => {
                    set_error.set(Some(format!("Failed to delete item: {}", err)));
                }
            }
        });
    };

    view! { cx,
        <div class="module-detail">
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading module..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <div class="error">{err}</div> }
                } else if let Some(module) = module.get() {
                    view! { cx,
                        <div>
                            <div class="module-header">
                                <h2>{module.title.unwrap_or_else(|| "Untitled Module".to_string())}</h2>
                                <div class="module-status">
                                    {if module.published {
                                        view! { cx, <span class="published">"Published"</span> }
                                    } else {
                                        view! { cx, <span class="unpublished">"Draft"</span> }
                                    }}
                                </div>
                            </div>
                            
                            {if let Some(desc) = module.description {
                                view! { cx, <div class="module-description">{desc}</div> }
                            } else { view! { cx, <></> } }}
                            
                            <div class="module-actions">
                                <A href=format!("/courses/{}/modules", course_id) class="button">
                                    "Back to Modules"
                                </A>
                                <A href=format!("/courses/{}/modules/{}/edit", course_id, module_id) class="button">
                                    "Edit Module"
                                </A>
                                <A href=format!("/courses/{}/modules/{}/items/new", course_id, module_id) class="button primary">
                                    "Add Item"
                                </A>
                            </div>
                            
                            <h3>"Module Items"</h3>
                            
                            {if items.get().is_empty() {
                                view! { cx, <div class="empty-state">"No items in this module yet. Add one to get started."</div> }
                            } else {
                                view! { cx,
                                    <div class="module-items">
                                        {items.get()
                                            .into_iter()
                                            .map(|item| {
                                                let id = item.id.unwrap_or(0);
                                                let content_type = item.content_type.clone().unwrap_or_else(|| "unknown".to_string());
                                                let content_id = item.content_id.unwrap_or(0);
                                                
                                                let content_link = match content_type.as_str() {
                                                    "assignment" => format!("/courses/{}/assignments/{}", course_id, content_id),
                                                    "page" => format!("/courses/{}/pages/{}", course_id, content_id),
                                                    "discussion" => format!("/courses/{}/discussions/{}", course_id, content_id),
                                                    "file" => format!("/courses/{}/files/{}", course_id, content_id),
                                                    "external_url" => "#".to_string(), // This would be stored in a different field
                                                    _ => "#".to_string()
                                                };
                                                
                                                view! { cx,
                                                    <div class="module-item" class:published={item.published}>
                                                        <div class="item-type">{content_type}</div>
                                                        <div class="item-title">
                                                            <A href={content_link}>
                                                                {item.title.clone().unwrap_or_else(|| "Untitled Item".to_string())}
                                                            </A>
                                                        </div>
                                                        <div class="item-actions">
                                                            <button 
                                                                class="button danger"
                                                                on:click=move |_| handle_delete_item(id)
                                                            >
                                                                "Remove"
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                        }
                                    </div>
                                }
                            }}
                        </div>
                    }
                } else {
                    view! { cx, <div class="error">"Module not found"</div> }
                }
            }}
        </div>
    }
}

#[component]
pub fn ModuleForm(cx: Scope, course_id: i64, module_id: Option<i64>) -> impl IntoView {
    let is_edit = module_id.is_some();
    let title = create_rw_signal(cx, String::new());
    let description = create_rw_signal(cx, String::new());
    let position = create_rw_signal(cx, String::new());
    let published = create_rw_signal(cx, false);
    
    let (loading, set_loading) = create_signal(cx, is_edit);
    let (saving, set_saving) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, None::<ApiError>);
    
    // Load module data if in edit mode
    create_effect(cx, move |_| {
        if let Some(id) = module_id {
            spawn_local(async move {
                set_loading.set(true);
                set_error.set(None);
                
                match LmsService::get_module(course_id, id).await {
                    Ok((module_data, _)) => {
                        title.set(module_data.title.unwrap_or_default());
                        description.set(module_data.description.unwrap_or_default());
                        position.set(module_data.position.map(|p| p.to_string()).unwrap_or_default());
                        published.set(module_data.published);
                        set_loading.set(false);
                    },
                    Err(err) => {
                        set_error.set(Some(err));
                        set_loading.set(false);
                    }
                }
            });
        }
    });
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        // Parse position if provided
        let pos = position.get().parse::<i32>().ok();
        
        let module = Module {
            id: module_id,
            course_id,
            title: Some(title.get()),
            description: if description.get().is_empty() { None } else { Some(description.get()) },
            position: pos,
            published: published.get(),
            created_at: None,
            updated_at: None,
        };
        
        set_saving.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            let result = if is_edit {
                LmsService::update_module(course_id, module_id.unwrap(), module).await.map(|_| module_id.unwrap())
            } else {
                LmsService::create_module(course_id, module).await
            };
            
            match result {
                Ok(id) => {
                    // Navigate to module detail page
                    let navigate = use_navigate(cx);
                    navigate(&format!("/courses/{}/modules/{}", course_id, id), Default::default());
                },
                Err(err) => {
                    set_error.set(Some(err));
                    set_saving.set(false);
                }
            }
        });
    };
    
    view! { cx,
        <div class="module-form">
            <h2>
                {if is_edit { "Edit Module" } else { "Create Module" }}
            </h2>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading module data..."</div> }
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
                                <label for="description">"Description (Optional)"</label>
                                <textarea 
                                    id="description"
                                    prop:value=description
                                    on:input=move |ev| {
                                        description.set(event_target_value(&ev));
                                    }
                                    rows="3"
                                ></textarea>
                            </div>
                            
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
                                <div class="field-hint">"Order in which this module appears"</div>
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
                                <A href=format!("/courses/{}/modules", course_id) class="button">
                                    "Cancel"
                                </A>
                                <button type="submit" class="button primary" disabled=saving>
                                    {if saving.get() { "Saving..." } else { if is_edit { "Update Module" } else { "Create Module" } }}
                                </button>
                            </div>
                        </form>
                    }
                }
            }}
        </div>
    }
}

#[component]
pub fn ModuleItemForm(cx: Scope, course_id: i64, module_id: i64, item_id: Option<i64>) -> impl IntoView {
    let is_edit = item_id.is_some();
    let title = create_rw_signal(cx, String::new());
    let content_type = create_rw_signal(cx, "page".to_string());
    let content_id = create_rw_signal(cx, String::new());
    let position = create_rw_signal(cx, String::new());
    let published = create_rw_signal(cx, false);
    
    // Content type options
    let content_types = vec![
        ("page", "Page"),
        ("assignment", "Assignment"),
        ("file", "File"),
        ("discussion", "Discussion"),
        ("quiz", "Quiz"),
        ("url", "External URL"),
    ];
    
    let (loading, set_loading) = create_signal(cx, false);
    let (saving, set_saving) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, None::<String>);
    
    // Load item data if in edit mode
    create_effect(cx, move |_| {
        if let Some(id) = item_id {
            spawn_local(async move {
                set_loading.set(true);
                set_error.set(None);
                
                match LmsService::get_module(course_id, module_id).await {
                    Ok(data) => {
                        if let Some(item) = data.items.iter().find(|i| i.id == item_id) {
                            title.set(item.title.clone().unwrap_or_default());
                            content_type.set(item.content_type.clone().unwrap_or_else(|| "page".to_string()));
                            content_id.set(item.content_id.map(|id| id.to_string()).unwrap_or_default());
                            position.set(item.position.map(|p| p.to_string()).unwrap_or_default());
                            published.set(item.published);
                        }
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
        let pos = position.get().parse::<i32>().ok();
        let c_id = content_id.get().parse::<i64>().ok();
        
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
                LmsService::add_module_item(course_id, module_id, item).await.map(|_| ())
            };
            
            match result {
                Ok(_) => {
                    // Redirect to module detail
                    let navigate = use_navigate(cx);
                    navigate(&format!("/courses/{}/modules/{}", course_id, module_id), Default::default());
                },
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_saving.set(false);
                }
            }
        });
    };
    
    view! { cx,
        <div class="module-item-form">
            <h2>
                {if is_edit { "Edit Item" } else { "Add Module Item" }}
            </h2>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading item data..."</div> }
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
                                <label for="content_type">"Content Type"</label>
                                <select 
                                    id="content_type"
                                    prop:value=content_type
                                    on:change=move |ev| {
                                        content_type.set(event_target_value(&ev));
                                    }
                                >
                                    {content_types.iter().map(|(value, label)| {
                                        view! { cx,
                                            <option value={value.to_string()}>{label}</option>
                                        }
                                    }).collect::<Vec<_>>()}
                                </select>
                            </div>
                            
                            <div class="form-group">
                                <label for="content_id">"Content ID"</label>
                                <input 
                                    type="text"
                                    id="content_id"
                                    prop:value=content_id
                                    on:input=move |ev| {
                                        content_id.set(event_target_value(&ev));
                                    }
                                />
                                <div class="help-text">"Enter the ID of the existing content (e.g. assignment ID, page ID)"</div>
                            </div>
                            
                            <div class="form-group">
                                <label for="position">"Position"</label>
                                <input 
                                    type="number"
                                    id="position"
                                    prop:value=position
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            position.set(val);
                                        }
                                    }
                                    min="0"
                                    step="1"
                                />
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

// Helper function to extract checked state from checkbox events
fn event_target_checked(event: &web_sys::Event) -> bool {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    element.checked()
}