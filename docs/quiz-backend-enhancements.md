# Quiz Module Backend Enhancements

This document describes the backend enhancements implemented for the Quiz Module, including the advanced spaced repetition algorithm, analytics and reporting functionality, and export/import features.

## 1. Advanced Spaced Repetition Algorithm

The Quiz Module now includes an enhanced implementation of the SuperMemo SM-2 algorithm for spaced repetition learning, with several improvements for better learning outcomes.

### Key Features

- **Enhanced SM-2 Algorithm**: Based on the proven SuperMemo SM-2 algorithm with additional optimizations
- **Partial Forgetting**: Instead of completely resetting progress when a card is forgotten, the algorithm retains some progress
- **Adaptive Intervals**: Intervals are adjusted based on the user's performance history
- **Jittered Scheduling**: Small random variations in scheduling to prevent cards from clumping together
- **Confidence-Based Ratings**: 5-point rating scale for more nuanced feedback:
  - Rating 1: Complete blackout (wrong response)
  - Rating 2: Incorrect response, but answer feels familiar
  - Rating 3: Incorrect response, but correct answer was easy to recall once shown
  - Rating 4: Correct response after hesitation
  - Rating 5: Correct response with perfect recall

### Usage

```rust
// Rate a flashcard
let rating = FlashcardRating::Hesitation; // Rating 4
let updated_data = quiz_engine.rate_flashcard(question_id, user_id, rating as i32).await?;

// Create a flashcard study session with due cards
let (session, questions) = quiz_engine.create_flashcard_session(user_id, 20).await?;

// Get flashcard statistics
let stats = quiz_engine.get_flashcard_stats(user_id).await?;
```

### Technical Implementation

The algorithm uses the following parameters:

- **Ease Factor**: Starts at 2.5 and is adjusted based on performance
- **Interval**: Number of days until the next review
- **Repetitions**: Number of successful reviews

When a card is reviewed:

1. The user provides a rating (1-5)
2. If the rating is ≥ 3 (correct response):
   - Interval is increased based on the ease factor
   - Repetition counter is incremented
3. If the rating is < 3 (incorrect response):
   - Repetition counter is reduced but not reset to zero
   - Interval is reduced based on the rating
4. The ease factor is adjusted based on the rating
5. A small random jitter is added to the interval
6. The next review date is calculated

## 2. Analytics and Reporting

The Quiz Module now includes comprehensive analytics and reporting functionality to track user progress and quiz performance.

### Key Features

- **User Study Statistics**: Track a user's overall study progress
- **Quiz Performance Analytics**: Analyze how users perform on specific quizzes
- **Question Difficulty Analysis**: Identify which questions are most challenging
- **Time Period Filtering**: View statistics for different time periods (day, week, month, year, all time)
- **PDF Report Generation**: Generate detailed reports for users and quizzes

### Available Statistics

#### User Statistics

- Total quizzes taken
- Total questions answered
- Correct answers and accuracy rate
- Total study time
- Average session time
- Study streak (consecutive days)
- Flashcard statistics (if using spaced repetition)

#### Quiz Analytics

- Total attempts
- Unique users
- Average score
- Completion rate
- Average time spent
- Question difficulties
- Most common mistakes

### Usage

```rust
// Get user statistics
let period = TimePeriod::Month;
let user_stats = quiz_engine.get_user_stats(user_id, period).await?;

// Get quiz analytics
let quiz_analytics = quiz_engine.get_quiz_analytics(quiz_id, period).await?;

// Generate reports
let user_report = quiz_engine.generate_user_report(user_id, period).await?;
let quiz_report = quiz_engine.generate_quiz_report(quiz_id, period).await?;
```

## 3. Export/Import Functionality

The Quiz Module now supports exporting and importing quizzes in various formats, making it easy to share quizzes and integrate with other learning platforms.

### Supported Formats

- **JSON**: Full quiz data with all metadata (default)
- **CSV**: Simple question-answer pairs
- **Markdown**: Human-readable format with formatting
- **Anki**: Compatible with Anki flashcard app
- **Quizlet**: Compatible with Quizlet

### Export Options

- **Include Explanations**: Whether to include explanations in the export
- **Include Metadata**: Whether to include creation dates, author, etc.
- **Include Statistics**: Whether to include usage statistics
- **Include Images**: Whether to include image URLs
- **Include Audio**: Whether to include audio URLs

### Usage

```rust
// Export a quiz to a file
let format = ExportFormat::Json;
let path = Path::new("quiz_export.json");
quiz_engine.export_quiz_to_file(quiz_id, path, format).await?;

// Export a quiz to a byte array
let quiz_data = quiz_engine.export_quiz(quiz_id, format).await?;

// Export with custom options
let options = ExportOptions {
    format: ExportFormat::Markdown,
    include_explanations: true,
    include_metadata: false,
    include_statistics: false,
    include_images: true,
    include_audio: false,
};
let quiz_data = quiz_engine.export_quiz_with_options(quiz_id, options).await?;

// Import a quiz from a file
let imported_quiz_id = quiz_engine.import_quiz_from_file(Path::new("quiz_import.json")).await?;

// Import a quiz from a byte array
let imported_quiz_id = quiz_engine.import_quiz(&data, ExportFormat::Json).await?;
```

### File Format Details

#### JSON Format

```json
{
  "quiz": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Example Quiz",
    "description": "An example quiz",
    "questions": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440001",
        "content": {
          "text": "What is the capital of France?"
        },
        "answer_type": "MultipleChoice",
        "choices": [
          {
            "id": "550e8400-e29b-41d4-a716-446655440002",
            "text": "Paris"
          },
          {
            "id": "550e8400-e29b-41d4-a716-446655440003",
            "text": "London"
          }
        ],
        "correct_answer": {
          "Choice": "550e8400-e29b-41d4-a716-446655440002"
        },
        "explanation": "Paris is the capital of France."
      }
    ]
  },
  "format": "Json",
  "timestamp": "2023-06-15T12:34:56Z",
  "version": "1.0.0"
}
```

#### CSV Format

```
Question,Answer,Explanation
"What is the capital of France?","Paris","Paris is the capital of France."
"What is 2+2?","4","Basic addition."
```

#### Markdown Format

```markdown
# Example Quiz

An example quiz

## Questions

### Question 1

What is the capital of France?

- [x] Paris
- [ ] London
- [ ] Berlin

**Explanation**: Paris is the capital of France.

### Question 2

What is 2+2?

**Answer**: 4

**Explanation**: Basic addition.
```

## Integration with Frontend

These backend enhancements are fully integrated with the frontend through Tauri commands, making them easily accessible from the UI. The following commands are available:

### Spaced Repetition

- `rate_flashcard`: Rate a flashcard and update its spaced repetition data
- `create_flashcard_session`: Create a flashcard study session with due cards
- `get_flashcard_stats`: Get flashcard statistics for a user

### Analytics

- `get_user_stats`: Get study statistics for a user
- `get_quiz_analytics`: Get analytics for a quiz
- `generate_user_report`: Generate a PDF report for a user's study statistics
- `generate_quiz_report`: Generate a PDF report for a quiz's analytics

### Export/Import

- `export_quiz`: Export a quiz to a byte array
- `export_quiz_to_file`: Export a quiz to a file
- `export_quiz_with_options`: Export a quiz with custom options
- `import_quiz_from_file`: Import a quiz from a file
- `import_quiz`: Import a quiz from a byte array
