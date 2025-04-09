use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use leptos_meta::*; // Make sure this is included for provide_meta_context
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::components::{
    CategoriesList, CategoryDetail, CategoryForm, 
    TopicsList, TopicForm, TopicDetail,
    AuthProvider, Login, Register, UserProfile,
    Layout, Home,
    // Uncomment these when the admin components are created
    // AdminDashboard, AdminCategories, AdminUsers, ContentModeration
};
use crate::services::integration_service::IntegrationService;
use crate::sync::SyncManager;
use crate::storage::LocalStorage;
use crate::components::sync_status::SyncStatus;
use crate::pages::course_forum::CourseForum;
use crate::services::websocket::WebSocketService;
use crate::components::module_manager::ModuleManager;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
extern "C" {
    async fn create_forum_thread(title: &str, category: &str) -> JsValue;
    async fn get_forum_threads() -> JsValue;
    async fn create_forum_post(thread_id: i32, author_id: i32, content: &str) -> JsValue;
    async fn get_forum_posts(thread_id: i32) -> JsValue;
    async fn create_assignment(course_id: i32, title: &str, description: &str, due_date: &str) -> JsValue;
    async fn get_assignments(course_id: i32) -> JsValue;
    async fn create_submission(assignment_id: i32, student_id: i32, content: &str) -> JsValue;
    async fn get_submissions(assignment_id: i32) -> JsValue;
    async fn create_grade(submission_id: i32, grader_id: i32, grade: f32, feedback: &str) -> JsValue;
    async fn get_grade(submission_id: i32) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::window;
use std::time::Duration;

use crate::components::AppLayout;
use crate::components::auth::{LoginForm, RegisterForm, UserProfile};
use crate::components::lms::{
    CoursesList, CourseDetail, CourseForm,
    AssignmentsList, AssignmentDetail, AssignmentForm,
    ModulesList, ModuleDetail, ModuleForm,
    ModuleItemForm
};
use crate::components::forum::{
    ForumCategories, ForumThreads, ThreadDetail, 
    CategoryDetail, CategoryForm, TopicForm, ForumSearch,
    UserProfile, ProfileEdit, AllNotifications, TagBrowser, TagDetail, TagManagement
};
use crate::components::shared::ErrorDisplay;
use crate::utils::sync::SyncClient;

use crate::pages::{ForumHomePage, CategoryPage, TopicPage};

// Critical CSS to include inline for faster initial paint
const CRITICAL_CSS: &str = r#"
/* Critical CSS for initial paint */
:root {
  --primary-color: #0070f3;
  --text-color: #333;
  --bg-color: #fff;
}
body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  color: var(--text-color);
  background: var(--bg-color);
  margin: 0;
  padding: 0;
}
.loading-spinner {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
}
/* Minimalist spinner */
.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(0,0,0,.1);
  border-radius: 50%;
  border-top-color: var(--primary-color);
  animation: spin 1s ease-in-out infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
"#;

// Resource priority management
#[derive(Clone, Debug)]
pub struct ResourcePriority {
    stylesheet_urls: Vec<(String, bool)>, // (url, is_critical)
    script_urls: Vec<(String, bool)>,     // (url, is_critical)
    preload_urls: Vec<String>,            // Additional resources to preload
}

impl ResourcePriority {
    pub fn new() -> Self {
        Self {
            stylesheet_urls: Vec::new(),
            script_urls: Vec::new(),
            preload_urls: Vec::new(),
        }
    }
    
    // Add CSS with priority flag
    pub fn add_stylesheet(&mut self, url: &str, is_critical: bool) {
        self.stylesheet_urls.push((url.to_string(), is_critical));
    }
    
    // Add JS with priority flag
    pub fn add_script(&mut self, url: &str, is_critical: bool) {
        self.script_urls.push((url.to_string(), is_critical));
    }
    
    // Add preload hint
    pub fn add_preload(&mut self, url: &str, resource_type: &str) {
        self.preload_urls.push(format!("{}|{}", url, resource_type));
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Set up resource priorities
    let mut resources = ResourcePriority::new();
    
    // Critical resources
    resources.add_stylesheet("/styles/main.css", true);
    resources.add_stylesheet("/styles/layout.css", true);
    
    // Non-critical resources
    resources.add_stylesheet("/styles/forum/theme.css", false);
    resources.add_stylesheet("/styles/forum/syntax-highlight.css", false);
    resources.add_script("/scripts/forum/editor.js", false);
    resources.add_script("/scripts/forum/syntax-highlight.js", false);
    
    // Add preload hints
    resources.add_preload("/api/forum/categories", "fetch");
    resources.add_preload("/assets/fonts/inter.woff2", "font");
    
    provide_context(resources.clone());
    
    // Load non-critical resources after page load
    let load_non_critical = create_rw_signal(false);
    
    create_effect(move |_| {
        let _ = load_non_critical.get();
        
        // Schedule loading of non-critical resources
        set_timeout(move || {
            load_non_critical.set(true);
        }, Duration::from_millis(100));
    });
    
    view! {
        <Stylesheet id="critical-css" content=CRITICAL_CSS />
        
        // Critical stylesheets loaded immediately
        {resources.stylesheet_urls.iter()
            .filter(|(_, is_critical)| *is_critical)
            .map(|(url, _)| view! { <Stylesheet href=url /> })
            .collect::<Vec<_>>()}
            
        // Non-critical stylesheets loaded after initial render
        {move || {
            if load_non_critical.get() {
                resources.stylesheet_urls.iter()
                    .filter(|(_, is_critical)| !is_critical)
                    .map(|(url, _)| view! { <Stylesheet href=url /> })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        }}
        
        // Add preload hints
        {resources.preload_urls.iter().map(|preload_info| {
            let parts: Vec<&str> = preload_info.split('|').collect();
            if parts.len() == 2 {
                let url = parts[0];
                let type_ = parts[1];
                view! {
                    <link rel="preload" href=url as=type_ />
                }
            } else {
                view! { <></> }
            }
        }).collect::<Vec<_>>()}
        
        // Optimal header configuration for performance
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Your LMS Forum - Connect and learn with your peers"/>
        <Meta http-equiv="X-UA-Compatible" content="IE=edge"/>
        <Link rel="dns-prefetch" href="/api"/>
        <Link rel="preconnect" href="/api"/>
        
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/forum" view=ForumPage />
                <Route path="/forum/category/:id" view=CategoryPage />
                <Route path="/forum/topic/:id" view=TopicPage />
                <Route path="/users/:username" view=UserProfilePage>
                    <Route path="/activity" view=move || view! { <UserProfilePage active_tab="activity".to_string() /> } />
                    <Route path="/topics" view=move || view! { <UserProfilePage active_tab="topics".to_string() /> } />
                    <Route path="/replies" view=move || view! { <UserProfilePage active_tab="replies".to_string() /> } />
                    <Route path="/following" view=FollowingPage />
                    <Route path="/followers" view=FollowersPage />
                </Route>
                <Route path="/*" view=NotFoundPage />
            </Routes>
        </Router>
        
        // Load non-critical scripts at the end
        {move || {
            if load_non_critical.get() {
                resources.script_urls.iter()
                    .filter(|(_, is_critical)| !is_critical)
                    .map(|(url, _)| view! { <Script src=url /> })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        }}
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container">
            <h1>"Welcome to LMS Platform"</h1>
            <p>"This is an offline-first Learning Management System with integrated forum capabilities."</p>
            <div class="actions">
                <a href="/courses" class="button">"Browse Courses"</a>
                <a href="/forum" class="button">"Visit Forum"</a>
            </div>
            <div class="offline-indicator">
                <OfflineIndicator/>
            </div>
        </div>
    }
}

#[component]
fn OfflineIndicator(cx: Scope) -> impl IntoView {
    let (is_online, set_is_online) = create_signal(cx, crate::utils::offline::is_online());
    
    // Register for online/offline events
    crate::utils::offline::register_online_status_listener(move |status| {
        set_is_online.set(status);
    });
    
    view! { cx,
        <div class="offline-indicator" class:offline={move || !is_online.get()}>
            {move || if is_online.get() {
                view! { cx, <span class="online-status">"Online"</span> }
            } else {
                view! { cx, <span class="offline-status">"Offline (Changes will sync when online)"</span> }
            }}
        </div>
    }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container">
            <h1>"404 - Not Found"</h1>
            <p>"The page you're looking for doesn't exist."</p>
            <a href="/" class="button">"Go Home"</a>
        </div>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumThread {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub created_at: String,
}

fn app_routes() -> Vec<RouteDef> {
    vec![
        // ...other routes...
        
        // Course modules route
        RouteDef::new("/courses/:id/modules")
            .component(move |cx| {
                let params = use_params_map(cx);
                let course_id = params.get("id").cloned().unwrap_or_default();
                
                view! { cx,
                    <MainLayout>
                        <ModuleManager course_id={course_id} />
                    </MainLayout>
                }
            }),
    ]
}
