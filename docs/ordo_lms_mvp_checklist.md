# Ordo LMS Minimum Viable Product (MVP) Checklist

This checklist focuses on the essential components needed to create a basic working version of the Ordo LMS application with quiz functionality. These items represent the minimum requirements to have a functional application that can be built and run.

## 1. Core Database Setup

- [ ] **Basic SQLite Initialization**
  - [ ] Create database file with proper permissions
  - [ ] Set up WAL journal mode for better performance
  - [ ] Implement basic error handling for database operations
  - [ ] Create database connection pool with appropriate size

- [ ] **Essential Schema**
  - [ ] Users table with basic authentication fields
    - [ ] id (UUID)
    - [ ] username
    - [ ] email
    - [ ] password_hash
    - [ ] created_at
    - [ ] updated_at
  - [ ] Courses table with basic metadata
    - [ ] id (UUID)
    - [ ] title
    - [ ] description
    - [ ] created_by
    - [ ] created_at
    - [ ] updated_at
  - [ ] Quiz tables
    - [ ] quizzes (id, title, description, course_id, settings)
    - [ ] questions (id, quiz_id, content, type)
    - [ ] answers (id, question_id, content, is_correct)
    - [ ] attempts (id, quiz_id, user_id, started_at, completed_at, score)

- [ ] **Basic Migrations**
  - [ ] Create initial schema migration
  - [ ] Implement migration runner in application startup
  - [ ] Add basic seed data for testing

## 2. Basic Authentication

- [ ] **User Authentication**
  - [ ] Implement password hashing with Argon2
  - [ ] Create JWT token generation and validation
  - [ ] Implement basic user registration
  - [ ] Add login endpoint with token return

- [ ] **Authentication Middleware**
  - [ ] Create middleware to validate JWT tokens
  - [ ] Implement user extraction from token
  - [ ] Add route protection based on authentication
  - [ ] Create basic error responses for authentication failures

- [ ] **User Management**
  - [ ] Implement user creation
  - [ ] Add user retrieval by ID and email
  - [ ] Create basic user profile update
  - [ ] Implement password change functionality

## 3. Quiz Module Core

- [ ] **Quiz Creation**
  - [ ] Implement basic quiz model
  - [ ] Create quiz repository with CRUD operations
  - [ ] Add API endpoints for quiz management
  - [ ] Implement question and answer creation

- [ ] **Quiz Taking**
  - [ ] Create quiz session management
  - [ ] Implement answer submission and validation
  - [ ] Add time tracking for timed quizzes
  - [ ] Create quiz completion and scoring

- [ ] **Quiz Results**
  - [ ] Implement basic scoring algorithm
  - [ ] Create attempt record storage
  - [ ] Add results retrieval endpoint
  - [ ] Implement basic statistics calculation

## 4. Minimal UI

- [ ] **Authentication UI**
  - [ ] Create login form with validation
  - [ ] Implement registration form
  - [ ] Add token storage in local storage
  - [ ] Create authenticated state management

- [ ] **Course Listing**
  - [ ] Implement basic course list component
  - [ ] Create course detail view
  - [ ] Add quiz listing within course
  - [ ] Implement navigation between views

- [ ] **Quiz Interface**
  - [ ] Create quiz information display
  - [ ] Implement question rendering for different types
  - [ ] Add answer selection interface
  - [ ] Create navigation between questions
  - [ ] Implement quiz submission

- [ ] **Results Display**
  - [ ] Create score summary view
  - [ ] Implement question review with correct answers
  - [ ] Add basic performance metrics
  - [ ] Create return to course navigation

## 5. Standalone Quiz Functionality

- [ ] **Basic Standalone Launch**
  - [ ] Create standalone binary entry point
  - [ ] Implement command-line arguments for quiz ID
  - [ ] Add configuration loading
  - [ ] Create window initialization

- [ ] **Quiz Data Loading**
  - [ ] Implement quiz retrieval in standalone mode
  - [ ] Create question and answer loading
  - [ ] Add quiz settings application
  - [ ] Implement offline data storage

- [ ] **Quiz Session**
  - [ ] Create standalone quiz session
  - [ ] Implement answer tracking
  - [ ] Add timer functionality
  - [ ] Create session persistence for interruptions

- [ ] **Results Saving**
  - [ ] Implement local results storage
  - [ ] Create results synchronization with main app
  - [ ] Add offline results handling
  - [ ] Implement basic analytics collection

## 6. Build Configuration

- [ ] **Tauri Setup**
  - [ ] Configure main application window
  - [ ] Create standalone quiz window configuration
  - [ ] Set up application permissions
  - [ ] Add basic menu structure

- [ ] **Build Scripts**
  - [ ] Create development build script
  - [ ] Implement production build configuration
  - [ ] Add standalone build option
  - [ ] Create basic installer configuration

- [ ] **Asset Bundling**
  - [ ] Add application icons
  - [ ] Create minimal branding assets
  - [ ] Implement static file bundling
  - [ ] Add license and documentation files

## 7. Basic API Server

- [ ] **Axum Setup**
  - [ ] Create main application router
  - [ ] Implement core middleware (auth, logging)
  - [ ] Add health check endpoint
  - [ ] Create error handling middleware

- [ ] **Core Endpoints**
  - [ ] Implement user endpoints (register, login, profile)
  - [ ] Create course endpoints (list, detail)
  - [ ] Add quiz endpoints (list, detail, attempt)
  - [ ] Implement results endpoints (submit, retrieve)

- [ ] **API Documentation**
  - [ ] Create basic API documentation
  - [ ] Add endpoint descriptions
  - [ ] Implement example requests and responses
  - [ ] Create authentication documentation

## 8. Testing Essentials

- [ ] **Basic Unit Tests**
  - [ ] Implement repository tests
  - [ ] Create service logic tests
  - [ ] Add API endpoint tests
  - [ ] Implement authentication tests

- [ ] **Integration Tests**
  - [ ] Create database integration tests
  - [ ] Implement API flow tests
  - [ ] Add authentication flow tests
  - [ ] Create quiz taking flow tests

- [ ] **Manual Testing Plan**
  - [ ] Create test cases for core functionality
  - [ ] Implement test data generation
  - [ ] Add test reporting template
  - [ ] Create bug tracking process
