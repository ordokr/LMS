use serde_json::{Map, Value};
use std::collections::HashMap;

/// Convert camelCase to snake_case (Canvas style)
/// 
/// # Arguments
/// * `s` - String to convert
/// 
/// # Returns
/// * Converted string
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert snake_case to camelCase (Discourse/JS style)
/// 
/// # Arguments
/// * `s` - String to convert
/// 
/// # Returns
/// * Converted string
pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert object keys based on naming convention
/// 
/// # Arguments
/// * `value` - JSON value to convert
/// * `to_style` - Target style ("camel" or "snake")
/// 
/// # Returns
/// * Converted JSON value
pub fn convert_json_keys(value: Value, to_style: &str) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = Map::new();
            
            for (key, val) in map {
                let new_key = if to_style == "camel" {
                    to_camel_case(&key)
                } else {
                    to_snake_case(&key)
                };
                
                let new_val = convert_json_keys(val, to_style);
                new_map.insert(new_key, new_val);
            }
            
            Value::Object(new_map)
        },
        Value::Array(arr) => {
            let new_arr = arr.into_iter()
                .map(|val| convert_json_keys(val, to_style))
                .collect();
            
            Value::Array(new_arr)
        },
        _ => value,
    }
}

/// Standard field mappings for common model types
struct StandardFields {
    user: HashMap<String, String>,
    course: HashMap<String, String>,
    discussion: HashMap<String, String>,
    assignment: HashMap<String, String>,
    submission: HashMap<String, String>,
    attachment: HashMap<String, String>,
}

impl StandardFields {
    fn new() -> Self {
        let mut user = HashMap::new();
        user.insert("id".to_string(), "id".to_string());
        user.insert("email".to_string(), "email".to_string());
        user.insert("name".to_string(), "name".to_string());
        user.insert("username".to_string(), "username".to_string());
        user.insert("created_at".to_string(), "createdAt".to_string());
        user.insert("updated_at".to_string(), "updatedAt".to_string());

        let mut course = HashMap::new();
        course.insert("id".to_string(), "id".to_string());
        course.insert("title".to_string(), "title".to_string());
        course.insert("description".to_string(), "description".to_string());
        course.insert("start_date".to_string(), "startDate".to_string());
        course.insert("end_date".to_string(), "endDate".to_string());

        let mut discussion = HashMap::new();
        discussion.insert("id".to_string(), "id".to_string());
        discussion.insert("title".to_string(), "title".to_string());
        discussion.insert("body".to_string(), "body".to_string());
        discussion.insert("created_at".to_string(), "createdAt".to_string());
        discussion.insert("updated_at".to_string(), "updatedAt".to_string());

        let mut assignment = HashMap::new();
        assignment.insert("id".to_string(), "id".to_string());
        assignment.insert("title".to_string(), "title".to_string());
        assignment.insert("description".to_string(), "description".to_string());
        assignment.insert("due_date".to_string(), "dueDate".to_string());
        assignment.insert("points_possible".to_string(), "pointsPossible".to_string());

        let mut submission = HashMap::new();
        submission.insert("id".to_string(), "id".to_string());
        submission.insert("user_id".to_string(), "userId".to_string());
        submission.insert("assignment_id".to_string(), "assignmentId".to_string());
        submission.insert("submitted_at".to_string(), "submittedAt".to_string());
        submission.insert("score".to_string(), "score".to_string());

        let mut attachment = HashMap::new();
        attachment.insert("id".to_string(), "id".to_string());
        attachment.insert("filename".to_string(), "filename".to_string());
        attachment.insert("content_type".to_string(), "contentType".to_string());
        attachment.insert("size".to_string(), "size".to_string());
        attachment.insert("url".to_string(), "url".to_string());

        Self {
            user,
            course,
            discussion,
            assignment,
            submission,
            attachment,
        }
    }

    fn get_field_map(&self, model_name: &str) -> Option<&HashMap<String, String>> {
        match model_name {
            "user" => Some(&self.user),
            "course" => Some(&self.course),
            "discussion" => Some(&self.discussion),
            "assignment" => Some(&self.assignment),
            "submission" => Some(&self.submission),
            "attachment" => Some(&self.attachment),
            _ => None,
        }
    }
}

/// Apply consistent naming to a model instance
/// 
/// # Arguments
/// * `model` - The model to standardize
/// * `model_name` - Name of the model type
/// 
/// # Returns
/// * Standardized JSON model
pub fn standardize_model(model: Value, model_name: &str) -> Value {
    if !model.is_object() {
        return model;
    }

    let standard_fields = StandardFields::new();
    let field_map = match standard_fields.get_field_map(model_name) {
        Some(map) => map,
        None => return model,
    };

    let mut standardized = Map::new();
    let model_obj = model.as_object().unwrap();

    // Map known fields
    for (original_field, standard_field) in field_map {
        if model_obj.contains_key(original_field) {
            standardized.insert(
                standard_field.clone(), 
                model_obj.get(original_field).unwrap().clone()
            );
        } else if model_obj.contains_key(standard_field) {
            standardized.insert(
                standard_field.clone(), 
                model_obj.get(standard_field).unwrap().clone()
            );
        }
    }

    // Copy any other fields
    for (key, value) in model_obj {
        if !standardized.contains_key(key) {
            standardized.insert(key.clone(), value.clone());
        }
    }

    Value::Object(standardized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("userId"), "user_id");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("user_id"), "userId");
        assert_eq!(to_camel_case("hello"), "hello");
    }

    #[test]
    fn test_convert_json_keys() {
        let snake_case = serde_json::json!({
            "user_id": 123,
            "first_name": "John",
            "nested_object": {
                "created_at": "2023-01-01"
            }
        });

        let camel_case = serde_json::json!({
            "userId": 123,
            "firstName": "John",
            "nestedObject": {
                "createdAt": "2023-01-01"
            }
        });

        assert_eq!(convert_json_keys(snake_case.clone(), "camel"), camel_case);
        assert_eq!(convert_json_keys(camel_case, "snake"), snake_case);
    }

    #[test]
    fn test_standardize_model() {
        let user_model = serde_json::json!({
            "id": 1,
            "name": "John Smith",
            "email": "john@example.com",
            "created_at": "2023-01-01",
            "custom_field": "value"
        });

        let standardized = standardize_model(user_model, "user");
        
        assert_eq!(standardized["id"], 1);
        assert_eq!(standardized["name"], "John Smith");
        assert_eq!(standardized["email"], "john@example.com");
        assert_eq!(standardized["createdAt"], "2023-01-01");
        assert_eq!(standardized["custom_field"], "value");
    }
}
