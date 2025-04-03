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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provide meta context for style and meta tag insertion
    provide_meta_context(cx);
    
    // Initialize sync client for online/offline handling
    SyncClient::register_online_sync();

    // Create services
    let auth_service = AuthService::new();
    let course_service = CourseService::new();
    let forum_service = ForumService::new();
    let integration_service = IntegrationService::new(course_service.clone(), forum_service.clone());
    
    // Create sync manager
    let sync_manager = SyncManager::new(cx, forum_service.clone(), course_service.clone());
    
    // Provide services to component tree
    provide_context(cx, auth_service);
    provide_context(cx, course_service);
    provide_context(cx, forum_service);
    provide_context(cx, integration_service);
    provide_context(cx, sync_manager);

    // Create WebSocketService
    let websocket_service = create_rw_signal(WebSocketService::new());
    
    // Connect to WebSocket when user is authenticated
    let auth_state = use_context::<AuthState>();
    
    create_effect(move |_| {
        if let Some(auth) = auth_state {
            if auth.is_authenticated() && !websocket_service.get().is_connected() {
                // Connect to WebSocket
                spawn_local(async move {
                    websocket_service.update(|ws| ws.connect());
                });
            } else if !auth.is_authenticated() && websocket_service.get().is_connected() {
                // Disconnect if user logs out
                websocket_service.update(|ws| ws.close());
            }
        }
    });
    
    // Provide WebSocketService to the rest of the app
    provide_context(websocket_service);

    view! { cx,
        <Stylesheet id="leptos" href="/styles.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Title text="LMS Platform"/>
        
        <Router>
            <Routes>
                <Route path="" view=|cx| view! { cx, <AppLayout><Outlet/></AppLayout> }>
                    <Route path="/" view=|cx| view! { cx, <Home/> }/>
                    <Route path="/login" view=|cx| view! { cx, <LoginForm/> }/>
                    <Route path="/register" view=|cx| view! { cx, <RegisterForm/> }/>
                    <Route path="/profile" view=|cx| view! { cx, <UserProfile/> }/>
                    
                    // Course Routes
                    <Route path="/courses" view=|cx| view! { cx, <CoursesList/> }/>
                    <Route path="/courses/new" view=|cx| view! { cx, <CourseForm course_id=None/> }/>
                    <Route path="/courses/:id" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <CourseDetail course_id=id/> }
                    }/>
                    <Route path="/courses/:id/edit" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <CourseForm course_id=Some(id)/> }
                    }/>
                    
                    // Assignment Routes
                    <Route path="/courses/:id/assignments" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <AssignmentsList course_id=id/> }
                    }/>
                    <Route path="/courses/:id/assignments/new" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <AssignmentForm course_id=id assignment_id=None/> }
                    }/>
                    <Route path="/courses/:id/assignments/:assignment_id" view=|cx| {
                        let params = use_params_map(cx);
                        let course_id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        let assignment_id = params.with(|p| p.get("assignment_id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <AssignmentDetail course_id=course_id assignment_id=assignment_id/> }
                    }/>
                    <Route path="/courses/:id/assignments/:assignment_id/edit" view=|cx| {
                        let params = use_params_map(cx);
                        let course_id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        let assignment_id = params.with(|p| p.get("assignment_id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <AssignmentForm course_id=course_id assignment_id=Some(assignment_id)/> }
                    }/>
                    
                    // Module Routes
                    <Route path="/courses/:id/modules" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <ModulesList course_id=id/> }
                    }/>
                    <Route path="/courses/:id/modules/new" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <ModuleForm course_id=id module_id=None/> }
                    }/>
                    <Route path="/courses/:id/modules/:module_id" view=|cx| {
                        let params = use_params_map(cx);
                        let course_id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        let module_id = params.with(|p| p.get("module_id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <ModuleDetail course_id=course_id module_id=module_id/> }
                    }/>
                    <Route path="/courses/:id/modules/:module_id/edit" view=|cx| {
                        let params = use_params_map(cx);
                        let course_id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        let module_id = params.with(|p| p.get("module_id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <ModuleForm course_id=course_id module_id=Some(module_id)/> }
                    }/>
                    <Route path="/courses/:id/modules/:module_id/items/new" view=|cx| {
                        let params = use_params_map(cx);
                        let course_id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        let module_id = params.with(|p| p.get("module_id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <ModuleItemForm course_id=course_id module_id=module_id/> }
                    }/>
                    
                    // Forum Routes
                    <Route path="/forum" view=|cx| view! { cx, <ForumCategories/> }/>
                    <Route path="/forum/category/:id" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <ForumThreads category_id=id/> }
                    }/>
                    // Then find the ThreadDetail route and update the prop name:
                    <Route path="/forum/thread/:id" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <ThreadDetail topic_id=id/> }
                    }/>
                    // New forum routes for categories and topics
                    <Route path="/forum/categories/new" view=|cx| view! { cx, <CategoryForm/> }/>
                    <Route path="/forum/categories/:id" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <CategoryDetail category_id=id/> }
                    }/>
                    <Route path="/forum/categories/:id/edit" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <CategoryForm category_id=Some(id)/> }
                    }/>
                    <Route path="/forum/categories/:id/topics/new" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <TopicForm category_id=Some(id)/> }
                    }/>
                    <Route path="/forum/topics/:id/edit" view=|cx| {
                        let params = use_params_map(cx);
                        let id = params.with(|p| p.get("id").cloned().unwrap_or_default().parse::<i64>().unwrap_or(0));
                        view! { cx, <TopicForm topic_id=Some(id)/> }
                    }/>
                    <Route path="/courses/:id/forum" view=CourseForum />
                    <Route path="/forum/categories" view=|cx| view! { cx, <ForumCategories/> }/>
                    <Route path="/forum/categories/:id" view=|cx| { /* ... */ }/>
                    <Route path="/forum/topics/:id" view=|cx| { /* ... */ }/>  // Instead of /forum/thread/:id
                    <Route path="/forum/search" view=|cx| view! { cx, <ForumSearch/> }/>
                    <Route path="/users/:id" view=|cx| view! { cx, <UserProfile/> }/>
                    <Route path="/profile/edit" view=|cx| view! { cx, <ProfileEdit/> }/>
                    <Route path="/notifications" view=|cx| view! { cx, <AllNotifications/> }/>
                    <Route path="/forum/tags" view=|cx| view! { cx, <TagBrowser/> }/>
                    <Route path="/forum/tags/:slug" view=|cx| view! { cx, <TagDetail/> }/>
                    <Route path="/admin/tags" view=|cx| view! { cx, <TagManagement/> }/>
                    <Route path="/categories" view=CategoryManagement/>
                    <Route path="/*any" view=|cx| view! { cx, <NotFound/> }/>
                </Route>
                <Route path="/login" view=|cx| view! { cx, <Login /> } />
                <Route path="/register" view=|cx| view! { cx, <Register /> } />
                
                <Route path="/courses" view=|cx| view! { cx, <CourseList /> } />
                <Route path="/courses/:id" view=|cx| {
                    let params = use_params_map(cx);
                    let id = params.with(|params| {
                        params.get("id")
                            .and_then(|id| id.parse::<i64>().ok())
                            .unwrap_or(0)
                    });
                    
                    view! { cx, <CourseDetail course_id=id /> }
                } />
                
                <Route path="/forum/*" view=|cx| view! { cx, <ForumComponent /> } />
                
                // Add other routes as needed
                
                <Route path="/*any" view=|cx| view! { cx, <h1>"404 - Not Found"</h1> } />
                
                // Add this section to your router configuration:
                <Route path="/admin" view=move || {
                    view! {
                        <AdminLayout>
                            <Outlet/>
                        </AdminLayout>
                    }
                }>
                    <Route path="/dashboard" view=AdminDashboard/>
                    <Route path="/users" view=UserManagement/>
                    <Route path="/reports" view=ReportedContent/>
                    <Route path="/settings" view=ForumSettings/>
                    <Route path="/logs" view=ActivityLog/>
                    <Route path="/categories" view=CategoryManagement/> // Fixed path
                    <Route path="/notifications" view=NotificationSettings/> // Add this line
                    <Route path="/groups" view=UserGroups/> // Add this line
                    <Route path="/customization" view=SiteCustomization/> // Add this line
                    <Route path="/import-export" view=ImportExport/> // Add this line
                    <Route path="" view=move || view! { <Redirect path="/admin/dashboard"/> }/>
                </Route>
                // Add route for user preferences, typically placed near your user profile routes

                <Route path="/user/:id" view=|cx| view! { cx, <UserProfile/> }/>
                <Route path="/user/:id/edit" view=|cx| view! { cx, <ProfileEdit/> }/>
                <Route path="/user/:id/preferences" view=|cx| view! { cx, <UserPreferences/> }/> // Add this line
                <Route path="/user/:id/topics" view=|cx| view! { cx, <UserSubscriptions/> }/> // Add this line
            </Routes>
        </Router>
    }

    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let (thread_title, set_thread_title) = signal(String::new());
    let (thread_category, set_thread_category) = signal(String::new());
    let (threads, set_threads) = signal(Vec::<ForumThread>::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let update_thread_title = move |ev| {
        let v = event_target_value(&ev);
        set_thread_title.set(v);
    };

    let update_thread_category = move |ev| {
        let v = event_target_value(&ev);
        set_thread_category.set(v);
    };

    // Define the closure first
    let get_threads = move || {
        // Closure implementation
        spawn_local(async move {
            let result = get_forum_threads().await;
            let threads: Vec<ForumThread> = serde_wasm_bindgen::from_value(result).unwrap();
            set_threads.set(threads);
        });
    };

    let create_thread = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let title = thread_title.get_untracked();
            let category = thread_category.get_untracked();

            if title.is_empty() || category.is_empty() {
                return;
            }

            let _ = create_forum_thread(title.as_str(), category.as_str()).await;
            // After creating a thread, refresh the list
            (get_threads)();
        });
    };

    let threads_signal = threads.clone();

    // Initial load of threads
    (get_threads)();

    let (course_name, set_course_name) = signal(String::new());
    let (course_description, set_course_description) = signal(String::new());
    let (create_course_msg, set_create_course_msg) = signal(String::new());

    let update_course_name = move |ev| {
        let v = event_target_value(&ev);
        set_course_name.set(v);
    };

    let update_course_description = move |ev| {
        let v = event_target_value(&ev);
        set_course_description.set(v);
    };

    let create_course = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = course_name.get_untracked();
            let description = course_description.get_untracked();

            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&CreateCourseArgs { name: &name, description: &description }).unwrap();
            let new_msg = invoke("create_course", args).await.as_string().unwrap_or_else(|| "Failed to create course".to_string());
            set_create_course_msg.set(new_msg);
        });
    };

    #[derive(Serialize, Deserialize)]
    struct CreateCourseArgs<'a> {
        name: &'a str,
        description: &'a str,
    }

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>

            <h2>"Create Course"</h2>
            <form class="row" on:submit=create_course>
                <input
                    id="course-name-input"
                    placeholder="Enter course name..."
                    on:input=update_course_name
                />
                <input
                    id="course-description-input"
                    placeholder="Enter course description..."
                    on:input=update_course_description
                />
                <button type="submit">"Create Course"</button>
            </form>
            <p>{ move || create_course_msg.get() }</p>

            <h2>"Forum"</h2>
            <h3>"Create Thread"</h3>
            <form class="row" on:submit=create_thread>
                <input
                    id="thread-title-input"
                    placeholder="Enter thread title..."
                    on:input=update_thread_title
                />
                <input
                    id="thread-category-input"
                    placeholder="Enter thread category..."
                    on:input=update_thread_category
                />
                <button type="submit">"Create Thread"</button>
            </form>
            <h3>"Threads"</h3>
            <ul>
                <For
                    each=move || threads_signal.get()
                    key=|thread| thread.id
                    children=move |thread| {
                        view! {
                            <li>
                                { thread.title.clone() }
                            </li>
                        }
                    }
                />
            </ul>
        </main>
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
