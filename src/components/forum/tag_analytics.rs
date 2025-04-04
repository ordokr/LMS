use leptos::*;
use crate::models::forum::tag::{Tag, TagAnalytics};
use crate::services::forum::ForumService;
use chrono::Duration;

#[component]
pub fn TagAnalytics() -> impl IntoView {
    // State signals
    let (tag_analytics, set_tag_analytics) = create_signal(None::<TagAnalytics>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (time_period, set_time_period) = create_signal("month".to_string());
    
    // Load tag analytics
    create_effect(move |_| {
        set_loading.set(true);
        
        spawn_local(async move {
            match ForumService::get_tag_analytics(&time_period()).await {
                Ok(analytics) => {
                    set_tag_analytics.set(Some(analytics));
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to load tag analytics: {}", e)));
                }
            }
            set_loading.set(false);
        });
    });
    
    view! {
        <div class="tag-analytics">
            <div class="card">
                <div class="card-header d-flex justify-content-between align-items-center">
                    <h3>"Tag Analytics"</h3>
                    <div class="btn-group">
                        <button 
                            class="btn btn-sm btn-outline-secondary" 
                            class:active=move || time_period() == "week"
                            on:click=move |_| set_time_period.set("week".to_string())
                        >"Week"</button>
                        <button 
                            class="btn btn-sm btn-outline-secondary"
                            class:active=move || time_period() == "month"
                            on:click=move |_| set_time_period.set("month".to_string())
                        >"Month"</button>
                        <button 
                            class="btn btn-sm btn-outline-secondary"
                            class:active=move || time_period() == "year"
                            on:click=move |_| set_time_period.set("year".to_string())
                        >"Year"</button>
                    </div>
                </div>
                <div class="card-body">
                    {move || if loading() {
                        view! { <div class="d-flex justify-content-center p-5"><div class="spinner-border" role="status"></div></div> }
                    } else if let Some(analytics) = tag_analytics() {
                        view! {
                            <div class="row">
                                <div class="col-md-6 mb-4">
                                    <h4>"Most Used Tags"</h4>
                                    <table class="table table-sm">
                                        <thead>
                                            <tr>
                                                <th>"Tag"</th>
                                                <th>"Topics"</th>
                                                <th>"Growth"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {analytics.most_used.iter().map(|tag_stat| {
                                                view! {
                                                    <tr>
                                                        <td>
                                                            <span 
                                                                class="tag-preview"
                                                                style={format!("background-color: {}; color: white;", 
                                                                    tag_stat.tag.color.clone().unwrap_or_else(|| "#6c757d".to_string()))}
                                                            >
                                                                {&tag_stat.tag.name}
                                                            </span>
                                                        </td>
                                                        <td>{tag_stat.count}</td>
                                                        <td>
                                                            {if tag_stat.growth_percentage > 0.0 {
                                                                view! {
                                                                    <span class="text-success">
                                                                        <i class="bi bi-arrow-up"></i>
                                                                        {format!("{:.1}%", tag_stat.growth_percentage)}
                                                                    </span>
                                                                }
                                                            } else if tag_stat.growth_percentage < 0.0 {
                                                                view! {
                                                                    <span class="text-danger">
                                                                        <i class="bi bi-arrow-down"></i>
                                                                        {format!("{:.1}%", tag_stat.growth_percentage.abs())}
                                                                    </span>
                                                                }
                                                            } else {
                                                                view! { <span>"-"</span> }
                                                            }}
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>
                                </div>
                                
                                <div class="col-md-6 mb-4">
                                    <h4>"Trending Tags"</h4>
                                    <table class="table table-sm">
                                        <thead>
                                            <tr>
                                                <th>"Tag"</th>
                                                <th>"Views"</th>
                                                <th>"Growth"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {analytics.trending.iter().map(|tag_stat| {
                                                view! {
                                                    <tr>
                                                        <td>
                                                            <span 
                                                                class="tag-preview"
                                                                style={format!("background-color: {}; color: white;", 
                                                                    tag_stat.tag.color.clone().unwrap_or_else(|| "#6c757d".to_string()))}
                                                            >
                                                                {&tag_stat.tag.name}
                                                            </span>
                                                        </td>
                                                        <td>{tag_stat.views}</td>
                                                        <td>
                                                            <span class="text-success">
                                                                <i class="bi bi-arrow-up"></i>
                                                                {format!("{:.1}%", tag_stat.growth_percentage)}
                                                            </span>
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>
                                </div>
                                
                                <div class="col-12">
                                    <h4>"Tag Co-occurrence"</h4>
                                    <p class="text-muted">"Tags that frequently appear together in topics"</p>
                                    <table class="table table-sm">
                                        <thead>
                                            <tr>
                                                <th>"Tag Pair"</th>
                                                <th>"Occurrences"</th>
                                                <th>"Correlation Score"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {analytics.co_occurrence.iter().map(|pair| {
                                                view! {
                                                    <tr>
                                                        <td>
                                                            <span 
                                                                class="tag-preview me-2"
                                                                style={format!("background-color: {}; color: white;", 
                                                                    pair.tag1.color.clone().unwrap_or_else(|| "#6c757d".to_string()))}
                                                            >
                                                                {&pair.tag1.name}
                                                            </span>
                                                            <i class="bi bi-plus"></i>
                                                            <span 
                                                                class="tag-preview ms-2"
                                                                style={format!("background-color: {}; color: white;", 
                                                                    pair.tag2.color.clone().unwrap_or_else(|| "#6c757d".to_string()))}
                                                            >
                                                                {&pair.tag2.name}
                                                            </span>
                                                        </td>
                                                        <td>{pair.count}</td>
                                                        <td>{format!("{:.2}", pair.correlation)}</td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </tbody>
                                    </table>
                                </div>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="alert alert-info">
                                <i class="bi bi-info-circle me-2"></i>
                                "No tag analytics available for the selected time period."
                            </div>
                        }
                    }}
                    
                    {move || error().map(|err| view! {
                        <div class="alert alert-danger mt-3">
                            <i class="bi bi-exclamation-triangle me-2"></i>
                            {err}
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}