use leptos::*;
use crate::models::forum::category::Category;
use crate::components::error_alert::ErrorAlert;

#[component]
pub fn CategoryList(
    course_id: Option<String>,
) -> impl IntoView {
    let (categories, set_categories) = create_signal(Vec::<Category>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    
    // Load categories
    let load_categories = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            let result = match course_id.as_ref() {
                Some(id) => {
                    invoke::<_, Vec<Category>>("list_categories_by_course", id).await
                },
                None => {
                    invoke::<_, Vec<Category>>("list_root_categories", &()).await
                }
            };
            
            match result {
                Ok(fetched_categories) => {
                    set_categories.set(fetched_categories);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load categories: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load categories on mount
    create_effect(move |_| {
        load_categories();
    });

    view! {
        <div class="category-list">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            {move || {
                if loading.get() {
                    view! { <div class="loading-state">"Loading categories..."</div> }
                } else if categories.get().is_empty() {
                    view! { 
                        <div class="empty-state">
                            <p>"No categories found"</p>
                            <a href="/categories/new" class="new-category-button">
                                "Create First Category"
                            </a>
                        </div> 
                    }
                } else {
                    view! {
                        <div class="categories