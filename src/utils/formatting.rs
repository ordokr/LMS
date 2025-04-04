// Add this to your formatting utilities

/// Format datetime to relative time (e.g., "2 hours ago", "5 days ago")
pub fn format_relative_time(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(date);
    
    let seconds = duration.num_seconds();
    let minutes = duration.num_minutes();
    let hours = duration.num_hours();
    let days = duration.num_days();
    let weeks = days / 7;
    let months = days / 30;
    let years = days / 365;
    
    if seconds < 60 {
        return "just now".to_string();
    } else if minutes < 60 {
        if minutes == 1 {
            return "1 minute ago".to_string();
        } else {
            return format!("{} minutes ago", minutes);
        }
    } else if hours < 24 {
        if hours == 1 {
            return "1 hour ago".to_string();
        } else {
            return format!("{} hours ago", hours);
        }
    } else if days < 7 {
        if days == 1 {
            return "1 day ago".to_string();
        } else {
            return format!("{} days ago", days);
        }
    } else if weeks < 4 {
        if weeks == 1 {
            return "1 week ago".to_string();
        } else {
            return format!("{} weeks ago", weeks);
        }
    } else if months < 12 {
        if months == 1 {
            return "1 month ago".to_string();
        } else {
            return format!("{} months ago", months);
        }
    } else {
        if years == 1 {
            return "1 year ago".to_string();
        } else {
            return format!("{} years ago", years);
        }
    }
}

/// Format datetime in standard format (e.g., "Jan 15, 2023 at 3:45 PM")
pub fn format_datetime(date: chrono::DateTime<chrono::Utc>) -> String {
    date.format("%b %e, %Y at %l:%M %p").to_string()
}