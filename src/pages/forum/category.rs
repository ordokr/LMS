use leptos::*;
use crate::api::forum::{Category, get_category};
use crate::components::forum::topic_list::TopicList;

#[component]
pub fn CategoryPage() -> impl IntoView {
    // Get category ID from URL
    let params = use_params_map();
    let category_id = move || {
        params.with(|p| p.get("id").and_then(|id| id.parse::<i64>().ok()))
    };
    
    // Resource for category details
    let category = create_resource(
        move || category_id(),
        move |id| async move {
            match id {
                Some(id) => get_category(id).await.ok(),
                None => None,
            }
        }
    );

    view! {
        <div class="category-page">
            {move || match category_id() {
                None => view! { <p>"Invalid category ID"</p> },
                Some(_) => {
                    view! {
                        {move || match category.get() {
                            None => view! { <p>"Loading category..."</p> },
                            Some(None) => view! { <p>"Category not found"</p> },
                            Some(Some(cat)) => {
                                view! {
                                    <div class="category-header">
                                        <h1>{cat.name}</h1>
                                        {cat.description.map(|desc| view! { <p>{desc}</p> })}
                                        <a href="/forum">"Back to Forum Home"</a>
                                    </div>
                                }
                            }
                        }}
                        
                        <TopicList category_id={category_id()} />
                    }
                }
            }}
        </div>
    }
}