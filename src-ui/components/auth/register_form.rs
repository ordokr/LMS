use leptos::*;
use leptos_router::{use_navigate, A};
use web_sys::SubmitEvent;
use crate::hooks::use_auth::{use_auth, RegisterData};

#[component]
pub fn RegisterForm() -> impl IntoView {
    let (form_data, set_form_data) = create_signal(RegisterData {
        username: String::new(),
        email: String::new(),
        password: String::new(),
        canvas_id: String::new(),
    });
    
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (error, set_error) = create_signal::<Option<String>>(None);
    
    let auth = use_auth();
    let navigate = use_navigate();
    
    let is_loading = create_memo(move |_| {
        auth.register.pending().get()
    });
    
    // Validation function
    let validate = move || {
        if form_data.get().password != confirm_password.get() {
            set_error(Some("Passwords don't match".to_string()));
            return false;
        }
        
        if form_data.get().password.len() < 8 {
            set_error(Some("Password must be at least 8 characters long".to_string()));
            return false;
        }
        
        true
    };
    
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_error(None);
        
        if !validate() {
            return;
        }
        
        let data = form_data.get().clone();
        
        auth.register.dispatch(data);
        
        create_effect(move |_| {
            match auth.register.value().get() {
                Some(Ok(_)) => {
                    navigate("/login", None, None);
                },
                Some(Err(err)) => {
                    set_error(Some(err));
                },
                None => {}
            }
        });
    };
    
    let handle_input = move |field: &'static str, value: String| {
        let mut data = form_data.get();
        match field {
            "username" => data.username = value,
            "email" => data.email = value,
            "password" => data.password = value,
            "canvas_id" => data.canvas_id = value,
            _ => {},
        }
        set_form_data(data);
    };
    
    view! {
        <div class="flex justify-center items-center min-h-screen py-8">
            <div class="p-6 w-full max-w-md rounded-lg shadow-md bg-white">
                <h1 class="text-2xl font-bold mb-2">Create an Account</h1>
                <p class="text-gray-600 mb-6">Join our learning platform today</p>
                
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
                    <div class="space-y-4">
                        <div>
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
                                    handle_input("username", event_target_value(&ev));
                                }
                            />
                        </div>
                        
                        <div>
                            <label class="block text-gray-700 text-sm font-bold mb-2" for="email">
                                "Email Address"
                            </label>
                            <input
                                id="email"
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                                type="email"
                                placeholder="Email"
                                required
                                disabled=move || is_loading.get()
                                on:input=move |ev| {
                                    handle_input("email", event_target_value(&ev));
                                }
                            />
                        </div>
                        
                        <div>
                            <label class="block text-gray-700 text-sm font-bold mb-2" for="canvas_id">
                                "Canvas ID"
                            </label>
                            <input
                                id="canvas_id"
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                                type="text"
                                placeholder="Canvas ID"
                                required
                                disabled=move || is_loading.get()
                                on:input=move |ev| {
                                    handle_input("canvas_id", event_target_value(&ev));
                                }
                            />
                            <p class="text-xs text-gray-500 mt-1">
                                "Your Canvas LMS ID for integration"
                            </p>
                        </div>
                        
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div>
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
                                        handle_input("password", event_target_value(&ev));
                                    }
                                />
                            </div>
                            
                            <div>
                                <label class="block text-gray-700 text-sm font-bold mb-2" for="confirm_password">
                                    "Confirm Password"
                                </label>
                                <input
                                    id="confirm_password"
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700"
                                    type="password"
                                    placeholder="Confirm Password"
                                    required
                                    disabled=move || is_loading.get()
                                    on:input=move |ev| {
                                        set_confirm_password(event_target_value(&ev));
                                    }
                                />
                            </div>
                        </div>
                    </div>
                    
                    <button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded 
                              focus:outline-none focus:shadow-outline w-full mt-6"
                        type="submit"
                        disabled=move || is_loading.get()
                    >
                        {move || if is_loading.get() { "Signing up..." } else { "Sign Up" }}
                    </button>
                </form>
                
                <hr class="my-6" />
                
                <div class="text-center">
                    <p class="text-sm">
                        "Already have an account? "
                        <A href="/login" class="text-blue-500 hover:text-blue-700">
                            "Sign in"
                        </A>
                    </p>
                </div>
            </div>
        </div>
    }
}