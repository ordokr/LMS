# Ordo Quiz - Quick Start Guide

This guide provides a quick introduction to using the Ordo Quiz module.

## Getting Started

### Running the Standalone Module

To run the Ordo Quiz module as a standalone application:

1. Open a terminal in the project directory
2. Run the following command:

```bash
cargo run --bin quiz-standalone
```

3. The application will start and display a message indicating it's running
4. Press Ctrl+C to exit when you're done

### Accessing the Module in the Main App

When the Ordo Quiz module is integrated with the main app:

1. Launch the main Ordo application
2. Navigate to the "Quizzes" section in the main menu
3. You'll see a list of available quizzes

## Creating a Quiz

1. Click the "Create Quiz" button
2. Enter a title for your quiz
3. (Optional) Add a description
4. (Optional) Set a time limit
5. (Optional) Set a passing score
6. Click "Save" to create the quiz

## Adding Questions

1. Open the quiz you want to add questions to
2. Click the "Add Question" button
3. Select the question type:
   - Multiple Choice
   - True/False
   - Short Answer
   - Matching
   - Essay
4. Enter the question text
5. Add answer options (for multiple choice and matching questions)
6. Mark the correct answer(s)
7. Click "Save" to add the question

## Taking a Quiz

1. Open the quiz you want to take
2. Click the "Start Quiz" button
3. Answer each question
4. Click "Next" to move to the next question
5. On the last question, click "Finish" to complete the quiz
6. View your results

## Viewing Quiz Results

1. Open the quiz you want to view results for
2. Click the "Results" tab
3. You'll see a list of your attempts
4. Click on an attempt to view detailed results

## Managing Quizzes

### Editing a Quiz

1. Open the quiz you want to edit
2. Click the "Edit" button
3. Make your changes
4. Click "Save" to update the quiz

### Deleting a Quiz

1. Open the quiz you want to delete
2. Click the "Delete" button
3. Confirm the deletion

## CMI5 Integration

The Ordo Quiz module supports CMI5 integration for tracking learning activities:

1. Create a quiz as usual
2. Enable CMI5 tracking in the quiz settings
3. When a learner takes the quiz, their progress and results will be tracked according to the CMI5 specification

## Troubleshooting

### Quiz Won't Load

If a quiz won't load:

1. Check your internet connection
2. Refresh the page
3. Try accessing the quiz from a different browser

### Can't Submit Answers

If you can't submit answers:

1. Check that you've answered all required questions
2. Make sure you're still within the time limit (if applicable)
3. Try refreshing the page (your answers should be saved)

### Score Calculation Issues

If your score seems incorrect:

1. Check the quiz settings to see how scores are calculated
2. Review your answers to see which ones were marked incorrect
3. Contact your instructor if you believe there's an error

## Getting Help

If you encounter any issues not covered in this guide:

1. Check the full documentation in the `docs` directory
2. Ask your instructor or administrator for assistance
3. Report bugs through the appropriate channels
