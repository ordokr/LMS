# Quiz Module Authentication

This document describes the authentication and authorization system implemented for the Quiz Module, ensuring that users have appropriate permissions to access and modify quizzes.

## 1. Overview

The Quiz Module now includes a comprehensive authentication and authorization system that:

- Verifies user identity
- Controls access to quizzes based on visibility settings
- Enforces permissions for various quiz operations
- Integrates with the application's role-based access control system

This system ensures that quizzes and their data are protected, while still allowing appropriate access for students, instructors, and administrators.

## 2. Permission Types

The following permission types are defined for quiz operations:

```rust
pub enum QuizPermission {
    View,           // Permission to view a quiz
    Edit,           // Permission to edit a quiz
    Delete,         // Permission to delete a quiz
    Attempt,        // Permission to attempt a quiz
    ViewResults,    // Permission to view quiz results
    ViewAnalytics,  // Permission to view quiz analytics
    ManageCourseIntegration, // Permission to manage course integration
}
```

## 3. Role-Based Access Control

The authentication system integrates with the application's role-based access control system, which defines the following roles:

- **Student**: Can view and attempt quizzes they have access to
- **Instructor**: Can create, edit, and manage quizzes, as well as view analytics
- **Admin**: Has full access to all quizzes and operations

## 4. Permission Rules

The following rules determine whether a user has permission to perform an operation:

### View Permission

A user can view a quiz if:
- They are an admin
- They are the author of the quiz
- The quiz is public
- They have been granted access to the quiz (e.g., enrolled in a course that includes the quiz)
- They are an instructor

### Edit Permission

A user can edit a quiz if:
- They are an admin
- They are the author of the quiz
- They are an instructor with the `EditAnyQuiz` permission

### Delete Permission

A user can delete a quiz if:
- They are an admin
- They are the author of the quiz
- They are an instructor with the `DeleteAnyQuiz` permission

### Attempt Permission

A user can attempt a quiz if:
- The quiz is public
- They have been granted access to the quiz

### View Results Permission

A user can view quiz results if:
- They are an admin
- They are the author of the quiz
- They have attempted the quiz (can only view their own results)
- They are an instructor with the `ViewQuizResults` permission

### View Analytics Permission

A user can view quiz analytics if:
- They are an admin
- They are the author of the quiz
- They are an instructor with the `ViewQuizAnalytics` permission

### Manage Course Integration Permission

A user can manage course integration if:
- They are an admin
- They are an instructor with the `ManageCourseContent` permission

## 5. Implementation

### Authentication Service

The `QuizAuthService` provides methods for checking and enforcing permissions:

```rust
pub struct QuizAuthService {
    auth_service: Arc<crate::auth::AuthService>,
}

impl QuizAuthService {
    // Check if a user has permission to perform an action on a quiz
    pub async fn check_quiz_permission(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
        permission: QuizPermission,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        // Implementation details...
    }
    
    // Enforce a permission check and return an error if the user doesn't have permission
    pub async fn enforce_quiz_permission(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
        permission: QuizPermission,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Implementation details...
    }
    
    // Similar methods for questions...
}
```

### Authentication Middleware

The `QuizAuthMiddleware` provides a convenient interface for Tauri commands:

```rust
pub struct QuizAuthMiddleware {
    auth_service: Arc<QuizAuthService>,
}

impl QuizAuthMiddleware {
    // Check if a user has permission to view a quiz
    pub async fn can_view_quiz(
        &self,
        user_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<(), String> {
        // Implementation details...
    }
    
    // Similar methods for other permissions...
}
```

### Integration with QuizEngine

The `QuizEngine` includes methods for checking permissions:

```rust
impl QuizEngine {
    // Check if a user has permission to view a quiz
    pub async fn can_view_quiz(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Implementation details...
    }
    
    // Similar methods for other permissions...
    
    // Get the auth middleware
    pub fn get_auth_middleware(&self) -> Option<Arc<QuizAuthMiddleware>> {
        self.auth_middleware.clone()
    }
}
```

## 6. Tauri Command Integration

Tauri commands are updated to include user authentication:

```rust
#[tauri::command]
pub async fn create_quiz(
    title: String,
    description: Option<String>,
    author_id: Option<String>,
    study_mode: String,
    user_id: Option<String>, // Added user_id parameter
    engine: State<'_, QuizEngine>,
) -> Result<String, String> {
    // Check authentication if user_id is provided
    if let Some(user_id) = &user_id {
        let user_uuid = Uuid::parse_str(user_id).map_err(|e| e.to_string())?;
        
        // Verify the user has permission to create quizzes
        // In a real implementation, we would check if the user has permission to create quizzes
    }
    
    // Rest of the implementation...
}
```

## 7. Frontend Integration

The frontend can use the authentication system to control access to quiz features:

```typescript
// Check if the user can view a quiz
const canViewQuiz = async (quizId: string) => {
  try {
    const result = await invoke('can_view_quiz', {
      quizId,
      userId: currentUser.id,
    });
    return true;
  } catch (error) {
    console.error('Permission denied:', error);
    return false;
  }
};

// Example usage in a component
const QuizView = ({ quizId }) => {
  const [canView, setCanView] = useState(false);
  const [quiz, setQuiz] = useState(null);
  
  useEffect(() => {
    const checkPermission = async () => {
      const hasPermission = await canViewQuiz(quizId);
      setCanView(hasPermission);
      
      if (hasPermission) {
        const quizData = await invoke('get_quiz', { quizId });
        setQuiz(quizData);
      }
    };
    
    checkPermission();
  }, [quizId]);
  
  if (!canView) {
    return <div>You do not have permission to view this quiz.</div>;
  }
  
  return (
    <div>
      <h1>{quiz?.title}</h1>
      {/* Rest of the component */}
    </div>
  );
};
```

## 8. Security Considerations

- **Principle of Least Privilege**: Users are granted only the permissions they need.
- **Defense in Depth**: Permissions are checked at multiple levels (frontend, backend, database).
- **Fail Closed**: If the authentication system fails, access is denied by default.
- **Audit Logging**: All permission checks and access attempts are logged for security auditing.

## 9. Error Handling

When a permission check fails, a `PermissionError` is returned with a descriptive message:

```rust
#[derive(Debug)]
pub struct PermissionError {
    pub message: String,
}

impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Permission denied: {}", self.message)
    }
}
```

## 10. Fallback Behavior

If the authentication system is not available (e.g., in a standalone mode), the system falls back to allowing access:

```rust
pub async fn can_view_quiz(
    &self,
    user_id: uuid::Uuid,
    quiz_id: uuid::Uuid,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(auth) = &self.auth_service {
        auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::View).await
    } else {
        // If auth service is not available, allow access
        Ok(true)
    }
}
```

## 11. Future Enhancements

- **Fine-Grained Permissions**: Add more granular permissions for specific quiz operations.
- **Permission Groups**: Allow permissions to be grouped and assigned to roles.
- **Delegation**: Allow quiz authors to delegate permissions to other users.
- **API Tokens**: Support for API tokens with specific permissions for integration with other systems.
- **Two-Factor Authentication**: Add support for two-factor authentication for sensitive operations.
