use leptos::*;
use leptos_router::*;
use crate::api::forum::{Topic, Category};
use crate::api::forum_server::{
    get_topics_handler, get_topics_by_category_handler, delete_topic_handler
};

#[component]
pub fn TopicsList(cx: Scope, #[prop(optional)] category_id: Option<i64>) -> impl IntoView {
    let (topics, set_topics) = create_signal(cx, Vec::<Topic>::new());
    let (loading, set_loading) = create_signal(cx, true);
    let (error, set_error) = create_signal(cx, None::<String>);
    let (page, set_page) = create_signal(cx, 1i64);
    let per_page = 10i64;

    create_effect(cx, move |_| {
        let page_val = page();
        let category_id_val = category_id;
        
        spawn_local(async move {
            set_loading.set(true);
            
            let result = match category_id_val {
                Some(cat_id) => get_topics_by_category_handler(cat_id, page_val, per_page).await,
                None => get_topics_handler(page_val, per_page).await,
            };
            
            match result {
                Ok(fetched_topics) => {
                    set_topics.set(fetched_topics);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load topics: {}", e)));
                    log::error!("Failed to load topics: {}", e);
                }
            }
            
            set_loading.set(false);
        });
    });

    let handle_delete = move |id: i64| {
        spawn_local(async move {
            // In a real app, you might want to add a confirmation dialog here
            match delete_topic_handler(id).await {
                Ok(_) => {
                    // Remove the topic from the list
                    set_topics.update(|t| {
                        t.retain(|topic| topic.id != id);
                    });
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to delete topic: {}", e)));
                    log::error!("Failed to delete topic: {}", e);
                }
            }
        });
    };

    let previous_page = move |_| {
        set_page.update(|p| {
            if *p > 1 { *p - 1 } else { *p }
        });
    };

    let next_page = move |_| {
        if topics().len() >= per_page as usize {
            set_page.update(|p| *p + 1);
        }
    };

    view! { cx,
        <div class="topics-list">
            <h2>"Discussion Topics"</h2>
            
            {move || match (loading(), error()) {
                (true, _) => view! { cx, <div>"Loading topics..."</div> }.into_view(cx),
                (false, Some(err)) => view! { cx, <div class="error">{err}</div> }.into_view(cx),
                (false, None) => {
                    if topics().is_empty() {
                        view! { cx, <p>"No topics found"</p> }.into_view(cx)
                    } else {
                        view! { cx, 
                            <table>
                                <thead>
                                    <tr>
                                        <th>"Title"</th>
                                        <th>"Created"</th>
                                        <th>"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {topics().into_iter().map(|topic| {
                                        let id = topic.id;
                                        view! { cx,
                                            <tr key={topic.id}>
                                                <td>
                                                    <A href={format!("/forum/topics/{}", topic.id)}>{topic.title}</A>
                                                </td>
                                                <td>{topic.created_at.to_string()}</td>
                                                <td>
                                                    <A href={format!("/forum/topics/{}/edit", topic.id)} class="button">"Edit"</A>
                                                    <button on:click=move |_| handle_delete(id) class="button delete">
                                                        "Delete"
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view(cx)}
                                </tbody>
                            </table>
                            
                            <div class="pagination">
                                <button 
                                    disabled={page() <= 1}
                                    on:click=previous_page
                                >
                                    "Previous"
                                </button>
                                <span>"Page "{page()}</span>
                                <button 
                                    disabled={topics().len() < per_page as usize}
                                    on:click=next_page
                                >
                                    "Next"
                                </button>
                            </div>
                        }.into_view(cx)
                    }
                }
            }}
            
            <A href="/forum/topics/new" class="button create">
                "Create New Topic"
            </A>
        </div>
    }
}