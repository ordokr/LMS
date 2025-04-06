use leptos::*;
use leptos_router::A;
use serde::Deserialize;
use crate::hooks::use_auth::use_auth;

#[derive(Clone, Debug, Deserialize)]
struct Topic {
    id: String,
    title: String,
    slug: String,
    post_count: i32,
    view_count: i32,
    created_at: String,
    updated_at: String,
    pinned: bool,
    closed: bool,
    author_id: String,
}

#[component]
pub fn TopicList(
    #[prop(into)] category_id: String,
) -> impl IntoView {
    let (topics, set_topics) = create_signal::<Vec<Topic>>(vec![]);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    
    // Load topics when component mounts
    create_effect(move |_| {
        let category = category_id.clone();
        
        spawn_local(async move {
            set_loading(true);
            
            match fetch_topics(&category).await {
                Ok(fetched_topics) => {
                    set_topics(fetched_topics);
                    set_error(None);
                },
                Err(err) => {
                    log::error!("Failed to fetch topics: {}", err);
                    set_error(Some(format!("Error loading topics: {}", err)));
                }
            }
            
            set_loading(false);
        });
    });
    
    view! {
        <div>
            <div class="bg-white shadow overflow-hidden sm:rounded-lg mb-4">
                <div class="px-4 py-5 sm:px-6 flex justify-between items-center">
                    <h3 class="text-lg leading-6 font-medium text-gray-900">
                        "Discussion Topics"
                    </h3>
                    <A
                        href={format!("/categories/{}/topics/create", category_id)}
                        class="inline-flex items-center px-3 py-1.5 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
                    >
                        "New Topic"
                    </A>
                </div>
            </div>
            
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
                } else if topics.get().is_empty() {
                    view! {
                        <div class="bg-gray-50 p-8 text-center rounded-lg">
                            <h3 class="text-lg font-medium text-gray-900">No topics in this category</h3>
                            <p class="mt-2 text-gray-500">
                                "Create the first topic to start a discussion."
                            </p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="bg-white shadow overflow-hidden sm:rounded-md">
                            <ul role="list" class="divide-y divide-gray-200">
                                {topics.get().into_iter().map(|topic| {
                                    view! {
                                        <li>
                                            <A href={format!("/topics/{}", topic.id)} class="block hover:bg-gray-50">
                                                <div class="px-4 py-4 sm:px-6">
                                                    <div class="flex items-center justify-between">
                                                        <div class="flex items-center">
                                                            {if topic.pinned {
                                                                view! {
                                                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-blue-500 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
                                                                    </svg>
                                                                }
                                                            } else {
                                                                view! { <></> }
                                                            }}
                                                            <p class="text-sm font-medium text-blue-600 truncate">
                                                                {topic.title}
                                                            </p>
                                                        </div>
                                                        <div class="ml-2 flex-shrink-0 flex">
                                                            {if topic.closed {
                                                                view! {
                                                                    <p class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800">
                                                                        "Closed"
                                                                    </p>
                                                                }
                                                            } else {
                                                                view! { <></> }
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="mt-2 sm:flex sm:justify-between">
                                                        <div class="sm:flex space-x-4">
                                                            <p class="flex items-center text-sm text-gray-500">
                                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z" />
                                                                </svg>
                                                                {format!("{} posts", topic.post_count)}
                                                            </p>
                                                            <p class="flex items-center text-sm text-gray-500">
                                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                                                </svg>
                                                                {format!("{} views", topic.view_count)}
                                                            </p>
                                                        </div>
                                                        <div class="mt-2 flex items-center text-sm text-gray-500 sm:mt-0">
                                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                            </svg>
                                                            {format_date(&topic.updated_at)}
                                                        </div>
                                                    </div>
                                                </div>
                                            </A>
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

// Helper function to fetch topics for a category
async fn fetch_topics(category_id: &str) -> Result<Vec<Topic>, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/categories/{}/topics", category_id), 
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
    
    let topics: Vec<Topic> = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(topics)
}

// Helper function to format date strings
fn format_date(date_str: &str) -> String {
    // Simple formatting - in a real app you might use a date library
    date_str.split('T').next().unwrap_or(date_str).to_string()
}