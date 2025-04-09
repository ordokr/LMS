use leptos::*;
use leptos_router::*;
use crate::api::forum::Topic;
use crate::api::forum_server::{get_topic_handler, delete_topic_handler};

#[component]
pub fn TopicView(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let navigate = use_navigate(cx);
    
    let (topic, set_topic) = create_signal(cx, None::<Topic>);
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);

    create_effect(cx, move |_| {
        let id_str = id();
        if let Ok(id_val) = id_str.parse::<i64>() {
            spawn_local(async move {
                match get_topic_handler(id_val).await {
                    Ok(topic_data) => {
                        set_topic.set(Some(topic_data));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load topic: {}", e)));
                        log::error!("Failed to load topic: {}", e);
                    }
                }
                set_loading.set(false);
            });
        } else {
            set_error.set(Some("Invalid topic ID".into()));
            set_loading.set(false);
        }
    });

    let handle_delete = move |_| {
        if let Some(t) = topic() {
            let id_val = t.id;
            // In a real app, you might want to add a confirmation dialog here
            spawn_local(async move {
                match delete_topic_handler(id_val).await {
                    Ok(_) => {
                        navigate("/forum/topics", NavigateOptions::default());
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to delete topic: {}", e)));
                        log::error!("Failed to delete topic: {}", e);
                    }
                }
            });
        }
    };

    view! { cx,
        <div class="topic-view">
            {move || match (loading(), topic(), error()) {
                (true, _, _) => view! { cx, <div>"Loading..."</div> }.into_view(cx),
                (false, _, Some(err)) => view! { cx, <div class="error">{err}</div> }.into_view(cx),
                (false, None, None) => view! { cx, <div>"Topic not found"</div> }.into_view(cx),
                (false, Some(t), None) => view! { cx,
                    <>
                        <h1>{&t.title}</h1>
                        <div class="topic-metadata">
                            <p>"Created: "{&t.created_at.to_string()}</p>
                            {if let Some(updated) = &t.updated_at {
                                view! { cx, <p>"Updated: "{updated.to_string()}</p> }.into_view(cx)
                            } else {
                                view! { cx, <></> }.into_view(cx)
                            }}
                        </div>
                        
                        <div class="topic-content" inner_html={&t.content} />
                        
                        <div class="topic-actions">
                            <A href={format!("/forum/topics/{}/edit", t.id)} class="button">
                                "Edit"
                            </A>
                            <button on:click=handle_delete class="button delete">
                                "Delete"
                            </button>
                            <A href="/forum/topics" class="button secondary">
                                "Back to Topics"
                            </A>
                        </div>
                    </>
                }.into_view(cx),
            }}
        </div>
    }
}