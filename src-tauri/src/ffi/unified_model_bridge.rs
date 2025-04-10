use std::ffi::{c_char, CStr, CString};
use std::os::raw::c_int;
use crate::models::unified::{User, Course, Assignment, Discussion};
use log::{info, error};

// Foreign function declarations for Haskell integration
extern "C" {
    fn processUserData(json_data: *const c_char) -> *const c_char;
    fn processCourseData(json_data: *const c_char) -> *const c_char;
    fn processAssignmentData(json_data: *const c_char) -> *const c_char;
    fn processDiscussionData(json_data: *const c_char) -> *const c_char;
    fn validateUserCredentials(email: *const c_char, password: *const c_char) -> c_int;
}

// Safe wrapper for Haskell user processing
pub fn process_user_with_haskell(user: &User) -> Result<String, String> {
    info!("Processing user {} with Haskell", user.id);
    
    // Convert user to JSON
    let user_json = match serde_json::to_string(user) {
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to serialize user: {}", e)),
    };
    
    // Create C string
    let c_user_json = match CString::new(user_json) {
        Ok(c_str) => c_str,
        Err(e) => return Err(format!("Failed to create CString: {}", e)),
    };
    
    // Call Haskell function
    unsafe {
        let result_ptr = processUserData(c_user_json.as_ptr());
        if result_ptr.is_null() {
            return Err("Haskell returned null pointer".to_string());
        }
        
        let c_result = CStr::from_ptr(result_ptr);
        let result = match c_result.to_str() {
            Ok(s) => s.to_string(),
            Err(e) => return Err(format!("Failed to convert result: {}", e)),
        };
        
        // Free the memory allocated by Haskell (if your Haskell code allocates memory)
        // You would need a corresponding function in your Haskell FFI
        // freeHaskellString(result_ptr);
        
        Ok(result)
    }
}

// Similar functions for other models
pub fn process_course_with_haskell(course: &Course) -> Result<String, String> {
    info!("Processing course with Haskell");
    
    let course_json = match serde_json::to_string(course) {
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to serialize course: {}", e)),
    };
    
    let c_course_json = match CString::new(course_json) {
        Ok(c_str) => c_str,
        Err(e) => return Err(format!("Failed to create CString: {}", e)),
    };
    
    unsafe {
        let result_ptr = processCourseData(c_course_json.as_ptr());
        if result_ptr.is_null() {
            return Err("Haskell returned null pointer".to_string());
        }
        
        let c_result = CStr::from_ptr(result_ptr);
        match c_result.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(format!("Failed to convert result: {}", e)),
        }
    }
}

// Validate user credentials against Haskell authentication system
pub fn validate_credentials(email: &str, password: &str) -> Result<bool, String> {
    let c_email = match CString::new(email) {
        Ok(c_str) => c_str,
        Err(_) => return Err("Email contains null bytes".to_string()),
    };
    
    let c_password = match CString::new(password) {
        Ok(c_str) => c_str,
        Err(_) => return Err("Password contains null bytes".to_string()),
    };
    
    unsafe {
        let result = validateUserCredentials(c_email.as_ptr(), c_password.as_ptr());
        Ok(result != 0)
    }
}