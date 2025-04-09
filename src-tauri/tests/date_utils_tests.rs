#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};
    use crate::utils::date_utils::{
        parse_iso_date,
        format_iso_date,
        format_date_for_display,
        serialize_optional_date,
        deserialize_optional_date
    };
    use serde::{Serialize, Deserialize};
    use serde_json;

    #[test]
    fn test_parse_iso_date() {
        // Test valid ISO 8601 format
        let date_str = "2025-01-15T12:30:45Z";
        let parsed = parse_iso_date(date_str);
        assert!(parsed.is_some());
        
        let expected = Utc.ymd(2025, 1, 15).and_hms(12, 30, 45);
        assert_eq!(parsed.unwrap(), expected);
        
        // Test empty string
        assert!(parse_iso_date("").is_none());
        
        // Test invalid format
        assert!(parse_iso_date("not-a-date").is_none());
        
        // Test alternative formats
        assert!(parse_iso_date("2025-01-15T12:30:45").is_some());
        assert!(parse_iso_date("2025-01-15 12:30:45").is_some());
    }
    
    #[test]
    fn test_format_iso_date() {
        let date = Utc.ymd(2025, 1, 15).and_hms(12, 30, 45);
        let formatted = format_iso_date(&date);
        
        // Should be RFC3339 format
        assert_eq!(formatted, "2025-01-15T12:30:45+00:00");
    }
    
    #[test]
    fn test_format_date_for_display() {
        let date = Utc.ymd(2025, 1, 15).and_hms(12, 30, 45);
        let formatted = format_date_for_display(Some(&date));
        
        // Should be in the specified display format
        assert_eq!(formatted, "Jan 15, 2025 12:30");
        
        // Test None case
        assert_eq!(format_date_for_display(None), "Not set");
    }
    
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestModel {
        #[serde(
            serialize_with = "serialize_optional_date",
            deserialize_with = "deserialize_optional_date"
        )]
        pub date: Option<DateTime<Utc>>,
    }
    
    #[test]
    fn test_serde_date() {
        // Test serializing Some date
        let model = TestModel {
            date: Some(Utc.ymd(2025, 1, 15).and_hms(12, 30, 45)),
        };
        
        let json = serde_json::to_string(&model).unwrap();
        assert_eq!(json, "{\"date\":\"2025-01-15T12:30:45+00:00\"}");
        
        // Test deserializing date
        let deserialized: TestModel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, model);
        
        // Test serializing None
        let null_model = TestModel { date: None };
        let json = serde_json::to_string(&null_model).unwrap();
        assert_eq!(json, "{\"date\":null}");
        
        // Test deserializing None
        let deserialized: TestModel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, null_model);
        
        // Test deserializing empty string as None
        let empty_json = "{\"date\":\"\"}";
        let deserialized: TestModel = serde_json::from_str(&empty_json).unwrap();
        assert_eq!(deserialized, null_model);
    }
}