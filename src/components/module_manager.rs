use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::components::error_alert::ErrorAlert;
use crate::components::module_item_manager::ModuleItemManager;
use crate::utils::date_utils::{serialize_optional_date, deserialize_optional_date};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Module {
    pub id: String,
    pub course_id: String,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub prerequisite_module_id: Option<String>,
    
    #[serde(
        serialize_with = "serialize_optional_date",
        deserialize_with = "deserialize_optional_date"
    )]
    pub unlock_at: Option<DateTime<Utc>>,
    
    #[serde(
        serialize_with = "serialize_optional_date",
        deserialize_with = "deserialize_optional_date"
    )]
    pub created_at: DateTime<Utc>,
    
    #[serde(
        serialize_with = "serialize_optional_date",
        deserialize_with = "deserialize_optional_date"
    )]
    pub updated_at: DateTime<Utc>,
    
    pub published: bool,
    pub items: Vec<ModuleItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ModuleItem {
    pub id: String,
    pub module_id: String,
    pub title: String,
    pub item_type: String,
    pub content_id: Option<String>,
    pub external_url: Option<String>,
    pub position: i32,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModuleRequest {
    pub course_id: String,
    pub name: String,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub prerequisite_module_id: Option<String>,
    pub unlock_at: Option<String>,
    pub published: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModuleItemRequest {
    pub module_id: String,
    pub title: String,
    pub item_type: String,
    pub content_id: Option<String>,
    pub external_url: Option<String>,
    pub position: Option<i32>,
    pub published: Option<bool>,
}

#[component]
pub fn ModuleManager(
    course_id: String,
    #[prop(optional)] is_editable: bool,
) -> impl IntoView {
    // State for modules
    let (modules, set_modules) = create_signal(Vec::<Module>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // State for module being edited
    let (editing_module, set_editing_module) = create_signal(None::<Module>);
    let (new_module_name, set_new_module_name) = create_signal(String::new());
    
    // State for drag and drop
    let (dragging_module_id, set_dragging_module_id) = create_signal(None::<String>);
    
    // Get prerequisites lookup map
    let prerequisites = create_memo(move |_| {
        let mut prereq_map = HashMap::new();
        for module in modules.get() {
            if let Some(prereq_id) = &module.prerequisite_module_id {
                prereq_map.insert(module.id.clone(), prereq_id.clone());
            }
        }
        prereq_map
    });

    // Load modules on component mount
    create_effect(move |_| {
        load_modules();
    });

    // Function to load modules from backend
    let load_modules = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, Vec<Module>>("get_course_modules", &course_id).await {
                Ok(loaded_modules) => {
                    set_modules.set(loaded_modules);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load modules: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Function to create a new module
    let create_module = move |_| {
        if new_module_name.get().trim().is_empty() {
            set_error.set(Some("Module name cannot be empty".to_string()));
            return;
        }
        
        let request = ModuleRequest {
            course_id: course_id.clone(),
            name: new_module_name.get(),
            description: None,
            position: None,
            prerequisite_module_id: None,
            unlock_at: None,
            published: Some(false),
        };
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<ModuleRequest, Module>("create_module", &request).await {
                Ok(new_module) => {
                    // Update modules list
                    set_modules.update(|mods| mods.push(new_module));
                    set_new_module_name.set(String::new());
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to create module: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Function to delete a module
    let delete_module = move |module_id: String| {
        if !window().confirm_with_message(&format!("Are you sure you want to delete this module?")).unwrap_or(false) {
            return;
        }
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, ()>("delete_module", &module_id).await {
                Ok(_) => {
                    // Remove from modules list
                    set_modules.update(|mods| {
                        mods.retain(|m| m.id != module_id);
                    });
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete module: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Function to toggle module published state
    let toggle_publish = move |module: Module| {
        let module_id = module.id.clone();
        let request = ModuleRequest {
            course_id: course_id.clone(),
            name: module.name,
            description: module.description,
            position: Some(module.position),
            prerequisite_module_id: module.prerequisite_module_id,
            unlock_at: module.unlock_at,
            published: Some(!module.published),
        };
        
        spawn_local(async move {
            match invoke::<(String, ModuleRequest), Module>("update_module", &(module_id.clone(), request)).await {
                Ok(updated_module) => {
                    // Update modules list
                    set_modules.update(|mods| {
                        if let Some(index) = mods.iter().position(|m| m.id == module_id) {
                            mods[index] = updated_module;
                        }
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update module: {}", e)));
                }
            }
        });
    };

    // Drag and drop handlers
    let handle_drag_start = move |module_id: String| {
        set_dragging_module_id.set(Some(module_id));
    };
    
    let handle_drag_over = move |e: web_sys::DragEvent| {
        e.prevent_default();
    };
    
    let handle_drop = move |target_module_id: String, e: web_sys::DragEvent| {
        e.prevent_default();
        
        if let Some(source_id) = dragging_module_id.get() {
            if source_id != target_module_id {
                // Reorder the modules
                set_modules.update(|mods| {
                    // Get positions
                    let source_pos = mods.iter().position(|m| m.id == source_id);
                    let target_pos = mods.iter().position(|m| m.id == target_module_id);
                    
                    if let (Some(from_idx), Some(to_idx)) = (source_pos, target_pos) {
                        // Move the element
                        let module = mods.remove(from_idx);
                        mods.insert(if from_idx > to_idx { to_idx } else { to_idx - 1 }, module);
                    }
                });
                
                // Save the new order to the backend
                let module_ids: Vec<String> = modules.get().iter().map(|m| m.id.clone()).collect();
                
                spawn_local(async move {
                    if let Err(e) = invoke::<_, ()>("reorder_modules", &(course_id.clone(), module_ids)).await {
                        set_error.set(Some(format!("Failed to reorder modules: {}", e)));
                    }
                });
            }
            
            set_dragging_module_id.set(None);
        }
    };

    // Function to sync modules from Canvas
    let sync_from_canvas = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, String>("sync_canvas_modules", &course_id).await {
                Ok(message) => {
                    // Reload modules
                    load_modules();
                    window().alert_with_message(&message).ok();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to sync modules: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Add this function inside the ModuleManager component
    let push_module_to_canvas = move |module: Module| {
        set_loading.set(true);
        
        let module_id = module.id.clone();
        
        spawn_local(async move {
            match invoke::<_, String>("push_module_to_canvas", &module_id).await {
                Ok(message) => {
                    window().alert_with_message(&message).ok();
                    
                    // Refresh the module to get updated Canvas IDs
                    match invoke::<_, Module>("get_module", &module_id).await {
                        Ok(updated_module) => {
                            set_modules.update(|mods| {
                                if let Some(index) = mods.iter().position(|m| m.id == module_id) {
                                    mods[index] = updated_module;
                                }
                            });
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to refresh module: {}", e)));
                        }
                    }
                    
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to push module to Canvas: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="module-manager">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            <div class="module-controls">
                {move || {
                    if is_editable {
                        view! {
                            <>
                                <div class="add-module-form">
                                    <input 
                                        type="text" 
                                        placeholder="New module name"
                                        prop:value=new_module_name
                                        on:input=move |ev| {
                                            set_new_module_name.set(event_target_value(&ev));
                                        }
                                        class="form-control"
                                    />
                                    <button 
                                        on:click=create_module
                                        disabled=move || loading.get() || new_module_name.get().trim().is_empty()
                                        class="btn btn-primary"
                                    >
                                        "Add Module"
                                    </button>
                                </div>
                                <button 
                                    on:click=sync_from_canvas
                                    disabled=move || loading.get()
                                    class="btn btn-secondary"
                                >
                                    "Sync from Canvas"
                                </button>
                            </>
                        }
                    } else {
                        view! { <></> }
                    }
                }}
            </div>
            
            {move || {
                if loading.get() && modules.get().is_empty() {
                    view! { <div class="loading-spinner">"Loading modules..."</div> }
                } else if modules.get().is_empty() {
                    view! { <div class="empty-state">"No modules found for this course"</div> }
                } else {
                    view! {
                        <div class="modules-list">
                            {move || {
                                modules.get().iter().map(|module| {
                                    let module_id = module.id.clone();
                                    let module = module.clone();
                                    
                                    view! {
                                        <div 
                                            class="module-card"
                                            class:unpublished=(!module.published)
                                            draggable=is_editable
                                            on:dragstart=move |_| handle_drag_start(module_id.clone())
                                            on:dragover=handle_drag_over
                                            on:drop=move |e| handle_drop(module_id.clone(), e)
                                        >
                                            <div class="module-header">
                                                <h3>{&module.name}</h3>
                                                
                                                {move || {
                                                    if is_editable {
                                                        view! {
                                                            <div class="module-actions">
                                                                <button 
                                                                    class=format!("publish-btn {}", if module.published { "published" } else { "unpublished" })
                                                                    on:click=move |_| toggle_publish(module.clone())
                                                                    title=if module.published { "Unpublish" } else { "Publish" }
                                                                >
                                                                    {if module.published { "✓" } else { "×" }}
                                                                </button>
                                                                
                                                                <button 
                                                                    class="edit-btn"
                                                                    on:click=move |_| set_editing_module.set(Some(module.clone()))
                                                                    title="Edit module"
                                                                >
                                                                    "✎"
                                                                </button>
                                                                
                                                                <button 
                                                                    class="delete-btn"
                                                                    on:click=move |_| delete_module(module_id.clone())
                                                                    title="Delete module"
                                                                >
                                                                    "🗑"
                                                                </button>

                                                                <button 
                                                                    class="push-btn"
                                                                    on:click=move |_| push_module_to_canvas(module.clone())
                                                                    title="Push to Canvas"
                                                                    disabled=loading
                                                                >
                                                                    "🔄"
                                                                </button>
                                                            </div>
                                                        }
                                                    } else {
                                                        view! { <></> }
                                                    }
                                                }}
                                            </div>
                                            
                                            {move || {
                                                if let Some(prereq_id) = prerequisites.get().get(&module.id) {
                                                    if let Some(prereq_module) = modules.get().iter().find(|m| &m.id == prereq_id) {
                                                        view! {
                                                            <div class="prerequisite-info">
                                                                "Prerequisite: " {&prereq_module.name}
                                                            </div>
                                                        }
                                                    } else {
                                                        view! { <></> }
                                                    }
                                                } else {
                                                    view! { <></> }
                                                }
                                            }}
                                            
                                            {move || {
                                                let module_clone = module.clone();
                                                view! {
                                                    <ModuleItemManager 
                                                        module=module_clone
                                                        is_editable={is_editable}
                                                        on_items_change={
                                                            let module_id = module.id.clone();
                                                            Callback::new(move |updated_items: Vec<ModuleItem>| {
                                                                // Update the module with new items
                                                                set_modules.update(|mods| {
                                                                    if let Some(idx) = mods.iter().position(|m| m.id == module_id) {
                                                                        mods[idx].items = updated_items;
                                                                    }
                                                                });
                                                            })
                                                        }
                                                    />
                                                }
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                    }
                }
            }}
            
            {move || {
                if let Some(module) = editing_module.get() {
                    view! {
                        <ModuleEditDialog 
                            module=module
                            modules=modules.get()
                            on_save=move |updated: Module| {
                                // Save changes to backend
                                let module_id = updated.id.clone();
                                let request = ModuleRequest {
                                    course_id: updated.course_id,
                                    name: updated.name,
                                    description: updated.description,
                                    position: Some(updated.position),
                                    prerequisite_module_id: updated.prerequisite_module_id,
                                    unlock_at: updated.unlock_at,
                                    published: Some(updated.published),
                                };
                                
                                spawn_local(async move {
                                    match invoke::<_, Module>("update_module", &(module_id.clone(), request)).await {
                                        Ok(updated_module) => {
                                            // Update modules list
                                            set_modules.update(|mods| {
                                                if let Some(index) = mods.iter().position(|m| m.id == module_id) {
                                                    mods[index] = updated_module;
                                                }
                                            });
                                            set_editing_module.set(None);
                                        },
                                        Err(e) => {
                                            set_error.set(Some(format!("Failed to update module: {}", e)));
                                        }
                                    }
                                });
                            }
                            on_cancel=move |_| {
                                set_editing_module.set(None);
                            }
                        />
                    }
                } else {
                    view! { <></> }
                }
            }}
        </div>
    }
}

#[component]
fn ModuleEditDialog(
    module: Module,
    modules: Vec<Module>,
    on_save: Callback<Module>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    // Create state variables for form fields
    let (name, set_name) = create_signal(module.name.clone());
    let (description, set_description) = create_signal(module.description.clone().unwrap_or_default());
    let (published, set_published) = create_signal(module.published);
    let (prerequisite_id, set_prerequisite_id) = create_signal(module.prerequisite_module_id.clone());
    let (unlock_date, set_unlock_date) = create_signal(module.unlock_at.clone().unwrap_or_default());
    
    // Available modules for prerequisite selection
    let available_modules = modules.iter()
        .filter(|m| m.id != module.id)
        .cloned()
        .collect::<Vec<_>>();
    
    // Handle save
    let handle_save = move |_| {
        let mut updated_module = module.clone();
        updated_module.name = name.get();
        updated_module.description = if description.get().is_empty() { None } else { Some(description.get()) };
        updated_module.published = published.get();
        updated_module.prerequisite_module_id = prerequisite_id.get();
        updated_module.unlock_at = if unlock_date.get().is_empty() { None } else { Some(unlock_date.get()) };
        
        on_save.call(updated_module);
    };
    
    // Handle cancel
    let handle_cancel = move |_| {
        on_cancel.call(());
    };

    view! {
        <div class="modal-overlay">
            <div class="modal-content">
                <h2>"Edit Module"</h2>
                
                <div class="form-group">
                    <label for="module-name">"Name"</label>
                    <input 
                        type="text" 
                        id="module-name"
                        prop:value=name
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        class="form-control"
                    />
                </div>
                
                <div class="form-group">
                    <label for="module-description">"Description"</label>
                    <textarea 
                        id="module-description"
                        prop:value=description
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        class="form-control"
                        rows="3"
                    ></textarea>
                </div>
                
                <div class="form-group">
                    <label for="module-prerequisite">"Prerequisite Module (optional)"</label>
                    <select 
                        id="module-prerequisite"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_prerequisite_id.set(if value.is_empty() { None } else { Some(value) });
                        }
                        class="form-control"
                    >
                        <option value="" selected=prerequisite_id.get().is_none()>
                            "None"
                        </option>
                        {move || available_modules.iter().map(|m| {
                            let id = m.id.clone();
                            let selected = prerequisite_id.get().as_ref().map(|p| p == &id).unwrap_or(false);
                            view! {
                                <option value=id selected=selected>
                                    {&m.name}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                
                <div class="form-group">
                    <label for="module-unlock">"Unlock Date (optional)"</label>
                    <input 
                        type="datetime-local"
                        id="module-unlock"
                        prop:value=unlock_date
                        on:change=move |ev| set_unlock_date.set(event_target_value(&ev))
                        class="form-control"
                    />
                </div>
                
                <div class="form-check">
                    <input 
                        type="checkbox"
                        id="module-published"
                        checked=published
                        on:change=move |ev| set_published.set(event_target_checked(&ev))
                        class="form-check-input"
                    />
                    <label for="module-published" class="form-check-label">"Published"</label>
                </div>
                
                <div class="modal-actions">
                    <button 
                        on:click=handle_save
                        class="btn btn-primary"
                    >
                        "Save"
                    </button>
                    <button 
                        on:click=handle_cancel
                        class="btn btn-secondary"
                    >
                        "Cancel"
                    </button>
                </div>
            </div>
        </div>
    }
}

// Helper function to get event target value
fn event_target_value(ev: &Event) -> String {
    let target: web_sys::HtmlInputElement = ev.target_dyn_into().unwrap();
    target.value()
}

// Helper function to get keycode
fn ev_key_code(ev: &Event) -> u32 {
    let keyboard_event = ev.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
    keyboard_event.key_code()
}

// Wrapper for window interactions
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}