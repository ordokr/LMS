# Modules API Reference

This document describes the Tauri command API for modules in the LMS Integration Project.

## Commands Overview

| Command | Function | Description | Status |
|---------|----------|-------------|--------|
| `get_modules` | `get_modules(course_id: string)` | Retrieves modules for a course | Implemented |
| `get_module` | `get_module(module_id: string)` | Gets a specific module by ID | Implemented |
| `create_module` | `create_module(module_create: ModuleCreate)` | Creates a new module | Implemented |
| `update_module` | `update_module(module_id: string, module_update: ModuleUpdate)` | Updates a module | Implemented |
| `delete_module` | `delete_module(module_id: string)` | Deletes a module | Implemented |
| `reorder_modules` | `reorder_modules(course_id: string, module_ids: string[])` | Reorders modules in a course | Implemented |
| `get_module_items` | `get_module_items(module_id: string)` | Gets items in a module | Implemented |
| `get_module_item` | `get_module_item(item_id: string)` | Gets a specific module item | Implemented |
| `create_module_item` | `create_module_item(item_create: ModuleItemCreate)` | Creates a new module item | Implemented |
| `update_module_item` | `update_module_item(item_id: string, item_update: ModuleItemUpdate)` | Updates a module item | Implemented |
| `delete_module_item` | `delete_module_item(item_id: string)` | Deletes a module item | Implemented |
| `reorder_module_items` | `reorder_module_items(module_id: string, item_ids: string[])` | Reorders items in a module | Implemented |

## Data Types

### Module

pub struct Module {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub position: i32,
    pub items_count: i32,
    pub publish_final_grade: bool,
    pub published: bool,
    pub status: ModuleStatus,
    pub created_at: String,
    pub updated_at: String,
}
```

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCreate {
    pub course_id: String,
    pub title: String,
    pub position: Option<i32>,
    pub publish_final_grade: Option<bool>,
    pub published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleUpdate {
    pub title: Option<String>,
    pub position: Option<i32>,
    pub publish_final_grade: Option<bool>,
    pub published: Option<bool>,
    pub status: Option<ModuleStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleStatus {
    Active,
    Locked,
    Completed,
    Unpublished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: String,
    pub module_id: String,
    pub title: String,
    pub position: i32,
    pub item_type: ModuleItemType,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub url: Option<String>,
    pub page_url: Option<String>,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItemCreate {
    pub module_id: String,
    pub title: String,
    pub position: Option<i32>,
    pub item_type: ModuleItemType,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub url: Option<String>,
    pub page_url: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItemUpdate {
    pub title: Option<String>,
    pub position: Option<i32>,
    pub published: Option<bool>,
    pub url: Option<String>,
    pub page_url: Option<String>,
}

// In your Leptos component
use crate::models::module::{Module, ModuleCreate, ModuleItem, ModuleItemCreate, ModuleItemType};
use leptos::*;
use tauri_sys::tauri::invoke;

#[component]
pub fn CourseModules(course_id: String) -> impl IntoView {
    // Fetch modules for this course
    let modules = create_resource(
        || course_id.clone(),
        |course_id| async move {
            invoke::<_, Vec<Module>>("get_modules", &serde_json::json!({
                "course_id": course_id
            })).await.ok()
        }
    );
    
    // State for new module form
    let (new_module_title, set_new_module_title) = create_signal(String::new());
    
    // Create module action
    let create_module_action = create_action(move |_| {
        let title = new_module_title.get();
        let course_id = course_id.clone();
        
        async move {
            if title.is_empty() {
                return Err("Title cannot be empty".to_string());
            }
            
            let module_create = ModuleCreate {
                course_id,
                title,
                position: None,
                publish_final_grade: Some(false),
                published: Some(true),
            };
            
            match invoke::<_, Module>("create_module", &serde_json::json!({
                "module_create": module_create
            })).await {
                Ok(created) => {
                    // Reset form
                    set_new_module_title.set("".to_string());
                    
                    // Refresh modules
                    modules.refetch();
                    
                    Ok(created)
                },
                Err(e) => Err(e.to_string())
            }
        }
    });
    
    // Active module state for showing items
    let (active_module_id, set_active_module_id) = create_signal(None::<String>);
    
    // Fetch module items for active module
    let module_items = create_resource(
        || active_module_id.get(),
        |maybe_module_id| async move {
            match maybe_module_id {
                Some(module_id) => {
                    invoke::<_, Vec<ModuleItem>>("get_module_items", &serde_json::json!({
                        "module_id": module_id
                    })).await.ok()
                },
                None => None
            }
        }
    );
    
    view! {
        <div class="course-modules">
            <h2>"Course Modules"</h2>
            
            <div class="new-module-form">
                <h3>"Add New Module"</h3>
                <div class="form-group">
                    <label for="module-title">"Module Title"</label>
                    <input
                        id="module-title"
                        type="text"
                        value=new_module_title
                        on:input=move |ev| set_new_module_title.set(event_target_value(&ev))
                    />
                </div>
                <button 
                    on:click=move |_| create_module_action.dispatch(())
                    disabled=create_module_action.pending()
                >
                    {move || if create_module_action.pending() { "Creating..." } else { "Create Module" }}
                </button>
                
                {move || create_module_action.value().map(|result| match result {
                    Ok(_) => view! { <p class="success">"Module created successfully"</p> },
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }
                })}
            </div>
            
            <div class="modules-list">
                <h3>"Modules"</h3>
                <Suspense fallback=move || view! { <p>"Loading modules..."</p> }>
                    {move || modules.get().map(|maybe_modules| match maybe_modules {
                        Some(list) if !list.is_empty() => {
                            view! {
                                <ul class="modules">
                                    {list.into_iter().map(|module| {
                                        let module_id = module.id.clone();
                                        let is_active = create_memo(move |_| {
                                            active_module_id.get().map_or(false, |id| id == module_id)
                                        });
                                        
                                        view! {
                                            <li class=move || {
                                                let mut classes = "module-item".to_string();
                                                if is_active() {
                                                    classes.push_str(" active");
                                                }
                                                classes
                                            }>
                                                <div class="module-header" on:click=move |_| {
                                                    if is_active() {
                                                        set_active_module_id.set(None);
                                                    } else {
                                                        set_active_module_id.set(Some(module_id.clone()));
                                                    }
                                                }>
                                                    <h4>{&module.title}</h4>
                                                    <span class="item-count">{format!("{} items", module.items_count)}</span>
                                                    <span class="module-status">{format!("Status: {}", module.status.to_string())}</span>
                                                </div>
                                                
                                                {move || {
                                                    if is_active() {
                                                        view! {
                                                            <div class="module-items">
                                                                <ModuleItemsList module_id=module_id.clone() />
                                                            </div>
                                                        }
                                                    } else {
                                                        view! { <></> }
                                                    }
                                                }}
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            }
                        },
                        _ => view! { <p>"No modules found for this course."</p> }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
pub fn ModuleItemsList(module_id: String) -> impl IntoView {
    // Fetch items for this module
    let items = create_resource(
        || module_id.clone(),
        |module_id| async move {
            invoke::<_, Vec<ModuleItem>>("get_module_items", &serde_json::json!({
                "module_id": module_id
            })).await.ok()
        }
    );
    
    // State for new item form
    let (new_item_title, set_new_item_title) = create_signal(String::new());
    let (new_item_type, set_new_item_type) = create_signal(ModuleItemType::Page);
    let (new_item_url, set_new_item_url) = create_signal(String::new());
    
    // Create item action
    let create_item_action = create_action(move |_| {
        let title = new_item_title.get();
        let item_type = new_item_type.get();
        let url = new_item_url.get();
        let module_id = module_id.clone();
        
        async move {
            if title.is_empty() {
                return Err("Title cannot be empty".to_string());
            }
            
            let item_create = ModuleItemCreate {
                module_id,
                title,
                position: None,
                item_type,
                content_id: None,
                content_type: None,
                url: if item_type == ModuleItemType::ExternalUrl { Some(url) } else { None },
                page_url: if item_type == ModuleItemType::Page { Some(url) } else { None },
                published: Some(true),
            };
            
            match invoke::<_, ModuleItem>("create_module_item", &serde_json::json!({
                "item_create": item_create
            })).await {
                Ok(created) => {
                    // Reset form
                    set_new_item_title.set("".to_string());
                    set_new_item_url.set("".to_string());
                    
                    // Refresh items
                    items.refetch();
                    
                    Ok(created)
                },
                Err(e) => Err(e.to_string())
            }
        }
    });
    
    view! {
        <div class="module-items-container">
            <Suspense fallback=move || view! { <p>"Loading module items..."</p> }>
                {move || items.get().map(|maybe_items| match maybe_items {
                    Some(list) if !list.is_empty() => {
                        view! {
                            <ul class="module-items-list">
                                {list.into_iter().map(|item| {
                                    view! {
                                        <li class=format!("module-item {}", item.item_type.to_string().to_lowercase())>
                                            <div class="item-icon">{
                                                match item.item_type {
                                                    ModuleItemType::File => "📄",
                                                    ModuleItemType::Page => "📝",
                                                    ModuleItemType::Discussion => "💬",
                                                    ModuleItemType::Assignment => "📚",
                                                    ModuleItemType::Quiz => "❓",
                                                    ModuleItemType::SubHeader => "📌",
                                                    ModuleItemType::ExternalUrl => "🔗",
                                                    ModuleItemType::ExternalTool => "🧰",
                                                }
                                            }</div>
                                            <div class="item-details">
                                                <h5>{&item.title}</h5>
                                                {match item.item_type {
                                                    ModuleItemType::ExternalUrl if item.url.is_some() => {
                                                        view! { <a href={item.url} target="_blank" rel="noopener noreferrer">"Open link"</a> }
                                                    },
                                                    _ => view! { <></> }
                                                }}
                                            </div>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }
                    },
                    _ => view! { <p>"No items in this module."</p> }
                })}
            </Suspense>
            
            <div class="new-item-form">
                <h4>"Add New Item"</h4>
                <div class="form-group">
                    <label for="item-title">"Title"</label>
                    <input
                        id="item-title"
                        type="text"
                        value=new_item_title
                        on:input=move |ev| set_new_item_title.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="form-group">
                    <label for="item-type">"Item Type"</label>
                    <select
                        id="item-type"
                        value=new_item_type
                        on:change=move |ev| set_new_item_type.set(event_target_value(&ev).parse().unwrap())
                    >
                        <option value="Page">"Page"</option>
                        <option value="File">"File"</option>
                        <option value="Discussion">"Discussion"</option>
                        <option value="Assignment">"Assignment"</option>
                        <option value="Quiz">"Quiz"</option>
                        <option value="SubHeader">"SubHeader"</option>
                        <option value="ExternalUrl">"ExternalUrl"</option>
                        <option value="ExternalTool">"ExternalTool"</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label for="item-url">"URL"</label>
                    <input
                        id="item-url"
                        type="text"
                        value=new_item_url
                        on:input=move |ev| set_new_item_url.set(event_target_value(&ev))
                    />
                </div>
                
                <button 
                    on:click=move |_| create_item_action.dispatch(())
                    disabled=create_item_action.pending()
                >
                    {move || if create_item_action.pending() { "Creating..." } else { "Create Item" }}
                </button>
                
                {move || create_item_action.value().map(|result| match result {
                    Ok(_) => view! { <p class="success">"Item created successfully"</p> },
                    Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }
                })}
            </div>
        </div>
    }
}
Error Handling
All API functions return a Result type where the error is a String describing what went wrong. Common errors include:

"Module not found with ID: {id}" - When trying to access a non-existent module
"Module item not found with ID: {id}" - When trying to access a non-existent module item
"Database error: {details}" - When a database operation fails
"Failed to create module: {details}" - When module creation fails
"Failed to update module: {details}" - When module update fails
"Failed to delete module: {details}" - When module deletion fails
"Item ID {id} does not belong to module {module_id}" - When reordering with invalid IDs
"Module IDs list cannot be empty" - When trying to reorder with an empty list