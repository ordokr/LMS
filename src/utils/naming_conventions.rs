// src/utils/naming_conventions.rs
use serde_json::{Value, Map};
use std::collections::HashMap;

/// Standardized naming conventions for the integrated system
/// This addresses the naming inconsistency issue

/// Convert camelCase to snake_case (Canvas style)
pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char_was_lowercase = false;
    
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && prev_char_was_lowercase {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_char_was_lowercase = false;
        } else {
            result.push(c);
            prev_char_was_lowercase = c.is_lowercase();
        }
    }
    
    result
}

/// Convert snake_case to camelCase (Discourse/JS style)
pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut next_is_uppercase = false;
    
    for c in input.chars() {
        if c == '_' {
            next_is_uppercase = true;
        } else if next_is_uppercase {
            result.push(c.to_uppercase().next().unwrap());
            next_is_uppercase = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Convert object keys based on naming convention
pub fn convert_object_keys(value: &Value, to_style: &str) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = Map::new();
            
            for (key, val) in map {
                let new_key = match to_style {
                    "camel" => to_camel_case(key),
                    "snake" => to_snake_case(key),
                    _ => key.clone(),
                };
                
                // Recursively convert nested objects
                let new_val = convert_object_keys(val, to_style);
                new_map.insert(new_key, new_val);
            }
            
            Value::Object(new_map)
        },
        Value::Array(arr) => {
            // Recursively convert items in arrays
            let new_arr: Vec<Value> = arr
                .iter()
                .map(|item| convert_object_keys(item, to_style))
                .collect();
            
            Value::Array(new_arr)
        },
        // For non-object values, return as is
        _ => value.clone(),
    }
}

/// Convert a HashMap's keys based on naming convention
pub fn convert_hash_map_keys<T: Clone>(
    map: &HashMap<String, T>,
    to_style: &str
) -> HashMap<String, T> {
    let mut new_map = HashMap::new();
    
    for (key, val) in map {
        let new_key = match to_style {
            "camel" => to_camel_case(key),
            "snake" => to_snake_case(key),
            _ => key.clone(),
        };
        
        new_map.insert(new_key, val.clone());
    }
    
    new_map
}
