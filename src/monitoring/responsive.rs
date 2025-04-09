use leptos::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use std::sync::Arc;

// Monitor application responsiveness and adapt behavior based on performance
pub struct ResponsivenessMonitor {
    frame_times: Arc<Mutex<VecDeque<Duration>>>,
    event_response_times: Arc<Mutex<VecDeque<Duration>>>,
    under_pressure: AtomicBool,
    last_frame: Arc<Mutex<Option<Instant>>>,
    quality_level: AtomicUsize,
    max_samples: usize,
}

#[derive(Clone, Debug)]
pub struct PerformanceStats {
    pub avg_frame_time: Duration,
    pub avg_event_response: Duration,
    pub p95_frame_time: Duration,
    pub p95_event_response: Duration,
    pub under_pressure: bool,
    pub quality_level: usize,
    pub fps: f64,
}

impl ResponsivenessMonitor {
    pub fn new(max_samples: usize) -> Self {
        let monitor = Self {
            frame_times: Arc::new(Mutex::new(VecDeque::with_capacity(max_samples))),
            event_response_times: Arc::new(Mutex::new(VecDeque::with_capacity(max_samples))),
            under_pressure: AtomicBool::new(false),
            last_frame: Arc::new(Mutex::new(None)),
            quality_level: AtomicUsize::new(2), // Default: medium quality (0=low, 1=medium, 2=high)
            max_samples,
        };
        
        // Start monitoring frame rate
        let monitor_clone = monitor.clone();
        spawn_local(async move {
            monitor_clone.monitor_frames().await;
        });
        
        monitor
    }
    
    // Monitor frame rate by scheduling animation frames
    async fn monitor_frames(&self) {
        let last_frame = self.last_frame.clone();
        
        {
            let mut last = last_frame.lock().await;
            *last = Some(Instant::now());
        }
        
        let frame_times = self.frame_times.clone();
        let under_pressure = self.under_pressure.clone();
        let quality_level = self.quality_level.clone();
        
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let now = Instant::now();
            
            spawn_local({
                let last_frame = last_frame.clone();
                let frame_times = frame_times.clone();
                let under_pressure = under_pressure.clone();
                let quality_level = quality_level.clone();
                let f = f.clone();
                
                async move {
                    // Measure frame time
                    let mut last = last_frame.lock().await;
                    if let Some(last_time) = *last {
                        let frame_time = now.duration_since(last_time);
                        *last = Some(now);
                        
                        // Update frame times
                        let mut times = frame_times.lock().await;
                        times.push_back(frame_time);
                        
                        if times.len() > 100 {
                            times.pop_front();
                        }
                        
                        // Calculate stats and adjust quality
                        let avg_frame_time = times.iter().sum::<Duration>() / times.len() as u32;
                        
                        // Check if we're under pressure (frame times > 30ms)
                        let is_pressured = avg_frame_time > Duration::from_millis(30);
                        under_pressure.store(is_pressured, Ordering::Relaxed);
                        
                        // Adjust quality level if needed
                        let current_quality = quality_level.load(Ordering::Relaxed);
                        
                        if is_pressured && current_quality > 0 {
                            // Reduce quality
                            quality_level.store(current_quality - 1, Ordering::Relaxed);
                        } else if !is_pressured && 
                                  current_quality < 2 && 
                                  avg_frame_time < Duration::from_millis(16) {
                            // Increase quality
                            quality_level.store(current_quality + 1, Ordering::Relaxed);
                        }
                    }
                    
                    // Schedule next frame
                    window().request_animation_frame(
                        f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
                    ).unwrap();
                }
            });
        }) as Box<dyn FnMut()>));
        
        // Start monitoring
        window().request_animation_frame(
            g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        ).unwrap();
    }
    
    // Track time to respond to a user event
    pub fn track_event_response<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        spawn_local({
            let event_times = self.event_response_times.clone();
            
            async move {
                let mut times = event_times.lock().await;
                times.push_back(duration);
                
                if times.len() > 100 {
                    times.pop_front();
                }
            }
        });
        
        result
    }
    
    // Get current quality level
    pub fn quality_level(&self) -> usize {
        self.quality_level.load(Ordering::Relaxed)
    }
    
    // Check if system is under pressure
    pub fn is_under_pressure(&self) -> bool {
        self.under_pressure.load(Ordering::Relaxed)
    }
    
    // Get performance statistics
    pub async fn get_stats(&self) -> PerformanceStats {
        let frame_times = self.frame_times.lock().await;
        let event_times = self.event_response_times.lock().await;
        
        let avg_frame_time = if frame_times.is_empty() {
            Duration::from_millis(16) // Default to 60fps
        } else {
            frame_times.iter().sum::<Duration>() / frame_times.len() as u32
        };
        
        let avg_event_response = if event_times.is_empty() {
            Duration::from_millis(0)
        } else {
            event_times.iter().sum::<Duration>() / event_times.len() as u32
        };
        
        // Calculate p95 times
        let p95_frame_time = if frame_times.is_empty() {
            Duration::from_millis(16)
        } else {
            let mut sorted: Vec<_> = frame_times.iter().cloned().collect();
            sorted.sort();
            sorted[((sorted.len() as f32) * 0.95) as usize]
        };
        
        let p95_event_response = if event_times.is_empty() {
            Duration::from_millis(0)
        } else {
            let mut sorted: Vec<_> = event_times.iter().cloned().collect();
            sorted.sort();
            sorted[((sorted.len() as f32) * 0.95) as usize]
        };
        
        // Calculate FPS
        let fps = if avg_frame_time.as_secs_f64() > 0.0 {
            1.0 / avg_frame_time.as_secs_f64()
        } else {
            60.0 // Default
        };
        
        PerformanceStats {
            avg_frame_time,
            avg_event_response,
            p95_frame_time,
            p95_event_response,
            under_pressure: self.under_pressure.load(Ordering::Relaxed),
            quality_level: self.quality_level.load(Ordering::Relaxed),
            fps,
        }
    }
}

impl Clone for ResponsivenessMonitor {
    fn clone(&self) -> Self {
        Self {
            frame_times: self.frame_times.clone(),
            event_response_times: self.event_response_times.clone(),
            under_pressure: AtomicBool::new(self.under_pressure.load(Ordering::Relaxed)),
            last_frame: self.last_frame.clone(),
            quality_level: AtomicUsize::new(self.quality_level.load(Ordering::Relaxed)),
            max_samples: self.max_samples,
        }
    }
}

// Hook for automatically adapting rendering quality
#[hook]
pub fn use_adaptive_quality() -> Signal<usize> {
    let monitor = use_context::<ResponsivenessMonitor>()
        .expect("ResponsivenessMonitor should be provided at app root");
        
    let (quality, set_quality) = create_signal(monitor.quality_level());
    
    create_effect(move |_| {
        // Check quality level every second
        let quality_level = monitor.quality_level();
        set_quality.set(quality_level);
        
        set_timeout(
            move || {
                // This will cause the effect to re-run
                request_animation_frame(|| {});
            },
            1000 // Check every second
        );
    });
    
    quality
}

// Create quality-aware components
#[component]
pub fn AdaptiveContent<F, V>(
    render_high: F,
    #[prop(optional)] render_medium: Option<F>,
    #[prop(optional)] render_low: Option<F>,
) -> impl IntoView
where
    F: Fn() -> V + 'static,
    V: IntoView,
{
    let quality = use_adaptive_quality();
    
    let render_med = render_medium.unwrap_or_else(|| render_high.clone());
    let render_lo = render_low.unwrap_or_else(|| render_med.clone());
    
    view! {
        {move || {
            match quality.get() {
                2 => render_high().into_view(),
                1 => render_med().into_view(),
                _ => render_lo().into_view(),
            }
        }}
    }
}