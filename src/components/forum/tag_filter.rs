use leptos::*;
use crate::models::forum::tag::Tag;
use crate::services::forum::ForumService;

#[component]
pub fn TagFilter(
    #[prop(into)] selected_tags: Signal<Vec<String>>,
    #[prop(into)] on_change: Callback<Vec<String>>,
) -> impl IntoView {
    let (tags, set_tags) = create_signal(Vec::<Tag>::new());
    let (loading, set_loading) = create_signal(true);
    let (filter, set_filter) = create_signal(String::new());
    
    // Load tags
    create_effect(move |_| {
        set_loading.set(true);
        
        spawn_local(async move {
            match ForumService::get_tags().await {
                Ok(all_tags) => {
                    set_tags.set(all_tags);
                },
                Err(_) => {
                    // Silently fail, not critical
                }
                
                set_loading.set(false);
            }
        });
    });
    
    // Filtered tags based on search term
    let filtered_tags = create_memo(move |_| {
        let search_term = filter().to_lowercase();
        if search_term.is_empty() {
            tags()
        } else {
            tags()
                .into_iter()
                .filter(|tag| tag.name.to_lowercase().contains(&search_term))
                .collect::<Vec<_>>()
        }
    });
    
    // Toggle tag selection
    let toggle_tag = move |tag_name: String| {
        let mut current = selected_tags.get();
        
        if current.contains(&tag_name) {
            current.retain(|t| t != &tag_name);
        } else {
            current.push(tag_name);
        }
        
        on_change.call(current);
    };
    
    view! {
        <div class="tag-filter">
            <div class="mb-3">
                <label class="form-label">"Filter by tags"</label>
                <div class="input-group mb-2">
                    <span class="input-group-text"><i class="bi bi-search"></i></span>
                    <input
                        type="text"
                        class="form-control"
                        placeholder="Search tags..."
                        prop:value=move || filter()
                        on:input=move |ev| set_filter.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="selected-tags mb-2">
                    {move || if selected_tags.get().is_empty() {
                        view! { <small class="text-muted">"No tags selected"</small> }
                    } else {
                        view! {
                            <div class="d-flex flex-wrap gap-1">
                                {selected_tags.get().iter().map(|selected_tag| {
                                    let tag_name = selected_tag.clone();
                                    let tag_data = tags().into_iter().find(|t| t.name == tag_name);
                                    
                                    view! {
                                        <span 
                                            class="badge d-flex align-items-center"
                                            style={format!("background-color: {};", 
                                                tag_data.as_ref().and_then(|t| t.color.clone()).unwrap_or_else(|| "#6c757d".to_string()))}
                                        >
                                            {tag_name.clone()}
                                            <button 
                                                type="button"
                                                class="btn-close btn-close-white ms-2"
                                                aria-label="Remove"
                                                on:click=move |_| toggle_tag(tag_name.clone())
                                            ></button>
                                        </span>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }}
                </div>
                
                <div class="tag-list" style="max-height: 200px; overflow-y: auto;">
                    {move || if loading() {
                        view! { <div class="text-center p-2"><div class="spinner-border spinner-border-sm" role="status"></div></div> }
                    } else if filtered_tags().is_empty() {
                        view! { <p class="text-muted small">"No tags found"</p> }
                    } else {
                        view! {
                            <div class="list-group list-group-flush">
                                {filtered_tags().into_iter().map(|tag| {
                                    let tag_name = tag.name.clone();
                                    let is_selected = create_memo(move |_| selected_tags.get().contains(&tag_name));
                                    
                                    view! {
                                        <a 
                                            href="javascript:void(0)"
                                            class="list-group-item list-group-item-action py-2"
                                            class:active=move || is_selected()
                                            on:click=move |_| toggle_tag(tag_name.clone())
                                        >
                                            <div class="d-flex align-items-center">
                                                <span 
                                                    class="tag-color-dot me-2"
                                                    style={format!("background-color: {};", tag.color.unwrap_or_else(|| "#6c757d".to_string()))}
                                                ></span>
                                                <div class="tag-name">{&tag.name}</div>
                                                <span class="badge bg-light text-dark ms-auto">{tag.topic_count.unwrap_or(0)}</span>
                                            </div>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}