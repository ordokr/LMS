use leptos::*;
use leptos_router::*;
use crate::models::Category;
use crate::components::auth::AuthData;
use crate::components::shared::OfflineIndicator;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // Get auth data from context
    let auth_data = use_context::<Signal<Option<AuthData>>>(cx)
        .expect("Auth data not provided");
    
    // Create a resource to fetch featured categories
    let categories = create_resource(
        cx,
        || (),
        |_| async move {
            // Fetch categories from our API (limit to featured ones if you want)
            let response = reqwest::get("http://localhost:3030/categories")
                .await
                .expect("Failed to fetch categories");
                
            if response.status().is_success() {
                response.json::<Vec<Category>>().await.ok()
            } else {
                None
            }
        },
    );

    view! { cx,
        <div class="home-page">
            <section class="hero">
                <div class="hero-content">
                    <h1>"Welcome to LMS Forum"</h1>
                    <p>"Join the conversation and connect with fellow students and educators."</p>
                    
                    {move || if auth_data.get().is_none() {
                        view! { cx,
                            <div class="hero-buttons">
                                <A href="/register" class="button primary large">"Join Now"</A>
                                <A href="/login" class="button secondary large">"Login"</A>
                            </div>
                        }.into_view(cx)
                    } else {
                        view! { cx,
                            <div class="hero-buttons">
                                <A href="/categories" class="button primary large">"Browse Forums"</A>
                            </div>
                        }.into_view(cx)
                    }}
                </div>
            </section>
            
            <section class="featured-categories">
                <div class="section-header">
                    <h2>"Featured Categories"</h2>
                    <A href="/categories" class="view-all">"View All"</A>
                </div>
                
                <div class="category-grid">
                    {move || match categories.read(cx) {
                        None => view! { cx, <p>"Loading categories..."</p> }.into_view(cx),
                        Some(None) => view! { cx, <p>"Failed to load categories."</p> }.into_view(cx),
                        Some(Some(categories)) => {
                            // Take just the first few categories to display
                            let featured = categories.iter().take(4);
                            
                            featured.map(|category| {
                                view! { cx,
                                    <div class="category-card">
                                        <a href={format!("/categories/{}", category.id.unwrap_or(0))}>
                                            <h3 class="category-name" style={format!("color: {}", category.text_color.clone().unwrap_or_else(|| "#FFFFFF".to_string()))}
                                                 style={"background-color: " + &category.color.clone().unwrap_or_else(|| "#3498DB".to_string())}>
                                                {&category.name}
                                            </h3>
                                        </a>
                                        <p class="category-description">
                                            {category.description.clone().unwrap_or_else(|| "No description available.".to_string())}
                                        </p>
                                        <a href={format!("/categories/{}", category.id.unwrap_or(0))} class="browse-button">
                                            "Browse Topics"
                                        </a>
                                    </div>
                                }
                            }).collect_view(cx)
                        }
                    }}
                </div>
            </section>
            
            <section class="features">
                <div class="section-header">
                    <h2>"Why Join Our Community"</h2>
                </div>
                
                <div class="features-grid">
                    <div class="feature-card">
                        <div class="feature-icon">💬</div>
                        <h3>"Connect with Peers"</h3>
                        <p>"Engage in discussions with fellow students and educators on topics that matter to you."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">📚</div>
                        <h3>"Share Knowledge"</h3>
                        <p>"Ask questions, share insights, and collaborate on educational content."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">🏆</div>
                        <h3>"Build Reputation"</h3>
                        <p>"Earn trust and recognition within the community through meaningful contributions."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">🔔</div>
                        <h3>"Stay Updated"</h3>
                        <p>"Receive notifications about topics you follow and new content in your areas of interest."</p>
                    </div>
                </div>
            </section>

            <div class="container">
                <h1>"Welcome to LMS Platform"</h1>
                <p>"This is an offline-first Learning Management System with integrated forum capabilities."</p>
                <div class="actions">
                    <a href="/courses" class="button">"Browse Courses"</a>
                    <a href="/forum" class="button">"Visit Forum"</a>
                </div>
                <div class="offline-indicator">
                    <OfflineIndicator/>
                </div>
            </div>
        </div>
    }
}