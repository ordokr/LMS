use leptos::*;
use crate::models::forum::tag::Tag;
use crate::services::forum::ForumService;
use web_sys::KeyboardEvent;

#[component]
pub fn TagSelector(
    #[prop()] selected_tags: Signal<Vec<String>>,
    #[prop()] set_selected_tags: SignalSetter<Vec<String>>,
    #[prop(optional)] max_tags: Option<usize>,
    #[prop(optional)] allow_create: Option<bool>,
    #[prop(optional)] restricted_mode: Option<bool>,
) -> impl IntoView {
    let max_tags = max_tags.unwrap_or(5);
    let allow_create = allow_create.unwrap_or(true);
    let restricted_mode = restricted_mode.unwrap_or(false);
    
    let (available_tags, set_available_tags) = create_signal(Vec::<Tag>::new());
    let (filtered_tags, set_filtered_tags) = create_signal(Vec::<Tag>::new());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (input_value, set_input_value) = create_signal(String::new());
    let (dropdown_visible, set_dropdown_visible) = create_signal(false);
    
    // Load available tags on component mount
    create_effect(move |_| {
        set_loading.set(true);
        
        spawn_local(async move {
            match ForumService::get_tags().await {
                Ok(tags) => {
                    set_available_tags.set(tags);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load tags: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });
    
    // Filter tags based on input
    create_effect(move |_| {
        let query = input_value().to_lowercase();
        let selected = selected_tags();
        
        if query.trim().is_empty() {
            // Show all unselected tags
            set_filtered_tags.set(
                available_tags()
                    .into_iter()
                    .filter(|tag| !selected.contains(&tag.name))
                    .collect()
            );
        } else {
            // Filter by query
            set_filtered_tags.set(
                available_tags()
                    .into_iter()
                    .filter(|tag| {
                        tag.name.to_lowercase().contains(&query) && !selected.contains(&tag.name)
                    })
                    .collect()
            );
        }
    });
    
    // Check if a tag can be added
    let can_add_tag = move || {
        selected_tags().len() < max_tags
    };
    
    // Add a tag to selected tags
    let add_tag = move |tag_name: String| {
        if can_add_tag() {
            set_selected_tags.update(|tags| {
                // Check if already added
                if !tags.contains(&tag_name) {
                    tags.push(tag_name);
                }
            });
            set_input_value.set(String::new());
        }
    };
    
    // Remove a tag from selected tags
    let remove_tag = move |tag_name: String| {
        set_selected_tags.update(|tags| {
            tags.retain(|t| t != &tag_name);
        });
    };
    
    // Handle key press in input field
    let handle_key_press = move |e: KeyboardEvent| {
        let key = e.key();
        
        match key.as_str() {
            "Enter" => {
                e.prevent_default();
                
                let current_value = input_value().trim().to_string();
                if !current_value.is_empty() {
                    // Check if the tag exists
                    let tag_exists = available_tags()
                        .iter()
                        .any(|t| t.name.to_lowercase() == current_value.to_lowercase());
                    
                    if tag_exists {
                        // Add existing tag
                        let exact_tag = available_tags()
                            .iter()
                            .find(|t| t.name.to_lowercase() == current_value.to_lowercase())
                            .unwrap();
                        add_tag(exact_tag.name.clone());
                    } else if allow_create && !restricted_mode {
                        // Create new tag
                        add_tag(current_value);
                    }
                    set_input_value.set(String::new());
                }
            },
            "Backspace" => {
                if input_value().is_empty() && !selected_tags().is_empty() {
                    // Remove last tag when backspace is pressed in empty input
                    set_selected_tags.update(|tags| {
                        tags.pop();
                    });
                }
            },
            _ => {}
        }
    };
    
    // Handle input focus
    let handle_focus = move |_| {
        set_dropdown_visible.set(true);
    };
    
    // Handle input blur
    let handle_blur = move |_| {
        // Small delay to allow click events on dropdown items
        setTimeout(
            move || set_dropdown_visible.set(false),
            100
        );
    };

    view! {
        <div class="tag-selector">
            <div class="form-control d-flex flex-wrap align-items-center gap-2 p-2"
                 class:is-invalid=move || !error().is_none()>
                
                {move || selected_tags().iter().map(|tag| {
                    let tag_name = tag.clone();
                    let tag_color = available_tags()
                        .iter()
                        .find(|t| t.name == tag_name)
                        .map(|t| t.color.clone().unwrap_or_else(|| "#0d6efd".to_string()))
                        .unwrap_or_else(|| "#0d6efd".to_string());
                    
                    view! {
                        <div class="tag-badge d-flex align-items-center" 
                             style={format!("background-color: {}; color: white;", tag_color)}>
                            <span class="me-1">{tag_name.clone()}</span>
                            <button type="button" class="btn-close btn-close-white btn-sm" 
                                    on:click=move |_| remove_tag(tag_name.clone())
                                    aria-label="Remove tag">
                            </button>
                        </div>
                    }
                }).collect::<Vec<_>>()}
                
                <input
                    type="text"
                    class="tag-input flex-grow-1 border-0"
                    placeholder=move || if can_add_tag() { 
                        "Add tags..."
                    } else { 
                        "Max tags reached"
                    }
                    prop:value=input_value
                    on:input=move |ev| set_input_value.set(event_target_value(&ev))
                    on:keydown=handle_key_press
                    on:focus=handle_focus
                    on:blur=handle_blur
                    disabled=move || !can_add_tag()
                />
            </div>
            
            {move || if let Some(err) = error() {
                view! { <div class="invalid-feedback">{err}</div> }
            } else {
                view! {}
            }}
            
            {move || if dropdown_visible() && !filtered_tags().is_empty() {
                view! {
                    <div class="tag-dropdown position-absolute bg-white border rounded shadow-sm mt-1 z-index-dropdown">
                        <ul class="list-group list-group-flush">
                            {filtered_tags().iter().map(|tag| {
                                let tag_name = tag.name.clone();
                                let tag_color = tag.color.clone().unwrap_or_else(|| "#0d6efd".to_string());
                                
                                view! {
                                    <li class="list-group-item list-group-item-action" on:mousedown=move |_| add_tag(tag_name.clone())>
                                        <div class="d-flex align-items-center">
                                            <span class="tag-color-dot me-2" 
                                                  style={format!("background-color: {}", tag_color)}>
                                            </span>
                                            <span class="tag-name">{tag_name.clone()}</span>
                                            {tag.topic_count.map(|count| {
                                                view! {
                                                    <small class="text-muted ms-2">
                                                        {"("}{count}{" topics)"}
                                                    </small>
                                                }
                                            })}
                                        </div>
                                        {tag.description.clone().map(|desc| {
                                            view! { <small class="text-muted d-block">{desc}</small> }
                                        })}
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                }
            } else {
                view! {}
            }}
            
            <small class="form-text">
                {move || format!("{}/{} tags", selected_tags().len(), max_tags)}
            </small>
        </div>
    }
}

// Helper timeout function
fn setTimeout<F>(f: F, ms: i32) 
where
    F: FnOnce() + 'static,
{
    use wasm_bindgen::prelude::*;
    
    let window = web_sys::window().unwrap();
    let closure = Closure::once(f);
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        ms,
    );
    closure.forget(); // Prevent memory leak
}