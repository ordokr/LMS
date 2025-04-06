use leptos::*;
use serde::{Deserialize, Serialize};
use crate::hooks::use_auth::use_auth;

#[component]
pub fn CourseCategoryMapper(
    #[prop(into)] course_id: String,
) -> impl IntoView {
    let (category_name, set_category_name) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (success, set_success) = create_signal::<Option<String>>(None);
    let (category, set_category) = create_signal::<Option<Category>>(None);

    // Load existing category mapping if it exists
    create_effect(move |_| {
        spawn_local(async move {
            match fetch_existing_category(&course_id).await {
                Ok(Some(cat)) => {
                    set_category(Some(cat));
                },
                Ok(None) => {
                    // No category mapping yet
                },
                Err(e) => {
                    log::error!("Failed to fetch category: {}", e);
                }
            }
        });
    });

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading(true);
        set_error(None);
        set_success(None);

        let cat_name = category_name.get();
        let course = course_id.clone();
        
        spawn_local(async move {
            match map_course_to_category(&course, &cat_name).await {
                Ok(cat) => {
                    set_category(Some(cat));
                    set_success(Some("Category successfully mapped to course".to_string()));
                    set_category_name(String::new());
                },
                Err(err) => {
                    set_error(Some(format!("Failed to map category: {}", err)));
                }
            }
            set_loading(false);
        });
    };

    view! {
        <div class="bg-white shadow overflow-hidden sm:rounded-lg mt-6">
            <div class="px-4 py-5 sm:px-6">
                <h2 class="text-lg leading-6 font-medium text-gray-900">Course-Category Mapping</h2>
                <p class="mt-1 text-sm text-gray-500">
                    "Connect this course to a discussion category"
                </p>
            </div>

            <div class="border-t border-gray-200 px-4 py-5 sm:p-6">
                {move || {
                    if let Some(cat) = category.get() {
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
                                            "This course is mapped to category:"
                                        </p>
                                        <p class="text-sm text-gray-500">{cat.name.clone()}</p>
                                    </div>
                                </div>
                                <button 
                                    class="mt-2 inline-flex items-center px-3 py-1.5 border border-gray-300 shadow-sm text-sm font-medium rounded text-gray-700 bg-white hover:bg-gray-50"
                                    on:click=move |_| {
                                        let course = course_id.clone();
                                        spawn_local(async move {
                                            match unmap_course_category(&course).await {
                                                Ok(_) => {
                                                    set_category(None);
                                                    set_success(Some("Category mapping removed".to_string()));
                                                },
                                                Err(e) => {
                                                    set_error(Some(format!("Failed to remove mapping: {}", e)));
                                                }
                                            }
                                        });
                                    }
                                >
                                    "Remove Mapping"
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <form on:submit=handle_submit>
                                <div>
                                    <label for="category-name" class="block text-sm font-medium text-gray-700">
                                        "Category Name"
                                    </label>
                                    <div class="mt-1">
                                        <input
                                            type="text"
                                            name="category-name"
                                            id="category-name"
                                            class="shadow-sm focus:ring-blue-500 focus:border-blue-500 block w-full sm:text-sm border-gray-300 rounded-md"
                                            placeholder="Enter category name"
                                            value=category_name
                                            on:input=move |ev| {
                                                set_category_name(event_target_value(&ev));
                                            }
                                            disabled=move || loading.get()
                                            required
                                        />
                                    </div>
                                    <p class="mt-2 text-sm text-gray-500">
                                        "The category will be created with this name."
                                    </p>
                                </div>
                                
                                <div class="mt-4">
                                    <button
                                        type="submit"
                                        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                        disabled=move || loading.get()
                                    >
                                        {move || if loading.get() { "Processing..." } else { "Map to Category" }}
                                    </button>
                                </div>
                            </form>
                        }.into_view()
                    }
                }}
                
                {move || {
                    error.get().map(|err| {
                        view! {
                            <div class="mt-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                                {err}
                            </div>
                        }
                    })
                }}
                
                {move || {
                    success.get().map(|msg| {
                        view! {
                            <div class="mt-4 bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded">
                                {msg}
                            </div>
                        }
                    })
                }}
            </div>
        </div>
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Category {
    id: String,
    name: String,
    slug: String,
    description: Option<String>,
    parent_id: Option<String>,
    course_id: Option<String>,
    position: i32,
}

// Helper function to fetch existing category mapping
async fn fetch_existing_category(course_id: &str) -> Result<Option<Category>, String> {
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
    
    // If 404, there's no mapping yet
    if resp.status() == 404 {
        return Ok(None);
    }
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json().map_err(|_| "Failed to parse response as JSON".to_string())?
    )
    .await
    .map_err(|_| "Failed to parse JSON".to_string())?;
    
    let category: Category = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(Some(category))
}

// Helper function to map course to category
async fn map_course_to_category(course_id: &str, category_name: &str) -> Result<Category, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    // Create request body
    let map_request = serde_json::json!({
        "category_name": category_name
    });
    let body = serde_json::to_string(&map_request)
        .map_err(|_| "Failed to serialize request body".to_string())?;
    
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&body)));
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/courses/{}/category", course_id), 
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
    
    let response: serde_json::Value = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    // Extract category from response
    let category: Category = serde_wasm_bindgen::from_value(response.get("category")
        .ok_or_else(|| "Category not found in response".to_string())?
        .clone().into())
        .map_err(|_| "Failed to deserialize category".to_string())?;
    
    Ok(category)
}

// Helper function to unmap course from category
async fn unmap_course_category(course_id: &str) -> Result<(), String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("DELETE");
    
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
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }
    
    Ok(())
}