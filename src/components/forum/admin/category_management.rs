use leptos::*;
use crate::models::forum::{Category, CategoryCreate, CategoryUpdate};
use crate::services::forum::ForumService;
use web_sys::SubmitEvent;

#[component]
pub fn CategoryManagement() -> impl IntoView {
    // Admin permission check
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (categories, set_categories) = create_signal(Vec::<Category>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Form state
    let (editing_category, set_editing_category) = create_signal(None::<Category>);
    let (is_new, set_is_new) = create_signal(false);
    let (form_name, set_form_name) = create_signal(String::new());
    let (form_slug, set_form_slug) = create_signal(String::new());
    let (form_description, set_form_description) = create_signal(String::new());
    let (form_color, set_form_color) = create_signal("#3498db".to_string());
    let (form_icon, set_form_icon) = create_signal("folder".to_string());
    let (form_parent_id, set_form_parent_id) = create_signal(None::<i64>);
    let (form_position, set_form_position) = create_signal(0);
    let (saving, set_saving) = create_signal(false);
    
    // Load categories
    let load_categories = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match ForumService::get_categories().await {
                Ok(loaded_categories) => {
                    set_categories.set(loaded_categories);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load categories: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initial load
    create_effect(move |_| {
        if is_admin() {
            load_categories();
        } else {
            set_loading.set(false);
        }
    });
    
    // Edit category
    let edit_category = move |category: Category| {
        set_form_name.set(category.name.clone());
        set_form_slug.set(category.slug.clone());
        set_form_description.set(category.description.clone().unwrap_or_default());
        set_form_color.set(category.color.clone().unwrap_or_else(|| "#3498db".to_string()));
        set_form_icon.set(category.icon.clone().unwrap_or_else(|| "folder".to_string()));
        set_form_parent_id.set(category.parent_id);
        set_form_position.set(category.position.unwrap_or(0));
        set_editing_category.set(Some(category));
        set_is_new.set(false);
    };
    
    // New category
    let new_category = move |_| {
        set_form_name.set(String::new());
        set_form_slug.set(String::new());
        set_form_description.set(String::new());
        set_form_color.set("#3498db".to_string());
        set_form_icon.set("folder".to_string());
        set_form_parent_id.set(None);
        set_form_position.set(categories().len() as i32);
        set_editing_category.set(None);
        set_is_new.set(true);
    };
    
    // Cancel edit
    let cancel_edit = move |_| {
        set_editing_category.set(None);
        set_is_new.set(false);
    };
    
    // Save category
    let save_category = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_saving.set(true);
        set_error.set(None);
        set_success.set(None);
        
        if is_new() {
            // Create new category
            let new_category = CategoryCreate {
                name: form_name(),
                slug: form_slug(),
                description: Some(form_description()),
                color: Some(form_color()),
                icon: Some(form_icon()),
                parent_id: form_parent_id(),
                position: Some(form_position()),
            };
            
            spawn_local(async move {
                match ForumService::create_category(new_category).await {
                    Ok(_) => {
                        set_success.set(Some("Category created successfully".to_string()));
                        load_categories();
                        set_editing_category.set(None);
                        set_is_new.set(false);
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to create category: {}", e)));
                    }
                }
                set_saving.set(false);
            });
        } else if let Some(category) = editing_category() {
            // Update existing category
            let update = CategoryUpdate {
                name: form_name(),
                slug: form_slug(),
                description: Some(form_description()),
                color: Some(form_color()),
                icon: Some(form_icon()),
                parent_id: form_parent_id(),
                position: Some(form_position()),
            };
            
            let category_id = category.id;
            
            spawn_local(async move {
                match ForumService::update_category(category_id, update).await {
                    Ok(_) => {
                        set_success.set(Some("Category updated successfully".to_string()));
                        load_categories();
                        set_editing_category.set(None);
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update category: {}", e)));
                    }
                }
                set_saving.set(false);
            });
        }
    };
    
    // Delete category
    let delete_category = move |category: Category| {
        if !window().confirm_with_message(&format!("Are you sure you want to delete the category \"{}\"? This will delete all topics within it and cannot be undone.", category.name))
            .unwrap_or(false) {
            return;
        }
        
        let category_id = category.id;
        
        spawn_local(async move {
            match ForumService::delete_category(category_id).await {
                Ok(_) => {
                    set_success.set(Some("Category deleted successfully".to_string()));
                    load_categories();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete category: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="category-management">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <div>
                        <div class="d-flex justify-content-between align-items-center mb-4">
                            <h1 class="mb-0">"Category Management"</h1>
                            
                            <button class="btn btn-primary" on:click=new_category>
                                <i class="bi bi-plus-circle me-1"></i>
                                "New Category"
                            </button>
                        </div>
                        
                        {move || error().map(|err| view! {
                            <div class="alert alert-danger mb-4">{err}</div>
                        })}
                        
                        {move || success().map(|msg| view! {
                            <div class="alert alert-success mb-4">{msg}</div>
                        })}
                        
                        <div class="row">
                            <div class="col-md-7">
                                {move || if loading() {
                                    view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                                } else if categories().is_empty() {
                                    view! {
                                        <div class="text-center p-5">
                                            <i class="bi bi-folder-x mb-3 d-block" style="font-size: 3rem;"></i>
                                            <h3>"No categories found"</h3>
                                            <p class="text-muted">"Create your first category to get started."</p>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="card">
                                            <div class="card-header">
                                                <h5 class="mb-0">"Categories"</h5>
                                            </div>
                                            <div class="list-group list-group-flush">
                                                {categories().into_iter().map(|category| {
                                                    let cat = category.clone();
                                                    let cat_for_delete = category.clone();
                                                    
                                                    view! {
                                                        <div class="list-group-item d-flex justify-content-between align-items-center">
                                                            <div class="d-flex align-items-center">
                                                                <div class="category-color-icon me-2" style=format!("background-color: {}", category.color.unwrap_or_else(|| "#3498db".to_string()))>
                                                                    <i class=format!("bi bi-{}", category.icon.unwrap_or_else(|| "folder".to_string()))></i>
                                                                </div>
                                                                <div>
                                                                    <h5 class="mb-0">{category.name}</h5>
                                                                    {category.description.filter(|d| !d.is_empty()).map(|desc| {
                                                                        view! { <small class="text-muted">{desc}</small> }
                                                                    })}
                                                                </div>
                                                            </div>
                                                            <div class="d-flex gap-2">
                                                                <button class="btn btn-sm btn-outline-primary" on:click=move |_| edit_category(cat.clone())>
                                                                    <i class="bi bi-pencil"></i>
                                                                </button>
                                                                <button class="btn btn-sm btn-outline-danger" on:click=move |_| delete_category(cat_for_delete.clone())>
                                                                    <i class="bi bi-trash"></i>
                                                                </button>
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }
                                }}
                            </div>
                            
                            <div class="col-md-5">
                                {move || if is_new() || editing_category().is_some() {
                                    view! {
                                        <div class="card">
                                            <div class="card-header">
                                                <h5 class="mb-0">
                                                    {if is_new() {
                                                        "New Category"
                                                    } else {
                                                        "Edit Category" 
                                                    }}
                                                </h5>
                                            </div>
                                            <div class="card-body">
                                                <form on:submit=save_category>
                                                    <div class="mb-3">
                                                        <label for="categoryName" class="form-label">"Name"</label>
                                                        <input
                                                            id="categoryName"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || form_name()
                                                            on:input=move |ev| set_form_name.set(event_target_value(&ev))
                                                            required
                                                        />
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="categorySlug" class="form-label">"Slug (URL path)"</label>
                                                        <input
                                                            id="categorySlug"
                                                            type="text"
                                                            class="form-control"
                                                            prop:value=move || form_slug()
                                                            on:input=move |ev| set_form_slug.set(event_target_value(&ev))
                                                            required
                                                        />
                                                        <div class="form-text">
                                                            "The slug determines the URL path for this category"
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="mb-3">
                                                        <label for="categoryDescription" class="form-label">"Description"</label>
                                                        <textarea
                                                            id="categoryDescription"
                                                            class="form-control"
                                                            rows="3"
                                                            prop:value=move || form_description()
                                                            on:input=move |ev| set_form_description.set(event_target_value(&ev))
                                                        ></textarea>
                                                    </div>
                                                    
                                                    <div class="row mb-3">
                                                        <div class="col-md-6">
                                                            <label for="categoryColor" class="form-label">"Color"</label>
                                                            <div class="input-group">
                                                                <input
                                                                    id="categoryColor"
                                                                    type="color"
                                                                    class="form-control form-control-color"
                                                                    prop:value=move || form_color()
                                                                    on:input=move |ev| set_form_color.set(event_target_value(&ev))
                                                                />
                                                                <input
                                                                    type="text"
                                                                    class="form-control"
                                                                    prop:value=move || form_color()
                                                                    on:input=move |ev| set_form_color.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="col-md-6">
                                                            <label for="categoryIcon" class="form-label">"Icon"</label>
                                                            <input
                                                                id="categoryIcon"
                                                                type="text"
                                                                class="form-control"
                                                                prop:value=move || form_icon()
                                                                on:input=move |ev| set_form_icon.set(event_target_value(&ev))
                                                                placeholder="folder"
                                                            />
                                                            <div class="form-text">
                                                                "Bootstrap Icons name (e.g., folder, chat, star)"
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="row mb-3">
                                                        <div class="col-md-6">
                                                            <label for="categoryParent" class="form-label">"Parent Category"</label>
                                                            <select
                                                                id="categoryParent"
                                                                class="form-select"
                                                                on:change=move |ev| {
                                                                    let value = event_target_value(&ev);
                                                                    set_form_parent_id.set(
                                                                        if value.is_empty() {
                                                                            None
                                                                        } else {
                                                                            value.parse::<i64>().ok()
                                                                        }
                                                                    );
                                                                }
                                                            >
                                                                <option value="" selected=move || form_parent_id().is_none()>
                                                                    "None (top-level category)"
                                                                </option>
                                                                
                                                                {categories().into_iter().filter(|c| {
                                                                    // Don't show current category or its children as parent options
                                                                    if let Some(editing) = editing_category() {
                                                                        c.id != editing.id && !is_child_of(c.id, editing.id, &categories())
                                                                    } else {
                                                                        true
                                                                    }
                                                                }).map(|c| {
                                                                    view! {
                                                                        <option 
                                                                            value={c.id.to_string()}
                                                                            selected=move || form_parent_id().map(|id| id == c.id).unwrap_or(false)
                                                                        >
                                                                            {c.name}
                                                                        </option>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="col-md-6">
                                                            <label for="categoryPosition" class="form-label">"Position"</label>
                                                            <input
                                                                id="categoryPosition"
                                                                type="number"
                                                                class="form-control"
                                                                prop:value=move || form_position()
                                                                on:input=move |ev| {
                                                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                                        set_form_position.set(val);
                                                                    }
                                                                }
                                                                min="0"
                                                            />
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="d-flex justify-content-end gap-2">
                                                        <button 
                                                            type="button"
                                                            class="btn btn-outline-secondary"
                                                            on:click=cancel_edit
                                                        >
                                                            "Cancel"
                                                        </button>
                                                        
                                                        <button 
                                                            type="submit"
                                                            class="btn btn-primary"
                                                            disabled=move || saving()
                                                        >
                                                            {move || if saving() {
                                                                view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Saving..." }
                                                            } else if is_new() {
                                                                view! { "Create Category" }
                                                            } else {
                                                                view! { "Update Category" }
                                                            }}
                                                        </button>
                                                    </div>
                                                </form>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="card">
                                            <div class="card-body text-center p-5">
                                                <i class="bi bi-folder-plus mb-3 d-block" style="font-size: 3rem;"></i>
                                                <h3>"Category Management"</h3>
                                                <p class="text-muted">
                                                    "Select a category to edit or click the 'New Category' button to create one."
                                                </p>
                                                <button class="btn btn-primary mt-3" on:click=new_category>
                                                    <i class="bi bi-plus-circle me-1"></i>
                                                    "New Category"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to determine if a category is a child of another
fn is_child_of(potential_child_id: i64, potential_parent_id: i64, categories: &[Category]) -> bool {
    for category in categories {
        if category.id == potential_child_id {
            if let Some(parent_id) = category.parent_id {
                if parent_id == potential_parent_id {
                    return true;
                }
                // Check if the parent is a child of potential_parent
                if is_child_of(parent_id, potential_parent_id, categories) {
                    return true;
                }
            }
        }
    }
    false
}