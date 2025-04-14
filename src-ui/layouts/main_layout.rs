use leptos::*;
use leptos_router::{use_navigate, A};
use crate::hooks::use_auth::use_auth;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let show_mobile_menu = create_rw_signal(false);
    
    let toggle_mobile_menu = move |_| {
        show_mobile_menu.update(|val| *val = !*val);
    };
    
    let handle_logout = move |_| {
        auth.logout.dispatch(());
        navigate("/login", Default::default(), None);
    };
    
    // Get user data for display
    let user_display = move || {
        auth.user.get().map(|u| format!("{} ({})", u.username, u.role))
    };
    
    let is_instructor = move || {
        auth.user.get().map(|u| u.role == "instructor").unwrap_or(false)
    };
    
    view! {
        <div class="min-h-screen bg-gray-100">
            // Header
            <header class="bg-white shadow">
                <nav class="mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between h-16">
                        <div class="flex">
                            <div class="flex-shrink-0 flex items-center">
                                <A href="/dashboard" class="text-xl font-bold text-blue-600">LMS</A>
                            </div>
                            <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                                <A 
                                    href="/dashboard"
                                    class="border-transparent text-gray-500 hover:border-blue-500 hover:text-blue-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                                    active_class="border-blue-500 text-blue-700"
                                >
                                    "Dashboard"
                                </A>
                                <A 
                                    href="/courses"
                                    class="border-transparent text-gray-500 hover:border-blue-500 hover:text-blue-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                                    active_class="border-blue-500 text-blue-700"
                                >
                                    "Courses"
                                </A>
                                  // Instructor only links
                                {move || {
                                    if is_instructor() {
                                        view! {
                                            <>
                                                <A 
                                                    href="/instructor/courses/create"
                                                    class="border-transparent text-gray-500 hover:border-blue-500 hover:text-blue-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                                                    active_class="border-blue-500 text-blue-700"
                                                >
                                                    "Create Course"
                                                </A>
                                                <A 
                                                    href="/integrations/discourse"
                                                    class="border-transparent text-gray-500 hover:border-blue-500 hover:text-blue-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                                                    active_class="border-blue-500 text-blue-700"
                                                >
                                                    "Discourse Integration"
                                                </A>
                                            </>
                                        }.into_view()
                                    } else {
                                        view! { <></> }.into_view()
                                    }
                                }}
                            </div>
                        </div>
                        
                        <div class="hidden sm:ml-6 sm:flex sm:items-center">
                            // User profile dropdown
                            <div class="ml-3 relative">
                                <div class="flex items-center">
                                    <span class="text-sm text-gray-600 mr-2">{user_display}</span>
                                    <button 
                                        type="button"
                                        class="bg-blue-500 p-2 rounded-md text-white text-sm"
                                        on:click=handle_logout
                                    >
                                        "Sign out"
                                    </button>
                                </div>
                            </div>
                        </div>
                        
                        // Mobile menu button
                        <div class="flex items-center sm:hidden">
                            <button 
                                type="button"
                                class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-blue-500"
                                aria-expanded="false"
                                on:click=toggle_mobile_menu
                            >
                                <span class="sr-only">"Open main menu"</span>
                                // Menu icon
                                <svg 
                                    class="h-6 w-6" 
                                    xmlns="http://www.w3.org/2000/svg" 
                                    fill="none" 
                                    viewBox="0 0 24 24" 
                                    stroke="currentColor" 
                                    aria-hidden="true"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M4 6h16M4 12h16M4 18h16"
                                    />
                                </svg>
                            </button>
                        </div>
                    </div>
                </nav>
                
                // Mobile menu
                {move || {
                    if show_mobile_menu.get() {
                        view! {
                            <div class="sm:hidden">
                                <div class="pt-2 pb-3 space-y-1">
                                    <A 
                                        href="/dashboard"
                                        class="bg-white text-gray-600 hover:bg-gray-50 hover:text-blue-700 block pl-3 pr-4 py-2 border-l-4 border-transparent text-base font-medium"
                                        active_class="bg-blue-50 border-blue-500 text-blue-700"
                                    >
                                        "Dashboard"
                                    </A>
                                    <A 
                                        href="/courses"
                                        class="bg-white text-gray-600 hover:bg-gray-50 hover:text-blue-700 block pl-3 pr-4 py-2 border-l-4 border-transparent text-base font-medium"
                                        active_class="bg-blue-50 border-blue-500 text-blue-700"
                                    >
                                        "Courses"
                                    </A>
                                    
                                    // Instructor only links
                                    {move || {
                                        if is_instructor() {
                                            view! {
                                                <A 
                                                    href="/instructor/courses/create"
                                                    class="bg-white text-gray-600 hover:bg-gray-50 hover:text-blue-700 block pl-3 pr-4 py-2 border-l-4 border-transparent text-base font-medium"
                                                    active_class="bg-blue-50 border-blue-500 text-blue-700"
                                                >
                                                    "Create Course"
                                                </A>
                                            }.into_view()
                                        } else {
                                            view! { <></> }.into_view()
                                        }
                                    }}
                                    
                                    <div class="pt-4 pb-3 border-t border-gray-200">
                                        <div class="flex items-center px-4">
                                            <div class="ml-3">
                                                <div class="text-sm font-medium text-gray-500">{user_display}</div>
                                            </div>
                                        </div>
                                        <div class="mt-3 space-y-1">
                                            <button
                                                class="block w-full text-left px-4 py-2 text-base font-medium text-gray-500 hover:text-gray-800 hover:bg-gray-100"
                                                on:click=handle_logout
                                            >
                                                "Sign out"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <></> }.into_view()
                    }
                }}
            </header>
            
            // Main content
            <main class="py-6 sm:py-10">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    {children()}
                </div>
            </main>
            
            // Footer
            <footer class="bg-white border-t border-gray-200 mt-12">
                <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                    <p class="text-center text-sm text-gray-500">
                        "Canvas-Discourse LMS Integration © 2025"
                    </p>
                </div>
            </footer>
        </div>
    }
}