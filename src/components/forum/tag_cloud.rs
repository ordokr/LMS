use leptos::*;
use leptos_router::use_navigate;
use crate::models::forum::tag::Tag;
use crate::services::forum::ForumService;

#[component]
pub fn TagCloud(
    #[prop(optional)] max_tags: Option<usize>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] on_tag_click: Option<Callback<Tag>>,
) -> impl IntoView {
    let (tags, set_tags) = create_signal(Vec::<Tag>::new());
    let (loading, set_loading) = create_signal(true);
    let navigate = use_navigate();
    
    // Load popular tags
    create_effect(move |_| {
        set_loading.set(true);
        
        spawn_local(async move {
            match ForumService::get_popular_tags(max_tags.unwrap_or(20)).await {
                Ok(popular_tags) => {
                    set_tags.set(popular_tags);
                },
                Err(_) => {
                    // Silently fail, not critical
                }
                
                set_loading.set(false);
            }
        });
    });
    
    let handle_tag_click = move |tag: Tag| {
        if let Some(callback) = on_tag_click.as_ref() {
            callback.call(tag);
        } else {
            // Default behavior - navigate to tag detail page
            navigate(&format!("/forum/tags/{}", tag.id), Default::default());
        }
    };
    
    view! {
        <div class="tag-cloud">
            <h5 class="mb-3">{title.unwrap_or_else(|| "Popular Tags".to_string())}</h5>
            
            {move || if loading() {
                view! { <div class="d-flex justify-content-center"><div class="spinner-border spinner-border-sm" role="status"></div></div> }
            } else if tags().is_empty() {
                view! { <p class="text-muted small">"No tags available"</p> }
            } else {
                view! {
                    <div class="d-flex flex-wrap gap-2">
                        {tags().into_iter().map(|tag| {
                            let tag_clone = tag.clone();
                            view! {
                                <a 
                                    href="javascript:void(0)" 
                                    class="tag-item d-inline-flex align-items-center"
                                    style={format!("background-color: {}; color: white;", 
                                        tag.color.clone().unwrap_or_else(|| "#6c757d".to_string()))}
                                    on:click=move |_| handle_tag_click(tag_clone.clone())
                                >
                                    {tag.icon.clone().map(|icon| {
                                        view! { <i class={format!("bi bi-{} me-1", icon)}></i> }
                                    })}
                                    <span>{tag.name.clone()}</span>
                                    <span class="badge bg-light text-dark ms-1">{tag.topic_count.unwrap_or(0)}</span>
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            }}
        </div>
    }
}