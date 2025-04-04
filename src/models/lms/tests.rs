#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::models::user::User;

    #[test]
    fn test_course_new() {
        let course = Course::new();
        assert_eq!(course.id, 0);
        assert_eq!(course.name, None);
    }

    #[test]
    fn test_course_available_to_user() {
        let mut course = Course::new();
        course.workflow_state = Some("available".to_string());
        
        let user = User::new();
        
        assert!(course.available_to_user(&user));
        
        course.workflow_state = Some("deleted".to_string());
        assert!(!course.available_to_user(&user));
    }
}