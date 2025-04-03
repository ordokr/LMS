use crate::models::lms::{Course, Module, ModuleItem, Assignment, Enrollment, CourseStatus, ModuleWithItems, CourseCreationRequest};
use crate::utils::auth::get_auth_token;
use crate::utils::errors::ApiError;
use crate::utils::sync::SyncClient;
use crate::utils::offline::is_online;

use gloo_net::http::{Request, RequestBuilder};
use serde_json::json;

const API_BASE_URL: &str = "/api";

pub struct LmsService;

impl LmsService {
    // Helper function to build authenticated requests
    fn build_request(method: &str, endpoint: &str) -> Result<RequestBuilder, ApiError> {
        let token = get_auth_token().ok_or_else(|| ApiError::Unauthorized("Not logged in".to_string()))?;
        
        let builder = match method {
            "GET" => Request::get(&format!("{}{}", API_BASE_URL, endpoint)),
            "POST" => Request::post(&format!("{}{}", API_BASE_URL, endpoint)),
            "PUT" => Request::put(&format!("{}{}", API_BASE_URL, endpoint)),
            "DELETE" => Request::delete(&format!("{}{}", API_BASE_URL, endpoint)),
            _ => return Err(ApiError::BadRequest("Invalid HTTP method".to_string())),
        };
        
        Ok(builder.header("Authorization", &format!("Bearer {}", token)))
    }

    // Course API Methods

    pub async fn get_courses() -> Result<Vec<Course>, ApiError> {
        let sync_client = SyncClient::new();
        
        // Check if we're online
        if is_online() {
            let resp = Self::build_request("GET", "/courses")?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let courses: Vec<Course> = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache courses locally for offline use
            sync_client.cache_data("courses", &courses)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(courses)
        } else {
            // Return cached courses when offline
            sync_client.get_cached_data("courses")
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn get_course(course_id: i64) -> Result<Course, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", &format!("/courses/{}", course_id))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let course: Course = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache course data
            sync_client.cache_data(&format!("course_{}", course_id), &course)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(course)
        } else {
            // Return cached course
            sync_client.get_cached_data(&format!("course_{}", course_id))
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn create_course(course: CourseCreationRequest) -> Result<i64, ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "create",
                "course",
                None,
                serde_json::to_value(course).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(-1); // Temporary ID for offline mode
        }
        
        let resp = Self::build_request("POST", "/courses")?
            .json(&course)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        let response_data: serde_json::Value = resp.json()
            .await
            .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
            
        let course_id = response_data["course_id"]
            .as_i64()
            .ok_or_else(|| ApiError::DeserializationError("Missing course_id in response".to_string()))?;
            
        Ok(course_id)
    }

    pub async fn update_course(course_id: i64, course: Course) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "update",
                "course",
                Some(&course_id.to_string()),
                serde_json::to_value(course).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("PUT", &format!("/courses/{}", course_id))?
            .json(&course)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    pub async fn delete_course(course_id: i64) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "delete",
                "course",
                Some(&course_id.to_string()),
                json!({})
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("DELETE", &format!("/courses/{}", course_id))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    // Module API Methods

    pub async fn get_modules(course_id: i64) -> Result<Vec<Module>, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", &format!("/courses/{}/modules", course_id))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let modules: Vec<Module> = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache modules
            sync_client.cache_data(&format!("modules_{}", course_id), &modules)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(modules)
        } else {
            // Return cached modules
            sync_client.get_cached_data(&format!("modules_{}", course_id))
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn get_module(course_id: i64, module_id: i64) -> Result<ModuleWithItems, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", &format!("/courses/{}/modules/{}", course_id, module_id))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let module_with_items: ModuleWithItems = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache module data
            let cache_key = format!("module_{}_{}", course_id, module_id);
            sync_client.cache_data(&cache_key, &module_with_items)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(module_with_items)
        } else {
            // Return cached module data
            let cache_key = format!("module_{}_{}", course_id, module_id);
            sync_client.get_cached_data(&cache_key)
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn create_module(course_id: i64, module: Module) -> Result<i64, ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "create",
                "module",
                None,
                serde_json::to_value(module).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(-1); // Temporary ID for offline mode
        }
        
        let resp = Self::build_request("POST", &format!("/courses/{}/modules", course_id))?
            .json(&module)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        let response_data: serde_json::Value = resp.json()
            .await
            .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
            
        let module_id = response_data["module_id"]
            .as_i64()
            .ok_or_else(|| ApiError::DeserializationError("Missing module_id in response".to_string()))?;
            
        Ok(module_id)
    }

    pub async fn update_module(course_id: i64, module_id: i64, module: Module) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "update",
                "module",
                Some(&module_id.to_string()),
                serde_json::to_value(module).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("PUT", &format!("/courses/{}/modules/{}", course_id, module_id))?
            .json(&module)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    pub async fn delete_module(course_id: i64, module_id: i64) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "delete",
                "module",
                Some(&module_id.to_string()),
                json!({})
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("DELETE", &format!("/courses/{}/modules/{}", course_id, module_id))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    // Module Items API
    
    pub async fn add_module_item(course_id: i64, module_id: i64, item: ModuleItem) -> Result<i64, ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "create",
                "module_item",
                None,
                serde_json::to_value(item).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(-1); // Temporary ID for offline mode
        }
        
        let resp = Self::build_request("POST", &format!("/courses/{}/modules/{}/items", course_id, module_id))?
            .json(&item)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        let response_data: serde_json::Value = resp.json()
            .await
            .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
            
        let item_id = response_data["item_id"]
            .as_i64()
            .ok_or_else(|| ApiError::DeserializationError("Missing item_id in response".to_string()))?;
            
        Ok(item_id)
    }
    
    pub async fn update_module_item(course_id: i64, module_id: i64, item_id: i64, item: ModuleItem) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "update",
                "module_item",
                Some(&item_id.to_string()),
                serde_json::to_value(item).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("PUT", &format!("/courses/{}/modules/{}/items/{}", course_id, module_id, item_id))?
            .json(&item)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }
    
    pub async fn delete_module_item(course_id: i64, module_id: i64, item_id: i64) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "delete",
                "module_item",
                Some(&item_id.to_string()),
                json!({})
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("DELETE", &format!("/courses/{}/modules/{}/items/{}", course_id, module_id, item_id))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }
    
    // Assignment API Methods
    
    pub async fn get_assignments(course_id: i64) -> Result<Vec<Assignment>, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", &format!("/courses/{}/assignments", course_id))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let assignments: Vec<Assignment> = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache assignments
            sync_client.cache_data(&format!("assignments_{}", course_id), &assignments)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(assignments)
        } else {
            // Return cached assignments
            sync_client.get_cached_data(&format!("assignments_{}", course_id))
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn get_assignment(course_id: i64, assignment_id: i64) -> Result<Assignment, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", &format!("/courses/{}/assignments/{}", course_id, assignment_id))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let assignment: Assignment = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache assignment
            sync_client.cache_data(&format!("assignment_{}_{}", course_id, assignment_id), &assignment)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(assignment)
        } else {
            // Return cached assignment
            sync_client.get_cached_data(&format!("assignment_{}_{}", course_id, assignment_id))
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn create_assignment(course_id: i64, assignment: Assignment) -> Result<i64, ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "create",
                "assignment",
                None,
                serde_json::to_value(assignment).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(-1); // Temporary ID for offline mode
        }
        
        let resp = Self::build_request("POST", &format!("/courses/{}/assignments", course_id))?
            .json(&assignment)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        let response_data: serde_json::Value = resp.json()
            .await
            .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
            
        let assignment_id = response_data["assignment_id"]
            .as_i64()
            .ok_or_else(|| ApiError::DeserializationError("Missing assignment_id in response".to_string()))?;
            
        Ok(assignment_id)
    }

    pub async fn update_assignment(course_id: i64, assignment_id: i64, assignment: Assignment) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "update",
                "assignment",
                Some(&assignment_id.to_string()),
                serde_json::to_value(assignment).unwrap()
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("PUT", &format!("/courses/{}/assignments/{}", course_id, assignment_id))?
            .json(&assignment)
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    pub async fn delete_assignment(course_id: i64, assignment_id: i64) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "delete",
                "assignment",
                Some(&assignment_id.to_string()),
                json!({})
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("DELETE", &format!("/courses/{}/assignments/{}", course_id, assignment_id))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }
    
    // Enrollments API
    
    pub async fn get_enrollments(course_id: i64) -> Result<Vec<Enrollment>, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", &format!("/courses/{}/enrollments", course_id))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let enrollments: Vec<Enrollment> = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache enrollments
            sync_client.cache_data(&format!("enrollments_{}", course_id), &enrollments)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(enrollments)
        } else {
            // Return cached enrollments
            sync_client.get_cached_data(&format!("enrollments_{}", course_id))
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn get_course_enrollments(course_id: i64) -> Result<Vec<Enrollment>, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", &format!("/courses/{}/enrollments", course_id))?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let enrollments: Vec<Enrollment> = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache enrollments
            sync_client.cache_data(&format!("enrollments_{}", course_id), &enrollments)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(enrollments)
        } else {
            // Return cached enrollments
            sync_client.get_cached_data(&format!("enrollments_{}", course_id))
                .map_err(|e| ApiError::CacheError(e))
        }
    }

    pub async fn enroll_user(course_id: i64, user_id: i64, role: &str) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "create",
                "enrollment",
                None,
                serde_json::json!({
                    "course_id": course_id,
                    "user_id": user_id,
                    "role": role,
                })
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("POST", &format!("/courses/{}/enrollments", course_id))?
            .json(&serde_json::json!({
                "user_id": user_id,
                "role": role,
            }))
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    pub async fn update_enrollment(course_id: i64, user_id: i64, role: Option<&str>, status: Option<&str>) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "update",
                "enrollment",
                Some(&format!("{}_{}", course_id, user_id)),
                serde_json::json!({
                    "role": role,
                    "status": status,
                })
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let mut payload = serde_json::Map::new();
        if let Some(r) = role {
            payload.insert("role".to_string(), serde_json::Value::String(r.to_string()));
        }
        if let Some(s) = status {
            payload.insert("status".to_string(), serde_json::Value::String(s.to_string()));
        }
        
        let resp = Self::build_request("PUT", &format!("/courses/{}/enrollments/{}", course_id, user_id))?
            .json(&serde_json::Value::Object(payload))
            .map_err(|e| ApiError::SerializationError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    pub async fn remove_enrollment(course_id: i64, user_id: i64) -> Result<(), ApiError> {
        if !is_online() {
            // Queue for later sync
            let sync_client = SyncClient::new();
            sync_client.queue_operation(
                "delete",
                "enrollment",
                Some(&format!("{}_{}", course_id, user_id)),
                serde_json::json!({})
            ).map_err(|e| ApiError::SyncError(e))?;
            
            return Ok(());
        }
        
        let resp = Self::build_request("DELETE", &format!("/courses/{}/enrollments/{}", course_id, user_id))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;
            
        if !resp.ok() {
            return Err(ApiError::from_response(resp).await);
        }
        
        Ok(())
    }

    // Add this method to your existing LmsService impl
    pub async fn get_user_courses() -> Result<Vec<Course>, ApiError> {
        let sync_client = SyncClient::new();
        
        if is_online() {
            let resp = Self::build_request("GET", "/user/courses")?
                .send()
                .await
                .map_err(|e| ApiError::NetworkError(e.to_string()))?;
                
            if !resp.ok() {
                return Err(ApiError::from_response(resp).await);
            }
            
            let courses: Vec<Course> = resp.json()
                .await
                .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                
            // Cache user courses
            sync_client.cache_data("user_courses", &courses)
                .map_err(|e| ApiError::CacheError(e))?;
                
            Ok(courses)
        } else {
            // Return cached user courses
            sync_client.get_cached_data("user_courses")
                .map_err(|e| ApiError::CacheError(e))
        }
    }
}