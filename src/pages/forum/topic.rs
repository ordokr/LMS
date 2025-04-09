use leptos::*;
use crate::components::forum::topic_detail::TopicDetail;

#[component]
pub fn TopicPage() -> impl IntoView {
    // Get topic ID from URL
    let params = use_params_map();
    let topic_id = move || {
        params.with(|p| p.get("id").and_then(|id| id.parse::<i64>().ok()))
    };

    view! {
        <div class="topic-page">
            {move || match topic_id() {
                None => view! { <p>"Invalid topic ID"</p> },
                Some(id) => view! { <TopicDetail topic_id={id} /> }
            }}
            
            <a href="/forum">"Back to Forum Home"</a>
        </div>
    }
}