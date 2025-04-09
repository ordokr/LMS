#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc, TimeZone};
    use crate::utils::date_utils::{parse_date_string, format_date};

    #[test]
    fn test_parse_date_string_iso_format() {
        // Standard ISO 8601 / RFC 3339 format
        let date_str = "2025-04-07T12:30:45Z";
        let parsed = parse_date_string(Some(date_str));
        
        assert!(parsed.is_some());
        let dt = parsed.unwrap();
        
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 4);
        assert_eq!(dt.day(), 7);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_parse_date_string_with_timezone() {
        // RFC 3339 with timezone
        let date_str = "2025-04-07T12:30:45+00:00";
        let parsed = parse_date_string(Some(date_str));
        
        assert!(parsed.is_some());
        let dt = parsed.unwrap();
        
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 4);
        assert_eq!(dt.day(), 7);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_parse_date_string_mysql_format() {
        // MySQL datetime format
        let date_str = "2025-04-07 12:30:45";
        let parsed = parse_date_string(Some(date_str));
        
        assert!(parsed.is_some());
        let dt = parsed.unwrap();
        
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 4);
        assert_eq!(dt.day(), 7);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_parse_date_string_simple_date() {
        // Simple date format (no time)
        let date_str = "2025-04-07";
        let parsed = parse_date_string(Some(date_str));
        
        assert!(parsed.is_some());
        let dt = parsed.unwrap();
        
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 4);
        assert_eq!(dt.day(), 7);
    }

    #[test]
    fn test_parse_date_string_empty() {
        // Empty string should return None
        assert_eq!(parse_date_string(Some("")), None);
    }

    #[test]
    fn test_parse_date_string_none() {
        // None input should return None
        assert_eq!(parse_date_string(None), None);
    }

    #[test]
    fn test_format_date() {
        // Format a known date
        let dt = Utc.with_ymd_and_hms(2025, 4, 7, 12, 30, 45).unwrap();
        let formatted = format_date(&Some(dt));
        
        assert!(formatted.is_some());
        let formatted_str = formatted.unwrap();
        
        // Should match RFC 3339 format
        assert_eq!(formatted_str, "2025-04-07T12:30:45+00:00");
        
        // Should be parseable back to the same date
        let parsed = DateTime::parse_from_rfc3339(&formatted_str).unwrap().with_timezone(&Utc);
        assert_eq!(parsed, dt);
    }

    #[test]
    fn test_format_date_none() {
        // None input should return None
        assert_eq!(format_date(&None), None);
    }
}