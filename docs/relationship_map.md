# Model Relationship Map

```mermaid
graph LR
  Course-->Module
  Course-->Assignment
  Course-->Submission
  Course-->Enrollment
  Course-->CourseStatus
  Course-->EnrollmentRole
  Module-->Course
  Module-->Assignment
  Module-->Submission
  Module-->Enrollment
  Module-->CourseStatus
  Module-->EnrollmentRole
  Assignment-->Course
  Assignment-->Module
  Assignment-->Submission
  Assignment-->Enrollment
  Assignment-->CourseStatus
  Assignment-->EnrollmentRole
  Submission-->Course
  Submission-->Module
  Submission-->Assignment
  Submission-->Enrollment
  Submission-->CourseStatus
  Submission-->EnrollmentRole
  Enrollment-->Course
  Enrollment-->Module
  Enrollment-->Assignment
  Enrollment-->Submission
  Enrollment-->CourseStatus
  Enrollment-->EnrollmentRole
  CourseStatus-->Course
  CourseStatus-->Module
  CourseStatus-->Assignment
  CourseStatus-->Submission
  CourseStatus-->Enrollment
  CourseStatus-->EnrollmentRole
  EnrollmentRole-->Course
  EnrollmentRole-->Module
  EnrollmentRole-->Assignment
  EnrollmentRole-->Submission
  EnrollmentRole-->Enrollment
  EnrollmentRole-->CourseStatus
  ForumCategory-->ForumTopic
  ForumCategory-->ForumPost
  ForumCategory-->ForumUserPreferences
  ForumCategory-->ForumTrustLevel
  ForumCategory-->Category
  ForumCategory-->|1..*|Post
  ForumCategory-->Topic
  ForumTopic-->ForumCategory
  ForumTopic-->ForumPost
  ForumTopic-->ForumUserPreferences
  ForumTopic-->ForumTrustLevel
  ForumTopic-->Category
  ForumTopic-->|1..*|Post
  ForumTopic-->Topic
  ForumPost-->ForumCategory
  ForumPost-->ForumTopic
  ForumPost-->ForumUserPreferences
  ForumPost-->ForumTrustLevel
  ForumPost-->Category
  ForumPost-->|1..*|Post
  ForumPost-->Topic
  ForumUserPreferences-->ForumCategory
  ForumUserPreferences-->ForumTopic
  ForumUserPreferences-->ForumPost
  ForumUserPreferences-->ForumTrustLevel
  ForumUserPreferences-->Category
  ForumUserPreferences-->|1..*|Post
  ForumUserPreferences-->Topic
  ForumTrustLevel-->ForumCategory
  ForumTrustLevel-->ForumTopic
  ForumTrustLevel-->ForumPost
  ForumTrustLevel-->ForumUserPreferences
  ForumTrustLevel-->Category
  ForumTrustLevel-->|1..*|Post
  ForumTrustLevel-->Topic
  User-->|1..*|UserRole
  User-->UserProfile
  User-->LoginRequest
  User-->RegisterRequest
  User-->AuthResponse
  UserRole-->User
  UserRole-->UserProfile
  UserRole-->LoginRequest
  UserRole-->RegisterRequest
  UserRole-->AuthResponse
  UserProfile-->User
  UserProfile-->|1..*|UserRole
  UserProfile-->LoginRequest
  UserProfile-->RegisterRequest
  UserProfile-->AuthResponse
  LoginRequest-->User
  LoginRequest-->|1..*|UserRole
  LoginRequest-->UserProfile
  LoginRequest-->RegisterRequest
  LoginRequest-->AuthResponse
  RegisterRequest-->User
  RegisterRequest-->|1..*|UserRole
  RegisterRequest-->UserProfile
  RegisterRequest-->LoginRequest
  RegisterRequest-->AuthResponse
  AuthResponse-->User
  AuthResponse-->|1..*|UserRole
  AuthResponse-->UserProfile
  AuthResponse-->LoginRequest
  AuthResponse-->RegisterRequest
  style Course fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style Module fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style Assignment fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style Submission fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style Enrollment fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style CourseStatus fill:#ffcdd2,stroke:#c62828,stroke-width:1px
  style EnrollmentRole fill:#ffcdd2,stroke:#c62828,stroke-width:1px
  style ForumCategory fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style ForumTopic fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style ForumPost fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style ForumUserPreferences fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style ForumTrustLevel fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style Category fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style Post fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style Topic fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style User fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style UserRole fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style UserProfile fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style LoginRequest fill:#ffcdd2,stroke:#c62828,stroke-width:1px
  style RegisterRequest fill:#fff9c4,stroke:#fbc02d,stroke-width:1px
  style AuthResponse fill:#ffcdd2,stroke:#c62828,stroke-width:1px
  style Tag fill:#fff9c4,stroke:#fbc02d,stroke-width:1px

```
