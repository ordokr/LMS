use leptos::*;
use leptos_router::*;

#[component]
pub fn CourseLayout(cx: Scope, course_id: String, course_title: String) -> impl IntoView {
    view! { cx,
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold mb-4">{course_title}</h1>
            
            <nav class="border-b mb-6">
                <ul class="flex space-x-4">
                    <li>
                        <A 
                            href={format!("/courses/{}/home", course_id)} 
                            class="py-2 px-1 inline-block border-b-2 border-transparent hover:border-blue-500"
                            active_class="border-blue-500 font-medium"
                        >
                            "Home"
                        </A>
                    </li>
                    <li>
                        <A 
                            href={format!("/courses/{}/modules", course_id)} 
                            class="py-2 px-1 inline-block border-b-2 border-transparent hover:border-blue-500"
                            active_class="border-blue-500 font-medium"
                        >
                            "Modules"
                        </A>
                    </li>
                    // ...other navigation links...
                </ul>
            </nav>
            
            <Outlet/>
        </div>
    }
}