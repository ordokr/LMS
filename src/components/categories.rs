use leptos::*;
use crate::models::forum::Category;
use crate::services::forum_service::ForumService;
use crate::utils::errors::ApiError;

#[component]
pub fn ForumCategories(cx: Scope) -> impl IntoView {
    let forum_service = use_context::<ForumService>(cx)
        .expect("ForumService should be provided");
    
    let categories = create_resource(
        cx,
        || (), 
        move |_| async move {
            forum_service.get_categories().await
        }
    );

    view! { cx,
        <div class="categories-container">
            <h1>"Forum Categories"</h1>
            
            <div class="categories-list">
                {move || match categories.read(cx) {
                    None => view! { cx, <p>"Loading categories..."</p> },
                    Some(Ok(cats)) => {
                        if cats.is_empty() {
                            view! { cx, <p class="empty-message">"No categories found"</p> }
                        } else {
                            view! { cx,
                                <ul class="category-list">
                                    {cats.into_iter().map(|cat| view! { cx, 
                                        <li class="category-item">
                                            <a href={format!("/forum/c/{}", cat.id)}>
                                                <div class="category-header">
                                                    <h2 class="category-name">{cat.name}</h2>
                                                    {move || match cat.color {
                                                        Some(color) => view! { cx, 
                                                            <span class="category-color" style={format!("background-color: {}", color)}></span>
                                                        },
                                                        None => view! { cx, <></> }
                                                    }}
                                                </div>
                                                <div class="category-description">
                                                    {cat.description.unwrap_or_default()}
                                                </div>
                                                <div class="category-stats">
                                                    <span class="topic-count">{cat.topic_count} " topics"</span>
                                                    <span class="post-count">{cat.post_count} " posts"</span>
                                                </div>
                                            </a>
                                        </li>
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }
                        }
                    },
                    Some(Err(e)) => view! { cx, 
                        <div class="error-message">
                            <p>"Error loading categories: " {e.to_string()}</p>
                            <button on:click=move |_| categories.refetch()>
                                "Try Again"
                            </button>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

// Component to create a new category (for admin users)
#[component]
pub fn CategoryForm(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (slug, set_slug) = create_signal(cx, String::new());
    let (description, set_description) = create_signal(cx, String::new());
    let (error, set_error) = create_signal(cx, None::<String>);
    let (success, set_success) = create_signal(cx, false);
    
    let create_category = create_action(cx, move |_: &()| {
        let name = name.get();
        let slug = slug.get();
        let description = description.get();
        
        async move {
            // Basic validation
            if name.trim().is_empty() {
                set_error.set(Some("Category name is required".to_string()));
                return false;
            }
            
            if slug.trim().is_empty() {
                set_error.set(Some("Category slug is required".to_string()));
                return false;
            }
            
            // Clear any previous errors
            set_error.set(None);
            
            // Create the category payload
            let payload = serde_json::json!({
                "name": name,
                "slug": slug,
                "description": description.trim().to_string(),
            });
            
            // Send the request to create a category
            let client = reqwest::Client::new();
            let response = client.post("http://localhost:3030/categories")
                .json(&payload)
                .send()
                .await;
                
            match response {
                Ok(resp) if resp.status().is_success() => {
                    set_success.set(true);
                    set_name.set(String::new());
                    set_slug.set(String::new());
                    set_description.set(String::new());
                    true
                },
                Ok(resp) => {
                    let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    set_error.set(Some(error_text));
                    false
                },
                Err(e) => {
                    set_error.set(Some(format!("Request failed: {}", e)));
                    false
                }
            }
        }
    });
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        create_category.dispatch(());
    };
    
    // Auto-generate slug from name
    create_effect(cx, move |_| {
        let current_name = name.get();
        if !current_name.is_empty() {
            let auto_slug = current_name
                .to_lowercase()
                .replace(" ", "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            set_slug.set(auto_slug);
        }
    });

    view! { cx,
        <div class="category-form">
            <h1>"Create New Category"</h1>
            
            {move || error.get().map(|err| view! { cx, <div class="error-message">{err}</div> })}
            
            {move || {
                if success.get() {
                    view! { cx, <div class="success-message">"Category created successfully!"</div> }.into_view(cx)
                } else {
                    view! {}.into_view(cx)
                }
            }}
            
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="category-name">"Category Name:"</label>
                    <input
                        id="category-name"
                        type="text"
                        value=move || name.get()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="category-slug">"Slug:"</label>
                    <input
                        id="category-slug"
                        type="text"
                        value=move || slug.get()
                        on:input=move |ev| set_slug.set(event_target_value(&ev))
                        required
                    />
                    <small>"Used in URLs, auto-generated from name"</small>
                </div>
                
                <div class="form-group">
                    <label for="category-description">"Description:"</label>
                    <textarea
                        id="category-description"
                        value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        rows="4"
                    ></textarea>
                </div>
                
                <div class="form-actions">
                    <button type="submit" disabled=move || create_category.pending().get()>
                        {move || if create_category.pending().get() { "Creating..." } else { "Create Category" }}
                    </button>
                    <a href="/categories" class="button secondary">
                        "Cancel"
                    </a>
                </div>
            </form>
        </div>
    }
}

// Component to display a single category and its topics
#[component]
pub fn CategoryDetail(cx: Scope, category_id: i64) -> impl IntoView {
    // Create a resource to fetch the category
    let category = create_resource(
        cx,
        move || category_id,
        move |id| async move {
            // Fetch category from our API
            let response = reqwest::get(&format!("http://localhost:3030/categories/{}", id))
                .await
                .expect("Failed to fetch category");
                
            if response.status().is_success() {
                response.json::<Category>().await.ok()
            } else {
                None
            }
        },
    );

    view! { cx,
        <div class="category-detail">
            {move || match category.read(cx) {
                None => view! { cx, <p>"Loading category..."</p> }.into_view(cx),
                Some(None) => view! { cx, <p>"Category not found."</p> }.into_view(cx),
                Some(Some(category)) => {
                    view! { cx,
                        <div class="category-header" style={"background-color: " + &category.color.clone().unwrap_or_else(|| "#3498DB".to_string())}>
                            <h1 style={format!("color: {}", category.text_color.clone().unwrap_or_else(|| "#FFFFFF".to_string()))}>
                                {&category.name}
                            </h1>
                            <p>{category.description.clone().unwrap_or_else(|| "".to_string())}</p>
                        </div>
                        
                        // Display the list of topics in this category
                        <TopicsList category_id={category_id} />
                    }.into_view(cx)
                }
            }}
        </div>
    }
}