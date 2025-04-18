# Quiz Module Course Integration

This document describes the course integration features implemented for the Quiz Module, allowing quizzes to be integrated with courses, modules, and sections.

## 1. Overview

The Quiz Module now includes comprehensive integration with the course system, allowing quizzes to be:

- Added to courses
- Organized within modules and sections
- Assigned to students
- Tracked for completion and grading
- Scheduled with due dates and availability windows

This integration enables instructors to use quizzes as part of their course curriculum, and students to access and complete quizzes within the context of their courses.

## 2. Data Model

### Quiz-Course Mapping

The `QuizCourseMapping` represents the relationship between a quiz and a course:

```rust
pub struct QuizCourseMapping {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub course_id: Uuid,
    pub module_id: Option<Uuid>,
    pub section_id: Option<Uuid>,
    pub position: i32,
    pub is_required: bool,
    pub passing_score: Option<f32>,
    pub due_date: Option<DateTime<Utc>>,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
    pub max_attempts: Option<i32>,
    pub time_limit: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Quiz Assignment

The `QuizAssignment` represents the assignment of a quiz to a student:

```rust
pub struct QuizAssignment {
    pub id: Uuid,
    pub mapping_id: Uuid,
    pub student_id: Uuid,
    pub status: QuizAssignmentStatus,
    pub attempts: i32,
    pub best_score: Option<f32>,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Assignment Status

The `QuizAssignmentStatus` represents the status of a quiz assignment:

```rust
pub enum QuizAssignmentStatus {
    NotStarted,
    InProgress,
    Completed,
    Overdue,
}
```

### Quiz with Context

The `QuizWithContext` provides a quiz with its course context:

```rust
pub struct QuizWithContext {
    pub quiz: Quiz,
    pub mapping: QuizCourseMapping,
    pub course: Course,
    pub module: Option<Module>,
    pub section: Option<Section>,
    pub assignment: Option<QuizAssignment>,
}
```

## 3. Database Schema

The course integration uses two main tables:

### Quiz-Course Mappings Table

```sql
CREATE TABLE IF NOT EXISTS quiz_course_mappings (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    course_id TEXT NOT NULL,
    module_id TEXT,
    section_id TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    is_required INTEGER NOT NULL DEFAULT 1,
    passing_score REAL,
    due_date TEXT,
    available_from TEXT,
    available_until TEXT,
    max_attempts INTEGER,
    time_limit INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);
```

### Quiz Assignments Table

```sql
CREATE TABLE IF NOT EXISTS quiz_assignments (
    id TEXT PRIMARY KEY,
    mapping_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    status TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    best_score REAL,
    last_attempt_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (mapping_id) REFERENCES quiz_course_mappings (id) ON DELETE CASCADE
);
```

## 4. Core Functionality

### Adding Quizzes to Courses

Quizzes can be added to courses, optionally within specific modules and sections:

```rust
// Add a quiz to a course
let mapping = course_integration_service.add_quiz_to_course(
    quiz_id,
    course_id,
    module_id,  // Optional
    section_id, // Optional
    position,   // Optional
).await?;
```

### Removing Quizzes from Courses

Quizzes can be removed from courses:

```rust
// Remove a quiz from a course
course_integration_service.remove_quiz_from_course(mapping_id).await?;
```

### Updating Quiz-Course Mappings

Quiz-course mappings can be updated to change settings:

```rust
// Update a quiz-course mapping
mapping.passing_score = Some(80.0);
mapping.due_date = Some(Utc::now() + Duration::days(7));
course_integration_service.update_mapping(&mapping).await?;
```

### Retrieving Quizzes for a Course

All quizzes for a course can be retrieved:

```rust
// Get all quizzes for a course
let mappings = course_integration_service.get_quizzes_for_course(course_id).await?;
```

### Retrieving Courses for a Quiz

All courses that include a quiz can be retrieved:

```rust
// Get all courses for a quiz
let courses = course_integration_service.get_courses_for_quiz(quiz_id).await?;
```

### Retrieving Quiz with Context

A quiz can be retrieved with its course context:

```rust
// Get a quiz with its course context
let context = course_integration_service.get_quiz_with_context(
    mapping_id,
    student_id, // Optional
).await?;
```

### Retrieving Quizzes for a Student

All quizzes for a student in a course can be retrieved:

```rust
// Get all quizzes for a student in a course
let quizzes = course_integration_service.get_student_quizzes(
    course_id,
    student_id,
).await?;
```

### Assigning Quizzes to Students

Quizzes can be assigned to students:

```rust
// Assign a quiz to a student
let assignment = course_integration_service.assign_quiz_to_student(
    mapping_id,
    student_id,
).await?;
```

### Updating Assignment Status

Assignment status can be updated based on quiz attempts:

```rust
// Update assignment status based on an attempt
let updated_assignment = course_integration_service.update_assignment_from_attempt(
    mapping_id,
    student_id,
    attempt,
).await?;
```

## 5. Tauri Commands

The following Tauri commands are available for course integration:

### Adding Quizzes to Courses

```typescript
// Add a quiz to a course
const mapping = await invoke('add_quiz_to_course', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  courseId: '550e8400-e29b-41d4-a716-446655440001',
  moduleId: '550e8400-e29b-41d4-a716-446655440002', // Optional
  sectionId: '550e8400-e29b-41d4-a716-446655440003', // Optional
  position: 0, // Optional
});
```

### Removing Quizzes from Courses

```typescript
// Remove a quiz from a course
await invoke('remove_quiz_from_course', {
  mappingId: '550e8400-e29b-41d4-a716-446655440004',
});
```

### Updating Quiz-Course Mappings

```typescript
// Update a quiz-course mapping
await invoke('update_quiz_course_mapping', {
  mapping: {
    id: '550e8400-e29b-41d4-a716-446655440004',
    quizId: '550e8400-e29b-41d4-a716-446655440000',
    courseId: '550e8400-e29b-41d4-a716-446655440001',
    moduleId: '550e8400-e29b-41d4-a716-446655440002',
    sectionId: '550e8400-e29b-41d4-a716-446655440003',
    position: 0,
    isRequired: true,
    passingScore: 80.0,
    dueDate: '2023-12-31T23:59:59Z',
    availableFrom: '2023-01-01T00:00:00Z',
    availableUntil: '2023-12-31T23:59:59Z',
    maxAttempts: 3,
    timeLimit: 60,
    createdAt: '2023-01-01T00:00:00Z',
    updatedAt: '2023-01-01T00:00:00Z',
  },
});
```

### Retrieving Quizzes for a Course

```typescript
// Get all quizzes for a course
const mappings = await invoke('get_quizzes_for_course', {
  courseId: '550e8400-e29b-41d4-a716-446655440001',
});
```

### Retrieving Courses for a Quiz

```typescript
// Get all courses for a quiz
const courses = await invoke('get_courses_for_quiz', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
});
```

### Retrieving Quiz with Context

```typescript
// Get a quiz with its course context
const context = await invoke('get_quiz_with_context', {
  mappingId: '550e8400-e29b-41d4-a716-446655440004',
  studentId: '550e8400-e29b-41d4-a716-446655440005', // Optional
});
```

### Retrieving Quizzes for a Student

```typescript
// Get all quizzes for a student in a course
const quizzes = await invoke('get_student_quizzes', {
  courseId: '550e8400-e29b-41d4-a716-446655440001',
  studentId: '550e8400-e29b-41d4-a716-446655440005',
});
```

### Assigning Quizzes to Students

```typescript
// Assign a quiz to a student
const assignment = await invoke('assign_quiz_to_student', {
  mappingId: '550e8400-e29b-41d4-a716-446655440004',
  studentId: '550e8400-e29b-41d4-a716-446655440005',
});
```

## 6. Frontend Integration

The course integration can be used in the frontend to:

### Display Quizzes in Course View

```tsx
// In a course view component
const [quizzes, setQuizzes] = useState([]);

useEffect(() => {
  const fetchQuizzes = async () => {
    const mappings = await invoke('get_quizzes_for_course', {
      courseId: courseId,
    });
    setQuizzes(mappings);
  };
  
  fetchQuizzes();
}, [courseId]);

return (
  <div className="course-quizzes">
    <h2>Quizzes</h2>
    <ul>
      {quizzes.map(mapping => (
        <li key={mapping.id}>
          <Link to={`/quiz/${mapping.quizId}`}>
            {mapping.quiz.title}
          </Link>
          {mapping.dueDate && (
            <span className="due-date">
              Due: {new Date(mapping.dueDate).toLocaleDateString()}
            </span>
          )}
        </li>
      ))}
    </ul>
  </div>
);
```

### Display Student Progress

```tsx
// In a student progress component
const [assignments, setAssignments] = useState([]);

useEffect(() => {
  const fetchAssignments = async () => {
    const quizzes = await invoke('get_student_quizzes', {
      courseId: courseId,
      studentId: studentId,
    });
    setAssignments(quizzes);
  };
  
  fetchAssignments();
}, [courseId, studentId]);

return (
  <div className="student-progress">
    <h2>Quiz Progress</h2>
    <table>
      <thead>
        <tr>
          <th>Quiz</th>
          <th>Status</th>
          <th>Score</th>
          <th>Due Date</th>
        </tr>
      </thead>
      <tbody>
        {assignments.map(quiz => (
          <tr key={quiz.mapping.id}>
            <td>{quiz.quiz.title}</td>
            <td>{quiz.assignment?.status || 'Not Started'}</td>
            <td>{quiz.assignment?.bestScore || '-'}</td>
            <td>
              {quiz.mapping.dueDate
                ? new Date(quiz.mapping.dueDate).toLocaleDateString()
                : 'No due date'}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  </div>
);
```

### Add Quiz to Course

```tsx
// In an add quiz to course component
const [selectedQuiz, setSelectedQuiz] = useState(null);
const [selectedModule, setSelectedModule] = useState(null);
const [selectedSection, setSelectedSection] = useState(null);

const handleAddQuiz = async () => {
  if (!selectedQuiz) return;
  
  await invoke('add_quiz_to_course', {
    quizId: selectedQuiz.id,
    courseId: courseId,
    moduleId: selectedModule?.id,
    sectionId: selectedSection?.id,
  });
  
  // Refresh quizzes
  fetchQuizzes();
};

return (
  <div className="add-quiz-form">
    <h2>Add Quiz to Course</h2>
    
    <div className="form-group">
      <label>Quiz</label>
      <select onChange={e => setSelectedQuiz(quizzes.find(q => q.id === e.target.value))}>
        <option value="">Select a quiz</option>
        {quizzes.map(quiz => (
          <option key={quiz.id} value={quiz.id}>
            {quiz.title}
          </option>
        ))}
      </select>
    </div>
    
    <div className="form-group">
      <label>Module (Optional)</label>
      <select onChange={e => setSelectedModule(modules.find(m => m.id === e.target.value))}>
        <option value="">None</option>
        {modules.map(module => (
          <option key={module.id} value={module.id}>
            {module.title}
          </option>
        ))}
      </select>
    </div>
    
    {selectedModule && (
      <div className="form-group">
        <label>Section (Optional)</label>
        <select onChange={e => setSelectedSection(sections.find(s => s.id === e.target.value))}>
          <option value="">None</option>
          {sections
            .filter(section => section.moduleId === selectedModule.id)
            .map(section => (
              <option key={section.id} value={section.id}>
                {section.title}
              </option>
            ))}
        </select>
      </div>
    )}
    
    <button onClick={handleAddQuiz}>Add Quiz</button>
  </div>
);
```

## 7. Security Considerations

- **Access Control**: Only instructors and administrators should be able to add, remove, or update quiz-course mappings.
- **Student Privacy**: Student assignment data should only be accessible to the student, instructors, and administrators.
- **Data Validation**: All inputs should be validated to prevent injection attacks and ensure data integrity.

## 8. Performance Considerations

- **Indexing**: The database schema includes indexes on foreign keys to improve query performance.
- **Caching**: Consider caching frequently accessed data, such as course quizzes and student assignments.
- **Pagination**: When retrieving large sets of data, consider implementing pagination to improve performance.

## 9. Future Enhancements

- **Bulk Operations**: Add support for bulk adding quizzes to courses or assigning quizzes to multiple students.
- **Quiz Groups**: Allow quizzes to be organized into groups within a course.
- **Conditional Access**: Allow quizzes to be conditionally available based on completion of other course items.
- **Grade Integration**: Integrate quiz scores with the course grading system.
- **Analytics**: Provide analytics on quiz performance across courses and students.
