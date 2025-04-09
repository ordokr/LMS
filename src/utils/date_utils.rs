use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serializer, Serialize};

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

/// Formats a DateTime<Utc> as an ISO 8601 string
pub fn format_iso_date(date: &DateTime<Utc>) -> String {
    date.to_rfc3339()
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