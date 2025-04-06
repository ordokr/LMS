use leptos::*;
use serde::{Deserialize, Serialize};
use crate::hooks::use_auth::use_auth;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
struct Assignment {
    id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    points_possible: Option<i32>,
    topic_id: Option<String>,
    // Other fields omitted for brevity
}

#[derive(Clone, Debug, Deserialize)]
struct Topic {
    id: String,
    title: String,
    slug: String,
    category_id: String,
    post_count: i32,
    // Other fields omitted for brevity
}

#[derive(Clone, Debug, Serialize)]
struct MapTopicRequest {
    category_id: String,
}

#[component]
pub fn AssignmentTopicMapper(
    #[prop(into)] assignment_id: String,
    #[prop(into)] course_id: String,
) -> impl IntoView {
    let (assignment, set_assignment) = create_signal::<Option<Assignment>>(None);
    let (topic, set_topic) = create_signal::<Option<Topic>>(None);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (success, set_success) = create_signal::<Option<String>>(None);
    
    // Load assignment and topic data when component mounts
    create_effect(move |_| {
        spawn_local(async move {
            set_loading(true);
            
            match fetch_assignment_with_topic(&assignment_id).await {
                Ok((fetched_assignment, fetched_topic)) => {
                    set_assignment(Some(fetched_assignment));
                    set_topic(fetched_topic);
                    set_error(None);
                },
                Err(err) => {
                    set_error(Some(format!("Error loading assignment: {}", err)));
                }
            }
            
            set_loading(false);
        });
    });
    
    let handle_create_topic = move || {
        set_loading(true);
        set_error(None);
        set_success(None);
        
        let assignment_id_clone = assignment_id.clone();
        let course_id_clone = course_id.clone();
        
        spawn_local(async move {
            // First, fetch the course's category
            match fetch_course_category(&course_id_clone).await {
                Ok(category_id) => {
                    // Now create a topic for the assignment
                    match create_topic_for_assignment(&assignment_id_clone, &category_id).await {
                        Ok(new_topic) => {
                            set_topic(Some(new_topic));
                            set_success(Some("Discussion topic created successfully".to_string()));
                        },
                        Err(err) => {
                            set_error(Some(format!("Failed to create topic: {}", err)));
                        }
                    }
                },
                Err(err) => {
                    set_error(Some(format!("Failed to fetch course category: {}", err)));
                }
            }
            
            set_loading(false);
        });
    };
    
    let handle_unlink_topic = move || {
        set_loading(true);
        set_error(None);
        set_success(None);
        
        let assignment_id_clone = assignment_id.clone();
        
        spawn_local(async move {
            match unlink_topic_from_assignment(&assignment_id_clone).await {
                Ok(_) => {
                    set_topic(None);
                    set_success(Some("Topic unlinked successfully".to_string()));
                },
                Err(err) => {
                    set_error(Some(format!("Failed to unlink topic: {}", err)));
                }
            }
            
            set_loading(false);
        });
    };

    view! {
        <div class="bg-white shadow overflow-hidden sm:rounded-lg mt-6">
            <div class="px-4 py-5 sm:px-6">
                <h3 class="text-lg leading-6 font-medium text-gray-900">
                    "Assignment-Discussion Integration"
                </h3>
                <p class="mt-1 text-sm text-gray-500">
                    "Connect this assignment to a discussion topic"
                </p>
            </div>
            
            <div class="border-t border-gray-200 px-4 py-5 sm:p-6">
                {move || {
                    if loading.get() {
                        view! {
                            <div class="flex justify-center py-4">
                                <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                            </div>
                        }.into_view()
                    } else if let Some(err) = error.get() {
                        view! {
                            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                                {err}
                            </div>
                        }.into_view()
                    } else if let Some(msg) = success.get() {
                        view! {
                            <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
                                {msg}
                            </div>
                        }.into_view()
                    } else {
                        view! { <></> }.into_view()
                    }
                }}
                
                {move || {
                    if let Some(t) = topic.get() {
                        view! {
                            <div>
                                <div class="flex items-center mb-4">
                                    <div class="bg-green-100 rounded-full p-2 mr-3">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-green-600" viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-sm font-medium text-gray-900">
                                            "This assignment is linked to discussion topic:"
                                        </p>
                                        <p class="text-sm text-blue-600">
                                            <a href={format!("/topics/{}", t.id)} class="hover:underline">
                                                {t.title.clone()}
                                            </a>
                                        </p>
                                        <p class="text-xs text-gray-500 mt-1">
                                            {format!("{} posts", t.post_count)}
                                        </p>
                                    </div>
                                </div>
                                
                                <div class="mt-4">
                                    <button 
                                        class="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                                        disabled=move || loading.get()
                                        on:click=move |_| handle_unlink_topic()
                                    >
                                        "Unlink Discussion Topic"
                                    </button>
                                </div>
                            </div>
                        }.into_view()
                    } else if assignment.get().is_some() {
                        view! {
                            <div>
                                <p class="text-sm text-gray-700 mb-4">
                                    "This assignment doesn't have an associated discussion topic. Creating a topic will allow students to discuss this assignment."
                                </p>
                                
                                <button 
                                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                    disabled=move || loading.get()
                                    on:click=move |_| handle_create_topic()
                                >
                                    "Create Discussion Topic"
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
                                "Assignment not found"
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}

// Helper function to fetch assignment with its topic
async fn fetch_assignment_with_topic(assignment_id: &str) -> Result<(Assignment, Option<Topic>), String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/assignments/{}/topic", assignment_id), 
        &opts
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|_| "Failed to parse response as JSON".to_string())?
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;
    
    // Deserialize response
    #[derive(Deserialize)]
    struct ApiResponse {
        assignment: Assignment,
        topic: Option<Topic>,
    }
    
    let response: ApiResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok((response.assignment, response.topic))
}

// Helper function to fetch course category
async fn fetch_course_category(course_id: &str) -> Result<String, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/courses/{}/category", course_id), 
        &opts
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if resp.status() == 404 {
        return Err("No category found for this course. Create a course category first.".to_string());
    }
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|_| "Failed to parse response as JSON".to_string())?
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;
    
    // Deserialize response
    #[derive(Deserialize)]
    struct CategoryResponse {
        category: Category,
    }
    
    #[derive(Deserialize)]
    struct Category {
        id: String,
    }
    
    let response: CategoryResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(response.category.id)
}

// Helper function to create a topic for an assignment
async fn create_topic_for_assignment(assignment_id: &str, category_id: &str) -> Result<Topic, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    // Create request body
    let map_request = MapTopicRequest {
        category_id: category_id.to_string(),
    };
    let body = serde_json::to_string(&map_request)
        .map_err(|_| "Failed to serialize request body".to_string())?;
    
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&body)));
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/assignments/{}/topic", assignment_id), 
        &opts
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| "Failed to set headers".to_string())?;
    request.headers().set("Content-Type", "application/json")
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|_| "Failed to parse response as JSON".to_string())?
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;
    
    // Deserialize response
    #[derive(Deserialize)]
    struct ApiResponse {
        topic: Topic,
    }
    
    let response: ApiResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(response.topic)
}

// Helper function to unlink a topic from an assignment
async fn unlink_topic_from_assignment(assignment_id: &str) -> Result<(), String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("DELETE");
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/assignments/{}/topic", assignment_id), 
        &opts
    ).map_err(|_| "Failed to create request".to_string())?;
    
    request.headers().set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| "Failed to set headers".to_string())?;
    
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Failed to fetch".to_string())?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| "Response is not a Response".to_string())?;
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    Ok(())
}