use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::DateTime;

use crate::{
    AppState,
    models::{assignment::Assignment, topic::Topic, post::Post},
    services::assignment_topic_mapper::AssignmentTopicMapper,
};

#[derive(Deserialize)]
pub struct CreateAssignmentRequest {
    canvas_id: String,
    course_id: String,
    title: String,
    description: Option<String>,
    points_possible: f64,
    due_date: Option<String>,
    unlock_date: Option<String>,
    lock_date: Option<String>,
}

#[derive(Serialize)]
pub struct AssignmentResponse {
    assignment: Assignment,
}

#[derive(Serialize)]
pub struct AssignmentTopicResponse {
    assignment: Assignment,
    topic: Topic,
    posts: Vec<Post>,
}

#[derive(Deserialize)]
pub struct CreateAssignmentTopicRequest {
    category_id: String,
}

pub async fn create_assignment(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAssignmentRequest>,
) -> Result<Json<Assignment>, StatusCode> {
    let course_id = Uuid::parse_str(&payload.course_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Parse dates if provided
    let due_date = payload.due_date
        .as_deref()
        .map(DateTime::parse_from_rfc3339)
        .transpose()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let unlock_date = payload.unlock_date
        .as_deref()
        .map(DateTime::parse_from_rfc3339)
        .transpose()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let lock_date = payload.lock_date
        .as_deref()
        .map(DateTime::parse_from_rfc3339)
        .transpose()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .map(|dt| dt.with_timezone(&chrono::Utc));
    
    // Create assignment
    let assignment = Assignment::new(
        payload.canvas_id,
        course_id,
        payload.title,
        payload.description,
        payload.points_possible,
        due_date,
        unlock_date,
        lock_date,
    );
    
    // Save assignment
    let created_assignment = state.assignment_repo.create_assignment(&assignment)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(created_assignment))
}

pub async fn get_assignment(
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<String>,
) -> Result<Json<AssignmentResponse>, StatusCode> {
    let assignment_id = Uuid::parse_str(&assignment_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let assignment = state.assignment_repo.find_assignment_by_id(&assignment_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(AssignmentResponse { assignment }))
}

#[derive(Serialize)]
pub struct AssignmentWithTopicResponse {
    assignment: Assignment,
    topic: Option<Topic>,
}

pub async fn get_assignment_with_topic(
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<String>,
) -> Result<Json<AssignmentWithTopicResponse>, StatusCode> {
    let id = Uuid::parse_str(&assignment_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Get assignment
    let assignment = state.assignment_repo.find_assignment_by_id(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Get topic if it exists
    let topic = match assignment.topic_id {
        Some(topic_id) => {
            state.topic_repo.find_topic_by_id(&topic_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        },
        None => None,
    };
    
    Ok(Json(AssignmentWithTopicResponse { assignment, topic }))
}

pub async fn get_assignments_by_course(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<Assignment>>, StatusCode> {
    let id = Uuid::parse_str(&course_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Get assignments
    let assignments = state.assignment_repo.find_assignments_by_course(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(assignments))
}

pub async fn list_course_assignments(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<Assignment>>, StatusCode> {
    let course_id = Uuid::parse_str(&course_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let assignments = state.assignment_repo.list_assignments_by_course(&course_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(assignments))
}

pub async fn list_course_discussion_assignments(
    State(state): State<Arc<AppState>>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<Assignment>>, StatusCode> {
    let course_id = Uuid::parse_str(&course_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let assignments = state.assignment_repo.list_discussion_assignments_by_course(&course_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(assignments))
}

#[derive(Deserialize)]
pub struct CreateTopicFromAssignmentRequest {
    category_id: String,
}

pub async fn create_topic_from_assignment(
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<String>,
    Json(payload): Json<CreateAssignmentTopicRequest>,
) -> Result<Json<AssignmentTopicResponse>, StatusCode> {
    let assignment_id = Uuid::parse_str(&assignment_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let category_id = Uuid::parse_str(&payload.category_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Get user ID from auth middleware or token
    let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000") // Placeholder - this should come from auth
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Create mapper
    let mapper = AssignmentTopicMapper::new(
        state.assignment_repo.clone(),
        state.topic_repo.clone(),
        state.post_repo.clone(),
    );
    
    // Create topic from assignment
    let (topic, _post) = mapper.create_topic_from_assignment(&assignment_id, category_id, user_id)
        .await
        .map_err(|e| {
            eprintln!("Error creating topic from assignment: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Get updated assignment
    let assignment = state.assignment_repo.find_assignment_by_id(&assignment_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Get all posts
    let posts = state.post_repo.list_posts_by_topic(&topic.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(AssignmentTopicResponse {
        assignment,
        topic,
        posts,
    }))
}

pub async fn get_assignment_topic(
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<String>,
) -> Result<Json<AssignmentTopicResponse>, StatusCode> {
    let assignment_id = Uuid::parse_str(&assignment_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mapper = AssignmentTopicMapper::new(
        state.assignment_repo.clone(),
        state.topic_repo.clone(),
        state.post_repo.clone(),
    );
    
    let result = mapper.get_topic_for_assignment(&assignment_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match result {
        Some((topic, posts)) => {
            // Get assignment
            let assignment = state.assignment_repo.find_assignment_by_id(&assignment_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::NOT_FOUND)?;
            
            Ok(Json(AssignmentTopicResponse {
                assignment,
                topic,
                posts,
            }))
        },
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Deserialize)]
pub struct MapTopicToAssignmentRequest {
    topic_id: String,
}

pub async fn map_topic_to_assignment(
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<String>,
    Json(payload): Json<MapTopicToAssignmentRequest>,
) -> Result<Json<AssignmentWithTopicResponse>, StatusCode> {
    let assignment_id = Uuid::parse_str(&assignment_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let topic_id = Uuid::parse_str(&payload.topic_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Create mapper service
    let mapper = AssignmentTopicMapper::new(
        state.assignment_repo.clone(),
        state.topic_repo.clone(),
        state.post_repo.clone(),
    );
    
    // Map topic to assignment
    let (assignment, topic) = mapper.map_topic_to_assignment(
        &topic_id,
        &assignment_id,
    ).await.map_err(|e| {
        eprintln!("Error mapping topic to assignment: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(AssignmentWithTopicResponse {
        assignment,
        topic: Some(topic),
    }))
}

pub async fn unmap_topic_from_assignment(
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let assignment_id = Uuid::parse_str(&assignment_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Get assignment
    let assignment = state.assignment_repo.find_assignment_by_id(&assignment_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Check if assignment has a topic
    let topic_id = match assignment.topic_id {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    
    // Create mapper service
    let mapper = AssignmentTopicMapper::new(
        state.assignment_repo.clone(),
        state.topic_repo.clone(),
        state.post_repo.clone(),
    );
    
    // Unmap topic from assignment
    mapper.unmap_topic_from_assignment(&topic_id)
        .await
        .map_err(|e| {
            eprintln!("Error unmapping topic from assignment: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(StatusCode::OK)
}

pub async fn unlink_assignment_topic(
    State(state): State<Arc<AppState>>,
    Path(assignment_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let assignment_id = Uuid::parse_str(&assignment_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mapper = AssignmentTopicMapper::new(
        state.assignment_repo.clone(),
        state.topic_repo.clone(),
        state.post_repo.clone(),
    );
    
    mapper.unlink_topic_from_assignment(&assignment_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
}