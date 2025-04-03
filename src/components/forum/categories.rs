use leptos::*;
use leptos_router::*;
use web_sys::MouseEvent;

use crate::models::forum::Category;
use crate::services::forum_service::ForumService;
use crate::utils::errors::ApiError;

#[component]
pub fn ForumCategories(cx: Scope) -> impl IntoView {
    let (categories, set_categories) = create_signal(cx, Vec::<Category>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);

    // Load categories when the component mounts
    create_effect(cx, move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            match ForumService::get_categories().await {
                Ok(data) => {
                    set_categories.set(data);
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(err.to_string()));
                    set_loading.set(false);
                }
            }
        });
    });

    view! { cx,
        <div class="forum-categories">
            <h1>"Forum Categories"</h1>
            
            {move || {
                if loading.get() {
                    view! { cx, <div class="loading">"Loading categories..."</div> }
                } else if let Some(err) = error.get() {
                    view! { cx, <div class="error">{err}</div> }
                } else if categories.get().is_empty() {
                    view! { cx, <div class="empty-state">"No categories found."</div> }
                } else {
                    view! { cx,
                        <div class="categories-list">
                            {categories.get()
                                .into_iter()
                                .map(|category| {
                                    let id = category.id.unwrap_or(0);
                                    view! { cx,
                                        <div class="category-card">
                                            <h2 class="category-name">
                                                <A href=format!("/forum/category/{}", id)>
                                                    {category.name.clone()}
                                                </A>
                                            </h2>
                                            
                                            {if let Some(desc) = category.description {
                                                view! { cx, <div class="category-description">{desc}</div> }
                                            } else {
                                                view! { cx, <div class="category-description empty">"No description"</div> }
                                            }}
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                            }
                        </div>
                    }
                }
            }}
        </div>
    }
}