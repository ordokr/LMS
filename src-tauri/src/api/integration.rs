use crate::models::forum::{Category, Topic, CreateCategoryRequest, CreateTopicRequest};
use crate::repository::{IntegrationRepository, ForumCategoryRepository, ForumTopicRepository, CourseRepository};
use crate::auth::AuthUser;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
    Extension,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tauri::{State as TauriState, command};
use crate::models::integration::{CourseCategory, CourseCategoryCreate, CourseCategoryUpdate};
use crate::db::course_category_repository::CourseCategoryRepository;
use crate::models::integration::{CourseCategoryMapping, CourseCategoryCreate};
use std::sync::Arc;
use tracing::{info, warn, error, instrument};

#[derive(Debug, Deserialize)]
pub struct ActivityQuery {
    limit: Option<usize>,
}

// Get or create a forum category for a course
pub async fn get_or_create_course_category(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(course_id): Path<i64>,
) -> Result<Json<Category>, Response> {
    let integration_repo = IntegrationRepository::new(state.db.clone());
    let category_repo = ForumCategoryRepository::new(state.db.clone());
    let course_repo = CourseRepository::new(state.db.clone());
    
    // First check if category already exists
    if let Ok(Some(category)) = integration_repo.get_category_for_course(course_id) {
        return Ok(Json(category));
    }
    
    // Get course details to create category
    let course = match course_repo.get_course(course_id) {
        Ok(course) => course,
        Err(_) => return Err((StatusCode::NOT_FOUND, "Course not found").into_response()),
    };
    
    // Create category for course
    let request = CreateCategoryRequest {
        name: format!("Course: {}", course.title),
        slug: slugify(&course.code),
        description: Some(course.description.clone().unwrap_or_default()),
        parent_id: None,
        course_id: Some(course.id),
        color: Some("#3498db".to_string()),
        text_color: Some("#ffffff".to_string()),
    };
    
    match category_repo.create_category(&request) {
        Ok(category) => {
            // Link category to course
            if let Err(e) = integration_repo.link_course_to_category(course_id, category.id) {
                eprintln!("Error linking course to category: {:?}", e);
                // Continue even if linking fails
            }
            Ok(Json(category))
        },
        Err(e) => {
            eprintln!("Error creating course category: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create course category").into_response())
        }
    }
}

// Get category for a course
pub async fn get_course_category(
    State(state): State<AppState>,
    Path(course_id): Path<i64>,
) -> Result<Json<Category>, Response> {
    let integration_repo = IntegrationRepository::new(state.db.clone());
    
    match integration_repo.get_category_for_course(course_id) {
        Ok(Some(category)) => Ok(Json(category)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "No category found for course").into_response()),
        Err(e) => {
            eprintln!("Error fetching course category: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch course category").into_response())
        }
    }
}

// Create a discussion topic for a module
pub async fn create_module_discussion(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((course_id, module_id)): Path<(i64, i64)>,
) -> Result<Json<Topic>, Response> {
    let integration_repo = IntegrationRepository::new(state.db.clone());
    let topic_repo = ForumTopicRepository::new(state.db.clone());
    let course_repo = CourseRepository::new(state.db.clone());
    
    // First check if a topic already exists for this module
    if let Ok(Some(topic)) = integration_repo.get_topic_for_module(module_id) {
        return Ok(Json(topic));
    }
    
    // Get module details
    let module = match course_repo.get_module(module_id) {
        Ok(module) => module,
        Err(_) => return Err((StatusCode::NOT_FOUND, "Module not found").into_response()),
    };
    
    // Get or create a category for the course
    let category = match get_or_create_course_category(
        State(state.clone()),
        Extension(auth_user.clone()),
        Path(course_id),
    ).await {
        Ok(Json(category)) => category,
        Err(e) => return Err(e),
    };
    
    // Create a topic for the module
    let request = CreateTopicRequest {
        title: format!("Discussion: {}", module.title),
        slug: slugify(&module.title),
        category_id: category.id,
        content: format!(
            "This is a discussion board for the \"{}\" module. Feel free to ask questions and discuss the content.",
            module.title
        ),
    };
    
    match topic_repo.create_topic(&request, auth_user.id) {
        Ok(topic) => {
            // Link the module to the topic
            if let Err(e) = integration_repo.link_module_to_topic(module_id, topic.id) {
                eprintln!("Error linking module to topic: {:?}", e);
                // Continue even if linking fails
            }
            Ok(Json(topic))
        },
        Err(e) => {
            eprintln!("Error creating module discussion: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create module discussion").into_response())
        }
    }
}

// Get topic for a module
pub async fn get_module_topic(
    State(state): State<AppState>,
    Path(module_id): Path<i64>,
) -> Result<Json<Topic>, Response> {
    let integration_repo = IntegrationRepository::new(state.db.clone());
    
    match integration_repo.get_topic_for_module(module_id) {
        Ok(Some(topic)) => Ok(Json(topic)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "No discussion topic found for module").into_response()),
        Err(e) => {
            eprintln!("Error fetching module topic: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch module topic").into_response())
        }
    }
}

// Create a discussion topic for an assignment
pub async fn create_assignment_discussion(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((course_id, assignment_id)): Path<(i64, i64)>,
) -> Result<Json<Topic>, Response> {
    let integration_repo = IntegrationRepository::new(state.db.clone());
    let topic_repo = ForumTopicRepository::new(state.db.clone());
    let course_repo = CourseRepository::new(state.db.clone());
    
    // First check if a topic already exists for this assignment
    if let Ok(Some(topic)) = integration_repo.get_topic_for_assignment(assignment_id) {
        return Ok(Json(topic));
    }
    
    // Get assignment details
    let assignment = match course_repo.get_assignment(assignment_id) {
        Ok(assignment) => assignment,
        Err(_) => return Err((StatusCode::NOT_FOUND, "Assignment not found").into_response()),
    };
    
    // Get or create a category for the course
    let category = match get_or_create_course_category(
        State(state.clone()),
        Extension(auth_user.clone()),
        Path(course_id),
    ).await {
        Ok(Json(category)) => category,
        Err(e) => return Err(e),
    };
    
    // Create a topic for the assignment
    let request = CreateTopicRequest {
        title: format!("Assignment: {}", assignment.title),
        slug: slugify(&assignment.title),
        category_id: category.id,
        content: format!(
            "This is a discussion board for the \"{}\" assignment. Post your questions and help others here.",
            assignment.title
        ),
    };
    
    match topic_repo.create_topic(&request, auth_user.id) {
        Ok(topic) => {
            // Link the assignment to the topic
            if let Err(e) = integration_repo.link_assignment_to_topic(assignment_id, topic.id) {
                eprintln!("Error linking assignment to topic: {:?}", e);
                // Continue even if linking fails
            }
            Ok(Json(topic))
        },
        Err(e) => {
            eprintln!("Error creating assignment discussion: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create assignment discussion").into_response())
        }
    }
}

// Get topic for an assignment
pub async fn get_assignment_topic(
    State(state): State<AppState>,
    Path(assignment_id): Path<i64>,
) -> Result<Json<Topic>, Response> {
    let integration_repo = IntegrationRepository::new(state.db.clone());
    
    match integration_repo.get_topic_for_assignment(assignment_id) {
        Ok(Some(topic)) => Ok(Json(topic)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "No discussion topic found for assignment").into_response()),
        Err(e) => {
            eprintln!("Error fetching assignment topic: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch assignment topic").into_response())
        }
    }
}

// Get recent forum activity for a course
pub async fn get_course_forum_activity(
    State(state): State<AppState>,
    Path(course_id): Path<i64>,
    Query(query): Query<ActivityQuery>,
) -> Result<Json<Vec<Topic>>, Response> {
    let integration_repo = IntegrationRepository::new(state.db.clone());
    let limit = query.limit.unwrap_or(5);
    
    match integration_repo.get_recent_course_activity(course_id, limit) {
        Ok(topics) => Ok(Json(topics)),
        Err(e) => {
            eprintln!("Error fetching course forum activity: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch course forum activity").into_response())
        }
    }
}

// Utility function to create URL-friendly slugs
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .replace("--", "-")
}

#[command]
pub async fn create_course_category_mapping(
    mapping: CourseCategoryCreate, 
    repo: State<'_, CourseCategoryRepository>
) -> Result<CourseCategory, String> {
    repo.create(mapping)
        .await
        .map_err(|e| format!("Failed to create mapping: {}", e))
}

#[command]
pub async fn get_course_category_mapping(
    id: String, 
    repo: State<'_, CourseCategoryRepository>
) -> Result<Option<CourseCategory>, String> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    repo.find_by_id(uuid)
        .await
        .map_err(|e| format!("Failed to get mapping: {}", e))
}

#[command]
pub async fn get_course_category_mapping_by_canvas_course(
    canvas_course_id: String,
    repo: State<'_, CourseCategoryRepository>
) -> Result<Option<CourseCategory>, String> {
    repo.find_by_canvas_course_id(&canvas_course_id)
        .await
        .map_err(|e| format!("Failed to get mapping: {}", e))
}

#[command]
pub async fn get_all_course_category_mappings(
    repo: State<'_, CourseCategoryRepository>
) -> Result<Vec<CourseCategory>, String> {
    repo.find_all()
        .await
        .map_err(|e| format!("Failed to get mappings: {}", e))
}

#[command]
pub async fn update_course_category_mapping(
    id: String,
    update_data: CourseCategoryUpdate,
    repo: State<'_, CourseCategoryRepository>
) -> Result<Option<CourseCategory>, String> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    repo.update(uuid, update_data)
        .await
        .map_err(|e| format!("Failed to update mapping: {}", e))
}

#[command]
pub async fn delete_course_category_mapping(
    id: String,
    repo: State<'_, CourseCategoryRepository>
) -> Result<bool, String> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    repo.delete(uuid)
        .await
        .map_err(|e| format!("Failed to delete mapping: {}", e))
}

#[command]
pub async fn sync_course_category(
    id: String,
    repo: State<'_, CourseCategoryRepository>
) -> Result<CourseCategory, String> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    // Get the mapping
    let mapping = repo.find_by_id(uuid)
        .await
        .map_err(|e| format!("Failed to get mapping: {}", e))?
        .ok_or_else(|| "Mapping not found".to_string())?;
    
    // Perform synchronization logic here
    // This would involve calling both Canvas and Discourse APIs
    // to ensure the course content is synced with the Discourse category
    
    // For now, we'll just update the last_synced_at timestamp
    let now = chrono::Utc::now();
    let update = CourseCategoryUpdate {
        sync_enabled: None,  // Don't change this
        last_synced_at: Some(now),
    };
    
    repo.update(uuid, update)
        .await
        .map_err(|e| format!("Failed to update sync timestamp: {}", e))?
        .ok_or_else(|| "Mapping not found after sync".to_string())
}

/// Creates a new mapping between a Canvas course and a Discourse category
///
/// # Arguments
/// * `mapping_create` - The mapping information to create
///
/// # Returns
/// * `CourseCategoryMapping` - The created mapping with ID
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn create_course_category_mapping(
    mapping_create: CourseCategoryCreate,
    repo: State<'_, Arc<dyn CourseCategoryRepository + Send + Sync>>
) -> Result<CourseCategoryMapping, String> {
    info!(
        event = "api_call", 
        endpoint = "create_course_category_mapping", 
        course_id = %mapping_create.course_id,
        category_id = %mapping_create.category_id
    );
    
    // Check if mapping already exists
    let existing = repo.find_by_course_and_category(
        &mapping_create.course_id, 
        &mapping_create.category_id
    ).await;
    
    if let Ok(Some(_)) = existing {
        warn!(
            event = "api_duplicate", 
            endpoint = "create_course_category_mapping",
            course_id = %mapping_create.course_id, 
            category_id = %mapping_create.category_id
        );
        return Err("Mapping already exists for this course and category".to_string());
    }
    
    // Generate new mapping with UUID
    let mapping = CourseCategoryMapping {
        id: Uuid::new_v4().to_string(),
        course_id: mapping_create.course_id,
        category_id: mapping_create.category_id,
        sync_topics: mapping_create.sync_topics.unwrap_or(true),
        sync_assignments: mapping_create.sync_assignments.unwrap_or(false),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    // Insert mapping
    match repo.create_mapping(mapping.clone()).await {
        Ok(_) => {
            info!(
                event = "api_success", 
                endpoint = "create_course_category_mapping",
                mapping_id = %mapping.id
            );
            Ok(mapping)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "create_course_category_mapping", 
                error = %e
            );
            Err(format!("Failed to create mapping: {}", e))
        }
    }
}

/// Retrieves a course-category mapping by ID
///
/// # Arguments
/// * `mapping_id` - The ID of the mapping to retrieve
///
/// # Returns
/// * `CourseCategoryMapping` - The retrieved mapping
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn get_course_category_mapping(
    mapping_id: String,
    repo: State<'_, Arc<dyn CourseCategoryRepository + Send + Sync>>
) -> Result<CourseCategoryMapping, String> {
    info!(
        event = "api_call", 
        endpoint = "get_course_category_mapping", 
        mapping_id = %mapping_id
    );
    
    match repo.find_by_id(&mapping_id).await {
        Ok(Some(mapping)) => {
            info!(
                event = "api_success", 
                endpoint = "get_course_category_mapping", 
                mapping_id = %mapping_id
            );
            Ok(mapping)
        },
        Ok(None) => {
            warn!(
                event = "api_not_found", 
                endpoint = "get_course_category_mapping", 
                mapping_id = %mapping_id
            );
            Err(format!("Mapping not found with ID: {}", mapping_id))
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "get_course_category_mapping", 
                error = %e
            );
            Err(format!("Database error: {}", e))
        }
    }
}

/// Syncs content between a Canvas course and Discourse category
///
/// # Arguments
/// * `mapping_id` - The ID of the mapping to sync
///
/// # Returns
/// * `SyncResult` - Results of the synchronization
#[tauri::command]
#[instrument(skip(repo, sync_service), err)]
pub async fn sync_course_category(
    mapping_id: String,
    repo: State<'_, Arc<dyn CourseCategoryRepository + Send + Sync>>,
    sync_service: State<'_, crate::services::sync::SyncService>
) -> Result<crate::models::sync::SyncResult, String> {
    info!(
        event = "api_call", 
        endpoint = "sync_course_category", 
        mapping_id = %mapping_id
    );
    
    // Perform synchronization using the new bidirectional sync service
    match sync_service.sync_course_category(&mapping_id).await {
        Ok(result) => {
            info!(
                event = "api_success", 
                endpoint = "sync_course_category", 
                mapping_id = %mapping_id,
                topics_synced = result.topics_synced,
                posts_synced = result.posts_synced
            );
            Ok(result)
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "sync_course_category", 
                mapping_id = %mapping_id,
                error = %e
            );
            Err(format!("Sync failed: {}", e))
        }
    }
}

/// Updates the sync direction for a course-category mapping
///
/// # Arguments
/// * `mapping_id` - The ID of the mapping to update
/// * `direction` - The new sync direction ("CanvasToDiscourse", "DiscourseToCanvas", or "Bidirectional")
///
/// # Returns
/// * `CourseCategory` - The updated mapping
#[tauri::command]
#[instrument(skip(repo), err)]
pub async fn update_course_category_sync_direction(
    mapping_id: String,
    direction: String,
    repo: State<'_, Arc<dyn CourseCategoryRepository + Send + Sync>>
) -> Result<CourseCategory, String> {
    info!(
        event = "api_call", 
        endpoint = "update_course_category_sync_direction", 
        mapping_id = %mapping_id,
        direction = %direction
    );
    
    // Parse UUID
    let uuid = Uuid::parse_str(&mapping_id)
        .map_err(|e| {
            error!("Invalid UUID: {}", e);
            format!("Invalid UUID: {}", e)
        })?;
    
    // Parse direction
    let sync_direction = match direction.as_str() {
        "CanvasToDiscourse" => crate::models::integration::SyncDirection::CanvasToDiscourse,
        "DiscourseToCanvas" => crate::models::integration::SyncDirection::DiscourseToCanvas,
        "Bidirectional" => crate::models::integration::SyncDirection::Bidirectional,
        _ => {
            error!("Invalid sync direction: {}", direction);
            return Err(format!("Invalid sync direction: {}. Must be 'CanvasToDiscourse', 'DiscourseToCanvas', or 'Bidirectional'", direction));
        }
    };
    
    // Update the mapping
    let update = crate::models::integration::CourseCategoryUpdate {
        sync_enabled: None,
        sync_direction: Some(sync_direction),
        last_synced_at: None,
    };
    
    match repo.update(uuid, update).await {
        Ok(Some(updated)) => {
            info!(
                event = "api_success", 
                endpoint = "update_course_category_sync_direction", 
                mapping_id = %mapping_id
            );
            Ok(updated)
        },
        Ok(None) => {
            warn!(
                event = "api_not_found", 
                endpoint = "update_course_category_sync_direction", 
                mapping_id = %mapping_id
            );
            Err(format!("Mapping not found with ID: {}", mapping_id))
        },
        Err(e) => {
            error!(
                event = "api_error", 
                endpoint = "update_course_category_sync_direction", 
                error = %e
            );
            Err(format!("Failed to update mapping: {}", e))
        }
    }
}