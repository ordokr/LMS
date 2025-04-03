// ... existing imports ...
use crate::components::module_discussion::ModuleDiscussion;

#[component]
pub fn ModuleDetail(cx: Scope) -> impl IntoView {
    // ... existing component code ...
    
    view! { cx,
        <div class="module-detail">
            // ... existing module content ...
            
            // Add the discussion component at the bottom of the page
            <div class="module-discussion-section">
                <ModuleDiscussion
                    course_id={course_id}
                    module_id={module_id}
                />
            </div>
        </div>
    }
}