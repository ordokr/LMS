use chrono::{DateTime, Utc};
use lms_integration::utils::date_utils::{parse_date_string, format_date};

#[test]
fn test_parse_date_string_various_formats() {
    // ISO8601/RFC3339
    let iso = "2025-04-07T12:30:45Z";
    let dt = parse_date_string(Some(iso)).unwrap();
    assert_eq!(dt.year(), 2025);
    assert_eq!(dt.month(), 4);
    assert_eq!(dt.day(), 7);
    assert_eq!(dt.hour(), 12);
    assert_eq!(dt.minute(), 30);
    assert_eq!(dt.second(), 45);
    
    // MySQL datetime format
    let mysql = "2025-04-07 12:30:45";
    let dt = parse_date_string(Some(mysql)).unwrap();
    assert_eq!(dt.year(), 2025);
    assert_eq!(dt.month(), 4);
    assert_eq!(dt.day(), 7);
    
    // Simple date format
    let simple = "2025-04-07";
    let dt = parse_date_string(Some(simple)).unwrap();
    assert_eq!(dt.year(), 2025);
    assert_eq!(dt.month(), 4);
    assert_eq!(dt.day(), 7);
    
    // Empty string
    assert_eq!(parse_date_string(Some("")), None);
    
    // None
    assert_eq!(parse_date_string(None), None);
}

#[test]
fn test_format_date() {
    let now = Utc::now();
    let formatted = format_date(&Some(now));
    
    assert!(formatted.is_some());
    let formatted_str = formatted.unwrap();
    
    // Should be parseable back to the same date
    let parsed = DateTime::parse_from_rfc3339(&formatted_str).unwrap().with_timezone(&Utc);
    
    // Allow for a small time difference due to precision issues
    let diff = (parsed - now).num_milliseconds().abs();
    assert!(diff < 1000); // Less than 1 second difference
}

#[test]
fn test_course_date_handling() {
    use lms_integration::models::course::Course;
    use serde_json::json;
    
    let canvas_json = json!({
        "id": 12345,
        "name": "Test Course",
        "course_code": "TEST101",
        "start_at": "2025-04-01T00:00:00Z",
        "end_at": "2025-08-31T23:59:59Z",
        "created_at": "2025-03-15T14:30:45Z",
        "updated_at": "2025-03-16T09:15:30Z",
        "workflow_state": "available"
    });
    
    let course = Course::from_canvas_api(&canvas_json).unwrap();
    
    // Verify dates were parsed correctly
    assert_eq!(course.start_date.unwrap().year(), 2025);
    assert_eq!(course.start_date.unwrap().month(), 4);
    assert_eq!(course.start_date.unwrap().day(), 1);
    
    assert_eq!(course.end_date.unwrap().year(), 2025);
    assert_eq!(course.end_date.unwrap().month(), 8);
    assert_eq!(course.end_date.unwrap().day(), 31);
    
    assert_eq!(course.created_at.unwrap().year(), 2025);
    assert_eq!(course.created_at.unwrap().month(), 3);
    assert_eq!(course.created_at.unwrap().day(), 15);
    
    // Serialization test
    let json = serde_json::to_string(&course).unwrap();
    let deserialized: Course = serde_json::from_str(&json).unwrap();
    
    // Dates should be preserved through serialization/deserialization
    assert_eq!(
        course.start_date.unwrap().timestamp(),
        deserialized.start_date.unwrap().timestamp()
    );
}