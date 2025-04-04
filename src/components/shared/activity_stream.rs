use leptos::*;
use chrono::{DateTime, Utc};
use crate::services::integration_service::{IntegrationService, ActivityEntry, EntityType, ActionType};

#[component]
pub fn ActivityStream(
    #[prop(default = 10)] limit: usize,
    #[prop(optional)] user_id: Option<String>,
    #[prop(optional)] entity_type: Option<EntityType>,
    #[prop(optional)] entity_id: Option<String>,
) -> impl IntoView {
    let integration_service = expect_context::<IntegrationService>();
    
    // Create resource to load activities
    let activities = create_resource(
        move || (user_id.clone(), entity_type.clone(), entity_id.clone(), limit),
        move |(user_id, entity_type, entity_id, limit)| {
            let service = integration_service.clone();
            
            async move {
                match (user_id, entity_type, entity_id) {
                    // User-specific activities
                    (Some(uid), None, None) => {
                        service.get_user_activities(&uid, limit).await
                    },
                    // Entity-specific activities
                    (None, Some(e_type), Some(e_id)) => {
                        service.get_entity_activities(&e_type, &e_id, limit).await
                    },
                    // General activities or other combinations
                    _ => {
                        service.get_recent_activities(limit).await
                    }
                }
            }
        }
    );
    
    view! {
        <div class="activity-stream">
            <h3>"Recent Activity"</h3>
            
            {move || match activities.get() {
                None => view! { <p class="loading">"Loading activities..."</p> },
                Some(Ok(activity_list)) => {
                    if activity_list.is_empty() {
                        view! { <p class="empty">"No activities found."</p> }
                    } else {
                        view! {
                            <ul class="activity-list">
                                {activity_list.into_iter().map(|activity| {
                                    view! { <ActivityItem activity={activity.clone()}/> }
                                }).collect::<Vec<_>>()}
                            </ul>
                        }
                    }
                },
                Some(Err(e)) => view! { <p class="error">"Error loading activities: " {e}</p> }
            }}
            
            <button class="btn refresh" on:click=move |_| activities.refetch()>
                "Refresh"
            </button>
        </div>
    }
}

#[component]
fn ActivityItem(activity: ActivityEntry) -> impl IntoView {
    let formatted_time = format_time(activity.created_at);
    let entity_text = format_entity_text(&activity.entity_type, &activity.entity_id);
    let action_text = format_action_text(&activity.action_type);
    
    view! {
        <li class="activity-item">
            <div class="activity-icon">
                {activity_icon(&activity.action_type)}
            </div>
            <div class="activity-content">
                <span class="user">"User " {&activity.user_id}</span>
                {" "}{action_text}{" "}
                <span class="entity">{entity_text}</span>
                <div class="activity-time">{formatted_time}</div>
            </div>
        </li>
    }
}

fn format_time(time: DateTime<Utc>) -> String {
    // Simple formatting for now - you can enhance this with relative time
    time.format("%b %d, %Y %H:%M").to_string()
}

fn format_entity_text(entity_type: &EntityType, entity_id: &str) -> String {
    match entity_type {
        EntityType::Course => format!("Course {}", entity_id),
        EntityType::Assignment => format!("Assignment {}", entity_id),
        EntityType::Module => format!("Module {}", entity_id),
        EntityType::Category => format!("Category {}", entity_id),
        EntityType::Topic => format!("Topic {}", entity_id),
        EntityType::Post => format!("Post {}", entity_id),
    }
}

fn format_action_text(action: &ActionType) -> String {
    match action {
        ActionType::Created => "created".to_string(),
        ActionType::Updated => "updated".to_string(),
        ActionType::Deleted => "deleted".to_string(),
        ActionType::Viewed => "viewed".to_string(),
        ActionType::Commented => "commented on".to_string(),
        ActionType::Submitted => "submitted to".to_string(),
        ActionType::Graded => "graded".to_string(),
    }
}

fn activity_icon(action: &ActionType) -> impl IntoView {
    match action {
        ActionType::Created => view! { <span class="icon create-icon">➕</span> },
        ActionType::Updated => view! { <span class="icon update-icon">🔄</span> },
        ActionType::Deleted => view! { <span class="icon delete-icon">❌</span> },
        ActionType::Viewed => view! { <span class="icon view-icon">👁️</span> },
        ActionType::Commented => view! { <span class="icon comment-icon">💬</span> },
        ActionType::Submitted => view! { <span class="icon submit-icon">📤</span> },
        ActionType::Graded => view! { <span class="icon grade-icon">📝</span> },
    }
}