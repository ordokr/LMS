use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

#[component]
pub fn App() -> impl IntoView {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumThread {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub created_at: String,
}
