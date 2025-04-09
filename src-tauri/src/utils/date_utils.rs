use chrono::{DateTime, Utc, ParseError};
use serde::{Deserialize, Deserializer, Serializer, Serialize};
use std::str::FromStr;
use log::warn;

/// Converts an ISO 8601 date string to a DateTime<Utc>
pub fn parse_iso_date(date_str: &str) -> Option<DateTime<Utc>> {
    if date_str.is_empty() {
        return None;
    }
    
    // Try to parse with different formats
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S")
                .map(|ndt| Utc.from_utc_datetime(&ndt))
        })
        .or_else(|_| {
            NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| Utc.from_utc_datetime(&ndt))
        })
        .ok()
}

/// Converts a string date to DateTime<Utc> if possible
/// Useful when integrating with external APIs that provide dates as strings
pub fn parse_date_string(date_str: Option<&str>) -> Option<DateTime<Utc>> {
    if let Some(s) = date_str {
        if s.is_empty() {
            return None;
        }
        
        // Try standard ISO format first
        if let Ok(dt) = DateTime::from_str(s) {
            return Some(dt);
        }
        
        // Try additional formats (Canvas often uses RFC3339)
        let formats = [
            "%Y-%m-%dT%H:%M:%S%.fZ", // ISO8601/RFC3339
            "%Y-%m-%dT%H:%M:%S%:z",   // ISO8601 with timezone
            "%Y-%m-%d %H:%M:%S",      // MySQL datetime format
            "%Y-%m-%d",               // Simple date format
        ];
        
        for format in formats {
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, format)
                .map(|dt| DateTime::<Utc>::from_utc(dt, Utc))
            {
                return Some(dt);
            }
        }
        
        warn!("Could not parse date string: {}", s);
    }
    
    None
}

/// Formats a DateTime<Utc> as an ISO 8601 string
pub fn format_iso_date(date: &DateTime<Utc>) -> String {
    date.to_rfc3339()
}

/// Format DateTime<Utc> as an ISO8601 string
pub fn format_date(date: &Option<DateTime<Utc>>) -> Option<String> {
    date.map(|dt| dt.to_rfc3339())
}

/// Helper function for pretty-formatting dates for UI display
pub fn format_date_for_display(date: Option<&DateTime<Utc>>) -> String {
    match date {
        Some(d) => d.format("%b %d, %Y %H:%M").to_string(),
        None => "Not set".to_string(),
    }
}

/// Custom serializer for Option<DateTime<Utc>> to ensure it's always serialized as RFC3339
pub fn serialize_optional_date<S>(
    date: &Option<DateTime<Utc>>, 
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => serializer.serialize_str(&format_iso_date(dt)),
        None => serializer.serialize_none(),
    }
}

/// Custom deserializer for Option<DateTime<Utc>> from various string formats
pub fn deserialize_optional_date<'de, D>(
    deserializer: D
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => parse_iso_date(&s).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid date format: {}", s))
        }),
        None => Ok(None),
    }
}

/// Convert legacy string dates to proper DateTime<Utc> dates
pub fn convert_model_string_dates<T, F>(models: Vec<T>, converter: F) -> Vec<T>
where
    F: Fn(T) -> T
{
    models.into_iter().map(converter).collect()
}