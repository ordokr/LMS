use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineIndicatorProps {
    pub is_offline: bool,
    pub pending_sync_count: usize,
}

// This is a placeholder for the UI component
// In a real implementation, this would be a Leptos or other UI framework component
pub fn render_offline_indicator(props: &OfflineIndicatorProps) -> String {
    let status_class = if props.is_offline { "offline" } else { "online" };
    let status_text = if props.is_offline { "Offline" } else { "Online" };
    
    let sync_text = if props.pending_sync_count > 0 {
        format!("{} item{} pending sync", 
            props.pending_sync_count,
            if props.pending_sync_count == 1 { "" } else { "s" }
        )
    } else {
        "".to_string()
    };
    
    format!(r#"
        <div class="offline-indicator {status_class}">
            <div class="status-dot"></div>
            <div class="status-text">{status_text}</div>
            {sync_text_element}
            <button class="sync-button" id="sync-now-button">Sync Now</button>
        </div>
    "#, 
    status_class = status_class,
    status_text = status_text,
    sync_text_element = if !sync_text.is_empty() {
        format!("<div class=\"sync-pending\">{}</div>", sync_text)
    } else {
        "".to_string()
    })
}
