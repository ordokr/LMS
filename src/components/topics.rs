use leptos::*;
use crate::models::{Topic, Category};

// Component to display a list of topics in a category
#[component]
pub fn TopicsList(cx: Scope, category_id: i64) -> impl IntoView {
    let topics = create_resource(
        cx,
        move || category_id,
        move |id| async move {
            // Fetch topics from our API
            let response = reqwest::get(&format!("http://localhost:3030/categories/{}/topics", id))
                .await
                .expect("Failed to fetch topics");
                
            if response.status().is_success() {
                response.json::<Vec<Topic>>().await.ok()
            } else {
                None
            }
        },
    );

    view! { cx,
        <div class="topics-list">
            <h2>"Topics"</h2>
            
            {move || match topics.read(cx) {
                None => view! { cx, <p>"Loading topics..."</p> }.into_view(cx),
                Some(None) => view! { cx, <p>"Failed to load topics."</p> }.into_view(cx),
                Some(Some(topics)) => {
                    if topics.is_empty() {
                        view! { cx, <p>"No topics in this category yet."</p> }.into_view(cx)
                    } else {
                        view! { cx,
                            <table class="topics-table">
                                <thead>
                                    <tr>
                                        <th class="topic-title">"Topic"</th>
                                        <th class="topic-replies">"Replies"</th>
                                        <th class="topic-views">"Views"</th>
                                        <th class="topic-activity">"Last Activity"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                {topics.iter().map(|topic| {
                                    view! { cx,
                                        <tr class={if topic.is_pinned { "pinned-topic" } else { "" }}>
                                            <td class="topic-title">
                                                {if topic.is_closed { "🔒 " } else { "" }}
                                                {if topic.is_pinned { "📌 " } else { "" }}
                                                <a href={format!("/topics/{}", topic.id.unwrap_or(0))}>
                                                    {&topic.title}
                                                </a>
                                            </td>
                                            <td class="topic-replies">"0"</td>
                                            <td class="topic-views">{topic.views.to_string()}</td>
                                            <td class="topic-activity">
                                                {topic.last_posted_at
                                                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                                                    .unwrap_or_else(|| "Never".to_string())}
                                            </td>
                                        </tr>
                                    }
                                }).collect_view(cx)}
                                </tbody>
                            </table>
                        }.into_view(cx)
                    }
                }
            }}
            
            // Create new topic button
            <div class="topic-actions">
                <a href={format!("/categories/{}/new-topic", category_id)} class="button create-button">
                    "Create New Topic"
                </a>
            </div>
        </div>
    }
}

// Component to create a new topic
#[component]
pub fn TopicForm(cx: Scope, category_id: i64) -> impl IntoView {
    let (title, set_title) = create_signal(cx, String::new());
    let (content, set_content) = create_signal(cx, String::new());
    let (error, set_error) = create_signal(cx, None::<String>);
    let (success, set_success) = create_signal(cx, false);
    
    // Fetch the category to display its name
    let category = create_resource(
        cx,
        || category_id,
        |id| async move {
            let response = reqwest::get(&format!("http://localhost:3030/categories/{}", id))
                .await
                .expect("Failed to fetch category");
                
            if response.status().is_success() {
                response.json::<Category>().await.ok()
            } else {
                None
            }
        },
    );
    
    let create_topic = create_action(cx, move |_: &()| {
        let title = title.get();
        let content = content.get();
        
        async move {
            // Basic validation
            if title.trim().is_empty() {
                set_error.set(Some("Topic title is required".to_string()));
                return None;
            }
            
            if content.trim().is_empty() {
                set_error.set(Some("Topic content is required".to_string()));
                return None;
            }
            
            // Clear any previous errors
            set_error.set(None);
            
            // For now we'll use a hardcoded user ID (1) since we don't have auth yet
            let user_id = 1;
            
            // Create the topic payload
            let payload = serde_json::json!({
                "title": title,
                "category_id": category_id,
                "content": content,
                "user_id": user_id
            });
            
            // Send the request to create a topic
            let client = reqwest::Client::new();
            let response = client.post("http://localhost:3030/topics")
                .json(&payload)
                .send()
                .await;
                
            match response {
                Ok(resp) if resp.status().is_success() => {
                    set_success.set(true);
                    // Try to get the ID of the created topic
                    let json = resp.json::<serde_json::Value>().await.ok();
                    let topic_id = json.and_then(|j| j["id"].as_i64());
                    topic_id
                },
                Ok(resp) => {
                    let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    set_error.set(Some(error_text));
                    None
                },
                Err(e) => {
                    set_error.set(Some(format!("Request failed: {}", e)));
                    None
                }
            }
        }
    });
    
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        create_topic.dispatch(());
    };
    
    // Redirect to the new topic if created successfully
    create_effect(cx, move |_| {
        if let Some(Some(topic_id)) = create_topic.value().get() {
            // In a real app you'd use history API or leptos_router to navigate
            let window = web_sys::window().unwrap();
            let _ = window.location().set_href(&format!("/topics/{}", topic_id));
        }
    });

    view! { cx,
        <div class="topic-form">
            {move || match category.read(cx) {
                None => view! { cx, <h1>"New Topic"</h1> }.into_view(cx),
                Some(None) => view! { cx, <h1>"Category Not Found"</h1> }.into_view(cx),
                Some(Some(category)) => {
                    view! { cx,
                        <h1>"New Topic in "{&category.name}</h1>
                    }.into_view(cx)
                }
            }}
            
            {move || error.get().map(|err| view! { cx, <div class="error-message">{err}</div> })}
            
            {move || {
                if success.get() {
                    view! { cx, <div class="success-message">"Topic created successfully!"</div> }.into_view(cx)
                } else {
                    view! {}.into_view(cx)
                }
            }}
            
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="topic-title">"Title:"</label>
                    <input
                        id="topic-title"
                        type="text"
                        value=move || title.get()
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="topic-content">"Content:"</label>
                    <textarea
                        id="topic-content"
                        value=move || content.get()
                        on:input=move |ev| set_content.set(event_target_value(&ev))
                        rows="10"
                        required
                    ></textarea>
                    <small>"Markdown formatting is supported"</small>
                </div>
                
                <div class="form-actions">
                    <button type="submit" disabled=move || create_topic.pending().get()>
                        {move || if create_topic.pending().get() { "Creating..." } else { "Create Topic" }}
                    </button>
                    <a href={format!("/categories/{}", category_id)} class="button secondary">
                        "Cancel"
                    </a>
                </div>
            </form>
        </div>
    }
}