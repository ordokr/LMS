mod app;
mod components;
mod lms;
mod forum;
mod utils;
mod config;
mod models;
mod services {
    pub mod course_category_mapper;
    pub mod discussion_topic_integration;
    pub mod assignment_topic_mapper;
    pub mod topic_service;
}
mod db {
    pub mod user_repository;
    pub mod course_repository;
    pub mod category_repository;
    pub mod topic_repository;
    pub mod post_repository;
    pub mod assignment_repository;
}
mod auth {
    pub mod jwt;
    pub mod middleware;
    pub mod routes;
}

use app::App;
use leptos::*;
use_stylesheet!("styles/group_management.css");
use_stylesheet!("styles/integration.css");
use_stylesheet!("styles/integration_dashboard.css");

use crate::config::Config;
use crate::models::lms::Course;
use crate::services::api::ApiClient;
use log::{info, error};
use clap::Parser;
use std::process;
use std::sync::Arc;
use axum::{Router, routing::post, middleware, Server};
use std::net::SocketAddr;

use crate::auth::routes::auth_routes;

/// LMS Application
#[derive(Parser, Debug)]
#[clap(
    name = "lms",
    version = "0.1.0",
    author = "Tim",
    about = "LMS integration tool"
)]
struct Args {
    /// List available courses
    #[clap(long)]
    list_courses: bool,
    
    /// Get a specific course by ID
    #[clap(long)]
    course_id: Option<i64>,
    
    /// List forum topics
    #[clap(long)]
    list_topics: bool,
    
    /// Verbose output
    #[clap(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure JWT_SECRET is set in the environment
    // Initialize the logger
    env_logger::init();
    
    // Parse command line arguments
    let args = Args::parse();
    
    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            error!("Failed to load configuration: {}", err);
            process::exit(1);
        }
    };
    
    // Create API client
    let api_client = ApiClient::new(config);
    
    // Execute commands based on arguments
    if args.list_courses {
        match Course::find_published(&api_client).await {
            Ok(courses) => {
                println!("Found {} published courses:", courses.len());
                for course in courses {
                    println!(" - [{}] {}", course.id, course.name.unwrap_or_else(|| "No name".to_string()));
                }
            },
            Err(err) => {
                error!("Failed to fetch courses: {}", err);
                process::exit(1);
            }
        }
    } else if let Some(id) = args.course_id {
        match api_client.get_course(id).await {
            Ok(course) => {
                println!("Course details for ID {}:", id);
                println!("  Name: {}", course.name.unwrap_or_else(|| "No name".to_string()));
                println!("  Code: {}", course.course_code.unwrap_or_else(|| "No code".to_string()));
                println!("  Status: {}", course.workflow_state.unwrap_or_else(|| "Unknown".to_string()));
                
                // If verbose, fetch and print assignments
                if args.verbose {
                    match course.assignments(&api_client).await {
                        Ok(assignments) => {
                            println!("  Assignments ({}):", assignments.len());
                            for assignment in assignments {
                                println!("    - {}", assignment.name.unwrap_or_else(|| "Unnamed".to_string()));
                            }
                        },
                        Err(err) => {
                            error!("Failed to fetch assignments: {}", err);
                        }
                    }
                }
            },
            Err(err) => {
                error!("Failed to fetch course with ID {}: {}", id, err);
                process::exit(1);
            }
        }
    } else if args.list_topics {
        match api_client.get_topics().await {
            Ok(topics) => {
                println!("Found {} topics:", topics.len());
                for topic in topics {
                    println!(" - [{}] {}", topic.id, topic.title.unwrap_or_else(|| "No title".to_string()));
                }
            },
            Err(err) => {
                error!("Failed to fetch topics: {}", err);
                process::exit(1);
            }
        }
    } else {
        println!("No command specified. Run with --help for usage information.");
    }

    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize database
    let database = Arc::new(Database::new().await?);
    
    // Create application state
    let state = Arc::new(AppState {
        db: database,
        // Initialize other state
    });
    
    // Create router with authentication routes
    let app = Router::new()
        .merge(auth_routes(state.clone()))
        .merge(mapping_routes(state.clone()))
        .merge(discussion_routes(state.clone()))
        // Add other routes
        .with_state(state);
    
    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub db: user_repository::UserRepository,
    pub course_repo: course_repository::CourseRepository,
    pub category_repo: category_repository::CategoryRepository,
    pub topic_repo: topic_repository::TopicRepository,
    pub post_repo: post_repository::PostRepository,
    pub assignment_repo: assignment_repository::AssignmentRepository,
}

async fn setup_routes(app_state: Arc<AppState>) -> Router {
    // Auth routes
    let auth_routes = Router::new()
        .route("/login", post(controllers::auth_controller::login))
        .route("/register", post(controllers::auth_controller::register));
        
    // Protected routes with auth middleware
    let protected_routes = Router::new()
        // Add your protected routes here
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::middleware::auth_middleware,
        ));
        
    // Course routes
    let course_routes = Router::new()
        .route("/", post(controllers::course_controller::create_course)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/", get(controllers::course_controller::list_courses))
        .route("/:id", get(controllers::course_controller::get_course))
        .route("/:id/assignments", get(controllers::assignment_controller::get_assignments_by_course));
        
    // Topic routes
    let topic_routes = Router::new()
        .route("/", post(controllers::topic_controller::create_topic)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        )
        .route("/", get(controllers::topic_controller::get_topics))
        .route("/:topic_id", get(controllers::topic_controller::get_topic))
        .route("/:topic_id", patch(controllers::topic_controller::update_topic)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        )
        .route("/:topic_id", delete(controllers::topic_controller::delete_topic)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        )
        .route("/:id/posts", get(controllers::topic_controller::get_topic_with_posts));
        
    // Post routes
    let post_routes = Router::new()
        .route("/:topic_id/posts", post(controllers::post_controller::create_post)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        )
        .route("/:topic_id/posts", get(controllers::post_controller::get_posts_by_topic))
        .route("/:topic_id/posts/:post_id", patch(controllers::post_controller::update_post)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        )
        .route("/:topic_id/posts/:post_id", delete(controllers::post_controller::delete_post)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        )
        .route("/posts/:post_id/replies", get(controllers::post_controller::get_replies));
    
    // Canvas integration routes
    let canvas_routes = Router::new()
        .route("/webhook", post(controllers::canvas_integration_controller::canvas_webhook))
        .route("/discussions/import", post(controllers::canvas_integration_controller::import_canvas_discussion)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/replies/import", post(controllers::canvas_integration_controller::import_canvas_reply)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        );
        
    // Assignment routes
    let assignment_routes = Router::new()
        .route("/", post(controllers::assignment_controller::create_assignment)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/:id", get(controllers::assignment_controller::get_assignment))
        .route("/:id/topic", get(controllers::assignment_controller::get_assignment_with_topic))
        .route("/:id/topic", post(controllers::assignment_controller::create_topic_from_assignment)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::auth_middleware,
            ))
        )
        .route("/:id/topic/map", post(controllers::assignment_controller::map_topic_to_assignment)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/:id/topic", delete(controllers::assignment_controller::unmap_topic_from_assignment)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/:id/topic", delete(controllers::assignment_controller::unlink_assignment_topic)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        );
        
    // Additional category routes for topics
    let category_routes = Router::new()
        .route("/", post(controllers::category_controller::create_category)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/", get(controllers::category_controller::list_categories))
        .route("/:id", get(controllers::category_controller::get_category))
        .route("/:id", patch(controllers::category_controller::update_category)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/:id", delete(controllers::category_controller::delete_category)
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth::middleware::require_role("instructor".to_string()),
            ))
        )
        .route("/:id/topics", get(controllers::topic_controller::get_topics_by_category));
        
    // Combine routes
    Router::new()
        .merge(auth_routes)
        .nest("/courses", course_routes)
        .nest("/categories", category_routes)
        .nest("/topics", topic_routes)
        .nest("/topics", post_routes) // Sharing the same base path as topic_routes
        .nest("/canvas", canvas_routes)
        .nest("/assignments", assignment_routes)
        .merge(protected_routes)
        .nest("/courses/:id/assignments", Router::new()
            .route("/", get(controllers::assignment_controller::list_course_assignments))
            .route("/discussions", get(controllers::assignment_controller::list_course_discussion_assignments))
        )
}
