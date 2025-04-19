# Quiz Module for Ordo LMS

This module provides a comprehensive quiz system for the Ordo LMS application, ported from Quenti's quiz functionality. It supports various question types, study modes, and customization options.

## Features

### Quiz Types and Formats

- **Multiple Choice Questions**: Single or multiple correct answers
- **True/False Questions**: Simple binary choice questions
- **Short Answer Questions**: Free-text responses with pattern matching
- **Essay Questions**: Long-form text responses
- **Matching Questions**: Pair related items
- **Ordering Questions**: Arrange items in the correct sequence
- **Drawing/Sketch Questions**: Create drawings or annotate images
- **Code Execution Questions**: Write and test code with real-time feedback
- **Math Equation Questions**: Create and validate mathematical expressions
- **Timeline Questions**: Arrange events in chronological order
- **Diagram Labeling Questions**: Label parts of diagrams or images

### Study Modes

- **Flashcards**: Spaced repetition learning with confidence ratings
- **Multiple Choice**: Traditional quiz format with immediate feedback
- **Written**: Text-based responses with manual or automatic grading
- **Mixed**: Combination of different question types

### User Experience

- **Responsive Design**: Works on all device sizes
- **Offline Support**: Full functionality without internet connection with enhanced sync capabilities:
  - Prioritized sync queue for critical data
  - Intelligent conflict resolution
  - Background sync with notifications
  - Sync status indicators
- **Progress Tracking**: Save and resume quiz sessions
- **Immediate Feedback**: Optional explanations for correct answers
- **Spaced Repetition**: SM-2 algorithm for optimal learning
- **Customizable Themes**: Match your app's design language

### Enhanced Analytics

- **Learning Insights Dashboard**: Personalized learning analytics
- **Performance Visualization**: Interactive charts and graphs using Charming
- **Comparative Analytics**: Compare performance across quizzes and users
- **Predictive Learning Patterns**: Identify optimal study patterns
- **Skill Mastery Tracking**: Monitor progress in specific knowledge areas
- **Exportable Reports**: Generate and share detailed reports

## Architecture

The quiz module is built using a hybrid architecture:

### Backend (Rust/Tauri)

- **Models**: Data structures for quizzes, questions, answers
- **Storage**: Hybrid SQLite + Redb for performance and reliability
- **Commands**: Tauri commands for frontend-backend communication
- **Session Management**: Track user progress and scoring
- **Sync Engine**: Offline-first operation with conflict resolution

### Frontend (Leptos)

- **Components**: Reusable UI components for quiz interaction
- **Pages**: Full-page views for quiz listing and taking
- **Theme System**: Customizable styling with Quenti's native theme
- **Animations**: Smooth transitions and feedback

## Usage

### Creating a Quiz

```rust
use crate::quiz::models::{Quiz, Question, QuestionContent, AnswerType};

// Create a new quiz
let mut quiz = Quiz::new("My Quiz".to_string(), Some(user_id));
quiz.description = Some("A sample quiz".to_string());

// Add a multiple choice question
let content = QuestionContent {
    text: "What is the capital of France?".to_string(),
    rich_text: None,
    image_url: None,
    audio_url: None,
};

let mut question = Question::new(quiz.id, content, AnswerType::MultipleChoice);

// Add choices
let paris_id = question.add_choice("Paris".to_string());
let london_id = question.add_choice("London".to_string());
let berlin_id = question.add_choice("Berlin".to_string());

// Set correct answer
question.set_correct_answer(Answer::Choice(paris_id));

// Add explanation
question.explanation = Some("Paris is the capital and most populous city of France.".to_string());

// Add question to quiz
quiz.add_question(question);

// Save quiz
quiz_engine.create_quiz(quiz).await?;
```

### Taking a Quiz

```rust
// Start a quiz session
let session = quiz_engine.start_session(quiz_id, user_id).await?;

// Submit an answer
let is_correct = quiz_engine.submit_answer(
    session.id,
    question_id,
    Answer::Choice(choice_id)
).await?;

// Complete the session
let score = quiz_engine.complete_session(session.id).await?;
```

## Customization

### Theme Customization

The quiz module's theme can be customized to match your app's design language. See the [Theme Customization Guide](quiz-theme-customization.md) for details.

### Font Customization

You can customize the fonts used in the quiz module:

```css
:root {
  --font-heading: 'Your Heading Font', sans-serif;
  --font-body: 'Your Body Font', sans-serif;
}
```

### Layout Customization

The quiz module's layout can be customized by overriding the CSS variables:

```css
:root {
  --quiz-spacing: 1.25rem; /* Increase spacing */
  --quiz-radius: 0.75rem; /* Rounder corners */
}
```

## Integration with Ordo LMS

The quiz module is designed to integrate seamlessly with the Ordo LMS application:

1. **Course Integration**: Quizzes can be associated with courses
2. **User Authentication**: Uses Ordo's authentication system
3. **Theme Inheritance**: Adopts Ordo's theme settings
4. **Offline Sync**: Works with Ordo's sync engine

## External System Integration

The quiz module can integrate with various external learning systems:

### LTI Integration

Supports Learning Tools Interoperability (LTI) standards for integration with learning management systems:

- LTI 1.0, 1.1, and 1.3 support
- Secure authentication and authorization
- Grade passback to LMS gradebooks
- Deep linking for seamless navigation

### cmi5 Integration

Supports cmi5 (an xAPI Profile) as the primary standard for e-learning integration:

- Modern xAPI-based tracking and reporting
- Content location independence (content can be hosted anywhere)
- Support for mobile and distributed learning scenarios
- Improved session management and state tracking
- Detailed learning experience tracking

### SCORM Compliance (Backup)

Maintains compatibility with Sharable Content Object Reference Model (SCORM) as a backup standard:

- SCORM 1.2 and 2004 (3rd and 4th editions) support
- Package import and export
- Runtime communication via SCORM API
- Session tracking and reporting

### xAPI Integration

Supports Experience API (xAPI) for tracking learning experiences:

- Statement generation for learning events
- Integration with Learning Record Stores (LRS)
- Detailed activity tracking
- Learning analytics support

### LMS Connectors

Direct integration with popular learning management systems:

- Canvas
- Moodle
- Blackboard
- Other LMS platforms via API

## Dependencies

- **Leptos**: Rust framework for building web applications
- **Tauri**: Desktop application framework
- **SQLite**: Local database for offline storage
- **Serde**: Serialization/deserialization library
- **Web APIs**: For browser-specific functionality
- **Charming**: Visualization library for analytics (preferred over Chart.js for better Rust/WASM integration)
- **OAuth**: For LTI and external system authentication
- **Anyhow**: Error handling
- **Chrono**: Date and time handling
- **xAPI**: For cmi5 and Experience API integration
- **Zip**: For cmi5/SCORM package handling
- **Base64**: For authentication token encoding
- **Uuid**: For generating unique identifiers

## Performance Considerations

- **Lazy Loading**: Question content is loaded on demand
- **Virtualized Lists**: Efficient rendering of large quiz lists
- **Hybrid Storage**: Fast in-memory access with persistent backup
- **Optimized Assets**: Images and audio are compressed and cached

## Accessibility

The quiz module is designed to be accessible to all users:

- **Keyboard Navigation**: Full keyboard support
- **Screen Reader Support**: Proper ARIA attributes
- **High Contrast Mode**: Compatible with high contrast settings
- **Reduced Motion**: Respects user motion preferences
- **Font Scaling**: Supports browser font size adjustments

## License

This module is part of the Ordo LMS application and is subject to the same license terms.
