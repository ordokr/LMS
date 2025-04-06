use leptos::*;
use leptos_router::use_navigate;
use web_sys::SubmitEvent;
use crate::hooks::use_auth::use_auth;

#[component]
pub fn LoginForm() -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal::<Option<String>>(None);
    let (is_loading, set_is_loading) = create_signal(false);
    
    let auth = use_auth();
    let navigate = use_navigate();
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_error(None);
        set_is_loading(true);
        
        spawn_local(async move {
            match auth.login(username.get(), password.get()).await {
                Ok(_) => navigate("/dashboard", None, None),
                Err(err) => {
                    log::error!("Login failed: {:?}", err);
                    set_error(Some("Invalid username or password. Please try again.".to_string()));
                }
            }
            set_is_loading(false);
        });
    };
    
    view! {
        <div class="flex justify-center items-center min-h-screen">
            <div class="p-6 w-full max-w-md rounded-lg shadow-md bg-white">
                <h1 class="text-2xl font-bold mb-2">Sign In</h1>
                <p class="text-gray-600 mb-6">Welcome back! Please sign in to your account</p>
                
                {move || {
                    error.get().map(|err| {
                        view! {
                            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                                {err}
                            </div>
                        }
                    })
                }}
                
                <form on:submit=handle_submit>
                    <div class="mb-4">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="username">
                            "Username"
                        </label>
                        <input
                            id="username"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                            type="text"
                            placeholder="Username"
                            required
                            autofocus
                            disabled=move || is_loading.get()
                            on:input=move |ev| {
                                set_username(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    <div class="mb-6">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="password">
                            "Password"
                        </label>
                        <input
                            id="password"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                            type="password"
                            placeholder="Password"
                            required
                            disabled=move || is_loading.get()
                            on:input=move |ev| {
                                set_password(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded 
                                  focus:outline-none focus:shadow-outline w-full"
                            type="submit"
                            disabled=move || is_loading.get()
                        >
                            {move || if is_loading.get() { "Loading..." } else { "Sign In" }}
                        </button>
                    </div>
                </form>
                
                <hr class="my-6" />
                
                <div class="text-center">
                    <p class="text-sm">
                        "Don't have an account? "
                        <a href="/register" class="text-blue-500 hover:text-blue-700">
                            "Sign up"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}