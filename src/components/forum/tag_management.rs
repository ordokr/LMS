use leptos::*;
use crate::models::forum::tag::{Tag, CreateTagRequest, UpdateTagRequest};
use crate::services::forum::ForumService;
use web_sys::SubmitEvent;

#[component]
pub fn TagManagement() -> impl IntoView {
    // Check if user is admin
    let auth_state = use_context::<AuthState>();
    let is_admin = move || auth_state.map(|s| s.is_admin()).unwrap_or(false);
    
    // State signals
    let (tags, set_tags) = create_signal(Vec::<Tag>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    
    // Form signals for new tag
    let (new_tag_name, set_new_tag_name) = create_signal(String::new());
    let (new_tag_description, set_new_tag_description) = create_signal(String::new());
    let (new_tag_color, set_new_tag_color) = create_signal("#0d6efd".to_string());
    let (new_tag_icon, set_new_tag_icon) = create_signal(String::new());
    let (new_tag_restricted, set_new_tag_restricted) = create_signal(false);
    let (creating_tag, set_creating_tag) = create_signal(false);
    
    // Edit mode signals
    let (edit_mode, set_edit_mode) = create_signal(false);
    let (edit_tag_id, set_edit_tag_id) = create_signal(0);
    let (edit_tag_name, set_edit_tag_name) = create_signal(String::new());
    let (edit_tag_description, set_edit_tag_description) = create_signal(String::new());
    let (edit_tag_color, set_edit_tag_color) = create_signal("#0d6efd".to_string());
    let (edit_tag_icon, set_edit_tag_icon) = create_signal(String::new());
    let (edit_tag_restricted, set_edit_tag_restricted) = create_signal(false);
    let (updating_tag, set_updating_tag) = create_signal(false);
    
    // Load all tags
    let load_tags = move || {
        set_loading.set(true);
        
        spawn_local(async move {
            match ForumService::get_tags().await {
                Ok(all_tags) => {
                    set_tags.set(all_tags);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load tags: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Initial load
    create_effect(move |_| {
        load_tags();
    });
    
    // Clear form values
    let clear_form = move || {
        set_new_tag_name.set(String::new());
        set_new_tag_description.set(String::new());
        set_new_tag_color.set("#0d6efd".to_string());
        set_new_tag_icon.set(String::new());
        set_new_tag_restricted.set(false);
    };
    
    // Create new tag
    let handle_create_tag = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        set_creating_tag.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let create_request = CreateTagRequest {
            name: new_tag_name(),
            description: if new_tag_description().is_empty() { None } else { Some(new_tag_description()) },
            color: Some(new_tag_color()),
            icon: if new_tag_icon().is_empty() { None } else { Some(new_tag_icon()) },
            is_restricted: Some(new_tag_restricted()),
        };
        
        spawn_local(async move {
            match ForumService::create_tag(&create_request).await {
                Ok(new_tag) => {
                    set_success.set(Some(format!("Tag \"{}\" created successfully", new_tag.name)));
                    clear_form();
                    load_tags();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to create tag: {}", e)));
                }
            }
            set_creating_tag.set(false);
        });
    };
    
    // Start editing a tag
    let start_edit = move |tag: Tag| {
        set_edit_tag_id.set(tag.id);
        set_edit_tag_name.set(tag.name);
        set_edit_tag_description.set(tag.description.unwrap_or_default());
        set_edit_tag_color.set(tag.color.unwrap_or_else(|| "#0d6efd".to_string()));
        set_edit_tag_icon.set(tag.icon.unwrap_or_default());
        set_edit_tag_restricted.set(tag.is_restricted);
        set_edit_mode.set(true);
    };
    
    // Cancel editing
    let cancel_edit = move |_| {
        set_edit_mode.set(false);
        set_edit_tag_id.set(0);
    };
    
    // Update tag
    let handle_update_tag = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let tag_id = edit_tag_id();
        if tag_id <= 0 {
            return;
        }
        
        set_updating_tag.set(true);
        set_error.set(None);
        set_success.set(None);
        
        let update_request = UpdateTagRequest {
            name: Some(edit_tag_name()),
            description: Some(if edit_tag_description().is_empty() { None } else { Some(edit_tag_description()) }),
            color: Some(Some(edit_tag_color())),
            icon: Some(if edit_tag_icon().is_empty() { None } else { Some(edit_tag_icon()) }),
            is_restricted: Some(edit_tag_restricted()),
        };
        
        spawn_local(async move {
            match ForumService::update_tag(tag_id, &update_request).await {
                Ok(updated_tag) => {
                    set_success.set(Some(format!("Tag \"{}\" updated successfully", updated_tag.name)));
                    set_edit_mode.set(false);
                    load_tags();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to update tag: {}", e)));
                }
            }
            set_updating_tag.set(false);
        });
    };
    
    // Delete tag
    let handle_delete_tag = move |tag_id: i64, tag_name: String| {
        if !window().confirm_with_message(&format!("Are you sure you want to delete the tag \"{}\"? This action cannot be undone.", tag_name)).unwrap_or(false) {
            return;
        }
        
        set_error.set(None);
        set_success.set(None);
        
        spawn_local(async move {
            match ForumService::delete_tag(tag_id).await {
                Ok(_) => {
                    set_success.set(Some(format!("Tag \"{}\" deleted successfully", tag_name)));
                    load_tags();
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete tag: {}", e)));
                }
            }
        });
    };

    view! {
        <div class="tag-management">
            {move || if !is_admin() {
                view! {
                    <div class="alert alert-danger">
                        "You don't have permission to access this page."
                    </div>
                }
            } else {
                view! {
                    <h1 class="mb-4">"Tag Management"</h1>
                    
                    {move || error().map(|err| view! { <div class="alert alert-danger mb-4">{err}</div> })}
                    {move || success().map(|msg| view! { <div class="alert alert-success mb-4">{msg}</div> })}
                    
                    <div class="card mb-4">
                        <div class="card-header">
                            <h5 class="mb-0">
                                {if edit_mode() { "Edit Tag" } else { "Create New Tag" }}
                            </h5>
                        </div>
                        <div class="card-body">
                            {move || if edit_mode() {
                                // Edit tag form
                                view! {
                                    <form on:submit=handle_update_tag>
                                        <div class="mb-3">
                                            <label for="editTagName" class="form-label">"Tag Name"</label>
                                            <input
                                                id="editTagName"
                                                type="text"
                                                class="form-control"
                                                prop:value=move || edit_tag_name()
                                                on:input=move |ev| set_edit_tag_name.set(event_target_value(&ev))
                                                required
                                            />
                                        </div>
                                        
                                        <div class="mb-3">
                                            <label for="editTagDescription" class="form-label">"Description"</label>
                                            <textarea
                                                id="editTagDescription"
                                                class="form-control"
                                                prop:value=move || edit_tag_description()
                                                on:input=move |ev| set_edit_tag_description.set(event_target_value(&ev))
                                                rows="2"
                                            ></textarea>
                                        </div>
                                        
                                        <div class="row g-3 mb-3">
                                            <div class="col-md-6">
                                                <label for="editTagColor" class="form-label">"Color"</label>
                                                <div class="input-group">
                                                    <input
                                                        id="editTagColor"
                                                        type="color"
                                                        class="form-control form-control-color"
                                                        prop:value=move || edit_tag_color()
                                                        on:input=move |ev| set_edit_tag_color.set(event_target_value(&ev))
                                                    />
                                                    <input
                                                        type="text"
                                                        class="form-control"
                                                        prop:value=move || edit_tag_color()
                                                        on:input=move |ev| set_edit_tag_color.set(event_target_value(&ev))
                                                    />
                                                </div>
                                            </div>
                                            
                                            <div class="col-md-6">
                                                <label for="editTagIcon" class="form-label">"Icon (Bootstrap Icon Name)"</label>
                                                <div class="input-group">
                                                    <span class="input-group-text">bi-</span>
                                                    <input
                                                        id="editTagIcon"
                                                        type="text"
                                                        class="form-control"
                                                        prop:value=move || edit_tag_icon()
                                                        on:input=move |ev| set_edit_tag_icon.set(event_target_value(&ev))
                                                        placeholder="tag"
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="mb-3 form-check">
                                            <input
                                                type="checkbox"
                                                class="form-check-input"
                                                id="editTagRestricted"
                                                prop:checked=move || edit_tag_restricted()
                                                on:change=move |ev| set_edit_tag_restricted.set(event_target_checked(&ev))
                                            />
                                            <label class="form-check-label" for="editTagRestricted">
                                                "Restricted (only moderators can assign this tag)"
                                            </label>
                                        </div>
                                        
                                        <div class="d-flex gap-2">
                                            <button type="submit" class="btn btn-primary" disabled=move || updating_tag()>
                                                {move || if updating_tag() {
                                                    view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Updating..." }
                                                } else {
                                                    view! { "Update Tag" }
                                                }}
                                            </button>
                                            <button type="button" class="btn btn-secondary" on:click=cancel_edit>
                                                "Cancel"
                                            </button>
                                        </div>
                                    </form>
                                }
                            } else {
                                // Create tag form
                                view! {
                                    <form on:submit=handle_create_tag>
                                        <div class="mb-3">
                                            <label for="tagName" class="form-label">"Tag Name"</label>
                                            <input
                                                id="tagName"
                                                type="text"
                                                class="form-control"
                                                prop:value=move || new_tag_name()
                                                on:input=move |ev| set_new_tag_name.set(event_target_value(&ev))
                                                required
                                            />
                                        </div>
                                        
                                        <div class="mb-3">
                                            <label for="tagDescription" class="form-label">"Description"</label>
                                            <textarea
                                                id="tagDescription"
                                                class="form-control"
                                                prop:value=move || new_tag_description()
                                                on:input=move |ev| set_new_tag_description.set(event_target_value(&ev))
                                                rows="2"
                                            ></textarea>
                                        </div>
                                        
                                        <div class="row g-3 mb-3">
                                            <div class="col-md-6">
                                                <label for="tagColor" class="form-label">"Color"</label>
                                                <div class="input-group">
                                                    <input
                                                        id="tagColor"
                                                        type="color"
                                                        class="form-control form-control-color"
                                                        prop:value=move || new_tag_color()
                                                        on:input=move |ev| set_new_tag_color.set(event_target_value(&ev))
                                                    />
                                                    <input
                                                        type="text"
                                                        class="form-control"
                                                        prop:value=move || new_tag_color()
                                                        on:input=move |ev| set_new_tag_color.set(event_target_value(&ev))
                                                    />
                                                </div>
                                            </div>
                                            
                                            <div class="col-md-6">
                                                <label for="tagIcon" class="form-label">"Icon (Bootstrap Icon Name)"</label>
                                                <div class="input-group">
                                                    <span class="input-group-text">bi-</span>
                                                    <input
                                                        id="tagIcon"
                                                        type="text"
                                                        class="form-control"
                                                        prop:value=move || new_tag_icon()
                                                        on:input=move |ev| set_new_tag_icon.set(event_target_value(&ev))
                                                        placeholder="tag"
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="mb-3 form-check">
                                            <input
                                                type="checkbox"
                                                class="form-check-input"
                                                id="tagRestricted"
                                                prop:checked=move || new_tag_restricted()
                                                on:change=move |ev| set_new_tag_restricted.set(event_target_checked(&ev))
                                            />
                                            <label class="form-check-label" for="tagRestricted">
                                                "Restricted (only moderators can assign this tag)"
                                            </label>
                                        </div>
                                        
                                        <button type="submit" class="btn btn-primary" disabled=move || creating_tag()>
                                            {move || if creating_tag() {
                                                view! { <span class="spinner-border spinner-border-sm me-2" role="status"></span> "Creating..." }
                                            } else {
                                                view! { "Create Tag" }
                                            }}
                                        </button>
                                    </form>
                                }
                            }}
                        </div>
                    </div>
                    
                    <div class="card">
                        <div class="card-header">
                            <h5 class="mb-0">"Existing Tags"</h5>
                        </div>
                        <div class="card-body">
                            {move || if loading() {
                                view! { <div class="d-flex justify-content-center p-3"><div class="spinner-border" role="status"></div></div> }
                            } else if tags().is_empty() {
                                view! { <p class="text-center text-muted">"No tags have been created yet."</p> }
                            } else {
                                view! {
                                    <div class="table-responsive">
                                        <table class="table table-striped">
                                            <thead>
                                                <tr>
                                                    <th>"Tag"</th>
                                                    <th>"Color"</th>
                                                    <th>"Topic Count"</th>
                                                    <th>"Restricted"</th>
                                                    <th>"Actions"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {tags().into_iter().map(|tag| {
                                                    let tag_clone = tag.clone();
                                                    let tag_id = tag.id;
                                                    let tag_name = tag.name.clone();
                                                    
                                                    view! {
                                                        <tr>
                                                            <td>
                                                                <div class="d-flex align-items-center">
                                                                    <span class="tag-color-dot me-2" 
                                                                          style={format!("background-color: {}", tag.color.unwrap_or_else(|| "#0d6efd".to_string()))}>
                                                                    </span>
                                                                    <div>
                                                                        <strong>{tag.name}</strong>
                                                                        {tag.description.map(|desc| {
                                                                            view! { <div class="small text-muted">{desc}</div> }
                                                                        })}
                                                                    </div>
                                                                </div>
                                                            </td>
                                                            <td>
                                                                <code>{tag.color.unwrap_or_else(|| "#0d6efd".to_string())}</code>
                                                            </td>
                                                            <td>
                                                                {tag.topic_count.unwrap_or(0)}
                                                            </td>
                                                            <td>
                                                                {if tag.is_restricted {
                                                                    view! { <span class="badge bg-warning">"Yes"</span> }
                                                                } else {
                                                                    view! { <span class="badge bg-secondary">"No"</span> }
                                                                }}
                                                            </td>
                                                            <td>
                                                                <div class="btn-group btn-group-sm">
                                                                    <button class="btn btn-outline-primary"
                                                                            on:click=move |_| start_edit(tag_clone.clone())>
                                                                        <i class="bi bi-pencil"></i>
                                                                    </button>
                                                                    <button class="btn btn-outline-danger"
                                                                            on:click=move |_| handle_delete_tag(tag_id, tag_name.clone())>
                                                                        <i class="bi bi-trash"></i>
                                                                    </button>
                                                                </div>
                                                            </td>
                                                        </tr>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tbody>
                                        </table>
                                    </div>
                                }
                            }}
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

// Helper function for window
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}