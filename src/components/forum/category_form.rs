use leptos::*;
use crate::models::forum::Category;
use crate::services::forum::ForumService;
use web_sys::SubmitEvent;
use chrono::Utc;

#[component]
pub fn CategoryForm(
    #[prop(optional)] category_id: Option<i64>,  // Change to i64 to match your model
) -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (color, set_color) = create_signal(String::from("#3498db"));  // Default blue color
    let (text_color, set_text_color) = create_signal(String::from("#ffffff"));  // Default white text
    let (saving, set_saving) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    
    // Load existing category if editing
    create_effect(move |_| {
        if let Some(id) = category_id {
            set_saving.set(true);
            
            // Update to use i64 ID
            spawn_local(async move {
                match ForumService::get_category(id).await {
                    Ok(category) => {
                        set_name.set(category.name);
                        set_description.set(category.description.unwrap_or_default());
                        
                        // Set colors if available
                        if let Some(cat_color) = category.color {
                            set_color.set(cat_color);
                        }
                        if let Some(cat_text_color) = category.text_color {
                            set_text_color.set(cat_text_color);
                        }
                        
                        set_saving.set(false);
                    },
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load category: {}", e)));
                        set_saving.set(false);
                    }
                }
            });
        }
    });
    
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_saving.set(true);
        set_error.set(None);
        
        // Generate a slug from the name
        let slug = name()
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        
        // Create Category object
        let category = Category {
            id: category_id.unwrap_or(0),  // ID will be assigned by server for new categories
            name: name(),
            slug,
            description: Some(description()),
            color: Some(color()),
            text_color: Some(text_color()),
            course_id: None,
            parent_id: None,
            topic_count: 0,
            post_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        spawn_local(async move {
            // Use the category object with the service
            let result = if category_id.is_some() {
                ForumService::update_category(&category).await
            } else {
                ForumService::create_category(&category).await
            };
            
            match result {
                Ok(_) => {
                    // Navigate back to categories list
                    let window = web_sys::window().unwrap();
                    let _ = window.location().set_href("/forum/categories");
                    set_saving.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Error saving category: {}", e)));
                    set_saving.set(false);
                }
            }
        });
    };
    
    view! {
        <div class="category-form card p-4">
            <h2 class="mb-4">{move || if category_id.is_some() { "Edit Category" } else { "Create New Category" }}</h2>
            
            {move || error().map(|err| view! {
                <div class="alert alert-danger">{err}</div>
            })}
            
            <form on:submit=on_submit>
                <div class="mb-3">
                    <label for="name" class="form-label">"Category Name"</label>
                    <input
                        type="text"
                        id="name"
                        class="form-control"
                        prop:value=move || name()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="mb-3">
                    <label for="description" class="form-label">"Description"</label>
                    <textarea
                        id="description"
                        class="form-control"
                        prop:value=move || description()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        rows="3"
                    ></textarea>
                </div>
                
                <div class="row mb-3">
                    <div class="col-md-6">
                        <label for="color" class="form-label">"Category Color"</label>
                        <div class="d-flex">
                            <input
                                type="color"
                                id="color"
                                class="form-control form-control-color me-2"
                                prop:value=move || color()
                                on:input=move |ev| set_color.set(event_target_value(&ev))
                            />
                            <input
                                type="text"
                                class="form-control"
                                prop:value=move || color()
                                on:input=move |ev| set_color.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                    <div class="col-md-6">
                        <label for="text_color" class="form-label">"Text Color"</label>
                        <div class="d-flex">
                            <input
                                type="color"
                                id="text_color"
                                class="form-control form-control-color me-2"
                                prop:value=move || text_color()
                                on:input=move |ev| set_text_color.set(event_target_value(&ev))
                            />
                            <input
                                type="text"
                                class="form-control"
                                prop:value=move || text_color()
                                on:input=move |ev| set_text_color.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                </div>
                
                <div class="mb-3">
                    <label class="form-label">"Preview"</label>
                    <div class="card p-2" style:background-color=move || color() style:color=move || text_color()>
                        <h5>{move || name()}</h5>
                        <p class="mb-0">{move || description()}</p>
                    </div>
                </div>
                
                <div class="d-flex gap-2">
                    <button 
                        type="submit" 
                        class="btn btn-primary"
                        disabled=move || saving()
                    >
                        {move || if saving() { "Saving..." } else { "Save Category" }}
                    </button>
                    <a href="/forum/categories" class="btn btn-outline-secondary">"Cancel"</a>
                </div>
            </form>
        </div>
    }
}

// In your ForumService implementation
pub async fn get_category(id: i64) -> Result<Category, String> {
    // Implementation
}

pub async fn create_category(category: &Category) -> Result<Category, String> {
    // Implementation
}

pub async fn update_category(category: &Category) -> Result<Category, String> {
    // Implementation
}