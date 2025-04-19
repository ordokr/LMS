use leptos::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cmi5Course {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub assignable_units: Vec<Cmi5AssignableUnit>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cmi5AssignableUnit {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cmi5Session {
    pub id: String,
    pub actor_id: String,
    pub course_id: String,
    pub au_id: String,
    pub state: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub result: Option<Cmi5Result>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cmi5Result {
    pub score: Option<f64>,
    pub success: Option<bool>,
    pub completion: Option<bool>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Cmi5Manager() -> impl IntoView {
    let (courses, set_courses) = create_signal(Vec::<Cmi5Course>::new());
    let (sessions, set_sessions) = create_signal(Vec::<Cmi5Session>::new());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    let (user_id, set_user_id) = create_signal("test@example.com".to_string());
    let (selected_course, set_selected_course) = create_signal(None::<Cmi5Course>);
    let (selected_au, set_selected_au) = create_signal(None::<Cmi5AssignableUnit>);
    let (launch_url, set_launch_url) = create_signal(None::<String>);

    // Load courses on component mount
    create_effect(move |_| {
        spawn_local(async move {
            load_courses().await;
            load_sessions().await;
        });
    });

    let load_courses = async move || {
        set_loading.set(true);
        set_error.set(None);

        let args = JsValue::from_serde(&serde_json::json!({})).unwrap();
        
        match invoke("get_cmi5_courses", args).await {
            Ok(result) => {
                let courses_result: Vec<Cmi5Course> = result.into_serde().unwrap_or_default();
                set_courses.set(courses_result);
            },
            Err(e) => {
                set_error.set(Some(format!("Failed to load courses: {}", e)));
                console_error!("Error loading courses: {:?}", e);
            }
        }

        set_loading.set(false);
    };

    let load_sessions = async move || {
        set_loading.set(true);
        set_error.set(None);

        let args = JsValue::from_serde(&serde_json::json!({
            "actorId": user_id.get()
        })).unwrap();
        
        match invoke("get_cmi5_user_sessions", args).await {
            Ok(result) => {
                let sessions_result: Vec<Cmi5Session> = result.into_serde().unwrap_or_default();
                set_sessions.set(sessions_result);
            },
            Err(e) => {
                set_error.set(Some(format!("Failed to load sessions: {}", e)));
                console_error!("Error loading sessions: {:?}", e);
            }
        }

        set_loading.set(false);
    };

    let import_course = async move || {
        set_loading.set(true);
        set_error.set(None);

        // Open file dialog to select a cmi5 package
        let args = JsValue::from_serde(&serde_json::json!({
            "multiple": false,
            "filters": [{
                "name": "cmi5 Package",
                "extensions": ["zip"]
            }]
        })).unwrap();

        match invoke("open", args).await {
            Ok(selected) => {
                if !selected.is_null() {
                    let package_path = if selected.is_array() {
                        selected.get(0).as_string().unwrap_or_default()
                    } else {
                        selected.as_string().unwrap_or_default()
                    };

                    let import_args = JsValue::from_serde(&serde_json::json!({
                        "packagePath": package_path
                    })).unwrap();

                    match invoke("import_cmi5_course", import_args).await {
                        Ok(course_id) => {
                            // Reload courses after import
                            load_courses().await;
                            
                            let id_str = course_id.as_string().unwrap_or_default();
                            window().alert_with_message(&format!("Course imported successfully with ID: {}", id_str)).unwrap();
                        },
                        Err(e) => {
                            set_error.set(Some(format!("Failed to import course: {}", e)));
                            console_error!("Error importing course: {:?}", e);
                        }
                    }
                }
            },
            Err(e) => {
                set_error.set(Some(format!("Failed to open file dialog: {}", e)));
                console_error!("Error opening file dialog: {:?}", e);
            }
        }

        set_loading.set(false);
    };

    let launch_assignable_unit = async move |course_id: String, au_id: String| {
        set_loading.set(true);
        set_error.set(None);

        let args = JsValue::from_serde(&serde_json::json!({
            "courseId": course_id,
            "auId": au_id,
            "actorId": user_id.get()
        })).unwrap();

        match invoke("launch_cmi5_assignable_unit", args).await {
            Ok(url) => {
                let url_str = url.as_string().unwrap_or_default();
                set_launch_url.set(Some(url_str.clone()));
                
                // In a real application, you would open this URL in a new window or iframe
                console_log!("Launch URL: {}", url_str);
            },
            Err(e) => {
                set_error.set(Some(format!("Failed to launch assignable unit: {}", e)));
                console_error!("Error launching assignable unit: {:?}", e);
            }
        }

        set_loading.set(false);
    };

    let complete_session = async move |session_id: String, score: f64, success: bool| {
        set_loading.set(true);
        set_error.set(None);

        let args = JsValue::from_serde(&serde_json::json!({
            "sessionId": session_id,
            "score": score,
            "success": success
        })).unwrap();

        match invoke("complete_cmi5_session", args).await {
            Ok(_) => {
                // Reload sessions after completion
                load_sessions().await;
                
                window().alert_with_message("Session completed successfully").unwrap();
            },
            Err(e) => {
                set_error.set(Some(format!("Failed to complete session: {}", e)));
                console_error!("Error completing session: {:?}", e);
            }
        }

        set_loading.set(false);
    };

    let abandon_session = async move |session_id: String| {
        set_loading.set(true);
        set_error.set(None);

        let args = JsValue::from_serde(&serde_json::json!({
            "sessionId": session_id
        })).unwrap();

        match invoke("abandon_cmi5_session", args).await {
            Ok(_) => {
                // Reload sessions after abandonment
                load_sessions().await;
                
                window().alert_with_message("Session abandoned successfully").unwrap();
            },
            Err(e) => {
                set_error.set(Some(format!("Failed to abandon session: {}", e)));
                console_error!("Error abandoning session: {:?}", e);
            }
        }

        set_loading.set(false);
    };

    view! {
        <div class="container">
            <h1 class="title">cmi5 Content Manager</h1>
            
            {move || error.get().map(|err| view! {
                <div class="error-box">
                    <p class="error-message">{err}</p>
                </div>
            })}
            
            <div class="section">
                <h2 class="section-title">User ID</h2>
                <div class="form-group">
                    <label for="user-id">User ID (email)</label>
                    <input 
                        id="user-id"
                        type="text"
                        value={user_id.get()}
                        on:input=move |ev| {
                            set_user_id.set(event_target_value(&ev));
                        }
                    />
                </div>
                <button 
                    class="button"
                    on:click=move |_| {
                        spawn_local(async move {
                            load_sessions().await;
                        });
                    }
                    disabled={loading.get()}
                >
                    "Load Sessions"
                </button>
            </div>
            
            <div class="section">
                <h2 class="section-title">Import Course</h2>
                <button 
                    class="button"
                    on:click=move |_| {
                        spawn_local(async move {
                            import_course().await;
                        });
                    }
                    disabled={loading.get()}
                >
                    "Import cmi5 Package"
                </button>
            </div>
            
            <div class="section">
                <h2 class="section-title">Available Courses</h2>
                {move || {
                    if loading.get() {
                        view! { <div class="loading">Loading...</div> }.into_view()
                    } else {
                        let courses_list = courses.get();
                        if courses_list.is_empty() {
                            view! {
                                <p class="empty-message">
                                    "No courses available. Import a cmi5 package to get started."
                                </p>
                            }.into_view()
                        } else {
                            view! {
                                <ul class="course-list">
                                    {courses_list.into_iter().map(|course| {
                                        let course_clone = course.clone();
                                        view! {
                                            <li class="course-item">
                                                <div class="course-header">
                                                    <h3 class="course-title">{course.title}</h3>
                                                    {course.description.map(|desc| view! {
                                                        <p class="course-description">{desc}</p>
                                                    })}
                                                </div>
                                                <h4 class="au-list-title">Assignable Units:</h4>
                                                <ul class="au-list">
                                                    {course.assignable_units.into_iter().map(|au| {
                                                        let au_clone = au.clone();
                                                        let course_id = course_clone.id.clone();
                                                        let au_id = au.id.clone();
                                                        view! {
                                                            <li class="au-item"
                                                                on:click=move |_| {
                                                                    set_selected_course.set(Some(course_clone.clone()));
                                                                    set_selected_au.set(Some(au_clone.clone()));
                                                                }
                                                            >
                                                                <div class="au-info">
                                                                    <h5 class="au-title">{au.title}</h5>
                                                                    {au.description.map(|desc| view! {
                                                                        <p class="au-description">{desc}</p>
                                                                    })}
                                                                </div>
                                                                <button 
                                                                    class="button"
                                                                    on:click=move |ev| {
                                                                        ev.stop_propagation();
                                                                        let c_id = course_id.clone();
                                                                        let a_id = au_id.clone();
                                                                        spawn_local(async move {
                                                                            launch_assignable_unit(c_id, a_id).await;
                                                                        });
                                                                    }
                                                                >
                                                                    "Launch"
                                                                </button>
                                                            </li>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </ul>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }.into_view()
                        }
                    }
                }}
            </div>
            
            {move || launch_url.get().map(|url| view! {
                <div class="section">
                    <h2 class="section-title">Launch URL</h2>
                    <div class="form-group">
                        <input 
                            type="text"
                            value={url.clone()}
                            readonly=true
                        />
                    </div>
                    <button 
                        class="button"
                        on:click=move |_| {
                            let window = web_sys::window().unwrap();
                            window.open_with_url_and_target(&url, "_blank").unwrap();
                        }
                    >
                        "Open in New Window"
                    </button>
                </div>
            })}
            
            <div class="section">
                <h2 class="section-title">Active Sessions</h2>
                {move || {
                    if loading.get() {
                        view! { <div class="loading">Loading...</div> }.into_view()
                    } else {
                        let sessions_list = sessions.get();
                        if sessions_list.is_empty() {
                            view! {
                                <p class="empty-message">
                                    "No active sessions found."
                                </p>
                            }.into_view()
                        } else {
                            view! {
                                <ul class="session-list">
                                    {sessions_list.into_iter().map(|session| {
                                        let session_id = session.id.clone();
                                        let session_state = session.state.clone();
                                        view! {
                                            <li class="session-item">
                                                <div class="session-info">
                                                    <p class="session-id">
                                                        <strong>"Session ID: "</strong>{session.id}
                                                    </p>
                                                    <p class="session-state">
                                                        <strong>"State: "</strong>{session.state}
                                                    </p>
                                                    <p class="session-start-time">
                                                        <strong>"Started: "</strong>
                                                        {format_date(&session.start_time)}
                                                    </p>
                                                    {session.end_time.map(|end_time| view! {
                                                        <p class="session-end-time">
                                                            <strong>"Ended: "</strong>
                                                            {format_date(&end_time)}
                                                        </p>
                                                    })}
                                                    
                                                    {session.result.map(|result| view! {
                                                        <div class="session-result">
                                                            {result.score.map(|score| view! {
                                                                <p class="session-score">
                                                                    <strong>"Score: "</strong>
                                                                    {format!("{:.1}%", score * 100.0)}
                                                                </p>
                                                            })}
                                                            {result.success.map(|success| view! {
                                                                <p class="session-success">
                                                                    <strong>"Success: "</strong>
                                                                    {if success { "Yes" } else { "No" }}
                                                                </p>
                                                            })}
                                                            {result.completion.map(|completion| view! {
                                                                <p class="session-completion">
                                                                    <strong>"Completion: "</strong>
                                                                    {if completion { "Yes" } else { "No" }}
                                                                </p>
                                                            })}
                                                        </div>
                                                    })}
                                                </div>
                                                
                                                {(session_state == "Initialized" || session_state == "InProgress").then(|| {
                                                    let session_id_pass = session_id.clone();
                                                    let session_id_fail = session_id.clone();
                                                    let session_id_abandon = session_id.clone();
                                                    view! {
                                                        <div class="session-actions">
                                                            <button 
                                                                class="button button-primary"
                                                                on:click=move |_| {
                                                                    let sid = session_id_pass.clone();
                                                                    spawn_local(async move {
                                                                        complete_session(sid, 0.85, true).await;
                                                                    });
                                                                }
                                                            >
                                                                "Complete (Pass)"
                                                            </button>
                                                            <button 
                                                                class="button button-danger"
                                                                on:click=move |_| {
                                                                    let sid = session_id_fail.clone();
                                                                    spawn_local(async move {
                                                                        complete_session(sid, 0.45, false).await;
                                                                    });
                                                                }
                                                            >
                                                                "Complete (Fail)"
                                                            </button>
                                                            <button 
                                                                class="button button-secondary"
                                                                on:click=move |_| {
                                                                    let sid = session_id_abandon.clone();
                                                                    spawn_local(async move {
                                                                        abandon_session(sid).await;
                                                                    });
                                                                }
                                                            >
                                                                "Abandon"
                                                            </button>
                                                        </div>
                                                    }
                                                })}
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }.into_view()
                        }
                    }
                }}
            </div>
        </div>
    }
}

fn format_date(date_str: &str) -> String {
    // Simple date formatting - in a real app, use a proper date library
    date_str.replace("T", " ").replace("Z", "")
}

fn console_log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

fn console_error(message: &str) {
    web_sys::console::error_1(&JsValue::from_str(message));
}
