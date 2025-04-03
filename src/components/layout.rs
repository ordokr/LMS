use leptos::*;
use leptos_router::*;
use crate::components::auth::AuthData;
use crate::components::shared::OfflineIndicator;
use crate::utils::auth::is_authenticated;
use crate::components::forum::search_bar::SearchBar;
use crate::components::forum::notification_indicator::NotificationIndicator;

// Header component with navigation and user menu
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    // Get auth data from context
    let auth_data = use_context::<Signal<Option<AuthData>>>(cx)
        .expect("Auth data not provided");
    
    let logout = use_context::<Callback<()>>(cx)
        .expect("Logout callback not provided");
    
    // State for mobile menu toggle
    let (is_mobile_menu_open, set_mobile_menu_open) = create_signal(cx, false);
    let toggle_mobile_menu = move |_| set_mobile_menu_open.update(|open| *open = !*open);
    
    // State for user dropdown
    let (is_user_dropdown_open, set_user_dropdown_open) = create_signal(cx, false);
    let toggle_user_dropdown = move |_| set_user_dropdown_open.update(|open| *open = !*open);
    
    // Close dropdowns when clicking outside
    let close_dropdowns = move |_| {
        set_user_dropdown_open.set(false);
        // Don't close the mobile menu as it might be the hamburger menu being clicked
    };
    
    // Handle logout
    let handle_logout = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        logout.call(());
    };

    view! { cx,
        <header class="site-header">
            <div class="header-container">
                <div class="header-logo">
                    <A href="/">"LMS Forum"</A>
                </div>
                
                <button class="mobile-menu-toggle" on:click=toggle_mobile_menu>
                    {move || if is_mobile_menu_open.get() { "✕" } else { "☰" }}
                </button>
                
                <nav class={move || {
                    let mut classes = "main-navigation".to_string();
                    if is_mobile_menu_open.get() {
                        classes.push_str(" mobile-menu-open");
                    }
                    classes
                }}>
                    <ul class="nav-links">
                        <li><A href="/">Home</A></li>
                        <li><A href="/categories">Forums</A></li>
                        <li><A href="/search">Search</A></li>
                        // Add more nav items as needed
                    </ul>
                </nav>
                
                <div class="header-actions">
                    <div class="search-box">
                        <input 
                            type="text" 
                            placeholder="Search..." 
                            class="search-input" 
                        />
                        <button class="search-button">🔍</button>
                    </div>
                    
                    {move || match auth_data.get() {
                        Some(data) => {
                            view! { cx,
                                <div class="user-menu">
                                    <button class="user-menu-toggle" on:click=toggle_user_dropdown>
                                        <div class="user-avatar">
                                            {match &data.user.avatar_url {
                                                Some(url) => view! { cx, <img src={url.clone()} alt="User avatar" /> }.into_view(cx),
                                                None => {
                                                    let initial = data.user.display_name.chars().next()
                                                        .unwrap_or('U').to_string();
                                                    view! { cx, <span class="avatar-initial">{initial}</span> }.into_view(cx)
                                                }
                                            }}
                                        </div>
                                        <span class="username">{&data.user.username}</span>
                                        <span class="dropdown-icon">▼</span>
                                    </button>
                                    
                                    {move || if is_user_dropdown_open.get() {
                                        view! { cx,
                                            <div class="user-dropdown" on:click={move |ev| ev.stop_propagation()}>
                                                <ul>
                                                    <li>
                                                        <A href="/profile" on:click=move |_| set_user_dropdown_open.set(false)>
                                                            "My Profile"
                                                        </A>
                                                    </li>
                                                    {if data.user.is_admin {
                                                        view! { cx,
                                                            <li>
                                                                <A href="/admin" on:click=move |_| set_user_dropdown_open.set(false)>
                                                                    "Admin Panel"
                                                                </A>
                                                            </li>
                                                        }.into_view(cx)
                                                    } else { view! {}.into_view(cx) }}
                                                    <li>
                                                        <A href="/profile/settings" on:click=move |_| set_user_dropdown_open.set(false)>
                                                            "Settings"
                                                        </A>
                                                    </li>
                                                    <li class="dropdown-divider"></li>
                                                    <li>
                                                        <a href="#" on:click=handle_logout>
                                                            "Logout"
                                                        </a>
                                                    </li>
                                                </ul>
                                            </div>
                                        }.into_view(cx)
                                    } else {
                                        view! {}.into_view(cx)
                                    }}
                                </div>
                            }.into_view(cx)
                        },
                        None => {
                            view! { cx,
                                <div class="auth-buttons">
                                    <A href="/login" class="login-button">"Login"</A>
                                    <A href="/register" class="register-button">"Register"</A>
                                </div>
                            }.into_view(cx)
                        }
                    }}
                </div>
            </div>
        </header>
        
        // Overlay to close dropdowns when clicking outside
        {move || if is_user_dropdown_open.get() {
            view! { cx, <div class="dropdown-overlay" on:click=close_dropdowns></div> }.into_view(cx)
        } else {
            view! {}.into_view(cx)
        }}
    }
}

// Footer component
#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="site-footer">
            <div class="footer-container">
                <div class="footer-info">
                    <h3>"LMS Forum"</h3>
                    <p>"A community forum for students and educators."</p>
                </div>
                
                <div class="footer-links">
                    <h4>"Quick Links"</h4>
                    <ul>
                        <li><A href="/">Home</A></li>
                        <li><A href="/categories">Forums</A></li>
                        <li><A href="/search">Search</A></li>
                        <li><A href="/about">About Us</A></li>
                        <li><A href="/privacy">Privacy Policy</A></li>
                    </ul>
                </div>
                
                <div class="footer-social">
                    <h4>"Connect With Us"</h4>
                    <div class="social-links">
                        <a href="#" class="social-icon">"Twitter"</a>
                        <a href="#" class="social-icon">"Facebook"</a>
                        <a href="#" class="social-icon">"LinkedIn"</a>
                        <a href="#" class="social-icon">"GitHub"</a>
                    </div>
                </div>
            </div>
            
            <div class="footer-copyright">
                <p>"© 2025 LMS Forum. All rights reserved."</p>
            </div>
        </footer>
    }
}

// Main layout component that wraps all pages
#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="site-container">
            <Header/>
            
            <main class="main-content">
                {children(cx)}
            </main>
            
            <Footer/>
        </div>
    }
}

#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="app-layout">
            <header>
                <h1>"Learning Management System"</h1>
                <nav class="navbar navbar-expand-md navbar-light bg-light">
                    <div class="container-fluid">
                        <a class="navbar-brand" href="/">LMS</a>
                        
                        <!-- Navigation links -->
                        <div class="collapse navbar-collapse">
                            <ul class="navbar-nav me-auto mb-2 mb-md-0">
                                <!-- Your existing nav items -->
                            </ul>
                            
                            <!-- Add search bar here -->
                            <SearchBar/>
                            
                            <!-- User authentication section -->
                        </div>
                        <div class="navbar-nav ms-auto">
                            // User authentication items
                            // ...
                            
                            // Add notification indicator
                            <NotificationIndicator/>
                            
                            // Other navbar items
                        </div>
                    </div>
                </nav>
            </header>
            
            <main>
                {children()}
            </main>
            
            <footer>
                // ... your existing footer ...
            </footer>
        </div>
    }
}