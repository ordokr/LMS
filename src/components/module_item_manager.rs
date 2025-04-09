use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::error_alert::ErrorAlert;

// We can reuse the ModuleItem struct from module_manager.rs
use super::module_manager::{Module, ModuleItem, ModuleItemRequest};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemContent {
    pub id: String,
    pub title: String,
    pub content_type: String,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[component]
pub fn ModuleItemManager(
    module: Module,
    #[prop(optional)] is_editable: bool,
    #[prop(optional)] on_items_change: Option<Callback<Vec<ModuleItem>>>,
) -> impl IntoView {
    // State for module items
    let (items, set_items) = create_signal(module.items.clone());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    
    // State for item being edited
    let (editing_item, set_editing_item) = create_signal(None::<ModuleItem>);
    
    // State for new item dialog
    let (showing_new_item_dialog, set_showing_new_item_dialog) = create_signal(false);
    let (available_content, set_available_content) = create_signal(Vec::<ItemContent>::new());
    let (selected_content_id, set_selected_content_id) = create_signal(None::<String>);
    let (selected_item_type, set_selected_item_type) = create_signal("Assignment".to_string());
    let (external_url, set_external_url) = create_signal(String::new());
    let (new_item_title, set_new_item_title) = create_signal(String::new());
    
    // Drag and drop state
    let (dragging_item_id, set_dragging_item_id) = create_signal(None::<String>);

    // Load available content when opening the new item dialog
    create_effect(move |_| {
        if showing_new_item_dialog.get() {
            load_available_content();
        }
    });
    
    // Notify parent component when items change
    create_effect(move |_| {
        if let Some(callback) = on_items_change {
            callback.call(items.get());
        }
    });

    // Function to load available content based on selected type
    let load_available_content = move || {
        set_loading.set(true);
        set_error.set(None);
        
        let item_type = selected_item_type.get();
        let course_id = module.course_id.clone();
        
        spawn_local(async move {
            match invoke::<_, Vec<ItemContent>>("get_available_content", &(course_id, item_type)).await {
                Ok(content) => {
                    set_available_content.set(content);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load content: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Function to create a new module item
    let create_module_item = move |_| {
        let request = match selected_item_type.get().as_str() {
            "ExternalUrl" => {
                if external_url.get().trim().is_empty() {
                    set_error.set(Some("External URL cannot be empty".to_string()));
                    return;
                }
                
                ModuleItemRequest {
                    module_id: module.id.clone(),
                    title: new_item_title.get(),
                    item_type: selected_item_type.get(),
                    content_id: None,
                    external_url: Some(external_url.get()),
                    position: None,
                    published: Some(true),
                }
            },
            _ => {
                if selected_content_id.get().is_none() {
                    set_error.set(Some("Please select content for this item".to_string()));
                    return;
                }
                
                let content_id = selected_content_id.get().unwrap();
                let title = if new_item_title.get().trim().is_empty() {
                    // Use title from selected content
                    available_content.get()
                        .iter()
                        .find(|c| c.id == content_id)
                        .map(|c| c.title.clone())
                        .unwrap_or_else(|| "Untitled Item".to_string())
                } else {
                    new_item_title.get()
                };
                
                ModuleItemRequest {
                    module_id: module.id.clone(),
                    title,
                    item_type: selected_item_type.get(),
                    content_id: Some(content_id),
                    external_url: None,
                    position: None,
                    published: Some(true),
                }
            }
        };
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<ModuleItemRequest, ModuleItem>("create_module_item", &request).await {
                Ok(new_item) => {
                    set_items.update(|items| items.push(new_item));
                    set_showing_new_item_dialog.set(false);
                    set_selected_content_id.set(None);
                    set_external_url.set(String::new());
                    set_new_item_title.set(String::new());
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to create module item: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Function to delete a module item
    let delete_module_item = move |item_id: String| {
        if !window().confirm_with_message(&format!("Are you sure you want to delete this item?")).unwrap_or(false) {
            return;
        }
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, ()>("delete_module_item", &item_id).await {
                Ok(_) => {
                    set_items.update(|items| {
                        items.retain(|item| item.id != item_id);
                    });
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete module item: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    // Function to toggle published status for a module item
    let toggle_item_published = move |item: ModuleItem| {
        let item_id = item.id.clone();
        let request = ModuleItemRequest {
            module_id: item.module_id,
            title: item.title,
            item_type: item.item_type,
            content_id: item.content_id,
            external_url: item.external_url,
            position: Some(item.position),
            published: Some(!item.published),
        };
        
        spawn_local(async move {
            match invoke::<(String, ModuleItemRequest), ModuleItem>("update_module_item", &(item_id.clone(), request)).await {
                Ok(updated_item) => {
                    set_items.update(|items| {
                        if let Some(index) = items.iter().position(|i| i.id == item_id) {
                            items[index] = updated_item;
                        }
                    });
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update module item: {}", e)));
                }
            }
        });
    };

    // Drag and drop handlers for reordering items
    let handle_item_drag_start = move |item_id: String| {
        set_dragging_item_id.set(Some(item_id));
    };
    
    let handle_item_drag_over = move |e: web_sys::DragEvent| {
        e.prevent_default();
    };
    
    let handle_item_drop = move |target_id: String, e: web_sys::DragEvent| {
        e.prevent_default();
        
        if let Some(source_id) = dragging_item_id.get() {
            if source_id != target_id {
                // Reorder the items
                set_items.update(|items| {
                    // Get positions
                    let source_pos = items.iter().position(|i| i.id == source_id);
                    let target_pos = items.iter().position(|i| i.id == target_id);
                    
                    if let (Some(from_idx), Some(to_idx)) = (source_pos, target_pos) {
                        // Move the element
                        let item = items.remove(from_idx);
                        items.insert(if from_idx > to_idx { to_idx } else { to_idx - 1 }, item);
                    }
                });
                
                // Save the new order to the backend
                let item_ids: Vec<String> = items.get().iter().map(|i| i.id.clone()).collect();
                
                spawn_local(async move {
                    if let Err(e) = invoke::<_, ()>("reorder_module_items", &(module.id.clone(), item_ids)).await {
                        set_error.set(Some(format!("Failed to reorder items: {}", e)));
                    }
                });
            }
            
            set_dragging_item_id.set(None);
        }
    };

    // Function to integrate a module item with Canvas
    let integrate_with_canvas = move |item: ModuleItem| {
        set_loading.set(true);
        
        let item_id = item.id.clone();
        
        spawn_local(async move {
            match invoke::<_, String>("integrate_module_item_with_canvas", &item_id).await {
                Ok(message) => {
                    window().alert_with_message(&message).ok();
                    
                    // Refresh the item data
                    if let Ok(updated_item) = invoke::<_, ModuleItem>("get_module_item", &item_id).await {
                        set_items.update(|items| {
                            if let Some(index) = items.iter().position(|i| i.id == item_id) {
                                items[index] = updated_item;
                            }
                        });
                    }
                    
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to integrate with Canvas: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Function to integrate all module items with Canvas
    let integrate_all_with_canvas = move |_| {
        set_loading.set(true);
        
        let module_id = module.id.clone();
        
        spawn_local(async move {
            match invoke::<_, String>("integrate_module_with_canvas", &module_id).await {
                Ok(message) => {
                    window().alert_with_message(&message).ok();
                    
                    // Refresh all items
                    if let Ok(updated_module) = invoke::<_, Module>("get_module", &module_id).await {
                        set_items.set(updated_module.items);
                    }
                    
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to integrate module with Canvas: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="module-item-manager">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            <div class="module-item-controls">
                {move || {
                    if is_editable {
                        view! {
                            <>
                                <button 
                                    on:click=move |_| set_showing_new_item_dialog.set(true)
                                    class="btn btn-primary"
                                >
                                    "Add Item"
                                </button>
                                
                                <button 
                                    on:click=integrate_all_with_canvas
                                    class="btn btn-secondary"
                                >
                                    "Integrate All with Canvas"
                                </button>
                            </>
                        }
                    } else {
                        view! { <></> }
                    }
                }}
            </div>
            
            {move || {
                if loading.get() && items.get().is_empty() {
                    view! { <div class="loading-spinner">"Loading items..."</div> }
                } else if items.get().is_empty() {
                    view! { <div class="empty-state">"No items in this module"</div> }
                } else {
                    view! {
                        <div class="module-items-list">
                            {move || {
                                items.get().iter().map(|item| {
                                    let item_id = item.id.clone();
                                    let item = item.clone();
                                    
                                    view! {
                                        <div 
                                            class="module-item-card"
                                            class:unpublished=(!item.published)
                                            draggable=is_editable
                                            on:dragstart=move |_| handle_item_drag_start(item_id.clone())
                                            on:dragover=handle_item_drag_over
                                            on:drop=move |e| handle_item_drop(item_id.clone(), e)
                                        >
                                            <div class="item-content">
                                                <span class="item-icon">
                                                    {match item.item_type.as_str() {
                                                        "Assignment" => "📝",
                                                        "Quiz" => "❓",
                                                        "Discussion" => "💬",
                                                        "ExternalUrl" => "🔗",
                                                        "Page" => "📄",
                                                        "Resource" => "📁",
                                                        _ => "📄"
                                                    }}
                                                </span>
                                                <span class="item-title">{&item.title}</span>
                                                
                                                {move || if item.item_type == "ExternalUrl" && item.external_url.is_some() {
                                                    view! {
                                                        <a 
                                                            href={item.external_url.clone().unwrap()}
                                                            target="_blank"
                                                            class="external-link"
                                                        >
                                                            "↗"
                                                        </a>
                                                    }
                                                } else {
                                                    view! { <></> }
                                                }}
                                            </div>
                                            
                                            {move || {
                                                if is_editable {
                                                    view! {
                                                        <div class="item-actions">
                                                            <button 
                                                                class=format!("publish-btn {}", if item.published { "published" } else { "unpublished" })
                                                                on:click=move |_| toggle_item_published(item.clone())
                                                                title=if item.published { "Unpublish" } else { "Publish" }
                                                            >
                                                                {if item.published { "✓" } else { "×" }}
                                                            </button>
                                                            
                                                            <button 
                                                                class="edit-btn"
                                                                on:click=move |_| set_editing_item.set(Some(item.clone()))
                                                                title="Edit item"
                                                            >
                                                                "✎"
                                                            </button>
                                                            
                                                            <button 
                                                                class="integrate-btn"
                                                                on:click=move |_| integrate_with_canvas(item.clone())
                                                                title="Integrate with Canvas"
                                                            >
                                                                "🔄"
                                                            </button>
                                                            
                                                            <button 
                                                                class="delete-btn"
                                                                on:click=move |_| delete_module_item(item_id.clone())
                                                                title="Delete item"
                                                            >
                                                                "🗑"
                                                            </button>
                                                        </div>
                                                    }
                                                } else {
                                                    view! { <></> }
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
            
            // New Item Dialog
            {move || {
                if showing_new_item_dialog.get() {
                    view! {
                        <div class="modal-overlay">
                            <div class="modal-content">
                                <h2>"Add Module Item"</h2>
                                
                                <div class="form-group">
                                    <label for="item-type">"Item Type"</label>
                                    <select 
                                        id="item-type"
                                        on:change=move |ev| {
                                            set_selected_item_type.set(event_target_value(&ev));
                                            set_selected_content_id.set(None);
                                            load_available_content();
                                        }
                                        class="form-control"
                                    >
                                        <option value="Assignment">"Assignment"</option>
                                        <option value="Quiz">"Quiz"</option>
                                        <option value="Discussion">"Discussion"</option>
                                        <option value="Page">"Page"</option>
                                        <option value="Resource">"File/Resource"</option>
                                        <option value="ExternalUrl">"External URL"</option>
                                    </select>
                                </div>
                                
                                {move || {
                                    if selected_item_type.get() == "ExternalUrl" {
                                        view! {
                                            <>
                                                <div class="form-group">
                                                    <label for="item-title">"Item Title"</label>
                                                    <input 
                                                        type="text"
                                                        id="item-title"
                                                        prop:value=new_item_title
                                                        on:input=move |ev| set_new_item_title.set(event_target_value(&ev))
                                                        class="form-control"
                                                        placeholder="Enter item title"
                                                    />
                                                </div>
                                                
                                                <div class="form-group">
                                                    <label for="external-url">"External URL"</label>
                                                    <input 
                                                        type="url"
                                                        id="external-url"
                                                        prop:value=external_url
                                                        on:input=move |ev| set_external_url.set(event_target_value(&ev))
                                                        class="form-control"
                                                        placeholder="https://"
                                                    />
                                                </div>
                                            </>
                                        }
                                    } else {
                                        view! {
                                            <>
                                                <div class="form-group">
                                                    <label for="content-select">{format!("Select {}", selected_item_type.get())}</label>
                                                    <select 
                                                        id="content-select"
                                                        on:change=move |ev| {
                                                            let value = event_target_value(&ev);
                                                            set_selected_content_id.set(if value.is_empty() { None } else { Some(value) });
                                                            
                                                            // Auto-populate title if not set
                                                            if new_item_title.get().is_empty() && !value.is_empty() {
                                                                if let Some(content) = available_content.get()
                                                                    .iter()
                                                                    .find(|c| c.id == value) {
                                                                    set_new_item_title.set(content.title.clone());
                                                                }
                                                            }
                                                        }
                                                        class="form-control"
                                                    >
                                                        <option value="">"-- Select content --"</option>
                                                        {move || available_content.get().iter().map(|content| {
                                                            let id = content.id.clone();
                                                            let selected = selected_content_id.get().as_ref().map(|c| c == &id).unwrap_or(false);
                                                            view! {
                                                                <option value=id selected=selected>
                                                                    {&content.title}
                                                                </option>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </select>
                                                </div>
                                                
                                                <div class="form-group">
                                                    <label for="item-title">"Item Title (Optional)"</label>
                                                    <input 
                                                        type="text"
                                                        id="item-title"
                                                        prop:value=new_item_title
                                                        on:input=move |ev| set_new_item_title.set(event_target_value(&ev))
                                                        class="form-control"
                                                        placeholder="Use content title if left blank"
                                                    />
                                                </div>
                                            </>
                                        }
                                    }
                                }}
                                
                                <div class="modal-actions">
                                    <button 
                                        on:click=create_module_item
                                        disabled=move || {
                                            loading.get() || 
                                            (selected_item_type.get() == "ExternalUrl" && external_url.get().trim().is_empty()) ||
                                            (selected_item_type.get() != "ExternalUrl" && selected_content_id.get().is_none())
                                        }
                                        class="btn btn-primary"
                                    >
                                        "Add Item"
                                    </button>
                                    <button 
                                        on:click=move |_| set_showing_new_item_dialog.set(false)
                                        class="btn btn-secondary"
                                    >
                                        "Cancel"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    view! { <></> }
                }
            }}
            
            // Edit Item Dialog
            {move || {
                if let Some(item) = editing_item.get() {
                    view! {
                        <ModuleItemEditDialog 
                            item=item.clone()
                            on_save=move |updated: ModuleItem| {
                                // Save changes to backend
                                let item_id = updated.id.clone();
                                let request = ModuleItemRequest {
                                    module_id: updated.module_id,
                                    title: updated.title,
                                    item_type: updated.item_type,
                                    content_id: updated.content_id,
                                    external_url: updated.external_url,
                                    position: Some(updated.position),
                                    published: Some(updated.published),
                                };
                                
                                spawn_local(async move {
                                    match invoke::<(String, ModuleItemRequest), ModuleItem>(
                                        "update_module_item", 
                                        &(item_id.clone(), request)
                                    ).await {
                                        Ok(updated_item) => {
                                            // Update items list
                                            set_items.update(|items| {
                                                if let Some(index) = items.iter().position(|i| i.id == item_id) {
                                                    items[index] = updated_item;
                                                }
                                            });
                                            set_editing_item.set(None);
                                        },
                                        Err(e) => {
                                            set_error.set(Some(format!("Failed to update item: {}", e)));
                                        }
                                    }
                                });
                            }
                            on_cancel=move |_| {
                                set_editing_item.set(None);
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

// Component for editing a module item
#[component]
fn ModuleItemEditDialog(
    item: ModuleItem,
    on_save: Callback<ModuleItem>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    // Create state variables for form fields
    let (title, set_title) = create_signal(item.title.clone());
    let (published, set_published) = create_signal(item.published);
    let (external_url, set_external_url) = create_signal(item.external_url.clone().unwrap_or_default());
    
    // Handle save
    let handle_save = move |_| {
        let mut updated_item = item.clone();
        updated_item.title = title.get();
        updated_item.published = published.get();
        
        if item.item_type == "ExternalUrl" {
            updated_item.external_url = Some(external_url.get());
        }
        
        on_save.call(updated_item);
    };
    
    // Handle cancel
    let handle_cancel = move |_| {
        on_cancel.call(());
    };

    view! {
        <div class="modal-overlay">
            <div class="modal-content">
                <h2>"Edit Module Item"</h2>
                
                <div class="form-group">
                    <label for="item-title">"Item Title"</label>
                    <input 
                        type="text"
                        id="item-title"
                        prop:value=title
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        class="form-control"
                    />
                </div>
                
                {move || {
                    if item.item_type == "ExternalUrl" {
                        view! {
                            <div class="form-group">
                                <label for="external-url">"External URL"</label>
                                <input 
                                    type="url"
                                    id="external-url"
                                    prop:value=external_url
                                    on:input=move |ev| set_external_url.set(event_target_value(&ev))
                                    class="form-control"
                                />
                            </div>
                        }
                    } else {
                        // For non-external URL items, show the content ID as read-only
                        view! {
                            <div class="form-group">
                                <label>{format!("{} ID", item.item_type)}</label>
                                <input 
                                    type="text"
                                    value=item.content_id.clone().unwrap_or_default()
                                    disabled=true
                                    class="form-control"
                                />
                                <small class="form-text text-muted">
                                    "Content ID cannot be changed. Delete this item and add a new one to change the content."
                                </small>
                            </div>
                        }
                    }
                }}
                
                <div class="form-check">
                    <input 
                        type="checkbox"
                        id="item-published"
                        checked=published
                        on:change=move |ev| set_published.set(event_target_checked(&ev))
                        class="form-check-input"
                    />
                    <label for="item-published" class="form-check-label">"Published"</label>
                </div>
                
                <div class="modal-actions">
                    <button 
                        on:click=handle_save
                        disabled=move || title.get().trim().is_empty() || 
                            (item.item_type == "ExternalUrl" && external_url.get().trim().is_empty())
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

// Helper function to get event target checked state
fn event_target_checked(ev: &Event) -> bool {
    let target: web_sys::HtmlInputElement = ev.target_dyn_into().unwrap();
    target.checked()
}

// Helper function to get event target value
fn event_target_value(ev: &Event) -> String {
    let target: web_sys::HtmlInputElement = ev.target_dyn_into().unwrap();
    target.value()
}

// Wrapper for window interactions
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: Serialize + ?Sized,
    R: for<'de> Deserialize<'de>,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}