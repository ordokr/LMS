use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{Touch, TouchEvent, TouchList};
use std::rc::Rc;

/// Props for the SwipeContainer component
#[derive(Props, Clone)]
pub struct SwipeContainerProps {
    /// Children to render inside the swipe container
    pub children: Children,
    
    /// Callback when swiped left
    #[prop(default = None)]
    pub on_swipe_left: Option<Callback<()>>,
    
    /// Callback when swiped right
    #[prop(default = None)]
    pub on_swipe_right: Option<Callback<()>>,
    
    /// Callback when swiped up
    #[prop(default = None)]
    pub on_swipe_up: Option<Callback<()>>,
    
    /// Callback when swiped down
    #[prop(default = None)]
    pub on_swipe_down: Option<Callback<()>>,
    
    /// Minimum distance to trigger swipe (pixels)
    #[prop(default = 50)]
    pub swipe_threshold: i32,
    
    /// CSS class for the container
    #[prop(default = "".to_string())]
    pub class: String,
    
    /// Whether to show swipe indicators
    #[prop(default = true)]
    pub show_indicators: bool,
}

/// A container that detects swipe gestures on touch devices
#[component]
pub fn SwipeContainer(props: SwipeContainerProps) -> impl IntoView {
    let SwipeContainerProps {
        children,
        on_swipe_left,
        on_swipe_right,
        on_swipe_up,
        on_swipe_down,
        swipe_threshold,
        class,
        show_indicators,
    } = props;
    
    // Track touch positions
    let start_x = create_rw_signal(0);
    let start_y = create_rw_signal(0);
    let end_x = create_rw_signal(0);
    let end_y = create_rw_signal(0);
    let is_swiping = create_rw_signal(false);
    
    // Handle touch start
    let on_touch_start = move |e: TouchEvent| {
        let touches: TouchList = e.touches();
        if touches.length() > 0 {
            if let Some(touch) = touches.get(0) {
                start_x.set(touch.client_x());
                start_y.set(touch.client_y());
                end_x.set(touch.client_x());
                end_y.set(touch.client_y());
                is_swiping.set(true);
            }
        }
    };
    
    // Handle touch move
    let on_touch_move = move |e: TouchEvent| {
        if !is_swiping.get() {
            return;
        }
        
        let touches: TouchList = e.touches();
        if touches.length() > 0 {
            if let Some(touch) = touches.get(0) {
                end_x.set(touch.client_x());
                end_y.set(touch.client_y());
            }
        }
    };
    
    // Handle touch end
    let on_touch_end = move |_: TouchEvent| {
        if !is_swiping.get() {
            return;
        }
        
        let dx = end_x.get() - start_x.get();
        let dy = end_y.get() - start_y.get();
        
        // Determine if the swipe was horizontal or vertical
        if dx.abs() > dy.abs() {
            // Horizontal swipe
            if dx > swipe_threshold {
                // Swiped right
                if let Some(callback) = on_swipe_right.clone() {
                    callback.call(());
                }
            } else if dx < -swipe_threshold {
                // Swiped left
                if let Some(callback) = on_swipe_left.clone() {
                    callback.call(());
                }
            }
        } else {
            // Vertical swipe
            if dy > swipe_threshold {
                // Swiped down
                if let Some(callback) = on_swipe_down.clone() {
                    callback.call(());
                }
            } else if dy < -swipe_threshold {
                // Swiped up
                if let Some(callback) = on_swipe_up.clone() {
                    callback.call(());
                }
            }
        }
        
        is_swiping.set(false);
    };
    
    // Prevent default to avoid scrolling while swiping
    let prevent_default = move |e: TouchEvent| {
        if is_swiping.get() {
            e.prevent_default();
        }
    };
    
    view! {
        <div
            class=format!("swipe-container {}", class)
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
            on:touchcancel=on_touch_end
        >
            {children()}
            
            {move || {
                if show_indicators {
                    view! {
                        <div class="swipe-indicators">
                            {on_swipe_left.is_some().then(|| view! {
                                <div class="swipe-indicator swipe-left">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M15 18l-6-6 6-6"/>
                                    </svg>
                                    <span>Swipe left</span>
                                </div>
                            })}
                            
                            {on_swipe_right.is_some().then(|| view! {
                                <div class="swipe-indicator swipe-right">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M9 18l6-6-6-6"/>
                                    </svg>
                                    <span>Swipe right</span>
                                </div>
                            })}
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </div>
    }
}

/// Props for the TouchFriendlyButton component
#[derive(Props, Clone)]
pub struct TouchFriendlyButtonProps {
    /// Text to display on the button
    pub text: String,
    
    /// Icon to display (optional)
    #[prop(default = None)]
    pub icon: Option<String>,
    
    /// Callback when clicked
    #[prop(default = None)]
    pub on_click: Option<Callback<()>>,
    
    /// CSS class for the button
    #[prop(default = "".to_string())]
    pub class: String,
    
    /// Whether the button is disabled
    #[prop(default = false)]
    pub disabled: bool,
    
    /// Button type (button, submit, reset)
    #[prop(default = "button".to_string())]
    pub button_type: String,
    
    /// Touch feedback color
    #[prop(default = None)]
    pub touch_color: Option<String>,
}

/// A touch-friendly button with appropriate sizing and feedback
#[component]
pub fn TouchFriendlyButton(props: TouchFriendlyButtonProps) -> impl IntoView {
    let TouchFriendlyButtonProps {
        text,
        icon,
        on_click,
        class,
        disabled,
        button_type,
        touch_color,
    } = props;
    
    // Track touch state for visual feedback
    let is_touched = create_rw_signal(false);
    
    // Handle touch start
    let on_touch_start = move |_| {
        if !disabled {
            is_touched.set(true);
        }
    };
    
    // Handle touch end
    let on_touch_end = move |_| {
        is_touched.set(false);
    };
    
    // Handle click
    let handle_click = move |_| {
        if !disabled && on_click.is_some() {
            on_click.unwrap().call(());
        }
    };
    
    view! {
        <button
            type=button_type
            class=format!("touch-friendly-button {} {}", 
                class,
                if is_touched.get() { "touched" } else { "" }
            )
            style=move || {
                if is_touched.get() && touch_color.is_some() {
                    format!("background-color: {};", touch_color.clone().unwrap())
                } else {
                    "".to_string()
                }
            }
            disabled=disabled
            on:touchstart=on_touch_start
            on:touchend=on_touch_end
            on:touchcancel=on_touch_end
            on:click=handle_click
        >
            {icon.map(|i| view! {
                <span class="button-icon">{i}</span>
            })}
            <span class="button-text">{text}</span>
        </button>
    }
}

/// Props for the MobileTabBar component
#[derive(Props, Clone)]
pub struct MobileTabBarProps {
    /// The active tab index
    pub active_tab: Signal<usize>,
    
    /// Callback when a tab is selected
    pub on_tab_change: Callback<usize>,
    
    /// The tabs to display
    pub tabs: Vec<MobileTab>,
}

/// A mobile tab item
#[derive(Clone)]
pub struct MobileTab {
    /// The tab label
    pub label: String,
    
    /// The tab icon (SVG string)
    pub icon: String,
}

/// A mobile-specific tab bar for navigation
#[component]
pub fn MobileTabBar(props: MobileTabBarProps) -> impl IntoView {
    let MobileTabBarProps {
        active_tab,
        on_tab_change,
        tabs,
    } = props;
    
    view! {
        <div class="mobile-tab-bar">
            {tabs.iter().enumerate().map(|(index, tab)| {
                let tab = tab.clone();
                let on_tab_change = on_tab_change.clone();
                
                view! {
                    <button
                        class=move || format!("mobile-tab {}", if active_tab.get() == index { "active" } else { "" })
                        on:click=move |_| on_tab_change.call(index)
                    >
                        <span class="tab-icon" inner_html=tab.icon.clone()></span>
                        <span class="tab-label">{tab.label}</span>
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

/// Props for the OfflineIndicator component
#[derive(Props, Clone)]
pub struct OfflineIndicatorProps {
    /// Whether the app is offline
    pub is_offline: Signal<bool>,
    
    /// Callback to retry connection
    #[prop(default = None)]
    pub on_retry: Option<Callback<()>>,
    
    /// Number of pending sync items
    #[prop(default = Signal::derive(move || 0))]
    pub pending_sync_count: Signal<usize>,
}

/// An indicator for offline status on mobile
#[component]
pub fn OfflineIndicator(props: OfflineIndicatorProps) -> impl IntoView {
    let OfflineIndicatorProps {
        is_offline,
        on_retry,
        pending_sync_count,
    } = props;
    
    view! {
        <div class="offline-indicator" style=move || if is_offline.get() { "display: block" } else { "display: none" }>
            <span>You are offline</span>
            
            {move || {
                let count = pending_sync_count.get();
                if count > 0 {
                    view! {
                        <span class="sync-status">
                            {count} {if count == 1 { "item" } else { "items" }} pending sync
                        </span>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
            
            {on_retry.map(|callback| view! {
                <button class="retry-button" on:click=move |_| callback.call(())>
                    Retry
                </button>
            })}
        </div>
    }
}
