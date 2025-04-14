use leptos::*;
use leptos_router::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct Category {
    id: i64,
    name: String,
    description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct Topic {
    id: i64,
    category_id: i64,
    title: String,
    content: Option<String>,
    // Add other fields as needed, e.g., user_id, created_at
}

async fn fetch_category(id: String) -> Result<Category, reqwasm::Error> {
    let url = format!("/api/categories/{}", id); // Assuming API is mounted at /api
    reqwasm::http::Request::get(&url)
        .send()
        .await?
        .json()
        .await
}

async fn fetch_topics_for_category(id: String) -> Result<Vec<Topic>, reqwasm::Error> {
    let url = format!("/api/topics?category_id={}", id); // Assuming API is mounted at /api
    reqwasm::http::Request::get(&url)
        .send()
        .await?
        .json()
        .await
}

#[component]
pub fn CategoryPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let category_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let category = create_resource(cx,
        move || category_id(),
        |id| async move { fetch_category(id).await }
    );

    let topics = create_resource(cx,
        move || category_id(),
        |id| async move { fetch_topics_for_category(id).await }
    );

    view! { cx,
        <main>
            <Suspense fallback=view! { cx, <p>"Loading category..."</p> }>
                {move || category.read(cx).map(|result| match result {
                    Ok(cat) => view! { cx,
                        <h1>{&cat.name}</h1>
                        {cat.description.map(|d| view! { cx, <p>{d}</p> })}
                    }.into_view(cx),
                    Err(e) => view! { cx, <p>"Error loading category: " {e.to_string()}</p> }.into_view(cx),
                })}
            </Suspense>

            <h2>"Topics"</h2>
            <Suspense fallback=view! { cx, <p>"Loading topics..."</p> }>
                {move || topics.read(cx).map(|result| match result {
                    Ok(tops) => view! { cx,
                        <ul>
                            {tops.into_iter().map(|topic| {
                                view! { cx,
                                    <li>
                                        <A href=format!("/topics/{}", topic.id)>
                                            {&topic.title}
                                        </A>
                                        // Optionally display brief content or metadata
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_view(cx),
                    Err(e) => view! { cx, <p>"Error loading topics: " {e.to_string()}</p> }.into_view(cx),
                })}
            </Suspense>
        </main>
    }
}
