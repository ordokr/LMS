use leptos::*;
use leptos_router::*;
use crate::api::forum::{Topic, NewTopic};
use crate::api::forum_server::{
    get_topic_handler, update_topic_handler, create_topic_handler
};

#[component]
pub fn TopicEdit(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let navigate = use_navigate(cx);
    
    let is_editing = move || id() != "new";
    
    let (topic, set_topic) = create_signal(cx, NewTopic {
        title: String::new(),
        content: String::new(),
        category_id: 1,  // Default category
    });
    
    let (loading, set_loading) = create_signal(cx, is_editing());
    let (error, set_error) = create_signal(cx, None::<String>);

    create_effect(cx, move |_| {
        if is_editing() {
            let id_str = id();
            if let Ok(id_val) = id_str.parse::<i64>() {
                spawn_local(async move {
                    match get_topic_handler(id_val).await {
                        Ok(topic_data) => {
                            set_topic.set(NewTopic {
                                title: topic_data.title,
                                content: topic_data.content,
                                category_id: topic_data.category_id,
                            });
                        }
                        Err(e) => {
                            set_error.set(Some(format!("Failed to load topic: {}", e)));
                            log::error!("Failed to load topic: {}", e);
                        }
                    }
                    set_loading.set(false);
                });
            }
        }
    });

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let current_topic = topic();
        let is_edit = is_editing();
        let id_str = id();
        
        spawn_local(async move {
            let result = if is_edit {
                if let Ok(id_val) = id_str.parse::<i64>() {
                    update_topic_handler(id_val, current_topic).await
                } else {
                    Err("Invalid ID".into())
                }
            } else {
                create_topic_handler(current_topic).await
            };
            
            match result {
                Ok(_) => navigate("/forum/topics", NavigateOptions::default()),
                Err(e) => {
                    set_error.set(Some(format!("Failed to save topic: {}", e)));
                    log::error!("Failed to save topic: {}", e);
                }
            }
        });
    };

    let handle_title_change = move |ev| {
        let value = event_target_value(&ev);
        set_topic.update(|t| t.title = value);
    };
    
    let handle_content_change = move |ev| {
        let value = event_target_value(&ev);
        set_topic.update(|t| t.content = value);
    };
    
    let handle_category_change = move |ev| {
        let value = event_target_value(&ev);
        if let Ok(category_id) = value.parse::<i64>() {
            set_topic.update(|t| t.category_id = category_id);
        }
    };

    view! { cx,
        <div class="topic-form">
            <h2>{move || if is_editing() { "Edit Topic" } else { "Create New Topic" }}</h2>
            
            {move || if let Some(err) = error() {
                view! { cx, <div class="error">{err}</div> }
            } else {
                view! { cx, <></> }
            }}
            
            {move || if loading() {
                view! { cx, <div>"Loading..."</div> }
            } else {
                view! { cx,
                    <form on:submit=handle_submit>
                        <div class="form-group">
                            <label for="title">"Title"</label>
                            <input
                                type="text"
                                id="title"
                                name="title"
                                value={move || topic().title}
                                on:input=handle_title_change
                                required
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="content">"Content"</label>
                            <textarea
                                id="content"
                                name="content"
                                value={move || topic().content}
                                on:input=handle_content_change
                                rows="10"
                                required
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="category_id">"Category"</label>
                            <select
                                id="category_id"
                                name="category_id"
                                value={move || topic().category_id.to_string()}
                                on:change=handle_category_change
                                required
                            >
                                <option value="1">"General Discussion"</option>
                                <option value="2">"Announcements"</option>
                                <option value="3">"Questions"</option>
                            </select>
                        </div>
                        
                        <div class="form-actions">
                            <button type="submit" class="button primary">
                                {move || if is_editing() { "Update Topic" } else { "Create Topic" }}
                            </button>
                            <button 
                                type="button" 
                                class="button secondary"
                                on:click=move |_| { navigate("/forum/topics", NavigateOptions::default()) }
                            >
                                "Cancel"
                            </button>
                        </div>
                    </form>
                }
            }}
        </div>
    }
}