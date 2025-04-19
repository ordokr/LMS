# Ordo Quiz Module

This document provides comprehensive information about the Ordo Quiz module, its architecture, implementation details, and usage instructions.

## Overview

The Ordo Quiz module is an integrated component of the Ordo LMS/forum that provides quiz functionality. It can be run both as part of the main application and as a standalone module, with synchronization capabilities between the two modes.

### Key Features

- Quiz creation and management
- Multiple question types (multiple choice, true/false, short answer, matching, essay)
- Quiz attempts tracking
- Score calculation and reporting
- CMI5 integration for learning record tracking
- Rich content support (text, images, media)
- Tagging and categorization
- Offline-first with sync capabilities
- Comprehensive activity tracking and analytics
- Learning analytics dashboard

## Architecture

The Ordo Quiz module follows a layered architecture pattern:

1. **Database Layer**: SQLite database with tables for quizzes, questions, answers, attempts, and CMI5 sessions
2. **Repository Layer**: Handles data access and persistence
3. **Service Layer**: Implements business logic
4. **API Layer**: Exposes functionality through Tauri commands
5. **UI Layer**: Provides user interface components

### Database Schema

The module uses the following database tables:

- `quizzes`: Stores quiz metadata
- `questions`: Stores quiz questions
- `answer_options`: Stores answer options for questions
- `quiz_settings`: Stores quiz settings
- `quiz_attempts`: Tracks user attempts
- `user_answers`: Stores user answers for each attempt
- `cmi5_sessions`: Tracks CMI5 sessions

## Implementation Details

### Models

The module defines the following key models:

- `Quiz`: Represents a quiz with metadata
- `Question`: Represents a quiz question
- `AnswerOption`: Represents an answer option for a question
- `QuizAttempt`: Represents a user's attempt at a quiz
- `UserAnswer`: Represents a user's answer to a question
- `QuizSettings`: Represents settings for a quiz
- `Cmi5Session`: Represents a CMI5 session

### Repository

The `QuizRepository` class provides methods for:

- Creating, reading, updating, and deleting quizzes
- Managing questions and answer options
- Starting, completing, and abandoning quiz attempts
- Tracking CMI5 sessions

### Integration with Main App

The Ordo Quiz module integrates with the main app through:

1. **Shared Database**: Uses the same SQLite database
2. **AppState Integration**: Accessible through the app's state management
3. **Common Models**: Uses compatible data models
4. **Sync Mechanism**: Synchronizes data between standalone and main app modes

## Activity Tracking

The Ordo Quiz module includes comprehensive activity tracking to monitor student engagement and progress:

### Tracked Activities

- **Quiz Activities**: Starting, completing, and abandoning quizzes
- **Question Activities**: Answering questions, time spent per question
- **Flashcard Activities**: Viewing, flipping, and rating flashcards
- **Study Sessions**: Start and end times, total duration
- **Content Viewing**: Time spent on specific content

### Analytics

The activity tracking system provides several analytics features:

1. **User Activity Summaries**: Overview of a user's activity across all quizzes
2. **Quiz Activity Summaries**: Detailed breakdown of activity for a specific quiz
3. **Activity Statistics**: Aggregated statistics like total study time, average quiz duration, etc.
4. **Time-based Analysis**: Activity patterns by day, week, or month
5. **Performance Metrics**: Correlation between activity and quiz performance

### Integration with LMS

All activity data is synchronized with the main LMS, allowing instructors to:

- Monitor student engagement
- Identify struggling students
- Optimize quiz content based on usage patterns
- Generate detailed reports on student activity

### Analytics Dashboard

The Ordo Quiz module includes a comprehensive analytics dashboard for instructors to visualize activity data:

#### Dashboard Features

1. **Overview Dashboard**: Provides a high-level summary of quiz activity, including total quizzes started, completed, questions answered, and total study time.

2. **Quiz Performance Analysis**: Detailed breakdown of quiz performance, including completion rates, average scores, and time spent per quiz.

3. **User Activity Tracking**: Visualizes user engagement patterns, including activity frequency, time spent, and performance metrics.

4. **Time-based Analysis**: Shows activity patterns by day, hour, and day of week to identify peak usage times.

5. **Question Analysis**: Analyzes question difficulty, discrimination index, and answer distribution to improve quiz content.

6. **Report Generation**: Creates detailed PDF reports for users, quizzes, and questions that can be exported and shared.

#### Visualization Components

The dashboard uses various visualization components to present data effectively:

- **Line Charts**: For time-series data like activity over time
- **Bar Charts**: For comparative data like quiz completion rates
- **Pie/Doughnut Charts**: For distribution data like activity types
- **Heat Maps**: For time-based patterns like activity by hour and day
- **Data Tables**: For detailed activity records and metrics

## Sync Functionality

The Ordo Quiz module includes a robust sync mechanism that allows it to work offline and synchronize data with the main app:

### How Sync Works

1. **Queue-Based System**: Changes are queued for sync when offline
2. **Priority Levels**: Sync items have different priority levels (Critical, High, Medium, Low)
3. **Conflict Resolution**: Smart conflict resolution based on timestamps and priorities
4. **Bidirectional Sync**: Data flows both ways between standalone and main app

### What Gets Synced

- Quiz attempts and progress
- User answers and scores
- Study activity and statistics
- User preferences and settings

### Sync Process

1. When the standalone app starts, it checks for a sync file from the main app
2. Changes made in the standalone app are tracked and queued for sync
3. When the app shuts down, pending changes are exported to a sync file
4. The main app can import this file to update its database
5. Similarly, the main app can export changes for the standalone app to import

## Running the Module

### As Part of the Main App

The Ordo Quiz module is automatically loaded when the main app starts. It can be accessed through the main navigation.

### As a Standalone Module

To run the Ordo Quiz module as a standalone application:

1. Navigate to the project directory
2. Run the following command:

```bash
cargo run --bin quiz-standalone
```

This will:
- Initialize the database
- Create test data if needed
- Start the standalone application

## Development

### Adding New Question Types

To add a new question type:

1. Add the new type to the `QuestionType` enum in `src-tauri/src/models/quiz/question.rs`
2. Update the database schema to support the new type
3. Implement the necessary UI components
4. Update the scoring logic in the repository

### Extending CMI5 Integration

The module currently supports basic CMI5 integration. To extend it:

1. Update the `Cmi5Session` model in `src-tauri/src/models/quiz/cmi5.rs`
2. Implement additional methods in the repository
3. Create new API endpoints for the extended functionality

## Testing

To run tests for the Ordo Quiz module:

```bash
cargo test --package lms-lib --lib -- database::repositories::quiz_repository
```

## Future Enhancements

Planned enhancements for the Ordo Quiz module include:

1. **Mobile Optimization**: Improving the mobile experience
2. **Offline Sync**: Enhanced offline functionality with sync
3. **Additional Question Types**: Support for more complex question types
4. **LTI Integration**: Support for Learning Tools Interoperability
5. **Enhanced Analytics**: More detailed analytics and reporting
6. **Charming Integration**: Using Charming for visualization components

## Troubleshooting

### Database Issues

If you encounter database-related errors:

1. Check that the database file exists
2. Verify that the schema is up-to-date
3. Run the database initialization script

### Compilation Errors

If you encounter compilation errors:

1. Make sure all dependencies are installed
2. Check that the Rust toolchain is up-to-date
3. Run `cargo clean` and try again

## References

- [CMI5 Specification](https://github.com/AICC/CMI-5_Spec_Current)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [Tauri Documentation](https://tauri.app/v1/guides/)
