use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub users: Mutex<HashMap<String, String>>, // email -> password
    pub courses: Mutex<HashMap<String, (String, String, Option<String>)>>, // id -> (name, code, description)
    pub assignments: Mutex<HashMap<String, (String, String, u32)>>, // id -> (title, description, points)
}
