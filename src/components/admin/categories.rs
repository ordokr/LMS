use leptos::*;
use crate::models::Category;
use crate::components::auth::AuthData;

#[component]
pub fn AdminCategories(cx: Scope) -> impl IntoView {
    // Auth check to ensure admin access
    let auth_data = use_context::<Signal<Option<AuthData>>>(cx)
        .expect("Auth data not provided");
    
    // Create signals for categories
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);
    let (categories, set_categories) = create_signal(cx, Vec::<Category>::new());
    
    // Form signals
    let (show_form, set_show_form) = create_signal(cx, false);
    let (edit_id, set_edit_id) = create_signal(cx, None::<i64>);
    let (name, set_name) = create_signal(cx, String::new());
    let (slug, set_slug) = create_signal(cx, String::new());
    let (description, set_description) = create_signal(cx, String::new());
    let (color, set_color) = create_signal(cx, String::from("#3498DB"));
    let (text_color, set_text_color) = create_signal(cx, String::from("#FFFFFF"));
    let (form_error, set_form_error) = create_signal(cx, None::<String>);
    
    // Function to load categories
    let load_categories = move || {
        if let Some(data) = auth_data.get() {
            spawn_local(async move {
                set_loading.set(true);
                set_error.set(None);
                
                // Fetch categories from API
                let client = reqwest::Client::new();
                let response = client.get("http://localhost:3030/categories")
                    .header("Authorization", format!("Bearer {}", data.token))
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            if let Ok(category_list) = resp.json::<Vec<Category>>().await {
                                set_categories.set(category_list);
                            } else {
                                set_error.set(Some("Failed to parse categories".to_string()));
                            }
                        } else {
                            set_error.set(Some(format!("Failed to fetch categories: {}", resp.status())));
                        }
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Request error: {}", e)));
                    }
                }
                
                set_loading.set(false);
            });
        }
    };
    
    // Initial load
    create_effect(cx, move |_| {
        // Make sure user is admin
        if let Some(data) = auth_data.get() {
            if !data.user.is_admin {
                // Redirect non-admins
                let window = web_sys::window().unwrap();
                let _ = window.location().set_href("/");
                return;
            }
            
            load_categories();
        } else {
            // User not logged in, redirect to login
            let window = web_sys::window().unwrap();
            let _ = window.location().set_href("/login");
        }
    });
    
    // Function to edit a category
    let edit_category = create_callback(cx, move |id: i64| {
        let categories = categories.get();
        if let Some(category) = categories.iter().find(|c| c.id == Some(id)) {
            set_edit_id.set(Some(id));
            set_name.set(category.name.clone());
            set_slug.set(category.slug.clone());
            set_description.set(category.description.clone().unwrap_or_default());
            set_color.set(category.color.clone().unwrap_or_else(|| "#3498DB".to_string()));
            set_text_color.set(category.text_color.clone().unwrap_or_else(|| "#FFFFFF".to_string()));
            set_show_form.set(true);
        }
    });
    
    // Function to create a new category or reset form
    let new_category = move |_| {
        set_edit_id.set(None);
        set_name.set(String::new());
        set_slug.set(String::new());
        set_description.set(String::new());
        set_color.set(String::from("#3498DB"));
        set_text_color.set(String::from("#FFFFFF"));
        set_show_form.set(true);
    };
    
    // Cancel form
    let cancel_form = move |_| {
        set_show_form.set(false);
        set_form_error.set(None);
    };
    
    // Handle form submission
    let save_category = create_action(cx, move |_: &()| {
        let current_name = name.get();
        let current_description = description.get();
        let current_color = color.get();
        let current_text_color = text_color.get();
        let current_slug = if slug.get().is_empty() {
            current_name.to_lowercase().replace(" ", "-")
        } else {
            slug.get()
        };
        let current_edit_id = edit_id.get();
        
        async move {
            if current_name.trim().is_empty() {
                set_form_error.set(Some("Category name is required".to_string()));
                return false;
            }
            
            let auth = auth_data.get();
            if auth.is_none() {
                return false;
            }
            
            let token = auth.unwrap().token;
            let client = reqwest::Client::new();
            
            // Prepare the payload
            let payload = serde_json::json!({
                "name": current_name,
                "slug": current_slug,
                "description": current_description,
                "color": current_color,
                "text_color": current_text_color
            });
            
            let response = if let Some(id) = current_edit_id {
                // Update existing category
                client.put(&format!("http://localhost:3030/categories/{}", id))
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&payload)
                    .send()
                    .await
            } else {
                // Create new category
                client.post("http://localhost:3030/categories")
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&payload)
                    .send()
                    .await
            };
            
            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        // Reset form and reload categories
                        set_show_form.set(false);
                        load_categories();
                        true
                    } else {
                        let error_text = resp.text().await.unwrap_or_else(|_| "Failed to save category".to_string());
                        set_form_error.set(Some(error_text));
                        false
                    }
                },
                Err(e) => {
                    set_form_error.set(Some(format!("Request failed: {}", e)));
                    false
                }
            }
        }
    });
    
    // Handle category deletion
    let delete_category = create_action(cx, move |id: &i64| {
        let category_id = *id;
        
        async move {
            let auth = auth_data.get();
            if auth.is_none() {
                return false;
            }
            
            let token = auth.unwrap().token;
            let client = reqwest::Client::new();
            
            let response = client.delete(&format!("http://localhost:3030/categories/{}", category_id))
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await;
                
            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        // Reload categories
                        load_categories();
                        true
                    } else {
                        let error_text = resp.text().await.unwrap_or_else(|_| "Failed to delete category".to_string());
                        set_error.set(Some(error_text));
                        false
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Request failed: {}", e)));
                    false
                }
            }
        }
    });
    
    // Confirm category deletion
    let confirm_delete = move |id: i64| {
        // In a real app, you might want to show a confirmation dialog
        if window().confirm_with_message("Are you sure you want to delete this category? This will delete all topics and posts within it.").unwrap_or(false) {
            delete_category.dispatch(id);
        }
    };
    
    view! { cx,
        <div class="admin-categories">
            <div class="section-header">
                <h1>"Category Management"</h1>
                <button class="button primary" on:click=new_category>
                    "Add Category"
                </button>
            </div>
            
            {move || if let Some(err) = error.get() {
                view! { cx, <div class="error-message">{err}</div> }.into_view(cx)
            } else { view! {}.into_view(cx) }}
            
            {move || if loading.get() {
                view! { cx, <div class="loading">"Loading categories..."</div> }.into_view(cx)
            } else {
                view! {}.into_view(cx)
            }}
            
            {move || if show_form.get() {
                view! { cx,
                    <div class="admin-form">
                        <h2>{if edit_id.get().is_some() { "Edit Category" } else { "New Category" }}</h2>
                        
                        {move || if let Some(err) = form_error.get() {
                            view! { cx, <div class="error-message">{err}</div> }.into_view(cx)
                        } else { view! {}.into_view(cx) }}
                        
                        <div class="form-row">
                            <div class="form-col">
                                <div class="form-group">
                                    <label for="category-name">"Name:"</label>
                                    <input
                                        id="category-name"
                                        type="text"
                                        value=move || name.get()
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                        required
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label for="category-slug">"Slug (URL-friendly name):"</label>
                                    <input
                                        id="category-slug"
                                        type="text"
                                        value=move || slug.get()
                                        on:input=move |ev| set_slug.set(event_target_value(&ev))
                                        placeholder="leave-blank-to-autogenerate"
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label for="category-description">"Description:"</label>
                                    <textarea
                                        id="category-description"
                                        value=move || description.get()
                                        on:input=move |ev| set_description.set(event_target_value(&ev))
                                        rows="3"
                                    ></textarea>
                                </div>
                            </div>
                            
                            <div class="form-col">
                                <div class="form-group">
                                    <label for="category-color">"Background Color:"</label>
                                    <div class="color-picker">
                                        <input
                                            id="category-color"
                                            type="color"
                                            value=move || color.get()
                                            on:input=move |ev| set_color.set(event_target_value(&ev))
                                        />
                                        <input
                                            type="text"
                                            value=move || color.get()
                                            on:input=move |ev| set_color.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                                
                                <div class="form-group">
                                    <label for="category-text-color">"Text Color:"</label>
                                    <div class="color-picker">
                                        <input
                                            id="category-text-color"
                                            type="color"
                                            value=move || text_color.get()
                                            on:input=move |ev| set_text_color.set(event_target_value(&ev))
                                        />
                                        <input
                                            type="text"
                                            value=move || text_color.get()
                                            on:input=move |ev| set_text_color.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                                
                                <div class="form-group">
                                    <label>"Preview:"</label>
                                    <div class="category-preview" 
                                         style={move || format!("background-color: {}; color: {}", color.get(), text_color.get())}>
                                        <span>{move || name.get()}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        <div class="form-footer">
                            <button type="button" class="button secondary" on:click=cancel_form>
                                "Cancel"
                            </button>
                            <button type="button" class="button primary" on:click=move |_| save_category.dispatch(())
                                   disabled=move || save_category.pending().get()>
                                {if edit_id.get().is_some() { "Update" } else { "Create" }}
                            </button>
                        </div>
                    </div>
                }.into_view(cx)
            } else {
                view! {}.into_view(cx)
            }}
            
            <table class="admin-table">
                <thead>
                    <tr>
                        <th>"ID"</th>
                        <th>"Name"</th>
                        <th>"Slug"</th>
                        <th>"Description"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let categories_list = categories.get();
                        if categories_list.is_empty() && !loading.get() {
                            view! { cx,
                                <tr>
                                    <td colspan="5" style="text-align: center">
                                        "No categories found. Create one to get started."
                                    </td>
                                </tr>
                            }.into_view(cx)
                        } else {
                            categories_list.iter().map(|category| {
                                let id = category.id.unwrap_or(0);
                                
                                view! { cx,
                                    <tr>
                                        <td>{id}</td>
                                        <td>
                                            <div class="category-name-cell"
                                                 style={format!("background-color: {}; color: {}", 
                                                              category.color.clone().unwrap_or_else(|| "#3498DB".to_string()),
                                                              category.text_color.clone().unwrap_or_else(|| "#FFFFFF".to_string()))}>
                                                {&category.name}
                                            </div>
                                        </td>
                                        <td>{&category.slug}</td>
                                        <td>{category.description.clone().unwrap_or_else(|| "-".to_string())}</td>
                                        <td>
                                            <div class="table-actions">
                                                <button class="edit-button" 
                                                        on:click=move |_| edit_category.call(id)
                                                        title="Edit Category">
                                                    "✏️"
                                                </button>
                                                <button class="delete-button"
                                                        on:click=move |_| confirm_delete(id)
                                                        title="Delete Category">
                                                    "🗑️"
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                }
                            }).collect_view(cx)
                        }
                    }}
                </tbody>
            </table>
        </div>
    }
}