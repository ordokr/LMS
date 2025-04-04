use leptos::*;
use crate::components::shared::{CourseCategoryLinker, ActivityStream};

#[component]
pub fn IntegrationDashboard() -> impl IntoView {
    view! {
        <div class="integration-dashboard">
            <h1>"Integration Dashboard"</h1>
            
            <div class="dashboard-section">
                <h2>"Course-Forum Integration"</h2>
                <div class="section-content">
                    <CourseCategoryLinker/>
                </div>
            </div>
            
            <div class="dashboard-section">
                <h2>"Recent Activity"</h2>
                <div class="section-content">
                    <ActivityStream limit=5/>
                </div>
            </div>
            
            <div class="dashboard-section">
                <h2>"Integration Status"</h2>
                <div class="section-content">
                    <div class="status-metrics">
                        <div class="metric-card">
                            <div class="metric-value">"8"</div>
                            <div class="metric-label">"Linked Courses"</div>
                        </div>
                        <div class="metric-card">
                            <div class="metric-value">"24"</div>
                            <div class="metric-label">"Linked Assignments"</div>
                        </div>
                        <div class="metric-card">
                            <div class="metric-value">"12"</div>
                            <div class="metric-label">"Linked Modules"</div>
                        </div>
                        <div class="metric-card">
                            <div class="metric-value">"86%"</div>
                            <div class="metric-label">"Sync Status"</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}