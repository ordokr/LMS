use leptos::*;
use crate::analytics::performance_tracker::{use_performance_metrics, PerformanceSummary};
use gloo_timers::future::TimeoutFuture;

/// Hook to integrate performance metrics with forum actions
#[hook]
pub fn use_forum_performance() -> (
    Signal<PerformanceSummary>,          // Current performance summary
    impl Fn(&str) -> ScopeDisposer,      // Start measuring a forum action
    impl Fn() -> i32,                    // Get performance score (0-100)
) {
    let metrics = use_performance_metrics();
    
    // Get the current performance summary
    let summary = use_performance_summary();
    
    // Start measuring a specific forum action (returns disposer)
    let start_measuring = move |action_name: &str| -> ScopeDisposer {
        let action_name = action_name.to_string();
        
        // Start measuring
        metrics.start_component_render(&format!("forum_action_{}", action_name));
        
        // Create a disposer to end measurement
        create_disposer(move || {
            metrics.end_component_render(&format!("forum_action_{}", action_name));
        })
    };
    
    // Calculate a performance score based on metrics
    let get_performance_score = move || -> i32 {
        let perf_summary = summary.get();
        let mut score = 100;
        
        // Penalize slow LCP
        if let Some(lcp) = perf_summary.web_vitals.lcp {
            if lcp > 2500.0 { score -= ((lcp - 2500.0) / 500.0).min(30.0) as i32; }
        }
        
        // Penalize slow FID
        if let Some(fid) = perf_summary.web_vitals.fid {
            if fid > 100.0 { score -= ((fid - 100.0) / 50.0).min(30.0) as i32; }
        }
        
        // Penalize slow resources
        if perf_summary.resources.avg_load_time > 300.0 {
            score -= ((perf_summary.resources.avg_load_time - 300.0) / 100.0).min(20.0) as i32;
        }
        
        // Penalize low cache hit rate
        let cache_rate = if perf_summary.resources.total_count > 0 {
            perf_summary.resources.cached_count as f64 / perf_summary.resources.total_count as f64
        } else {
            1.0
        };
        
        if cache_rate < 0.5 {
            score -= ((0.5 - cache_rate) * 20.0) as i32;
        }
        
        // Ensure score is in valid range
        score.max(0).min(100)
    };
    
    (summary, start_measuring, get_performance_score)
}

/// Mark performance of a heavy forum operation
pub async fn measure_async_operation(name: &str, operation: impl std::future::Future<Output = ()>) {
    let metrics = use_performance_metrics();
    
    // Start timing
    metrics.start_component_render(&format!("async_{}", name));
    
    // Run the operation
    operation.await;
    
    // End timing
    metrics.end_component_render(&format!("async_{}", name));
}

/// Track large forum page loading
#[hook]
pub fn use_forum_page_tracking() -> impl Fn() {
    let metrics = use_performance_metrics();
    
    // Create a tracking function
    move || {
        // Track visibility state changes
        document_event_listener(ev::visibilitychange, move |_| {
            if document().visibility_state() == web_sys::VisibilityState::Hidden {
                // User navigated away, save current metrics for analysis
                let summary = metrics.get_summary();
                
                // Could send to server or store locally
                log::info!("Forum performance data: {:?}", summary);
            }
        });
    }
}

/// Example user-facing performance feedback
#[component]
pub fn PerformanceFeedback() -> impl IntoView {
    let (_, _, get_score) = use_forum_performance();
    let (show_feedback, set_show_feedback) = create_signal(false);
    let (score, set_score) = create_signal(100);
    
    // Check performance after page load
    create_effect(move |_| {
        spawn_local(async move {
            // Wait for page to fully load
            TimeoutFuture::new(3000).await;
            
            // Calculate performance score
            let performance_score = get_score();
            set_score(performance_score);
            
            // Only show feedback for poor performance
            if performance_score < 70 {
                set_show_feedback(true);
            }
        });
    });
    
    // If performance is poor, show a feedback widget
    view! {
        <Show when=show_feedback>
            <div class="performance-feedback">
                <div class="feedback-header">
                    <div class="feedback-score">{score}</div>
                    <div class="feedback-title">
                        {move || {
                            let s = score.get();
                            if s < 50 {
                                "Performance Issues Detected"
                            } else {
                                "Performance Could Be Better"
                            }
                        }}
                    </div>
                    <button class="feedback-dismiss" on:click=move |_| set_show_feedback(false)>
                        "×"
                    </button>
                </div>
                <div class="feedback-body">
                    {move || {
                        let s = score.get();
                        if s < 50 {
                            "Your device is experiencing performance issues with our forum. Using a lighter theme or clearing your browser cache might help."
                        } else {
                            "Forum performance is acceptable but not optimal. Consider closing unused browser tabs or reloading the page."
                        }
                    }}
                </div>
                <div class="feedback-actions">
                    <button class="feedback-action" 
                        on:click=move |_| {
                            // Simple reload action
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().reload();
                            }
                        }
                    >
                        "Reload Page"
                    </button>
                    <button class="feedback-action secondary" on:click=move |_| set_show_feedback(false)>
                        "Dismiss"
                    </button>
                </div>
            </div>
        </Show>
    }
}