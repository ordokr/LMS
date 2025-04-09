use leptos::*;
use crate::analytics::performance_tracker::{use_performance_summary, PerformanceSummary};
use std::collections::HashMap;

// Performance dashboard for admins/developers
#[component]
pub fn PerformanceDashboard() -> impl IntoView {
    let summary = use_performance_summary();
    
    // Show different views based on selected tab
    let (active_tab, set_active_tab) = create_signal("overview");
    
    // Chart data
    let chart_data = create_memo(move |_| {
        // Create chart-ready data from performance summary
        let summary = summary.get();
        
        // Component render times for bar chart
        let component_data = summary.components.iter()
            .filter(|(_, metrics)| metrics.count > 0)
            .map(|(name, metrics)| (name.clone(), metrics.avg_render_time_ms))
            .collect::<Vec<_>>();
            
        // Resource timing data
        let resource_types = summary.resources.total_count.min(20);
        
        ChartData {
            component_render_times: component_data,
            resource_count: resource_types,
        }
    });
    
    view! {
        <div class="performance-dashboard">
            <h2>"Forum Performance Dashboard"</h2>
            
            <div class="dashboard-tabs">
                <button 
                    class=move || if active_tab.get() == "overview" { "tab-active" } else { "" }
                    on:click=move |_| set_active_tab.set("overview")
                >
                    "Overview"
                </button>
                <button 
                    class=move || if active_tab.get() == "components" { "tab-active" } else { "" }
                    on:click=move |_| set_active_tab.set("components")
                >
                    "Component Performance"
                </button>
                <button 
                    class=move || if active_tab.get() == "resources" { "tab-active" } else { "" }
                    on:click=move |_| set_active_tab.set("resources")
                >
                    "Resource Loading"
                </button>
                <button 
                    class=move || if active_tab.get() == "interactions" { "tab-active" } else { "" }
                    on:click=move |_| set_active_tab.set("interactions")
                >
                    "User Interactions"
                </button>
            </div>
            
            <div class="dashboard-content">
                {move || match active_tab.get().as_str() {
                    "overview" => view! {
                        <OverviewPanel summary=summary />
                    },
                    "components" => view! {
                        <ComponentsPanel summary=summary />
                    },
                    "resources" => view! {
                        <ResourcesPanel summary=summary />
                    },
                    "interactions" => view! {
                        <InteractionsPanel summary=summary />
                    },
                    _ => view! { <div>"Select a tab"</div> },
                }}
            </div>
        </div>
    }
}

// Overview performance panel
#[component]
fn OverviewPanel(
    #[prop(into)] summary: Signal<PerformanceSummary>
) -> impl IntoView {
    // Format numbers with 2 decimal places
    let format_ms = |ms: f64| format!("{:.2}ms", ms);
    
    // Calculate health score based on web vitals
    let health_score = create_memo(move |_| {
        let summary = summary.get();
        let mut score = 100.0;
        
        // Penalize for slow LCP (should be under 2.5s)
        if let Some(lcp) = summary.web_vitals.lcp {
            if lcp > 2500.0 { score -= ((lcp - 2500.0) / 100.0).min(30.0); }
        }
        
        // Penalize for slow FID (should be under 100ms)
        if let Some(fid) = summary.web_vitals.fid {
            if fid > 100.0 { score -= ((fid - 100.0) / 10.0).min(30.0); }
        }
        
        // Penalize for high CLS (should be under 0.1)
        if let Some(cls) = summary.web_vitals.cls {
            if cls > 0.1 { score -= ((cls - 0.1) / 0.01).min(30.0); }
        }
        
        // Penalize for slow resources
        if summary.resources.avg_load_time > 500.0 {
            score -= ((summary.resources.avg_load_time - 500.0) / 100.0).min(20.0);
        }
        
        // Ensure score is in range 0-100
        score.max(0.0).min(100.0)
    });
    
    // Health status text and color
    let health_status = move || {
        let score = health_score.get();
        if score > 90.0 {
            ("Excellent", "status-excellent")
        } else if score > 70.0 {
            ("Good", "status-good")
        } else if score > 50.0 {
            ("Fair", "status-fair")
        } else {
            ("Poor", "status-poor")
        }
    };
    
    view! {
        <div class="overview-panel">
            <div class="health-score-card">
                <h3>"Forum Performance Health"</h3>
                <div class="health-score">
                    <div class=move || format!("score-circle {}", health_status().1)>
                        {move || format!("{:.0}", health_score.get())}
                    </div>
                    <div class="health-label">
                        {move || health_status().0}
                    </div>
                </div>
            </div>
            
            <div class="web-vitals-card">
                <h3>"Core Web Vitals"</h3>
                <table class="metrics-table">
                    <thead>
                        <tr>
                            <th>"Metric"</th>
                            <th>"Value"</th>
                            <th>"Status"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"LCP"</td>
                            <td>{move || summary.get().web_vitals.lcp.map_or("N/A".to_string(), |v| format!("{:.2}ms", v))}</td>
                            <td class=move || {
                                match summary.get().web_vitals.lcp {
                                    Some(lcp) if lcp <= 2500.0 => "metric-good",
                                    Some(_) => "metric-poor",
                                    None => "",
                                }
                            }>{move || {
                                match summary.get().web_vitals.lcp {
                                    Some(lcp) if lcp <= 2500.0 => "Good",
                                    Some(_) => "Needs Improvement",
                                    None => "Not Available",
                                }
                            }}</td>
                        </tr>
                        <tr>
                            <td>"FID"</td>
                            <td>{move || summary.get().web_vitals.fid.map_or("N/A".to_string(), |v| format!("{:.2}ms", v))}</td>
                            <td class=move || {
                                match summary.get().web_vitals.fid {
                                    Some(fid) if fid <= 100.0 => "metric-good",
                                    Some(_) => "metric-poor",
                                    None => "",
                                }
                            }>{move || {
                                match summary.get().web_vitals.fid {
                                    Some(fid) if fid <= 100.0 => "Good",
                                    Some(_) => "Needs Improvement",
                                    None => "Not Available",
                                }
                            }}</td>
                        </tr>
                        <tr>
                            <td>"CLS"</td>
                            <td>{move || summary.get().web_vitals.cls.map_or("N/A".to_string(), |v| format!("{:.3}", v))}</td>
                            <td class=move || {
                                match summary.get().web_vitals.cls {
                                    Some(cls) if cls <= 0.1 => "metric-good",
                                    Some(_) => "metric-poor",
                                    None => "",
                                }
                            }>{move || {
                                match summary.get().web_vitals.cls {
                                    Some(cls) if cls <= 0.1 => "Good",
                                    Some(_) => "Needs Improvement",
                                    None => "Not Available",
                                }
                            }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            
            <div class="summary-cards">
                <div class="summary-card">
                    <h4>"Page Navigation"</h4>
                    {move || {
                        if let Some(nav) = summary.get().navigation {
                            view! {
                                <div class="card-metrics">
                                    <div class="metric">
                                        <span class="metric-label">"TTFB: "</span>
                                        <span>{format_ms(nav.ttfb)}</span>
                                    </div>
                                    <div class="metric">
                                        <span class="metric-label">"DOM Interactive: "</span>
                                        <span>{format_ms(nav.dom_interactive)}</span>
                                    </div>
                                    <div class="metric">
                                        <span class="metric-label">"Load Complete: "</span>
                                        <span>{format_ms(nav.load_event)}</span>
                                    </div>
                                </div>
                            }
                        } else {
                            view! { <p>"No navigation timing available"</p> }
                        }
                    }}
                </div>
                
                <div class="summary-card">
                    <h4>"Resources"</h4>
                    <div class="card-metrics">
                        <div class="metric">
                            <span class="metric-label">"Total: "</span>
                            <span>{move || summary.get().resources.total_count}</span>
                        </div>
                        <div class="metric">
                            <span class="metric-label">"Cached: "</span>
                            <span>{move || format!("{} ({}%)", 
                                summary.get().resources.cached_count,
                                if summary.get().resources.total_count > 0 {
                                    (summary.get().resources.cached_count as f64 * 100.0 / summary.get().resources.total_count as f64) as u32
                                } else { 0 }
                            )}</span>
                        </div>
                        <div class="metric">
                            <span class="metric-label">"Avg Load Time: "</span>
                            <span>{move || format_ms(summary.get().resources.avg_load_time)}</span>
                        </div>
                    </div>
                </div>
                
                <div class="summary-card">
                    <h4>"Components"</h4>
                    <div class="card-metrics">
                        <div class="metric">
                            <span class="metric-label">"Rendered: "</span>
                            <span>{move || summary.get().components.len()}</span>
                        </div>
                        <div class="metric">
                            <span class="metric-label">"Total Renders: "</span>
                            <span>{move || summary.get().components.values().map(|m| m.count).sum::<u32>()}</span>
                        </div>
                        <div class="metric">
                            <span class="metric-label">"Avg Render Time: "</span>
                            <span>{move || {
                                let components = summary.get().components;
                                if components.is_empty() {
                                    "N/A".to_string()
                                } else {
                                    let sum: f64 = components.values().map(|m| m.avg_render_time_ms).sum();
                                    format_ms(sum / components.len() as f64)
                                }
                            }}</span>
                        </div>
                    </div>
                </div>
                
                <div class="summary-card">
                    <h4>"User Interactions"</h4>
                    <div class="card-metrics">
                        <div class="metric">
                            <span class="metric-label">"Count: "</span>
                            <span>{move || summary.get().interactions.count}</span>
                        </div>
                        <div class="metric">
                            <span class="metric-label">"Response Time: "</span>
                            <span>{move || {
                                if summary.get().interactions.count > 0 {
                                    format_ms(summary.get().interactions.avg_response_time)
                                } else {
                                    "N/A".to_string()
                                }
                            }}</span>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="optimization-suggestions">
                <h3>"Optimization Suggestions"</h3>
                <ul>
                    {move || {
                        let mut suggestions = Vec::new();
                        let summary = summary.get();
                        
                        // Suggest optimizations based on metrics
                        if let Some(lcp) = summary.web_vitals.lcp {
                            if lcp > 2500.0 {
                                suggestions.push("Improve Largest Contentful Paint (LCP) by optimizing critical resources and reducing render-blocking scripts.");
                            }
                        }
                        
                        if let Some(fid) = summary.web_vitals.fid {
                            if fid > 100.0 {
                                suggestions.push("Improve First Input Delay (FID) by breaking up long tasks and minimizing main thread work.");
                            }
                        }
                        
                        if summary.resources.avg_load_time > 500.0 {
                            suggestions.push("Optimize resource loading by using compression, reducing resource size, or implementing better caching strategies.");
                        }
                        
                        if summary.resources.cached_count as f64 / summary.resources.total_count.max(1) as f64 < 0.5 {
                            suggestions.push("Improve caching strategy - less than 50% of resources are served from cache.");
                        }
                        
                        // Find slow components
                        let slow_components: Vec<_> = summary.components.iter()
                            .filter(|(_, metrics)| metrics.avg_render_time_ms > 50.0)
                            .map(|(name, _)| name)
                            .collect();
                            
                        if !slow_components.is_empty() {
                            suggestions.push(format!("Optimize slow components: {}", slow_components.join(", ")));
                        }
                        
                        if suggestions.is_empty() {
                            suggestions.push("No specific optimizations needed. Performance looks good!");
                        }
                        
                        suggestions.into_iter()
                            .map(|suggestion| view! { <li>{suggestion}</li> })
                            .collect::<Vec<_>>()
                    }}
                </ul>
            </div>
        </div>
    }
}

// Component performance details panel
#[component]
fn ComponentsPanel(
    #[prop(into)] summary: Signal<PerformanceSummary>
) -> impl IntoView {
    // Sort component data by render time
    let sorted_components = create_memo(move |_| {
        let mut components: Vec<_> = summary.get()
            .components
            .into_iter()
            .filter(|(_, metrics)| metrics.count > 0)
            .collect();
            
        components.sort_by(|(_, a), (_, b)| b.avg_render_time_ms.partial_cmp(&a.avg_render_time_ms).unwrap_or(std::cmp::Ordering::Equal));
        components
    });
    
    view! {
        <div class="components-panel">
            <h3>"Component Render Performance"</h3>
            
            <div class="components-chart">
                <div class="chart-container" style="height: 300px;">
                    {move || {
                        let components = sorted_components.get();
                        let max_time = components.iter()
                            .map(|(_, metrics)| metrics.avg_render_time_ms)
                            .fold(0.0, f64::max);
                            
                        components.iter().take(15).map(|(name, metrics)| {
                            let percentage = (metrics.avg_render_time_ms / max_time.max(1.0)) * 100.0;
                            let bar_class = if metrics.avg_render_time_ms > 50.0 {
                                "chart-bar-slow"
                            } else if metrics.avg_render_time_ms > 20.0 {
                                "chart-bar-medium"
                            } else {
                                "chart-bar-fast"
                            };
                            
                            view! {
                                <div class="chart-item">
                                    <div class="chart-label" title=name>
                                        {if name.len() > 20 { format!("{}...", &name[0..17]) } else { name.clone() }}
                                    </div>
                                    <div class="chart-bar-container">
                                        <div 
                                            class=bar_class
                                            style=format!("width: {}%", percentage)
                                        >
                                            {format!("{:.1}ms", metrics.avg_render_time_ms)}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
            
            <table class="metrics-table components-table">
                <thead>
                    <tr>
                        <th>"Component"</th>
                        <th>"Render Count"</th>
                        <th>"Avg Time (ms)"</th>
                        <th>"Max Time (ms)"</th>
                        <th>"Total Time (ms)"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        sorted_components.get().into_iter().map(|(name, metrics)| {
                            view! {
                                <tr class=if metrics.avg_render_time_ms > 50.0 { "row-highlight" } else { "" }>
                                    <td>{name}</td>
                                    <td>{metrics.count}</td>
                                    <td>{format!("{:.2}", metrics.avg_render_time_ms)}</td>
                                    <td>{format!("{:.2}", metrics.max_render_time_ms)}</td>
                                    <td>{format!("{:.2}", metrics.total_time_ms)}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </tbody>
            </table>
        </div>
    }
}

// Resources panel
#[component]
fn ResourcesPanel(
    #[prop(into)] summary: Signal<PerformanceSummary>
) -> impl IntoView {
    // Resource type statistics
    let resource_types = create_memo(move |_| {
        let resources = summary.get().resource_timing;
        let mut type_stats: HashMap<String, (usize, f64, u64)> = HashMap::new();
        
        for resource in resources.iter() {
            let entry = type_stats.entry(resource.resource_type.clone())
                .or_insert((0, 0.0, 0));
                
            entry.0 += 1; // count
            entry.1 += resource.duration; // total duration
            entry.2 += resource.size.unwrap_or(0); // total size
        }
        
        let mut stats: Vec<ResourceTypeStats> = type_stats.into_iter()
            .map(|(type_name, (count, total_duration, total_size))| {
                ResourceTypeStats {
                    name: type_name,
                    count,
                    avg_duration: if count > 0 { total_duration / count as f64 } else { 0.0 },
                    total_size,
                }
            })
            .collect();
            
        stats.sort_by(|a, b| b.count.cmp(&a.count));
        stats
    });
    
    view! {
        <div class="resources-panel">
            <h3>"Resource Loading Performance"</h3>
            
            <div class="resource-summary">
                <div class="metric-card">
                    <div class="metric-value">{move || summary.get().resources.total_count}</div>
                    <div class="metric-label">"Total Resources"</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{move || {
                        let total_size = summary.get().resources.total_size;
                        if total_size > 1024 * 1024 {
                            format!("{:.1} MB", total_size as f64 / (1024.0 * 1024.0))
                        } else if total_size > 1024 {
                            format!("{:.1} KB", total_size as f64 / 1024.0)
                        } else {
                            format!("{} B", total_size)
                        }
                    }}</div>
                    <div class="metric-label">"Total Size"</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{move || {
                        format!("{:.1} ms", summary.get().resources.avg_load_time)
                    }}</div>
                    <div class="metric-label">"Avg Load Time"</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{move || {
                        let summary = summary.get();
                        let percentage = if summary.resources.total_count > 0 {
                            (summary.resources.cached_count as f64 * 100.0 / summary.resources.total_count as f64) as u32
                        } else { 0 };
                        format!("{}%", percentage)
                    }}</div>
                    <div class="metric-label">"Cache Hit Rate"</div>
                </div>
            </div>
            
            <h4>"Resource Types"</h4>
            <table class="metrics-table resources-table">
                <thead>
                    <tr>
                        <th>"Type"</th>
                        <th>"Count"</th>
                        <th>"Avg Duration (ms)"</th>
                        <th>"Total Size"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        resource_types.get().into_iter().map(|stats| {
                            view! {
                                <tr>
                                    <td>{stats.name}</td>
                                    <td>{stats.count}</td>
                                    <td>{format!("{:.2}", stats.avg_duration)}</td>
                                    <td>{format_bytes(stats.total_size)}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </tbody>
            </table>
            
            <h4>"Slowest Resources"</h4>
            <table class="metrics-table resources-table">
                <thead>
                    <tr>
                        <th>"URL"</th>
                        <th>"Type"</th>
                        <th>"Duration (ms)"</th>
                        <th>"Size"</th>
                        <th>"Cached"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        let mut resources = summary.get().resource_timing;
                        resources.sort_by(|a, b| b.duration.partial_cmp(&a.duration).unwrap_or(std::cmp::Ordering::Equal));
                        
                        resources.into_iter().take(10).map(|resource| {
                            // Extract filename from URL
                            let url_parts: Vec<&str> = resource.url.split('/').collect();
                            let display_url = if let Some(file) = url_parts.last() {
                                if file.is_empty() && url_parts.len() > 1 {
                                    url_parts[url_parts.len() - 2].to_string()
                                } else {
                                    file.to_string()
                                }
                            } else {
                                resource.url.clone()
                            };
                            
                            view! {
                                <tr class=if resource.duration > 500.0 { "row-highlight" } else { "" }>
                                    <td title=resource.url>{display_url}</td>
                                    <td>{resource.resource_type}</td>
                                    <td>{format!("{:.2}", resource.duration)}</td>
                                    <td>{resource.size.map_or("Unknown".to_string(), format_bytes)}</td>
                                    <td>{if resource.is_cached { "Yes" } else { "No" }}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </tbody>
            </table>
        </div>
    }
}

// User interactions panel
#[component]
fn InteractionsPanel(
    #[prop(into)] summary: Signal<PerformanceSummary>
) -> impl IntoView {
    view! {
        <div class="interactions-panel">
            <h3>"User Interaction Performance"</h3>
            
            <div class="metrics-summary">
                <div class="metric-card">
                    <div class="metric-value">{move || summary.get().interactions.count}</div>
                    <div class="metric-label">"Total Interactions"</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{move || {
                        let avg_time = summary.get().interactions.avg_response_time;
                        format!("{:.1} ms", avg_time)
                    }}</div>
                    <div class="metric-label">"Avg Response Time"</div>
                </div>
                <div class="metric-card">
                    <div class="metric-value">{move || {
                        if let Some(fid) = summary.get().web_vitals.fid {
                            format!("{:.1} ms", fid)
                        } else {
                            "N/A".to_string()
                        }
                    }}</div>
                    <div class="metric-label">"First Input Delay"</div>
                </div>
            </div>
            
            <h4>"Interaction Timeline"</h4>
            <div class="interaction-timeline">
                // Simple horizontal timeline visualization
                {move || {
                    let interaction_data = summary.get().interactions;
                    if interaction_data.count == 0 {
                        view! { <div class="empty-state">"No interaction data available"</div> }
                    } else {
                        view! {
                            <div class="timeline-container">
                                <div class="timeline-ruler">
                                    <span class="timeline-marker" style="left: 0%">"0ms"</span>
                                    <span class="timeline-marker" style="left: 25%">"100ms"</span>
                                    <span class="timeline-marker" style="left: 50%">"200ms"</span>
                                    <span class="timeline-marker" style="left: 75%">"300ms"</span>
                                    <span class="timeline-marker" style="left: 100%">"400ms+"</span>
                                </div>
                                <div class="performance-threshold good" style="left: 0%; width: 25%;">"Good"</div>
                                <div class="performance-threshold needs-improvement" style="left: 25%; width: 25%;">"Needs Improvement"</div>
                                <div class="performance-threshold poor" style="left: 50%; width: 50%;">"Poor"</div>
                            </div>
                        }
                    }
                }}
            </div>
            
            <table class="metrics-table interactions-table">
                <thead>
                    <tr>
                        <th>"Type"</th>
                        <th>"Element"</th>
                        <th>"Response Time (ms)"</th>
                        <th>"Status"</th>
                    </tr>
                </thead>
                <tbody>
                    // No access to individual interaction data in the summary
                    // This would need to be extended in the PerformanceSummary type
                    <tr>
                        <td colspan="4">
                            "Detailed interaction data not available in summary view"
                        </td>
                    </tr>
                </tbody>
            </table>
            
            <div class="performance-tips">
                <h4>"Interaction Performance Tips"</h4>
                <ul>
                    <li>"Keep main thread work minimal during user interactions."</li>
                    <li>"Break up long tasks to ensure the main thread stays responsive."</li>
                    <li>"Debounce frequent events like scrolling and resizing."</li>
                    <li>"Use requestAnimationFrame for visual updates."</li>
                    <li>"Consider web workers for CPU-intensive operations."</li>
                </ul>
            </div>
        </div>
    }
}

// Helper struct for charts
struct ChartData {
    component_render_times: Vec<(String, f64)>,
    resource_count: usize,
}

struct ResourceTypeStats {
    name: String,
    count: usize,
    avg_duration: f64,
    total_size: u64,
}

// Format bytes to human-readable form
fn format_bytes(bytes: u64) -> String {
    if bytes > 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes > 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}