use leptos::*;
use crate::models::forum::topic::TopicSummary;
use crate::components::error_alert::ErrorAlert;
use crate::utils::date_utils::format_date_for_display;

#[component]
pub fn UserTopicsList(
    user_id: String,
) -> impl IntoView {
    // State
    let (topics, set_topics) = create_signal(Vec::<TopicSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    let per_page = 10;
    
    // Load topics
    let load_topics = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, Vec<TopicSummary>>(
                "get_user_topics", 
                &(user_id.clone(), Some(current_page.get()), Some(per_page))
            ).await {
                Ok(fetched_topics) => {
                    set_topics.set(fetched_topics);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load topics: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load topics on mount and when page changes
    create_effect(move |_| {
        load_topics();
    });
    
    // Handle pagination
    let next_page = move |_| {
        set_current_page.update(|p| *p += 1);
        load_topics();
    };
    
    let prev_page = move |_| {
        if current_page.get() > 1 {
            set_current_page.update(|p| *p -= 1);
            load_topics();
        }
    };

    view! {
        <div class="user-topics">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            <h2 class="section-title">"Topics Started"</h2>
            
            {move || {
                if loading.get() && topics.get().is_empty() {
                    view! { <div class="loading-state">"Loading topics..."</div> }
                } else if topics.get().is_empty() {
                    view! { <div class="empty-state">"No topics found"</div> }
                } else {
                    view! {
                        <div class="topics-container">
                            <table class="topics-table">
                                <thead>
                                    <tr>
                                        <th>"Topic"</th>
                                        <th>"Category"</th>
                                        <th class="replies-col">"Replies"</th>
                                        <th class="views-col">"Views"</th>
                                        <th class="activity-col">"Activity"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {topics.get().into_iter().map(|topic| {
                                        let replies = topic.posts_count - 1; // Subtract the initial post
                                        
                                        view! {
                                            <tr class="topic-row">
                                                <td class="topic-details">
                                                    <a href={format!("/topics/{}", topic.id)} class="topic-title">
                                                        {topic.title}
                                                    </a>
                                                    {if let Some(excerpt) = &topic.excerpt {
                                                        view! {
                                                            <div class="topic-excerpt">{excerpt}</div>
                                                        }
                                                    } else {
                                                        view! { <div></div> }
                                                    }}
                                                </td>
                                                <td class="topic-category">
                                                    <a href={format!("/categories/{}", topic.category_id)} class="category-link">
                                                        {topic.category_name}
                                                    </a>
                                                </td>
                                                <td class="topic-replies">{replies}</td>
                                                <td class="topic-views">{topic.views}</td>
                                                <td class="topic-activity">
                                                    {format_date_for_display(Some(&topic.created_at))}
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                            
                            <div class="pagination-controls">
                                <button 
                                    class="pagination-button" 
                                    on:click=prev_page 
                                    disabled=move || current_page.get() <= 1
                                >
                                    "Previous"
                                </button>
                                <span class="page-indicator">{"Page "}{current_page}</span>
                                <button 
                                    class="pagination-button" 
                                    on:click=next_page 
                                    disabled=move || topics.get().len() < per_page
                                >
                                    "Next"
                                </button>
                            </div>
                        </div>
                    }
                }
            }}
        </div>
    }
}

// Helper function to invoke Tauri commands
async fn invoke<T, R>(cmd: &str, args: &T) -> Result<R, String>
where
    T: serde::Serialize + ?Sized,
    R: for<'de> serde::de::DeserializeOwned,
{
    tauri_sys::tauri::invoke(cmd, args)
        .await
        .map_err(|e| e.to_string())
}