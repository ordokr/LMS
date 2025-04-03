// ... existing imports ...
use crate::components::assignment_discussion::AssignmentDiscussion;

#[component]
pub fn AssignmentDetail(cx: Scope) -> impl IntoView {
    // ... existing component code ...
    
    view! { cx,
        <div class="assignment-detail">
            // ... existing assignment content ...
            
            // Add the discussion component at the bottom of the page
            <div class="assignment-discussion-section">
                <AssignmentDiscussion
                    course_id={course_id}
                    assignment_id={assignment_id}
                />
            </div>
        </div>
    }
}