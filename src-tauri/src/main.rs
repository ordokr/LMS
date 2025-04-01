// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

mod database;

#[tokio::main]
async fn main() {
    use database::establish_connection;

    establish_connection().expect("Failed to establish database connection");

    // Define routes
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/courses", get(get_courses).post(create_course))
        .route("/forum_threads", get(get_forum_threads).post(create_forum_thread))
        .route("/forum_posts", get(get_forum_posts).post(create_forum_post))
        .route("/assignments", get(get_assignments).post(create_assignment))
        .route("/submissions", get(get_submissions).post(create_submission))
        .route("/grades", get(get_grade).post(create_grade))
        .route("/course_progress", get(get_course_progress).post(create_course_progress))
        .route("/student_performance", get(get_student_performance).post(create_student_performance));

    // Define address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    // Run server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    //lms_lib::run()
}

use lms_lib::{
    Course, ForumPost, ForumThread, Assignment, Submission, Grade, CourseProgress, StudentPerformance,
    create_course as tauri_create_course, get_courses as tauri_get_courses,
    create_forum_thread as tauri_create_forum_thread, get_forum_threads as tauri_get_forum_threads,
    create_forum_post as tauri_create_forum_post, get_forum_posts as tauri_get_forum_posts,
    create_assignment as tauri_create_assignment, get_assignments as tauri_get_assignments,
    create_submission as tauri_create_submission, get_submissions as tauri_get_submissions,
    create_grade as tauri_create_grade, get_grade as tauri_get_grade,
    create_course_progress as tauri_create_course_progress, get_course_progress as tauri_get_course_progress,
    create_student_performance as tauri_create_student_performance, get_student_performance as tauri_get_student_performance,
};
use axum::extract::Query;
use serde::Deserialize;
use axum::Json;

use lms_lib::{
    Course, ForumPost, ForumThread, Assignment, Submission, Grade, CourseProgress, StudentPerformance,
    create_course as tauri_create_course, get_courses as tauri_get_courses,
    create_forum_thread as tauri_create_forum_thread, get_forum_threads as tauri_get_forum_threads,
    create_forum_post as tauri_create_forum_post, get_forum_posts as tauri_get_forum_posts,
    create_assignment as tauri_create_assignment, get_assignments as tauri_get_assignments,
    create_submission as tauri_create_submission, get_submissions as tauri_get_submissions,
    create_grade as tauri_create_grade, get_grade as tauri_get_grade,
    create_course_progress as tauri_create_course_progress, get_course_progress as tauri_get_course_progress,
    create_student_performance as tauri_create_student_performance, get_student_performance as tauri_get_student_performance,
};
use axum::extract::Query;
use serde::Deserialize;
use axum::Json;
use tauri::{generate_handler, Builder, Context};

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// Define query parameters for get requests
#[derive(Deserialize)]
struct CourseIdParams {
    course_id: i32,
}

#[derive(Deserialize)]
struct ThreadIdParams {
    thread_id: i32,
}

#[derive(Deserialize)]
struct SubmissionIdParams {
    submission_id: i32,
}

#[derive(Deserialize)]
struct StudentIdParams {
    student_id: i32,
}

// Define handler functions for routes
async fn create_course(Json(payload): Json<Course>) -> String {
    tauri_create_course(payload.name, payload.description).await.unwrap()
}

async fn get_courses() -> Json<Vec<Course>> {
    Json(tauri_get_courses().await.unwrap())
}

async fn create_forum_thread(Json(payload): Json<ForumThread>) -> String {
    tauri_create_forum_thread(payload.title, payload.category).await.unwrap()
}

async fn get_forum_threads() -> Json<Vec<ForumThread>> {
    Json(tauri_get_forum_threads().await.unwrap())
}

async fn create_forum_post(Json(payload): Json<ForumPost>) -> String {
    tauri_create_forum_post(payload.thread_id, payload.author_id, payload.content).await.unwrap()
}

async fn get_forum_posts(Query(params): Query<ThreadIdParams>) -> Json<Vec<ForumPost>> {
    Json(tauri_get_forum_posts(params.thread_id).await.unwrap())
}

async fn create_assignment(Json(payload): Json<Assignment>) -> String {
    tauri_create_assignment(payload.course_id, payload.title, payload.description, payload.due_date).await.unwrap()
}

async fn get_assignments(Query(params): Query<CourseIdParams>) -> Json<Vec<Assignment>> {
    Json(tauri_get_assignments(params.course_id).await.unwrap())
}

async fn create_submission(Json(payload): Json<Submission>) -> String {
    tauri_create_submission(payload.assignment_id, payload.student_id, payload.content).await.unwrap()
}

async fn get_submissions(Query(params): Query<CourseIdParams>) -> Json<Vec<Submission>> {
    Json(tauri_get_submissions(params.course_id).await.unwrap())
}

async fn create_grade(Json(payload): Json<Grade>) -> String {
    tauri_create_grade(payload.submission_id, payload.grader_id, payload.grade, payload.feedback).await.unwrap()
}

async fn get_grade(Query(params): Query<SubmissionIdParams>) -> Json<Option<Grade>> {
    Json(tauri_get_grade(params.submission_id).await.unwrap())
}

async fn create_course_progress(Json(payload): Json<CourseProgress>) -> String {
    tauri_create_course_progress(payload.course_id, payload.student_id, payload.completed_modules, payload.total_modules).await.unwrap()
}

async fn get_course_progress(Query(params): Query<CourseIdParams>, Query(student_params): Query<StudentIdParams>) -> Json<Option<CourseProgress>> {
    Json(tauri_get_course_progress(params.course_id, student_params.student_id).await.unwrap())
}

async fn create_student_performance(Json(payload): Json<StudentPerformance>) -> String {
    tauri_create_student_performance(payload.student_id, payload.course_id, payload.average_grade, payload.time_spent).await.unwrap()
}

async fn get_student_performance(Query(params): Query<CourseIdParams>, Query(student_params): Query<StudentIdParams>) -> Json<Option<StudentPerformance>> {
    Json(tauri_get_student_performance(params.course_id, student_params.student_id).await.unwrap())
}

// Define query parameters for get requests
#[derive(Deserialize)]
struct CourseIdParams {
    course_id: i32,
}

#[derive(Deserialize)]
struct ThreadIdParams {
    thread_id: i32,
}

#[derive(Deserialize)]
struct SubmissionIdParams {
    submission_id: i32,
}

#[derive(Deserialize)]
struct StudentIdParams {
    student_id: i32,
}

// Define handler functions for routes
async fn create_course(Json(payload): Json<Course>) -> String {
    tauri_create_course(payload.name, payload.description).await.unwrap()
}

async fn get_courses() -> Json<Vec<Course>> {
    Json(tauri_get_courses().await.unwrap())
}

async fn create_forum_thread(Json(payload): Json<ForumThread>) -> String {
    tauri_create_forum_thread(payload.title, payload.category).await.unwrap()
}

async fn get_forum_threads() -> Json<Vec<ForumThread>> {
    Json(tauri_get_forum_threads().await.unwrap())
}

async fn create_forum_post(Json(payload): Json<ForumPost>) -> String {
    tauri_create_forum_post(payload.thread_id, payload.author_id, payload.content).await.unwrap()
}

async fn get_forum_posts(Query(params): Query<ThreadIdParams>) -> Json<Vec<ForumPost>> {
    Json(tauri_get_forum_posts(params.thread_id).await.unwrap())
}

async fn create_assignment(Json(payload): Json<Assignment>) -> String {
    tauri_create_assignment(payload.course_id, payload.title, payload.description, payload.due_date).await.unwrap()
}

async fn get_assignments(Query(params): Query<CourseIdParams>) -> Json<Vec<Assignment>> {
    Json(tauri_get_assignments(params.course_id).await.unwrap())
}

async fn create_submission(Json(payload): Json<Submission>) -> String {
    tauri_create_submission(payload.assignment_id, payload.student_id, payload.content).await.unwrap()
}

async fn get_submissions(Query(params): Query<CourseIdParams>) -> Json<Vec<Submission>> {
    Json(tauri_get_submissions(params.course_id).await.unwrap())
}

async fn create_grade(Json(payload): Json<Grade>) -> String {
    tauri_create_grade(payload.submission_id, payload.grader_id, payload.grade, payload.feedback).await.unwrap()
}

async fn get_grade(Query(params): Query<SubmissionIdParams>) -> Json<Option<Grade>> {
    Json(tauri_get_grade(params.submission_id).await.unwrap())
}

async fn create_course_progress(Json(payload): Json<CourseProgress>) -> String {
    tauri_create_course_progress(payload.course_id, payload.student_id, payload.completed_modules, payload.total_modules).await.unwrap()
}

async fn get_course_progress(Query(params): Query<CourseIdParams>, Query(student_params): Query<StudentIdParams>) -> Json<Option<CourseProgress>> {
    Json(tauri_get_course_progress(params.course_id, student_params.student_id).await.unwrap())
}

async fn create_student_performance(Json(payload): Json<StudentPerformance>) -> String {
    tauri_create_student_performance(payload.student_id, payload.course_id, payload.average_grade, payload.time_spent).await.unwrap()
}

async fn get_student_performance(Query(params): Query<CourseIdParams>, Query(student_params): Query<StudentIdParams>) -> Json<Option<StudentPerformance>> {
    Json(tauri_get_student_performance(params.course_id, student_params.student_id).await.unwrap())
}
