// Add to your imports:
use crate::components::CourseIntegrationSettings;

// Then within your CourseSettings component's view function:
view! {
    <div class="course-settings-page">
        <h1>"Course Settings: "{course_name}</h1>

        // Add tabs for different settings sections
        <div class="settings-tabs">
            <button 
                class=move || format!("tab-button {}", if active_tab.get() == "general" { "active" } else { "" })
                on:click=move |_| set_active_tab.set("general".to_string())
            >
                "General"
            </button>
            <button 
                class=move || format!("tab-button {}", if active_tab.get() == "integrations" { "active" } else { "" })
                on:click=move |_| set_active_tab.set("integrations".to_string())
            >
                "Integrations"
            </button>
            // Add other tabs as needed
        </div>

        // Tab content
        {move || {
            match active_tab.get().as_str() {
                "general" => view! {
                    // Your existing general settings content
                    <div class="general-settings">
                        // ...existing form fields, etc.
                    </div>
                },
                "integrations" => view! {
                    <div class="integration-settings">
                        <CourseIntegrationSettings course_id=course_id.clone() />
                    </div>
                },
                // Other tabs
                _ => view! { <div>"Unknown tab selected"</div> }
            }
        }}
    </div>
}