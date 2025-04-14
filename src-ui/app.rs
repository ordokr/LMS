use leptos::*;
use leptos_router::{Routes, Route, Router};
use crate::{
    components::{
        auth::{
            login_form::LoginForm,
            register_form::RegisterForm,
            protected_route::ProtectedRoute,
        },
        layouts::main_layout::MainLayout,
    },
    pages::{
        dashboard::Dashboard,
        courses::{
            courses_list::CoursesList,
            course_detail::CourseDetail,
            course_create::CreateCourse,
            course_edit::EditCourse,
        },
        integration::{DiscourseIntegrationPage, CanvasIntegrationPage, IntegrationDashboard},
        not_found::NotFound,
        unauthorized::Unauthorized,
    },
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/login" view=|| view! { <LoginForm /> } />
                <Route path="/register" view=|| view! { <RegisterForm /> } />
                <Route path="/unauthorized" view=|| view! { <Unauthorized /> } />

                // Protected routes
                <Route
                    path="/"
                    view=|| {
                        view! {
                            <ProtectedRoute>
                                <MainLayout />
                            </ProtectedRoute>
                        }
                    }
                >                    <Route path="/" view=|| view! { <Dashboard /> } />
                    <Route path="/dashboard" view=|| view! { <Dashboard /> } />
                    <Route path="/courses" view=|| view! { <CoursesList /> } />
                    <Route path="/courses/:course_id" view=|| view! { <CourseDetail /> } />
                    <Route path="/integrations" view=|| view! { <IntegrationDashboard /> } />
                    <Route path="/integrations/discourse" view=|| view! { <DiscourseIntegrationPage /> } />
                    <Route path="/integrations/canvas" view=|| view! { <CanvasIntegrationPage /> } />
                    <Route path="/integrations/settings" view=|| view! { <IntegrationSettingsPage /> } />
                </Route>

                // Instructor-only routes
                <Route
                    path="/instructor"
                    view=|| {
                        view! {
                            <ProtectedRoute required_role="instructor">
                                <MainLayout />
                            </ProtectedRoute>
                        }
                    }
                >
                    <Route path="/courses/create" view=|| view! { <CreateCourse /> } />
                    <Route path="/courses/:course_id/edit" view=|| view! { <EditCourse /> } />
                </Route>

                // 404 Not Found
                <Route path="/*" view=|| view! { <NotFound /> } />
            </Routes>
        </Router>
    }
}