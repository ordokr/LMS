use leptos::*;
use leptos_router::use_params;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::SubmitEvent;
use crate::hooks::use_auth::use_auth;

#[derive(Clone, Debug, Deserialize)]
struct TopicParams {
    topic_id: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Topic {
    id: String,
    title: String,
    slug: String,
    category_id: String,
    post_count: i32,
    view_count: i32,
    created_at: String,
    updated_at: String,
    pinned: bool,
    closed: bool,
    author_id: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Post {
    id: String,
    topic_id: String,
    author_id: String,
    content: String,
    parent_id: Option<String>,
    created_at: String,
    updated_at: String,
    author_username: Option<String>, // For display
}

#[derive(Clone, Debug, Deserialize)]
struct TopicWithPosts {
    topic: Topic,
    posts: Vec<Post>,
    assignment: Option<Assignment>,
}

#[derive(Clone, Debug, Deserialize)]
struct Assignment {
    id: String,
    title: String,
    course_id: String,
    due_date: Option<String>,
    points_possible: Option<f32>,
}

#[component]
pub fn TopicDetail() -> impl IntoView {
    let params = use_params::<TopicParams>();
    let topic_id = move || params.get().map(|p| p.topic_id).unwrap_or_default();
    
    let (topic_data, set_topic_data) = create_signal::<Option<TopicWithPosts>>(None);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    
    // For creating new posts
    let (new_post_content, set_new_post_content) = create_signal(String::new());
    let (posting, set_posting) = create_signal(false);
    
    let auth = use_auth();
    
    // Load topic data when component mounts
    create_effect(move |_| {
        let id = topic_id();
        if id.is_empty() {
            return;
        }
        
        spawn_local(async move {
            set_loading(true);
            
            match fetch_topic_with_posts(&id).await {
                Ok(data) => {
                    set_topic_data(Some(data));
                    set_error(None);
                },
                Err(err) => {
                    log::error!("Failed to fetch topic: {}", err);
                    set_error(Some(format!("Error loading topic: {}", err)));
                }
            }
            
            set_loading(false);
        });
    });
    
    // Handler for submitting new posts
    let handle_post_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_posting(true);
        
        let content = new_post_content.get();
        if content.trim().is_empty() {
            return;
        }
        
        let id = topic_id();
        
        spawn_local(async move {
            match create_post(&id, &content).await {
                Ok(post) => {
                    // Update the topic data with the new post
                    set_topic_data.update(|data| {
                        if let Some(data) = data {
                            let mut updated_data = data.clone();
                            updated_data.posts.push(post);
                            *data = updated_data;
                        }
                    });
                    set_new_post_content(String::new());
                },
                Err(err) => {
                    log::error!("Failed to create post: {}", err);
                    set_error(Some(format!("Error creating post: {}", err)));
                }
            }
            
            set_posting(false);
        });
    };
    
    view! {
        <div>
            {move || {
                if loading.get() {
                    view! {
                        <div class="flex justify-center my-8">
                            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
                        </div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                            {err}
                        </div>
                    }.into_view()
                } else if let Some(data) = topic_data.get() {
                    view! {
                        <div>
                            <div class="bg-white shadow overflow-hidden sm:rounded-lg mb-6">
                                <div class="px-4 py-5 sm:px-6">
                                    <div class="flex justify-between">
                                        <h2 class="text-xl font-bold text-gray-900">
                                            {data.topic.title}
                                        </h2>
                                        <div class="flex space-x-2">
                                            {if data.topic.pinned {
                                                view! {
                                                    <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800">
                                                        "Pinned"
                                                    </span>
                                                }
                                            } else {
                                                view! { <></> }
                                            }}
                                            
                                            {if data.topic.closed {
                                                view! {
                                                    <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800">
                                                        "Closed"
                                                    </span>
                                                }
                                            } else {
                                                view! { <></> }
                                            }}
                                        </div>
                                    </div>
                                    <div class="mt-2 flex flex-wrap gap-x-4 text-sm text-gray-500">
                                        <p>
                                            <span class="font-medium">Created: </span>
                                            {format_date(&data.topic.created_at)}
                                        </p>
                                        <p>
                                            <span class="font-medium">Posts: </span>
                                            {data.topic.post_count}
                                        </p>
                                        <p>
                                            <span class="font-medium">Views: </span>
                                            {data.topic.view_count}
                                        </p>
                                    </div>
                                    
                                    // Show assignment info if this topic is linked to an assignment
                                    {move || {
                                        if let Some(assignment) = &data.assignment {
                                            view! {
                                                <div class="mt-3 p-3 bg-blue-50 rounded-md">
                                                    <div class="flex items-center">
                                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-blue-500 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                                                        </svg>
                                                        <span class="font-medium text-blue-700">Assignment: </span>
                                                        <span class="ml-1 text-blue-700">{&assignment.title}</span>
                                                    </div>
                                                    {move || {
                                                        if let Some(due_date) = &assignment.due_date {
                                                            view! {
                                                                <div class="mt-1 text-sm text-blue-600">
                                                                    <span class="font-medium">Due: </span>
                                                                    {format_date(due_date)}
                                                                </div>
                                                            }
                                                        } else {
                                                            view! { <></> }
                                                        }
                                                    }}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <></> }.into_view()
                                        }
                                    }}
                                </div>
                            </div>
                            
                            // Posts
                            <div class="bg-white shadow overflow-hidden sm:rounded-lg mb-6">
                                <ul class="divide-y divide-gray-200">
                                    {data.posts.iter().map(|post| {
                                        let post_clone = post.clone();
                                        view! {
                                            <li class="p-4 sm:p-6">
                                                <div class="flex space-x-3">
                                                    <div class="flex-shrink-0">
                                                        <div class="h-10 w-10 rounded-full bg-gray-200 flex items-center justify-center">
                                                            <span class="text-gray-500 font-medium">
                                                                {post_clone.author_username.clone()
                                                                  .unwrap_or_else(|| "User".to_string())
                                                                  .chars().next().unwrap_or('U')}
                                                            </span>
                                                        </div>
                                                    </div>
                                                    <div class="min-w-0 flex-1">
                                                        <p class="text-sm font-medium text-gray-900">
                                                            {post_clone.author_username.unwrap_or_else(|| "Unknown User".to_string())}
                                                        </p>
                                                        <p class="text-sm text-gray-500">
                                                            {format_date(&post_clone.created_at)}
                                                        </p>
                                                        <div class="mt-2 text-sm text-gray-700 whitespace-pre-line">
                                                            {post_clone.content}
                                                        </div>
                                                    </div>
                                                </div>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                            
                            // Reply form
                            {move || {
                                if !data.topic.closed {
                                    view! {
                                        <div class="bg-white shadow sm:rounded-lg mb-6">
                                            <div class="px-4 py-5 sm:p-6">
                                                <h3 class="text-lg leading-6 font-medium text-gray-900">
                                                    "Post a reply"
                                                </h3>
                                                <form on:submit=handle_post_submit>
                                                    <div class="mt-5">
                                                        <textarea
                                                            id="post-content"
                                                            rows="4"
                                                            class="shadow-sm focus:ring-blue-500 focus:border-blue-500 block w-full sm:text-sm border-gray-300 rounded-md"
                                                            placeholder="Write your reply here..."
                                                            required
                                                            disabled=move || posting.get()
                                                            prop:value=new_post_content
                                                            on:input=move |ev| {
                                                                let target = event_target(&ev);
                                                                let textarea = target.dyn_ref::<web_sys::HtmlTextAreaElement>()
                                                                    .expect("target should be a textarea");
                                                                set_new_post_content(textarea.value());
                                                            }
                                                        ></textarea>
                                                    </div>
                                                    <div class="mt-5">
                                                        <button
                                                            type="submit"
                                                            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                                            disabled=move || posting.get()
                                                        >
                                                            {move || if posting.get() { "Posting..." } else { "Post Reply" }}
                                                        </button>
                                                    </div>
                                                </form>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="bg-gray-100 p-4 text-center rounded-lg mb-6">
                                            <p class="text-gray-600">This topic is closed. No new replies can be posted.</p>
                                        </div>
                                    }.into_view()
                                }
                            }}
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
                            "Topic not found"
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

// Helper function to fetch topic with its posts
async fn fetch_topic_with_posts(topic_id: &str) -> Result<TopicWithPosts, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/topics/{}", topic_id), 
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
    
    let topic_data: TopicWithPosts = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(topic_data)
}

// Helper function to create a new post
async fn create_post(topic_id: &str, content: &str) -> Result<Post, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    // Create request body
    let post_request = serde_json::json!({
        "content": content
    });
    let body = serde_json::to_string(&post_request)
        .map_err(|_| "Failed to serialize request body".to_string())?;
    
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&body)));
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/topics/{}/posts", topic_id), 
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
    
    let post: Post = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(post)
}

// Helper function to format date strings
fn format_date(date_str: &str) -> String {
    // Simple formatting - in a real app you might use a date library
    date_str.split('T').next().unwrap_or(date_str).to_string()
}