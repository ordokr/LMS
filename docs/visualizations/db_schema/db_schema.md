# Database Schema

```mermaid
erDiagram
    User {
        i64 id
        String name
        String email
        DateTime created_at
    }
    Course {
        i64 id
        String title
        String description
        i64 instructor_id
        DateTime created_at
    }
    Enrollment {
        i64 id
        i64 user_id
        i64 course_id
        String role
        DateTime created_at
    }
    Assignment {
        i64 id
        i64 course_id
        String title
        String description
        DateTime due_date
        f64 points_possible
        DateTime created_at
    }
    Submission {
        i64 id
        i64 assignment_id
        i64 user_id
        String content
        Option<f64> score
        DateTime submitted_at
    }
    Discussion {
        i64 id
        i64 course_id
        String title
        String content
        i64 user_id
        DateTime created_at
    }
    DiscussionPost {
        i64 id
        i64 discussion_id
        i64 user_id
        String content
        Option<i64> parent_id
        DateTime created_at
    }
    Course 1--* Enrollment : "has"
    User 1--* Enrollment : "has"
    Course 1--* Assignment : "has"
    Assignment 1--* Submission : "has"
    User 1--* Submission : "makes"
    Course 1--* Discussion : "has"
    Discussion 1--* DiscussionPost : "has"
    User 1--* DiscussionPost : "creates"

```
