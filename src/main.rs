mod app;
mod components;
mod lms;
mod forum;
mod utils;
mod config;
mod models;
mod services;

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
async fn main() {
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
}
