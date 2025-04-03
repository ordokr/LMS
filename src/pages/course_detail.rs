// ... existing imports ...
use crate::components::course_forum_activity::CourseForumActivity;

#[component]
pub fn CourseDetail(cx: Scope) -> impl IntoView {
    // ... existing component code ...
    
    view! { cx,
        <div class="course-detail">
            // ... existing course content ...
            
            // Add the forum activity component to the course sidebar
            <div class="course-sidebar">
                // ... other sidebar content ...
                
                <CourseForumActivity
                    course_id={course_id}
                    limit={5}
                />
            </div>
        </div>
    }
}