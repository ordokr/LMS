use leptos::*;
use leptos_router::A;
use serde::Deserialize;
use crate::hooks::use_auth::use_auth;

#[derive(Clone, Debug, Deserialize)]
struct Course {
    id: String,
    name: String,
    code: String,
    description: Option<String>,
    instructor_id: String,
    is_published: bool,
}

#[component]
pub fn CoursesList() -> impl IntoView {
    let (courses, set_courses) = create_signal::<Vec<Course>>(vec![]);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);
    
    let auth = use_auth();
    
    // Check if user is instructor
    let is_instructor = move || {
        auth.user.get().map(|u| u.role == "instructor").unwrap_or(false)
    };
    
    // Load courses when component mounts
    create_effect(move |_| {
        spawn_local(async move {
            set_loading(true);
            
            match fetch_courses().await {
                Ok(fetched_courses) => {
                    set_courses(fetched_courses);
                    set_error(None);
                },
                Err(err) => {
                    set_error(Some(format!("Error loading courses: {}", err)));
                }
            }
            
            set_loading(false);
        });
    });

    view! {
        <div>
            <div class="bg-white shadow px-4 py-5 sm:px-6 mb-6 rounded-lg">
                <div class="flex items-center justify-between">
                    <h1 class="text-xl font-bold text-gray-900">Courses</h1>
                    
                    {move || {
                        if is_instructor() {
                            view! {
                                <A
                                    href="/instructor/courses/create"
                                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
                                >
                                    "Create New Course"
                                </A>
                            }.into_view()
                        } else {
                            view! { <></> }.into_view()
                        }
                    }}
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
                } else if courses.get().is_empty() {
                    view! {
                        <div class="bg-gray-50 p-8 text-center rounded-lg">
                            <h3 class="text-lg font-medium text-gray-900">No courses found</h3>
                            <p class="mt-2 text-gray-500">
                                {move || {
                                    if is_instructor() {
                                        "Create your first course to get started."
                                    } else {
                                        "Enroll in a course to see it here."
                                    }
                                }}
                            </p>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="bg-white shadow overflow-hidden sm:rounded-md">
                            <ul role="list" class="divide-y divide-gray-200">
                                {courses.get().into_iter().map(|course| {
                                    view! {
                                        <li>
                                            <A href={format!("/courses/{}", course.id)} class="block hover:bg-gray-50">
                                                <div class="px-4 py-4 sm:px-6">
                                                    <div class="flex items-center justify-between">
                                                        <p class="text-sm font-medium text-blue-600 truncate">
                                                            {course.name}
                                                        </p>
                                                        <div class="ml-2 flex-shrink-0 flex">
                                                            {if course.is_published {
                                                                view! {
                                                                    <p class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                                                                        "Published"
                                                                    </p>
                                                                }
                                                            } else {
                                                                view! {
                                                                    <p class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-yellow-100 text-yellow-800">
                                                                        "Draft"
                                                                    </p>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                    <div class="mt-2 sm:flex sm:justify-between">
                                                        <div class="sm:flex">
                                                            <p class="flex items-center text-sm text-gray-500">
                                                                {course.code}
                                                            </p>
                                                        </div>
                                                        {course.description.as_ref().map(|desc| {
                                                            view! {
                                                                <p class="mt-2 flex items-center text-sm text-gray-500 sm:mt-0">
                                                                    {format!("{:.50}{}",
                                                                        desc,
                                                                        if desc.len() > 50 { "..." } else { "" }
                                                                    )}
                                                                </p>
                                                            }
                                                        })}
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

// Helper function to fetch all courses
async fn fetch_courses() -> Result<Vec<Course>, String> {
    let auth = use_auth();
    
    // Ensure we have a token
    let token = match auth.token.get() {
        Some(token) => token,
        None => return Err("Authentication required".into()),
    };
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    
    let request = web_sys::Request::new_with_str_and_init(
        "/api/courses", 
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
    
    let courses: Vec<Course> = serde_wasm_bindgen::from_value(json)
        .map_err(|_| "Failed to deserialize response".to_string())?;
    
    Ok(courses)
}