use leptos::*;
use crate::models::user::activity::UserActivity;
use crate::components::error_alert::ErrorAlert;
use crate::utils::date_utils::format_date_for_display;

#[component]
pub fn UserActivityFeed(
    user_id: String,
) -> impl IntoView {
    // State
    let (activities, set_activities) = create_signal(Vec::<UserActivity>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (current_page, set_current_page) = create_signal(1);
    
    // Load activities
    let load_activities = move || {
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match invoke::<_, Vec<UserActivity>>(
                "get_user_activities", 
                &(user_id.clone(), Some(current_page.get()), Some(20))
            ).await {
                Ok(fetched_activities) => {
                    set_activities.set(fetched_activities);
                    set_loading.set(false);
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load user activities: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load activities on mount and when page changes
    create_effect(move |_| {
        load_activities();
    });
    
    // Handle pagination
    let next_page = move |_| {
        set_current_page.update(|p| *p += 1);
        load_activities();
    };
    
    let prev_page = move |_| {
        if current_page.get() > 1 {
            set_current_page.update(|p| *p -= 1);
            load_activities();
        }
    };

    view! {
        <div class="user-activity-feed">
            {move || error.get().map(|err| view! { <ErrorAlert message=err /> })}
            
            {move || {
                if loading.get() && activities.get().is_empty() {
                    view! { <div class="loading-state">"Loading activities..."</div> }
                } else if activities.get().is_empty() {
                    view! { <div class="empty-state">"No activities found"</div> }
                } else {
                    view! {
                        <div class="activities-list">
                            {activities.get().into_iter().map(|activity| {
                                let activity_label = format_activity_label(&activity);
                                let activity_icon = get_activity_icon(&activity.activity_type);
                                
                                view! {
                                    <div class="activity-item">
                                        <div class="activity-icon">{activity_icon}</div>
                                        <div class="activity-content">
                                            <div class="activity-label" inner_html={activity_label}></div>
                                            <div class="activity-date">{format_date_for_display(Some(&activity.created_at))}</div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                            
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
                                    disabled=move || activities.get().len() < 20  // If we have fewer than our limit, no more pages
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

// Helper function to format activity labels
fn format_activity_label(activity: &UserActivity) -> String {
    use crate::models::user::activity::ActivityType;
    
    match activity.activity_type {
        ActivityType::TopicCreated => {
            format!("Created a new topic: <a href='/topics/{}'>{}</a>", 
                activity.target_id,
                get_topic_title(&activity.target_id).unwrap_or_else(|| "Topic".to_string())
            )
        },
        ActivityType::TopicReplied => {
            format!("Replied to topic: <a href='/topics/{}'>{}</a>", 
                activity.target_id,
                get_topic_title(&activity.target_id).unwrap_or_else(|| "Topic".to_string())
            )
        },
        ActivityType::TopicLiked => {
            format!("Liked a topic: <a href='/topics/{}'>{}</a>", 
                activity.target_id,
                get_topic_title(&activity.target_id).unwrap_or_else(|| "Topic".to_string())
            )
        },
        ActivityType::PostLiked => {
            "Liked a post".to_string()
        },
        ActivityType::UserFollowed => {
            format!("Started following user: <a href='/users/{}'>{}</a>", 
                activity.target_id,
                get_user_name(&activity.target_id).unwrap_or_else(|| "User".to_string())
            )
        },
        ActivityType::CategoryFollowed => {
            format!("Started following category: <a href='/categories/{}'>{}</a>", 
                activity.target_id,
                get_category_name(&activity.target_id).unwrap_or_else(|| "Category".to_string())
            )
        },
        ActivityType::BadgeAwarded => {
            format!("Earned a badge: {}", 
                get_badge_name(&activity.target_id).unwrap_or_else(|| "Badge".to_string())
            )
        },
        ActivityType::CourseEnrolled => {
            format!("Enrolled in course: <a href='/courses/{}'>{}</a>", 
                activity.target_id,
                get_course_title(&activity.target_id).unwrap_or_else(|| "Course".to_string())
            )
        },
        ActivityType::CourseCompleted => {
            format!("Completed course: <a href='/courses/{}'>{}</a>", 
                activity.target_id,
                get_course_title(&activity.target_id).unwrap_or_else(|| "Course".to_string())
            )
        },
        ActivityType::ModuleCompleted => {
            format!("Completed module: <a href='/modules/{}'>{}</a>", 
                activity.target_id,
                get_module_title(&activity.target_id).unwrap_or_else(|| "Module".to_string())
            )
        },
        ActivityType::AssignmentSubmitted => {
            format!("Submitted assignment: <a href='/assignments/{}'>{}</a>", 
                activity.target_id,
                get_assignment_title(&activity.target_id).unwrap_or_else(|| "Assignment".to_string())
            )
        },
        ActivityType::QuizCompleted => {
            format!("Completed quiz: <a href='/quizzes/{}'>{}</a>", 
                activity.target_id,
                get_quiz_title(&activity.target_id).unwrap_or_else(|| "Quiz".to_string())
            )
        },
    }
}

// Helper to get activity icon
fn get_activity_icon(activity_type: &crate::models::user::activity::ActivityType) -> &'static str {
    use crate::models::user::activity::ActivityType;
    
    match activity_type {
        ActivityType::TopicCreated => "📝",
        ActivityType::TopicReplied => "💬",
        ActivityType::TopicLiked => "❤️",
        ActivityType::PostLiked => "👍",
        ActivityType::UserFollowed => "👤",
        ActivityType::CategoryFollowed => "📚",
        ActivityType::BadgeAwarded => "🏆",
        ActivityType::CourseEnrolled => "🎓",
        ActivityType::CourseCompleted => "🎉",
        ActivityType::ModuleCompleted => "✅",
        ActivityType::AssignmentSubmitted => "📄",
        ActivityType::QuizCompleted => "❓",
    }
}

// These functions would be implemented to fetch entity names from the backend
// For this example, we'll return None to use the fallbacks
fn get_topic_title(_id: &str) -> Option<String> { None }
fn get_user_name(_id: &str) -> Option<String> { None }
fn get_category_name(_id: &str) -> Option<String> { None }
fn get_badge_name(_id: &str) -> Option<String> { None }
fn get_course_title(_id: &str) -> Option<String> { None }
fn get_module_title(_id: &str) -> Option<String> { None }
fn get_assignment_title(_id: &str) -> Option<String> { None }
fn get_quiz_title(_id: &str) -> Option<String> { None }

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