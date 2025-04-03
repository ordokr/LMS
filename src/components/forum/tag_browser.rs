use leptos::*;
use crate::models::forum::tag::Tag;
use crate::services::forum::ForumService;

#[component]
pub fn TagBrowser() -> impl IntoView {
    let (tags, set_tags) = create_signal(Vec::<Tag>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (filter, set_filter) = create_signal(String::new());
    
    // Load all tags
    create_effect(move |_| {
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
    });
    
    // Filter tags based on search input
    let filtered_tags = create_memo(move |_| {
        let query = filter().to_lowercase();
        
        if query.is_empty() {
            tags()
        } else {
            tags().into_iter()
                .filter(|tag| tag.name.to_lowercase().contains(&query))
                .collect()
        }
    });
    
    // Sort tags: popular first, then alphabetically
    let sorted_and_filtered_tags = create_memo(move |_| {
        let mut filtered = filtered_tags();
        filtered.sort_by(|a, b| {
            // First compare by topic count (desc)
            let count_cmp = b.topic_count.unwrap_or(0).cmp(&a.topic_count.unwrap_or(0));
            
            // If counts are equal, sort alphabetically
            if count_cmp == std::cmp::Ordering::Equal {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            } else {
                count_cmp
            }
        });
        filtered
    });

    view! {
        <div class="tag-browser">
            <div class="d-flex justify-content-between align-items-center mb-4">
                <h1>"Tags"</h1>
                <div class="search-bar">
                    <div class="input-group">
                        <span class="input-group-text"><i class="bi bi-search"></i></span>
                        <input
                            type="text"
                            class="form-control"
                            placeholder="Filter tags..."
                            prop:value=move || filter()
                            on:input=move |ev| set_filter.set(event_target_value(&ev))
                        />
                    </div>
                </div>
            </div>
            
            {move || if loading() {
                view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
            } else if let Some(err) = error() {
                view! { <div class="alert alert-danger">{err}</div> }
            } else if sorted_and_filtered_tags().is_empty() {
                view! {
                    <div class="text-center p-5">
                        <i class="bi bi-tags mb-3 d-block" style="font-size: 3rem;"></i>
                        <h3>"No tags found"</h3>
                        {if filter().is_empty() {
                            view! { <p class="text-muted">"No tags have been created yet."</p> }
                        } else {
                            view! { <p class="text-muted">"No tags match your search criteria."</p> }
                        }}
                    </div>
                }
            } else {
                view! {
                    <div class="row g-4">
                        {sorted_and_filtered_tags().into_iter().map(|tag| {
                            view! {
                                <div class="col-md-4 col-lg-3">
                                    <div class="card h-100">
                                        <div class="card-body">
                                            <a href={format!("/forum/tags/{}", tag.slug)} class="text-decoration-none">
                                                <h5 class="card-title d-flex align-items-center gap-2">
                                                    <span class="tag-color" style={format!("background-color: {}", tag.color.clone().unwrap_or_else(|| "#0d6efd".to_string()))}></span>
                                                    {tag.name.clone()}
                                                </h5>
                                            </a>
                                            {tag.description.clone().map(|desc| {
                                                view! { <p class="card-text text-muted small">{desc}</p> }
                                            })}
                                        </div>
                                        <div class="card-footer d-flex justify-content-between align-items-center bg-white">
                                            <small class="text-muted">
                                                {format!("{} topics", tag.topic_count.unwrap_or(0))}
                                            </small>
                                            <a href={format!("/forum/tags/{}", tag.slug)} class="btn btn-sm btn-outline-primary">
                                                "Browse"
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }
            }}
        </div>
    }
}