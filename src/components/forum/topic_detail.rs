use leptos::*;
use crate::api::forum::{Topic, get_topic};

#[component]
pub fn TopicDetail(
    #[prop()] topic_id: i64,
) -> impl IntoView {
    // Create resource for topic
    let topic = create_resource(
        move || topic_id,
        move |id| async move {
            get_topic(id).await.ok()
        }
    );

    // Format date helper
    let format_date = |date: chrono::DateTime<chrono::Utc>| {
        date.format("%B %d, %Y at %H:%M").to_string()
    };

    view! {
        <div class="topic-detail">
            {move || match topic.get() {
                None => view! { <p>"Loading topic..."</p> },
                Some(None) => view! { <p>"Topic not found"</p> },
                Some(Some(t)) => {
                    view! {
                        <div class="topic-header">
                            <h1>{t.title}</h1>
                            <div class="topic-meta">
                                <span>"Posted on: " {format_date(t.created_at)}</span>
                                <a href={format!("/forum/category/{}", t.category_id)}>
                                    "Back to category"
                                </a>
                            </div>
                        </div>

                        // Topic content would go here
                        <div class="topic-content">
                            <p>"This would display the topic content from the API."</p>
                        </div>

                        // Replies would go here
                        <div class="topic-replies">
                            <h3>"Replies"</h3>
                            <p>"Replies would be loaded here."</p>
                        </div>
                    }
                }
            }}
        </div>
    }
}