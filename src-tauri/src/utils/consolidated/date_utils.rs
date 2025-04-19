use chrono::{DateTime, Utc, NaiveDateTime, TimeZone, Duration};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use log::warn;

/// Parse a date string into a DateTime<Utc>
/// 
/// Supports multiple date formats:
/// - ISO 8601 / RFC 3339 (e.g., "2023-07-17T12:34:56Z")
/// - ISO 8601 with timezone (e.g., "2023-07-17T12:34:56+00:00")
/// - MySQL datetime format (e.g., "2023-07-17 12:34:56")
/// - Simple date format (e.g., "2023-07-17")
/// 
/// # Arguments
/// * `date_str` - The date string to parse
/// 
/// # Returns
/// * `Option<DateTime<Utc>>` - The parsed date, or None if parsing failed
pub fn parse_date(date_str: &str) -> Option<DateTime<Utc>> {
    if date_str.is_empty() {
        return None;
    }
    
    // Try standard ISO format first
    if let Ok(dt) = DateTime::from_str(date_str) {
        return Some(dt);
    }
    
    // Try additional formats
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.fZ",     // ISO8601/RFC3339
        "%Y-%m-%dT%H:%M:%S%:z",       // ISO8601 with timezone
        "%Y-%m-%d %H:%M:%S",          // MySQL datetime format
        "%Y-%m-%d",                   // Simple date format
        "%d/%m/%Y",                   // European date format
        "%m/%d/%Y",                   // US date format
        "%Y/%m/%d",                   // Japanese date format
        "%d-%m-%Y",                   // European date format with dashes
        "%m-%d-%Y",                   // US date format with dashes
        "%b %d, %Y",                  // Month name, day, year
        "%B %d, %Y",                  // Full month name, day, year
        "%d %b %Y",                   // Day, month name, year
        "%d %B %Y",                   // Day, full month name, year
    ];
    
    for format in formats {
        if let Ok(dt) = NaiveDateTime::parse_from_str(date_str, format) {
            return Some(DateTime::<Utc>::from_utc(dt, Utc));
        } else if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, format) {
            return Some(DateTime::<Utc>::from_utc(
                date.and_hms_opt(0, 0, 0).unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                Utc,
            ));
        }
    }
    
    warn!("Could not parse date string: {}", date_str);
    None
}

/// Format a DateTime<Utc> as an ISO 8601 string
/// 
/// # Arguments
/// * `date` - The date to format
/// 
/// # Returns
/// * `String` - The formatted date string
pub fn format_date(date: &DateTime<Utc>) -> String {
    date.to_rfc3339()
}

/// Format a DateTime<Utc> for display
/// 
/// # Arguments
/// * `date` - The date to format
/// * `format` - The format string (optional, defaults to "%b %d, %Y %H:%M")
/// 
/// # Returns
/// * `String` - The formatted date string
pub fn format_date_for_display(date: &DateTime<Utc>, format: Option<&str>) -> String {
    let format_str = format.unwrap_or("%b %d, %Y %H:%M");
    date.format(format_str).to_string()
}

/// Serialize a DateTime<Utc> to a string
/// 
/// # Arguments
/// * `date` - The date to serialize
/// * `serializer` - The serializer
/// 
/// # Returns
/// * `Result<S::Ok, S::Error>` - The serialization result
pub fn serialize_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format_date(date))
}

/// Deserialize a string to a DateTime<Utc>
/// 
/// # Arguments
/// * `deserializer` - The deserializer
/// 
/// # Returns
/// * `Result<DateTime<Utc>, D::Error>` - The deserialization result
pub fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_date(&s).ok_or_else(|| serde::de::Error::custom(format!("Invalid date format: {}", s)))
}

/// Serialize an Option<DateTime<Utc>> to a string
/// 
/// # Arguments
/// * `date` - The optional date to serialize
/// * `serializer` - The serializer
/// 
/// # Returns
/// * `Result<S::Ok, S::Error>` - The serialization result
pub fn serialize_optional_date<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => serializer.serialize_str(&format_date(dt)),
        None => serializer.serialize_none(),
    }
}

/// Deserialize a string to an Option<DateTime<Utc>>
/// 
/// # Arguments
/// * `deserializer` - The deserializer
/// 
/// # Returns
/// * `Result<Option<DateTime<Utc>>, D::Error>` - The deserialization result
pub fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => parse_date(&s).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid date format: {}", s))
        }).map(Some),
        None => Ok(None),
    }
}

/// Check if a date is before another date
/// 
/// # Arguments
/// * `date` - The date to check
/// * `other` - The date to compare against
/// 
/// # Returns
/// * `bool` - True if date is before other
pub fn is_date_before(date: &DateTime<Utc>, other: &DateTime<Utc>) -> bool {
    date < other
}

/// Check if a date is after another date
/// 
/// # Arguments
/// * `date` - The date to check
/// * `other` - The date to compare against
/// 
/// # Returns
/// * `bool` - True if date is after other
pub fn is_date_after(date: &DateTime<Utc>, other: &DateTime<Utc>) -> bool {
    date > other
}

/// Check if a date is between two other dates
/// 
/// # Arguments
/// * `date` - The date to check
/// * `start` - The start date
/// * `end` - The end date
/// * `inclusive` - Whether to include the start and end dates (default: true)
/// 
/// # Returns
/// * `bool` - True if date is between start and end
pub fn is_date_between(date: &DateTime<Utc>, start: &DateTime<Utc>, end: &DateTime<Utc>, inclusive: Option<bool>) -> bool {
    let inclusive = inclusive.unwrap_or(true);
    
    if inclusive {
        date >= start && date <= end
    } else {
        date > start && date < end
    }
}

/// Get the current date and time
/// 
/// # Returns
/// * `DateTime<Utc>` - The current date and time
pub fn get_current_date() -> DateTime<Utc> {
    Utc::now()
}

/// Add days to a date
/// 
/// # Arguments
/// * `date` - The date to add days to
/// * `days` - The number of days to add
/// 
/// # Returns
/// * `DateTime<Utc>` - The new date
pub fn add_days(date: &DateTime<Utc>, days: i64) -> DateTime<Utc> {
    *date + Duration::days(days)
}

/// Subtract days from a date
/// 
/// # Arguments
/// * `date` - The date to subtract days from
/// * `days` - The number of days to subtract
/// 
/// # Returns
/// * `DateTime<Utc>` - The new date
pub fn subtract_days(date: &DateTime<Utc>, days: i64) -> DateTime<Utc> {
    *date - Duration::days(days)
}

/// Get the difference between two dates in days
/// 
/// # Arguments
/// * `date1` - The first date
/// * `date2` - The second date
/// 
/// # Returns
/// * `i64` - The difference in days
pub fn date_diff_in_days(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> i64 {
    let duration = *date1 - *date2;
    duration.num_days()
}

/// Get the difference between two dates in hours
/// 
/// # Arguments
/// * `date1` - The first date
/// * `date2` - The second date
/// 
/// # Returns
/// * `i64` - The difference in hours
pub fn date_diff_in_hours(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> i64 {
    let duration = *date1 - *date2;
    duration.num_hours()
}

/// Get the difference between two dates in minutes
/// 
/// # Arguments
/// * `date1` - The first date
/// * `date2` - The second date
/// 
/// # Returns
/// * `i64` - The difference in minutes
pub fn date_diff_in_minutes(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> i64 {
    let duration = *date1 - *date2;
    duration.num_minutes()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_date() {
        // Test ISO 8601 format
        let date_str = "2023-07-17T12:34:56Z";
        let date = parse_date(date_str).unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 17);
        assert_eq!(date.hour(), 12);
        assert_eq!(date.minute(), 34);
        assert_eq!(date.second(), 56);
        
        // Test MySQL datetime format
        let date_str = "2023-07-17 12:34:56";
        let date = parse_date(date_str).unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 17);
        assert_eq!(date.hour(), 12);
        assert_eq!(date.minute(), 34);
        assert_eq!(date.second(), 56);
        
        // Test simple date format
        let date_str = "2023-07-17";
        let date = parse_date(date_str).unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 17);
        assert_eq!(date.hour(), 0);
        assert_eq!(date.minute(), 0);
        assert_eq!(date.second(), 0);
        
        // Test empty string
        let date_str = "";
        let date = parse_date(date_str);
        assert!(date.is_none());
        
        // Test invalid format
        let date_str = "invalid";
        let date = parse_date(date_str);
        assert!(date.is_none());
    }
    
    #[test]
    fn test_format_date() {
        let date = Utc.ymd(2023, 7, 17).and_hms(12, 34, 56);
        let date_str = format_date(&date);
        assert_eq!(date_str, "2023-07-17T12:34:56+00:00");
    }
    
    #[test]
    fn test_format_date_for_display() {
        let date = Utc.ymd(2023, 7, 17).and_hms(12, 34, 56);
        let date_str = format_date_for_display(&date, None);
        assert_eq!(date_str, "Jul 17, 2023 12:34");
        
        let date_str = format_date_for_display(&date, Some("%Y-%m-%d"));
        assert_eq!(date_str, "2023-07-17");
    }
    
    #[test]
    fn test_date_comparison() {
        let date1 = Utc.ymd(2023, 7, 17).and_hms(12, 34, 56);
        let date2 = Utc.ymd(2023, 7, 18).and_hms(12, 34, 56);
        
        assert!(is_date_before(&date1, &date2));
        assert!(!is_date_before(&date2, &date1));
        
        assert!(is_date_after(&date2, &date1));
        assert!(!is_date_after(&date1, &date2));
        
        let date3 = Utc.ymd(2023, 7, 19).and_hms(12, 34, 56);
        assert!(is_date_between(&date2, &date1, &date3, None));
        assert!(is_date_between(&date1, &date1, &date3, None));
        assert!(is_date_between(&date3, &date1, &date3, None));
        assert!(!is_date_between(&date1, &date2, &date3, None));
        
        assert!(is_date_between(&date2, &date1, &date3, Some(true)));
        assert!(!is_date_between(&date1, &date1, &date3, Some(false)));
        assert!(!is_date_between(&date3, &date1, &date3, Some(false)));
    }
    
    #[test]
    fn test_date_arithmetic() {
        let date = Utc.ymd(2023, 7, 17).and_hms(12, 34, 56);
        
        let new_date = add_days(&date, 1);
        assert_eq!(new_date.day(), 18);
        
        let new_date = subtract_days(&date, 1);
        assert_eq!(new_date.day(), 16);
        
        let date1 = Utc.ymd(2023, 7, 17).and_hms(12, 34, 56);
        let date2 = Utc.ymd(2023, 7, 18).and_hms(12, 34, 56);
        
        assert_eq!(date_diff_in_days(&date2, &date1), 1);
        assert_eq!(date_diff_in_hours(&date2, &date1), 24);
        assert_eq!(date_diff_in_minutes(&date2, &date1), 24 * 60);
    }
}
