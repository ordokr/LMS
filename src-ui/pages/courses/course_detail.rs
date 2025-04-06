use leptos::*;
use leptos_router::use_params;
use serde::Deserialize;
use crate::{
    hooks::use_auth::use_auth,
    components::courses::course_category_mapper::CourseCategoryMapper,
};

#[derive(Deserialize, Clone)]
struct CourseParams {
    course_id: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Course {
    id: String,
    name: String,
    code: String,
    description: Option<String>,
    instructor_id: String,
    is_published: bool,
    created_at: String,
    updated_at: String,
}

#[component]
pub fn CourseDetail() -> impl IntoView {
    let params = use_params::<CourseParams>();
    let course_id = move || {
        params.with(|p| match p {
            Ok(p) => p.course_id.clone(),
            Err(_) => "".to_string(),
        })
    };
    
    let (course, set_course) = create_signal::<Option<Course>>(None);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    
    let auth = use_auth();
    
    // Load course data when component mounts
    create_effect(move |_| {
        let current_id = course_id();
        if current_id.is_empty() {
            set_error(Some("Invalid course ID".to_string()));
            set_loading(false);
            return;
        }
        
        spawn_local(async move {
            set_loading(true);
            
            match fetch_course(&current_id).await {
                Ok(fetched_course) => {
                    set_course(Some(fetched_course));
                    set_error(None);
                },
                Err(err) => {
                    set_error(Some(format!("Error loading course: {}", err)));
                    set_course(None);
                }
            }
            
            set_loading(false);
        });
    });
    
    // Check if user is instructor
    let is_instructor = move || {
        auth.user.get().map(|u| u.role == "instructor").unwrap_or(false)
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
                } else if let Some(c) = course.get() {
                    view! {
                        <div>
                            <div class="bg-white shadow overflow-hidden sm:rounded-lg mb-6">
                                <div class="px-4 py-5 sm:px-6 flex justify-between items-center">
                                    <div>
                                        <h1 class="text-2xl font-bold text-gray-900">{c.name.clone()}</h1>
                                        <p class="mt-1 text-sm text-gray-500">Course Code: {c.code.clone()}</p>
                                    </div>
                                    
                                    {move || {
                                        if is_instructor() {
                                            view! {
                                                <a
                                                    href={format!("/instructor/courses/{}/edit", c.id)}
                                                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
                                                >
                                                    "Edit Course"
                                                </a>
                                            }.into_view()
                                        } else {
                                            view! { <></> }.into_view()
                                        }
                                    }}
                                </div>
                                
                                <div class="border-t border-gray-200">
                                    <dl>
                                        {move || {
                                            c.description.as_ref().map(|desc| {
                                                view! {
                                                    <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                                        <dt class="text-sm font-medium text-gray-500">Description</dt>
                                                        <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">{desc.clone()}</dd>
                                                    </div>
                                                }
                                            })
                                        }}
                                        
                                        <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                            <dt class="text-sm font-medium text-gray-500">Status</dt>
                                            <dd class="mt-1 text-sm sm:mt-0 sm:col-span-2">
                                                {move || {
                                                    if c.is_published {
                                                        view! {
                                                            <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                                                                "Published"
                                                            </span>
                                                        }.into_view()
                                                    } else {
                                                        view! {
                                                            <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-yellow-100 text-yellow-800">
                                                                "Draft"
                                                            </span>
                                                        }.into_view()
                                                    }
                                                }}
                                            </dd>
                                        </div>
                                        
                                        <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                            <dt class="text-sm font-medium text-gray-500">Created</dt>
                                            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                                {format_date(&c.created_at)}
                                            </dd>
                                        </div>
                                        
                                        <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                            <dt class="text-sm font-medium text-gray-500">Last Updated</dt>
                                            <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                                                {format_date(&c.updated_at)}
                                            </dd>
                                        </div>
                                    </dl>
                                </div>
                            </div>
                            
                            // Category mapping section (instructors only)
                            {move || {
                                if is_instructor() {
                                    view! {
                                        <CourseCategoryMapper course_id={c.id.clone()} />
                                    }.into_view()
                                } else {
                                    view! { <></> }.into_view()
                                }
                            }}
                            
                            // More course content would go here
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
                            "Course not found"
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

// Helper function to fetch course data
async fn fetch_course(course_id: &str) -> Result<Course, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("/api/courses/{}", course_id), 
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
    
    let course_response: Course = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(course_response)
}

// Helper function to format date strings
fn format_date(date_str: &str) -> String {
    // Simple formatting - in a real app you might use a date library
    date_str.split('T').next().unwrap_or(date_str).to_string()
}