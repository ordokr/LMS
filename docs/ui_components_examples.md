# Ordo UI Components: Code Examples

_Generated on: 2025-04-17_

This document provides concrete code examples for key UI components in the Ordo project, focusing on Leptos components and integration points between Canvas and Discourse functionality.

## Leptos UI Components

### Course List Component

```rust
use leptos::*;
use crate::models::course::Course;
use crate::services::course_service::CourseService;

#[component]
pub fn CourseList() -> impl IntoView {
    // Create a resource that fetches courses
    let courses = create_resource(
        || (),
        |_| async move {
            CourseService::get_courses().await.unwrap_or_default()
        }
    );

    // Render the course list
    view! {
        <div class="course-list-container">
            <h1 class="text-2xl font-bold mb-4">"My Courses"</h1>

            // Show loading state while fetching
            {move || match courses.get() {
                None => view! { <p>"Loading courses..."</p> }.into_view(),
                Some(courses) if courses.is_empty() => view! { <p>"No courses found."</p> }.into_view(),
                Some(courses) => view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        {courses.into_iter().map(|course| view! {
                            <CourseCard course={course} />
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_view()
            }}
        </div>
    }
}

#[component]
fn CourseCard(course: Course) -> impl IntoView {
    let navigate = use_navigate();

    let handle_click = move |_| {
        navigate(&format!("/courses/{}", course.id), NavigateOptions::default());
    };

    view! {
        <div
            class="course-card bg-white rounded-lg shadow-md p-4 hover:shadow-lg transition-shadow cursor-pointer"
            on:click=handle_click
        >
            <h2 class="text-xl font-semibold mb-2">{&course.title}</h2>
            <p class="text-gray-600 mb-4">{&course.description}</p>
            <div class="flex justify-between items-center">
                <span class="text-sm text-gray-500">
                    {format!("{} students", course.enrollment_count)}
                </span>
                <span class="text-sm bg-blue-100 text-blue-800 px-2 py-1 rounded">
                    {&course.status}
                </span>
            </div>
        </div>
    }
}
```

### Assignment Submission Component

```rust
use leptos::*;
use crate::models::assignment::Assignment;
use crate::models::submission::Submission;
use crate::services::submission_service::SubmissionService;

#[component]
pub fn SubmissionForm(
    assignment: Assignment,
    #[prop(optional)] existing_submission: Option<Submission>,
) -> impl IntoView {
    // Form state
    let (content, set_content) = create_signal(
        existing_submission.as_ref().map(|s| s.content.clone()).unwrap_or_default()
    );
    let (files, set_files) = create_signal::<Vec<File>>(vec![]);
    let (submitting, set_submitting) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);

    // Handle form submission
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Validate form
        if content.get().trim().is_empty() && files.get().is_empty() {
            set_error(Some("Please provide content or attach files".to_string()));
            return;
        }

        set_submitting(true);
        set_error(None);

        // Create submission data
        let submission_data = SubmissionData {
            assignment_id: assignment.id.clone(),
            content: content.get(),
            files: files.get(),
        };

        // Submit asynchronously
        spawn_local(async move {
            match SubmissionService::submit(submission_data).await {
                Ok(_) => {
                    // Redirect to assignment page on success
                    use_navigate()(&format!("/assignments/{}", assignment.id), NavigateOptions::default());
                }
                Err(e) => {
                    set_error(Some(format!("Error submitting: {}", e)));
                    set_submitting(false);
                }
            }
        });
    };

    // Handle file selection
    let handle_file_change = move |ev: ev::Event| {
        let input = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(files_list) = input.files() {
            let mut selected_files = Vec::new();
            for i in 0..files_list.length() {
                if let Some(file) = files_list.get(i) {
                    selected_files.push(file);
                }
            }
            set_files(selected_files);
        }
    };

    view! {
        <form class="submission-form space-y-6" on:submit=handle_submit>
            <div>
                <label for="content" class="block text-sm font-medium text-gray-700">
                    "Submission Content"
                </label>
                <textarea
                    id="content"
                    name="content"
                    rows="6"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                    placeholder="Enter your submission here..."
                    prop:value=move || content.get()
                    on:input=move |ev| set_content(event_target_value(&ev))
                ></textarea>
            </div>

            <div>
                <label for="files" class="block text-sm font-medium text-gray-700">
                    "Attachments"
                </label>
                <input
                    type="file"
                    id="files"
                    name="files"
                    multiple
                    class="mt-1 block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
                    on:change=handle_file_change
                />
                <p class="mt-2 text-sm text-gray-500">
                    "Attach files related to your submission"
                </p>

                // Show selected files
                <Show when=move || !files.get().is_empty()>
                    <div class="mt-2 space-y-2">
                        <p class="text-sm font-medium text-gray-700">"Selected Files:"</p>
                        <ul class="list-disc pl-5 space-y-1">
                            {move || files.get().into_iter().map(|file| view! {
                                <li class="text-sm text-gray-600">{file.name()}</li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    </div>
                </Show>
            </div>

            // Show error message if any
            <Show when=move || error.get().is_some()>
                <div class="rounded-md bg-red-50 p-4">
                    <div class="flex">
                        <div class="text-sm text-red-700">
                            {move || error.get().unwrap_or_default()}
                        </div>
                    </div>
                </div>
            </Show>

            <div class="flex justify-end">
                <button
                    type="submit"
                    class="inline-flex justify-center rounded-md border border-transparent bg-blue-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    disabled=move || submitting.get()
                >
                    {move || if submitting.get() { "Submitting..." } else { "Submit Assignment" }}
                </button>
            </div>
        </form>
    }
}
```

### Discussion Forum Component

This component demonstrates the integration between Canvas discussions and Discourse forums:

```rust
use leptos::*;
use crate::models::discussion::{Discussion, DiscussionPost};
use crate::services::discussion_service::DiscussionService;

#[component]
pub fn DiscussionBoard(course_id: String) -> impl IntoView {
    // Fetch discussions for this course
    let discussions = create_resource(
        move || course_id.clone(),
        |course_id| async move {
            DiscussionService::get_discussions_for_course(&course_id).await.unwrap_or_default()
        }
    );

    // Create a new discussion
    let (show_new_form, set_show_new_form) = create_signal(false);

    view! {
        <div class="discussion-board">
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold">"Course Discussions"</h1>
                <button
                    class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md"
                    on:click=move |_| set_show_new_form.update(|v| *v = !*v)
                >
                    {move || if show_new_form.get() { "Cancel" } else { "New Discussion" }}
                </button>
            </div>

            // New discussion form
            <Show when=move || show_new_form.get()>
                <NewDiscussionForm
                    course_id=course_id.clone()
                    on_created=move |_| {
                        set_show_new_form(false);
                        discussions.refetch();
                    }
                />
            </Show>

            // Discussions list
            <div class="mt-6">
                {move || match discussions.get() {
                    None => view! { <p>"Loading discussions..."</p> }.into_view(),
                    Some(discussions) if discussions.is_empty() => view! {
                        <div class="text-center py-8 bg-gray-50 rounded-lg">
                            <p class="text-gray-500">"No discussions yet. Start a new discussion!"</p>
                        </div>
                    }.into_view(),
                    Some(discussions) => view! {
                        <div class="space-y-4">
                            {discussions.into_iter().map(|discussion| view! {
                                <DiscussionCard discussion={discussion} />
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_view()
                }}
            </div>
        </div>
    }
}

#[component]
fn DiscussionCard(discussion: Discussion) -> impl IntoView {
    let navigate = use_navigate();

    let handle_click = move |_| {
        navigate(&format!("/discussions/{}", discussion.id), NavigateOptions::default());
    };

    // Format date for display
    let formatted_date = {
        let date = discussion.created_at;
        format!("{}-{:02}-{:02}", date.year(), date.month(), date.day())
    };

    view! {
        <div
            class="discussion-card bg-white rounded-lg shadow p-4 hover:shadow-md transition-shadow cursor-pointer"
            on:click=handle_click
        >
            <div class="flex justify-between items-start">
                <h3 class="text-lg font-semibold">{&discussion.title}</h3>
                <span class="text-sm bg-blue-100 text-blue-800 px-2 py-1 rounded">
                    {format!("{} replies", discussion.reply_count)}
                </span>
            </div>
            <p class="text-gray-600 mt-2 line-clamp-2">{&discussion.message}</p>
            <div class="flex justify-between items-center mt-4 text-sm text-gray-500">
                <div class="flex items-center">
                    <img
                        src={discussion.author.avatar_url}
                        alt="Author avatar"
                        class="w-6 h-6 rounded-full mr-2"
                    />
                    <span>{&discussion.author.name}</span>
                </div>
                <span>{formatted_date}</span>
            </div>
        </div>
    }
}

#[component]
fn NewDiscussionForm(
    course_id: String,
    #[prop(into)] on_created: Callback<Discussion>,
) -> impl IntoView {
    // Form state
    let (title, set_title) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (submitting, set_submitting) = create_signal(false);
    let (error, set_error) = create_signal::<Option<String>>(None);

    // Handle form submission
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Validate form
        if title.get().trim().is_empty() {
            set_error(Some("Title is required".to_string()));
            return;
        }

        if message.get().trim().is_empty() {
            set_error(Some("Message is required".to_string()));
            return;
        }

        set_submitting(true);
        set_error(None);

        // Create discussion data
        let discussion_data = NewDiscussion {
            course_id: course_id.clone(),
            title: title.get(),
            message: message.get(),
        };

        // Submit asynchronously
        let on_created_callback = on_created.clone();
        spawn_local(async move {
            match DiscussionService::create_discussion(discussion_data).await {
                Ok(discussion) => {
                    on_created_callback(discussion);
                }
                Err(e) => {
                    set_error(Some(format!("Error creating discussion: {}", e)));
                    set_submitting(false);
                }
            }
        });
    };

    view! {
        <form class="new-discussion-form bg-gray-50 p-4 rounded-lg space-y-4" on:submit=handle_submit>
            <div>
                <label for="title" class="block text-sm font-medium text-gray-700">
                    "Title"
                </label>
                <input
                    type="text"
                    id="title"
                    name="title"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                    placeholder="Discussion title"
                    prop:value=move || title.get()
                    on:input=move |ev| set_title(event_target_value(&ev))
                />
            </div>

            <div>
                <label for="message" class="block text-sm font-medium text-gray-700">
                    "Message"
                </label>
                <textarea
                    id="message"
                    name="message"
                    rows="4"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                    placeholder="What would you like to discuss?"
                    prop:value=move || message.get()
                    on:input=move |ev| set_message(event_target_value(&ev))
                ></textarea>
            </div>

            // Show error message if any
            <Show when=move || error.get().is_some()>
                <div class="rounded-md bg-red-50 p-4">
                    <div class="flex">
                        <div class="text-sm text-red-700">
                            {move || error.get().unwrap_or_default()}
                        </div>
                    </div>
                </div>
            </Show>

            <div class="flex justify-end">
                <button
                    type="submit"
                    class="inline-flex justify-center rounded-md border border-transparent bg-blue-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    disabled=move || submitting.get()
                >
                    {move || if submitting.get() { "Creating..." } else { "Create Discussion" }}
                </button>
            </div>
        </form>
    }
}
```

## Integration Components

### Canvas-Discourse Integration Component

This component demonstrates how Canvas and Discourse functionality is integrated:

```rust
use leptos::*;
use crate::models::course::Course;
use crate::models::discussion::Discussion;
use crate::models::forum::ForumTopic;
use crate::services::integration_service::IntegrationService;

#[component]
pub fn CourseForumIntegration(course_id: String) -> impl IntoView {
    // Fetch course data
    let course = create_resource(
        move || course_id.clone(),
        |course_id| async move {
            IntegrationService::get_course_with_forum_data(&course_id).await.ok()
        }
    );

    // Toggle between course discussions and forum view
    let (view_mode, set_view_mode) = create_signal(ViewMode::Discussions);

    view! {
        <div class="course-forum-integration">
            // Course header
            {move || course.get().map(|course_data| {
                view! {
                    <div class="course-header bg-white rounded-lg shadow-md p-6 mb-6">
                        <h1 class="text-2xl font-bold">{course_data.as_ref().map(|c| &c.course.title).unwrap_or(&"Loading...".to_string())}</h1>
                        <p class="text-gray-600 mt-2">{course_data.as_ref().map(|c| &c.course.description).unwrap_or(&"".to_string())}</p>
                    </div>
                }
            })}

            // View toggle
            <div class="flex border-b border-gray-200 mb-6">
                <button
                    class=move || format!(
                        "py-4 px-6 font-medium text-sm {} {}",
                        if view_mode.get() == ViewMode::Discussions {
                            "text-blue-600 border-b-2 border-blue-600"
                        } else {
                            "text-gray-500 hover:text-gray-700"
                        },
                        if view_mode.get() == ViewMode::Discussions {
                            "bg-blue-50"
                        } else {
                            ""
                        }
                    )
                    on:click=move |_| set_view_mode(ViewMode::Discussions)
                >
                    "Course Discussions"
                </button>
                <button
                    class=move || format!(
                        "py-4 px-6 font-medium text-sm {} {}",
                        if view_mode.get() == ViewMode::Forum {
                            "text-blue-600 border-b-2 border-blue-600"
                        } else {
                            "text-gray-500 hover:text-gray-700"
                        },
                        if view_mode.get() == ViewMode::Forum {
                            "bg-blue-50"
                        } else {
                            ""
                        }
                    )
                    on:click=move |_| set_view_mode(ViewMode::Forum)
                >
                    "Community Forum"
                </button>
            </div>

            // Content based on view mode
            {move || match view_mode.get() {
                ViewMode::Discussions => view! {
                    <CourseDiscussions course_id=course_id.clone() />
                }.into_view(),
                ViewMode::Forum => view! {
                    <CourseForum course_id=course_id.clone() />
                }.into_view(),
            }}
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Discussions,
    Forum,
}

#[component]
fn CourseDiscussions(course_id: String) -> impl IntoView {
    // Fetch course discussions (Canvas-style)
    let discussions = create_resource(
        move || course_id.clone(),
        |course_id| async move {
            IntegrationService::get_course_discussions(&course_id).await.unwrap_or_default()
        }
    );

    view! {
        <div class="course-discussions">
            <h2 class="text-xl font-semibold mb-4">"Course Discussions"</h2>

            // Discussions list
            {move || match discussions.get() {
                None => view! { <p>"Loading discussions..."</p> }.into_view(),
                Some(discussions) if discussions.is_empty() => view! {
                    <div class="text-center py-8 bg-gray-50 rounded-lg">
                        <p class="text-gray-500">"No discussions yet."</p>
                    </div>
                }.into_view(),
                Some(discussions) => view! {
                    <div class="space-y-4">
                        {discussions.into_iter().map(|discussion| view! {
                            <div class="bg-white rounded-lg shadow p-4">
                                <h3 class="font-medium">{&discussion.title}</h3>
                                <p class="text-sm text-gray-600 mt-1">{&discussion.message}</p>
                                <div class="flex justify-between items-center mt-3 text-xs text-gray-500">
                                    <span>{&discussion.author.name}</span>
                                    <span>{format!("{} replies", discussion.reply_count)}</span>
                                </div>
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_view()
            }}
        </div>
    }
}

#[component]
fn CourseForum(course_id: String) -> impl IntoView {
    // Fetch forum topics (Discourse-style)
    let topics = create_resource(
        move || course_id.clone(),
        |course_id| async move {
            IntegrationService::get_course_forum_topics(&course_id).await.unwrap_or_default()
        }
    );

    view! {
        <div class="course-forum">
            <h2 class="text-xl font-semibold mb-4">"Community Forum"</h2>

            // Topics list
            {move || match topics.get() {
                None => view! { <p>"Loading forum topics..."</p> }.into_view(),
                Some(topics) if topics.is_empty() => view! {
                    <div class="text-center py-8 bg-gray-50 rounded-lg">
                        <p class="text-gray-500">"No forum topics yet."</p>
                    </div>
                }.into_view(),
                Some(topics) => view! {
                    <div class="space-y-4">
                        {topics.into_iter().map(|topic| view! {
                            <div class="bg-white rounded-lg shadow p-4">
                                <div class="flex items-start">
                                    <img
                                        src={topic.author.avatar_url}
                                        alt="Author avatar"
                                        class="w-10 h-10 rounded-full mr-3"
                                    />
                                    <div>
                                        <h3 class="font-medium">{&topic.title}</h3>
                                        <p class="text-sm text-gray-600 mt-1 line-clamp-2">{&topic.excerpt}</p>
                                        <div class="flex items-center mt-3 text-xs text-gray-500 space-x-4">
                                            <span>{&topic.author.name}</span>
                                            <span>{format!("{} replies", topic.reply_count)}</span>
                                            <span>{format!("{} views", topic.views)}</span>
                                            {topic.tags.iter().map(|tag| view! {
                                                <span class="bg-gray-100 px-2 py-1 rounded">{tag}</span>
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_view()
            }}
        </div>
    }
}
```

## Offline-First Components

This component demonstrates the offline-first capabilities:

```rust
use leptos::*;
use crate::models::sync::SyncStatus;
use crate::services::sync_service::SyncService;

#[component]
pub fn OfflineStatusBar() -> impl IntoView {
    // Track online status
    let (is_online, set_is_online) = create_signal(true);

    // Track sync status
    let (sync_status, set_sync_status) = create_signal(SyncStatus::Synced);
    let (pending_changes, set_pending_changes) = create_signal(0);

    // Initialize online status and add event listeners
    create_effect(move |_| {
        // Initial status
        set_is_online(web_sys::window().unwrap().navigator().on_line());

        // Set up event listeners for online/offline events
        let online_callback = Closure::wrap(Box::new(move || {
            set_is_online(true);
        }) as Box<dyn FnMut()>);

        let offline_callback = Closure::wrap(Box::new(move || {
            set_is_online(false);
        }) as Box<dyn FnMut()>);

        let window = web_sys::window().unwrap();
        window.add_event_listener_with_callback("online", online_callback.as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("offline", offline_callback.as_ref().unchecked_ref()).unwrap();

        // Keep callbacks alive
        online_callback.forget();
        offline_callback.forget();
    });

    // Set up interval to check sync status
    create_effect(move |_| {
        let interval_id = window().set_interval_with_callback_and_timeout_and_arguments(
            Closure::wrap(Box::new(move || {
                spawn_local(async move {
                    if let Ok(status) = SyncService::get_sync_status().await {
                        set_sync_status(status.status);
                        set_pending_changes(status.pending_changes);
                    }
                });
            }) as Box<dyn FnMut()>).into_js_value().unchecked_ref(),
            5000, // Check every 5 seconds
            &js_sys::Array::new(),
        ).unwrap();

        on_cleanup(move || {
            window().clear_interval_with_handle(interval_id);
        });
    });

    // Trigger manual sync
    let trigger_sync = move |_| {
        spawn_local(async move {
            set_sync_status(SyncStatus::Syncing);
            if let Err(e) = SyncService::sync_now().await {
                log::error!("Sync error: {:?}", e);
                set_sync_status(SyncStatus::Error);
            }
        });
    };

    view! {
        <div class=move || format!(
            "offline-status-bar fixed bottom-0 left-0 right-0 px-4 py-2 text-sm flex justify-between items-center {}",
            match (is_online.get(), sync_status.get()) {
                (false, _) => "bg-yellow-100 text-yellow-800",
                (true, SyncStatus::Synced) => "bg-green-100 text-green-800",
                (true, SyncStatus::Syncing) => "bg-blue-100 text-blue-800",
                (true, SyncStatus::Pending) => "bg-yellow-100 text-yellow-800",
                (true, SyncStatus::Error) => "bg-red-100 text-red-800",
            }
        )>
            <div class="flex items-center">
                <div class=move || format!(
                    "w-3 h-3 rounded-full mr-2 {}",
                    if is_online.get() { "bg-green-500" } else { "bg-yellow-500" }
                )></div>
                <span>
                    {move || match (is_online.get(), sync_status.get()) {
                        (false, _) => "You're offline. Changes will sync when you reconnect.",
                        (true, SyncStatus::Synced) => "All changes synced.",
                        (true, SyncStatus::Syncing) => "Syncing changes...",
                        (true, SyncStatus::Pending) => format!("{} changes pending sync.", pending_changes.get()),
                        (true, SyncStatus::Error) => "Sync error. Click to retry.",
                    }}
                </span>
            </div>
            <Show when=move || is_online.get() && (sync_status.get() == SyncStatus::Pending || sync_status.get() == SyncStatus::Error)>
                <button
                    class="text-xs bg-white bg-opacity-20 hover:bg-opacity-30 px-2 py-1 rounded"
                    on:click=trigger_sync
                >
                    "Sync Now"
                </button>
            </Show>
        </div>
    }
}
```

## Conclusion

These code examples demonstrate how to implement key UI components in the Ordo project using Leptos. The examples show:

1. How to create reactive components with Leptos signals and resources
2. How to integrate Canvas and Discourse functionality in a unified interface
3. How to implement offline-first capabilities with sync status tracking
4. How to handle form submissions and user interactions

These examples can be used as a starting point for implementing the UI components in the Ordo project.
