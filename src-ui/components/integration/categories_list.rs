use leptos::*;
use crate::models::integration::DiscourseCategory;

#[component]
pub fn CategoriesList(
    categories: Signal<Vec<DiscourseCategory>>,
) -> impl IntoView {
    // Pagination state
    let (page, set_page) = create_signal(0);
    let (rows_per_page, set_rows_per_page) = create_signal(6);
    
    // Handle page change
    let handle_page_change = move |new_page: usize| {
        set_page.set(new_page);
    };
    
    // Handle rows per page change
    let handle_rows_per_page_change = move |new_rows: usize| {
        set_rows_per_page.set(new_rows);
        set_page.set(0);
    };
    
    view! {
        <div class="categories-list-container">
            {move || if categories().is_empty() {
                view! {
                    <div class="empty-list">
                        <p>"No categories found. Categories will appear here once they are synchronized with Discourse."</p>
                    </div>
                }
            } else {
                view! {
                    <div class="categories-grid">
                        {move || {
                            let start = page() * rows_per_page();
                            let end = start + rows_per_page();
                            
                            categories()
                                .iter()
                                .skip(start)
                                .take(rows_per_page())
                                .map(|category| {
                                    view! {
                                        <div class="category-card" style=format!("border-color: {};", category.color.clone().unwrap_or_else(|| "#1976d2".to_string()))>
                                            <div class="category-header" style=format!("background-color: {};", category.color.clone().unwrap_or_else(|| "#1976d2".to_string()))>
                                                <span class="category-icon">
                                                    <i class="icon-folder"></i>
                                                </span>
                                                <h3 class="category-name">{&category.name}</h3>
                                            </div>
                                            
                                            <div class="category-content">
                                                <p class="category-description">
                                                    {if let Some(desc) = &category.description {
                                                        if !desc.is_empty() {
                                                            desc.clone()
                                                        } else {
                                                            "No description".to_string()
                                                        }
                                                    } else {
                                                        "No description".to_string()
                                                    }}
                                                </p>
                                                
                                                <div class="category-meta">
                                                    <span class="category-topics">
                                                        <i class="icon-topic"></i>
                                                        {category.topic_count.unwrap_or(0)}
                                                        " topics"
                                                    </span>
                                                    
                                                    {if let Some(permission) = &category.permissions {
                                                        view! {
                                                            <span class="category-permission">{permission}</span>
                                                        }
                                                    } else {
                                                        view! { <></> }
                                                    }}
                                                    
                                                    {if category.parent_id.is_some() {
                                                        view! {
                                                            <span class="category-subcategory-badge">"Sub-category"</span>
                                                        }
                                                    } else {
                                                        view! { <></> }
                                                    }}
                                                    
                                                    <span class=format!("status-badge status-{}", get_status_class(&category.sync_status.clone().unwrap_or_else(|| "Unknown".to_string())))>
                                                        {&category.sync_status.clone().unwrap_or_else(|| "Unknown".to_string())}
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                    
                    <div class="pagination-controls">
                        <div class="rows-per-page">
                            <span>"Categories per page:"</span>
                            <select 
                                on:change=move |ev| {
                                    if let Ok(value) = event_target_value(&ev).parse::<usize>() {
                                        handle_rows_per_page_change(value);
                                    }
                                }
                            >
                                <option value="6" selected={rows_per_page() == 6}>"6"</option>
                                <option value="12" selected={rows_per_page() == 12}>"12"</option>
                                <option value="24" selected={rows_per_page() == 24}>"24"</option>
                            </select>
                        </div>
                        
                        <div class="page-navigation">
                            <button 
                                class="btn btn-sm"
                                disabled={page() == 0}
                                on:click=move |_| {
                                    if page() > 0 {
                                        handle_page_change(page() - 1);
                                    }
                                }
                            >
                                "Previous"
                            </button>
                            
                            <span class="page-info">
                                {"Page "}{page() + 1}{" of "}{(categories().len() + rows_per_page() - 1) / rows_per_page()}
                            </span>
                            
                            <button
                                class="btn btn-sm"
                                disabled={page() >= (categories().len() - 1) / rows_per_page()}
                                on:click=move |_| {
                                    if page() < (categories().len() - 1) / rows_per_page() {
                                        handle_page_change(page() + 1);
                                    }
                                }
                            >
                                "Next"
                            </button>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

// Helper function to determine class name based on sync status
fn get_status_class(status: &str) -> &'static str {
    match status.to_lowercase().as_str() {
        "synced" => "success",
        "pending" => "warning",
        "error" => "error",
        "local only" => "default",
        _ => "default",
    }
}

// Helper to get value from an event
fn event_target_value(ev: &web_sys::Event) -> String {
    let target: web_sys::EventTarget = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}
