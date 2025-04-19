# Ordo Quiz API Documentation

This document provides an overview of the Ordo Quiz API endpoints and their usage.

## Base URL

All API endpoints are relative to the base URL: `http://localhost:3000/api`

## Authentication

Most endpoints require authentication using a JWT token. Include the token in the Authorization header:

```
Authorization: Bearer <token>
```

To obtain a token, use the login endpoint:

```
POST /auth/login
{
  "username": "your_username",
  "password": "your_password"
}
```

## Quiz Endpoints

### List Quizzes

```
GET /quizzes
```

Returns a list of all quizzes.

### Get Quiz

```
GET /quizzes/:id
```

Returns a specific quiz by ID.

### Create Quiz

```
POST /quizzes
{
  "title": "Quiz Title",
  "description": "Quiz Description",
  "time_limit": 30,
  "passing_score": 70,
  "shuffle_questions": true,
  "show_results": true,
  "visibility": "private",
  "tags": ["tag1", "tag2"],
  "study_mode": "multiple_choice"
}
```

Creates a new quiz.

### Update Quiz

```
PUT /quizzes/:id
{
  "title": "Updated Quiz Title",
  "description": "Updated Quiz Description",
  ...
}
```

Updates an existing quiz.

### Delete Quiz

```
DELETE /quizzes/:id
```

Deletes a quiz.

## Question Endpoints

### List Questions

```
GET /quizzes/:id/questions
```

Returns all questions for a quiz.

### Get Question

```
GET /quizzes/:quiz_id/questions/:question_id
```

Returns a specific question.

### Create Question

```
POST /quizzes/:id/questions
{
  "quiz_id": "quiz_id",
  "question_text": "What is the capital of France?",
  "question_type": "multiple_choice",
  "points": 10,
  "position": 1
}
```

Creates a new question for a quiz.

### Update Question

```
PUT /quizzes/:quiz_id/questions/:question_id
{
  "question_text": "Updated question text",
  ...
}
```

Updates an existing question.

### Delete Question

```
DELETE /quizzes/:quiz_id/questions/:question_id
```

Deletes a question.

## Answer Endpoints

### List Answers

```
GET /quizzes/:quiz_id/questions/:question_id/answers
```

Returns all answer options for a question.

### Get Answer

```
GET /quizzes/:quiz_id/questions/:question_id/answers/:answer_id
```

Returns a specific answer option.

### Create Answer

```
POST /quizzes/:quiz_id/questions/:question_id/answers
{
  "question_id": "question_id",
  "option_text": "Paris",
  "is_correct": true,
  "position": 1
}
```

Creates a new answer option for a question.

### Update Answer

```
PUT /quizzes/:quiz_id/questions/:question_id/answers/:answer_id
{
  "option_text": "Updated answer text",
  ...
}
```

Updates an existing answer option.

### Delete Answer

```
DELETE /quizzes/:quiz_id/questions/:question_id/answers/:answer_id
```

Deletes an answer option.

## Quiz Attempt Endpoints

### List Attempts

```
GET /quizzes/:id/attempts
```

Returns all attempts for a quiz.

### Get Attempt

```
GET /quizzes/:quiz_id/attempts/:attempt_id
```

Returns a specific attempt.

### Start Attempt

```
POST /quizzes/:id/attempts
{
  "quiz_id": "quiz_id",
  "user_id": "user_id"
}
```

Starts a new quiz attempt.

### Submit Answers

```
PUT /quizzes/:quiz_id/attempts/:attempt_id
{
  "answers": [
    {
      "question_id": "question_id",
      "answer_id": "answer_id"
    },
    ...
  ]
}
```

Submits answers for a quiz attempt.

### Complete Attempt

```
POST /quizzes/:quiz_id/attempts/:attempt_id/complete
{
  "attempt_id": "attempt_id"
}
```

Completes a quiz attempt.

### Abandon Attempt

```
DELETE /quizzes/:quiz_id/attempts/:attempt_id
```

Abandons a quiz attempt.

## Quiz Settings Endpoints

### Get Settings

```
GET /quizzes/:id/settings
```

Returns settings for a quiz.

### Create Settings

```
POST /quizzes/:id/settings
{
  "quiz_id": "quiz_id",
  "allow_retakes": true,
  "max_attempts": 3,
  "show_correct_answers": true,
  "show_correct_answers_after_completion": true,
  "time_limit": 30,
  "passing_score": 70,
  "shuffle_questions": true
}
```

Creates settings for a quiz.

### Update Settings

```
PUT /quizzes/:id/settings
{
  "allow_retakes": false,
  ...
}
```

Updates settings for a quiz.

## Analytics Endpoints

### Get Quiz Analytics

```
GET /quizzes/:id/analytics
```

Returns analytics for a quiz.

### Get User Quiz Analytics

```
GET /quizzes/:id/analytics/user/:user_id
```

Returns analytics for a specific user on a quiz.

## Error Handling

All endpoints return standardized error responses:

```json
{
  "error": "Error message",
  "details": "Detailed error information"
}
```

HTTP status codes are used to indicate the result of the request:
- 200: Success
- 201: Created
- 204: No Content
- 400: Bad Request
- 401: Unauthorized
- 404: Not Found
- 500: Internal Server Error
