use crate::models::forum::{PostWithUser, CreatePostRequest, UpdatePostRequest};
use crate::repository::ForumPostRepository;
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

#[derive(Debug, Deserialize)]
pub struct PostsQuery {
    page: Option<usize>,
    per_page: Option<usize>,
}

// Get posts for a topic
pub async fn get_posts_for_topic(
    State(state): State<AppState>,
    Path(topic_id): Path<i64>,
    Query(query): Query<PostsQuery>,
) -> Result<Json<Vec<PostWithUser>>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    match repository.get_posts_for_topic(topic_id, query.page, query.per_page) {
        Ok(posts) => Ok(Json(posts)),
        Err(e) => {
            eprintln!("Error fetching posts: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch posts").into_response())
        }
    }
}

// Get a single post
pub async fn get_post(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
) -> Result<Json<PostWithUser>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    match repository.get_post_with_user(post_id) {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            eprintln!("Error fetching post: {:?}", e);
            Err((StatusCode::NOT_FOUND, "Post not found").into_response())
        }
    }
}

// Create a new post
pub async fn create_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreatePostRequest>,
) -> Result<Json<PostWithUser>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    match repository.create_post(&request, auth_user.id) {
        Ok(post) => match repository.get_post_with_user(post.id) {
            Ok(post_with_user) => Ok(Json(post_with_user)),
            Err(e) => {
                eprintln!("Error fetching created post: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch created post").into_response())
            }
        },
        Err(e) => {
            eprintln!("Error creating post: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create post").into_response())
        }
    }
}

// Update an existing post
pub async fn update_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<i64>,
    Json(request): Json<UpdatePostRequest>,
) -> Result<Json<PostWithUser>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    // Check if user owns the post or is admin
    match repository.get_post(post_id) {
        Ok(post) => {
            if post.user_id != auth_user.id && auth_user.role != "admin" {
                return Err((StatusCode::FORBIDDEN, "You don't have permission to update this post").into_response());
            }
        },
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "Post not found").into_response());
        }
    }
    
    match repository.update_post(post_id, &request) {
        Ok(_) => match repository.get_post_with_user(post_id) {
            Ok(post_with_user) => Ok(Json(post_with_user)),
            Err(e) => {
                eprintln!("Error fetching updated post: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch updated post").into_response())
            }
        },
        Err(e) => {
            eprintln!("Error updating post: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update post").into_response())
        }
    }
}

// Delete a post
pub async fn delete_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<i64>,
) -> Result<(), Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    // Check if user owns the post or is admin
    match repository.get_post(post_id) {
        Ok(post) => {
            if post.user_id != auth_user.id && auth_user.role != "admin" {
                return Err((StatusCode::FORBIDDEN, "You don't have permission to delete this post").into_response());
            }
        },
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "Post not found").into_response());
        }
    }
    
    match repository.delete_post(post_id) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error deleting post: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete post").into_response())
        }
    }
}

// Mark a post as solution
pub async fn mark_as_solution(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<i64>,
) -> Result<Json<PostWithUser>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    // Get the post to check the topic
    let post = match repository.get_post(post_id) {
        Ok(post) => post,
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "Post not found").into_response());
        }
    };
    
    // TODO: Check if user is allowed to mark solution (topic author or admin)
    
    match repository.mark_as_solution(post_id) {
        Ok(_) => match repository.get_post_with_user(post_id) {
            Ok(post_with_user) => Ok(Json(post_with_user)),
            Err(e) => {
                eprintln!("Error fetching solution post: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch solution post").into_response())
            }
        },
        Err(e) => {
            eprintln!("Error marking post as solution: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to mark post as solution").into_response())
        }
    }
}

// Like a post
pub async fn like_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<i64>,
) -> Result<Json<PostWithUser>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    match repository.like_post(post_id, auth_user.id) {
        Ok(_) => match repository.get_post_with_user(post_id) {
            Ok(post_with_user) => Ok(Json(post_with_user)),
            Err(e) => {
                eprintln!("Error fetching liked post: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch liked post").into_response())
            }
        },
        Err(e) => {
            eprintln!("Error liking post: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to like post").into_response())
        }
    }
}

// Unlike a post
pub async fn unlike_post(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<i64>,
) -> Result<Json<PostWithUser>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    
    match repository.unlike_post(post_id, auth_user.id) {
        Ok(_) => match repository.get_post_with_user(post_id) {
            Ok(post_with_user) => Ok(Json(post_with_user)),
            Err(e) => {
                eprintln!("Error fetching unliked post: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch unliked post").into_response())
            }
        },
        Err(e) => {
            eprintln!("Error unliking post: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to unlike post").into_response())
        }
    }
}

// Get recent posts (for activity feed)
pub async fn get_recent_posts(
    State(state): State<AppState>,
    Query(query): Query<PostsQuery>,
) -> Result<Json<Vec<PostWithUser>>, Response> {
    let repository = ForumPostRepository::new(state.db.clone());
    let limit = query.per_page.unwrap_or(10);
    
    match repository.get_recent_posts(limit) {
        Ok(posts) => Ok(Json(posts)),
        Err(e) => {
            eprintln!("Error fetching recent posts: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch recent posts").into_response())
        }
    }
}