use leptos::*;
use leptos_router::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct Topic {
    id: i64,
    category_id: i64,
    title: String,
    content: Option<String>,
    // Add other fields like user_id, created_at
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct Post {
    id: i64,
    topic_id: i64,
    user_id: i64,
    content: String,
    // Add other fields like created_at
}

async fn fetch_topic(id: String) -> Result<Topic, reqwasm::Error> {
    let url = format!("/api/topics/{}", id); // Assuming API is mounted at /api
    reqwasm::http::Request::get(&url)
        .send()
        .await?
        .json()
        .await
}

async fn fetch_posts_for_topic(id: String) -> Result<Vec<Post>, reqwasm::Error> {
    let url = format!("/api/posts?topic_id={}", id); // Assuming API is mounted at /api
    reqwasm::http::Request::get(&url)
        .send()
        .await?
        .json()
        .await
}

#[component]
pub fn TopicPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let topic_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let topic = create_resource(cx,
        move || topic_id(),
        |id| async move { fetch_topic(id).await }
    );

    let posts = create_resource(cx,
        move || topic_id(),
        |id| async move { fetch_posts_for_topic(id).await }
    );

    view! { cx,
        <main>
            <Suspense fallback=view! { cx, <p>"Loading topic..."</p> }>
                {move || topic.read(cx).map(|result| match result {
                    Ok(top) => view! { cx,
                        <h1>{&top.title}</h1>
                        {top.content.map(|c| view! { cx, <p>{c}</p> })}
                    }.into_view(cx),
                    Err(e) => view! { cx, <p>"Error loading topic: " {e.to_string()}</p> }.into_view(cx),
                })}
            </Suspense>

            <h2>"Posts"</h2>
            <Suspense fallback=view! { cx, <p>"Loading posts..."</p> }>
                {move || posts.read(cx).map(|result| match result {
                    Ok(ps) => view! { cx,
                        <ul>
                            {ps.into_iter().map(|post| {
                                view! { cx,
                                    <li>
                                        <p>{&post.content}</p>
                                        // Add user info, timestamp, etc.
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_view(cx),
                    Err(e) => view! { cx, <p>"Error loading posts: " {e.to_string()}</p> }.into_view(cx),
                })}
            </Suspense>
            // Add a form here to create a new post
        </main>
    }
}
