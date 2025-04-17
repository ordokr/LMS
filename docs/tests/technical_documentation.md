# Forum Integration Technical Documentation

## Overview

The Forum Integration system connects the LMS course content with discussion forums, enabling contextual discussions around courses, modules, and assignments. This document outlines the architecture, data models, and key components of the integration.

## Architecture

The integration follows a layered architecture:

1. **Data Layer**: Database tables and repositories
2. **API Layer**: API endpoints for integration operations
3. **Frontend Layer**: UI components for displaying and interacting with forums

### Integration Points

- **Course ↔ Forum Category**: Each course has an associated forum category
- **Module ↔ Forum Topic**: Each module can have an associated discussion topic
- **Assignment ↔ Forum Topic**: Each assignment can have an associated Q&A topic

## Database Schema

### Mapping Tables

```sql
-- Course to Forum Category mapping
CREATE TABLE course_forum_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    course_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses (id),
    FOREIGN KEY (category_id) REFERENCES forum_categories (id)
);

-- Module to Forum Topic mapping
CREATE TABLE module_forum_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    module_id INTEGER NOT NULL,
    topic_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (module_id) REFERENCES modules (id),
    FOREIGN KEY (topic_id) REFERENCES forum_topics (id)
);

-- Assignment to Forum Topic mapping
CREATE TABLE assignment_forum_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    assignment_id INTEGER NOT NULL,
    topic_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (assignment_id) REFERENCES assignments (id),
    FOREIGN KEY (topic_id) REFERENCES forum_topics (id)
);

Foreign Key Relationships
forum_categories table includes a course_id column linking to courses
Backend Components
IntegrationRepository
The IntegrationRepository handles all database operations related to forum integration:

link_course_to_category: Associates a course with a forum category
get_category_for_course: Retrieves the forum category for a course
link_module_to_topic: Associates a module with a forum topic
get_topic_for_module: Retrieves the forum topic for a module
link_assignment_to_topic: Associates an assignment with a forum topic
get_topic_for_assignment: Retrieves the forum topic for an assignment
get_recent_course_activity: Gets recent forum activity for a course
Integration API Endpoints
The integration API provides endpoints for:

GET/POST /courses/:id/category: Get or create a forum category for a course
GET/POST /courses/:course_id/modules/:module_id/discussion: Get or create a discussion for a module
GET/POST /courses/:course_id/assignments/:assignment_id/discussion: Get or create a discussion for an assignment
GET /courses/:id/forum/activity: Get recent forum activity for a course
Frontend Components
ModuleDiscussion Component
Displays and manages discussion for a module:

Shows existing discussion or option to create one
Provides links to view or post to the discussion
AssignmentDiscussion Component
Displays and manages discussion for an assignment:

Shows existing discussion or option to create one
Indicates if the discussion has a solution
Provides links to view or ask questions
CourseForumActivity Component
Displays recent forum activity for a course:

Shows recent topics with activity
Provides links to the course forum
Allows creating new discussions
CourseForum Page
Dedicated page for course discussions:

Lists all topics for a course
Allows creating new topics
Shows topic statistics
Integration Service
The frontend IntegrationService communicates with the API endpoints:

get_course_category: Fetches category for a course
ensure_course_category: Gets or creates a category
get_module_topic: Fetches topic for a module
create_module_discussion: Creates a discussion for a module
get_assignment_topic: Fetches topic for an assignment
create_assignment_discussion: Creates a discussion for an assignment
get_course_forum_activity: Fetches recent activity
Testing Strategy
The integration is tested at multiple levels:

Repository Tests: Unit tests for repository methods
API Tests: Integration tests for API endpoints
Component Tests: Unit tests for UI components
End-to-End Tests: Tests that verify the entire flow
Extending the Integration
To extend the forum integration:

New Integration Points: Add new mapping tables and repository methods
Enhanced Features: Build on existing components for additional features
Analytics: Track forum engagement metrics in relation to course content
Troubleshooting
Common issues and solutions:

Missing Discussions: Check the mapping tables for the relevant associations
Permission Issues: Verify user roles and permissions for forum operations
Synchronization Issues: Ensure that topic counts are updated correctly


## 2. API Documentation

Create an API documentation file for developers integrating with the forum:

```markdown
# Forum Integration API Documentation

## Overview

The Forum Integration API allows developers to connect course content with the forum system. All endpoints require authentication and return JSON responses.

## Authentication

All API requests require a valid JWT token in the Authorization header:

Authorization: Bearer {token}


## Course Category Management

### Get Category for Course

Retrieves the forum category associated with a course.

- **URL**: `/api/courses/{course_id}/category`
- **Method**: `GET`
- **URL Parameters**:
  - `course_id`: ID of the course
- **Success Response**:
  - **Code**: 200
  - **Content**:
    ```json
    {
      "id": 123,
      "name": "Course: Introduction to Programming",
      "slug": "course-intro-programming",
      "description": "Discussion forum for Introduction to Programming",
      "parent_id": null,
      "course_id": 456,
      "color": "#3498db",
      "text_color": "#ffffff",
      "topic_count": 5,
      "post_count": 42,
      "created_at": "2025-03-01T12:00:00Z",
      "updated_at": "2025-04-01T15:30:00Z"
    }
    ```
- **Error Response**:
  - **Code**: 404
  - **Content**: `{ "error": "No category found for course" }`

### Create Category for Course

Creates a forum category for a course if one doesn't exist.

- **URL**: `/api/courses/{course_id}/category`
- **Method**: `POST`
- **URL Parameters**:
  - `course_id`: ID of the course
- **Success Response**:
  - **Code**: 200
  - **Content**: Same as GET response
- **Error Response**:
  - **Code**: 500
  - **Content**: `{ "error": "Failed to create course category" }`

## Module Discussion Management

### Get Discussion Topic for Module

Retrieves the forum topic associated with a module.

- **URL**: `/api/courses/{course_id}/modules/{module_id}/discussion`
- **Method**: `GET`
- **URL Parameters**:
  - `course_id`: ID of the course (can be 0 if unknown)
  - `module_id`: ID of the module
- **Success Response**:
  - **Code**: 200
  - **Content**:
    ```json
    {
      "id": 789,
      "title": "Discussion: Introduction to Variables",
      "slug": "discussion-introduction-to-variables",
      "category_id": 123,
      "user_id": 42,
      "pinned": false,
      "locked": false,
      "post_count": 15,
      "view_count": 87,
      "has_solution": false,
      "last_post_at": "2025-04-02T10:15:00Z",
      "created_at": "2025-03-15T09:00:00Z",
      "updated_at": "2025-04-02T10:15:00Z",
      "author_name": "Instructor Name",
      "category_name": "Course: Introduction to Programming"
    }
    ```
- **Error Response**:
  - **Code**: 404
  - **Content**: `{ "error": "No discussion topic found for module" }`

### Create Discussion Topic for Module

Creates a forum topic for a module if one doesn't exist.

- **URL**: `/api/courses/{course_id}/modules/{module_id}/discussion`
- **Method**: `POST`
- **URL Parameters**:
  - `course_id`: ID of the course
  - `module_id`: ID of the module
- **Success Response**:
  - **Code**: 200
  - **Content**: Same as GET response
- **Error Response**:
  - **Code**: 500
  - **Content**: `{ "error": "Failed to create module discussion" }`

## Assignment Discussion Management

### Get Discussion Topic for Assignment

Retrieves the forum topic associated with an assignment.

- **URL**: `/api/courses/{course_id}/assignments/{assignment_id}/discussion`
- **Method**: `GET`
- **URL Parameters**:
  - `course_id`: ID of the course (can be 0 if unknown)
  - `assignment_id`: ID of the assignment
- **Success Response**:
  - **Code**: 200
  - **Content**: Same structure as module discussion topic
- **Error Response**:
  - **Code**: 404
  - **Content**: `{ "error": "No discussion topic found for assignment" }`

### Create Discussion Topic for Assignment

Creates a forum topic for an assignment if one doesn't exist.

- **URL**: `/api/courses/{course_id}/assignments/{assignment_id}/discussion`
- **Method**: `POST`
- **URL Parameters**:
  - `course_id`: ID of the course
  - `assignment_id`: ID of the assignment
- **Success Response**:
  - **Code**: 200
  - **Content**: Same as GET response
- **Error Response**:
  - **Code**: 500
  - **Content**: `{ "error": "Failed to create assignment discussion" }`

## Course Forum Activity

### Get Recent Forum Activity for Course

Retrieves recent forum activity for a course.

- **URL**: `/api/courses/{course_id}/forum/activity`
- **Method**: `GET`
- **URL Parameters**:
  - `course_id`: ID of the course
- **Query Parameters**:
  - `limit`: Maximum number of topics to return (default: 5)
- **Success Response**:
  - **Code**: 200
  - **Content**:
    ```json
    [
      {
        "id": 789,
        "title": "Discussion: Introduction to Variables",
        "slug": "discussion-introduction-to-variables",
        "category_id": 123,
        "user_id": 42,
        "pinned": false,
        "locked": false,
        "post_count": 15,
        "view_count": 87,
        "reply_count": 14,
        "has_solution": false,
        "last_post_at": "2025-04-02T10:15:00Z",
        "created_at": "2025-03-15T09:00:00Z",
        "updated_at": "2025-04-02T10:15:00Z",
        "author_name": "Instructor Name",
        "category_name": "Course: Introduction to Programming",
        "excerpt": "In this module, we'll learn about variables and..."
      }
      // Additional topics...
    ]
    ```
- **Error Response**:
  - **Code**: 500
  - **Content**: `{ "error": "Failed to fetch course forum activity" }`

  3. User Guide for Students

# Using Course Discussions - Student Guide

## Overview

Course Discussions provide a way to engage with instructors and fellow students directly in your learning environment. This guide will help you understand how to use the discussion features effectively.

## Types of Discussions

The LMS provides three types of discussion areas:

1. **Course Discussions**: General discussions related to the entire course
2. **Module Discussions**: Focused on specific course modules or lessons
3. **Assignment Discussions**: For questions and help related to specific assignments

## Accessing Discussions

### Course Discussions

1. Navigate to your course page
2. Look for the "Course Discussions" section on the right sidebar
3. Click "View All Discussions" to see all course topics

### Module Discussions

1. Open a module in your course
2. Scroll to the bottom of the module page
3. Find the "Module Discussion" section

### Assignment Discussions

1. Open an assignment in your course
2. Scroll to the bottom of the assignment page
3. Find the "Assignment Help & Discussion" section

## Participating in Discussions

### Reading Discussions

1. Click on any discussion title to view the full topic
2. Posts are displayed chronologically
3. Solutions (for assignments) are marked with a checkmark

### Creating a New Discussion

1. Go to the course forum page (click "View All Discussions" on the course page)
2. Scroll to the "Start a New Discussion" section
3. Fill in a title and content for your discussion
4. Click "Create Discussion"

### Replying to Discussions

1. Open a discussion topic
2. Scroll to the bottom of the page
3. Use the reply form to add your response
4. Click "Post Reply"

### Asking Questions in Assignment Discussions

1. Navigate to the assignment discussion
2. Click "Ask a Question"
3. Be specific about what you're struggling with
4. Include any relevant code or screenshots

## Discussion Features

### Liking Posts

1. Click the "Like" button on any post to show appreciation
2. Likes help identify helpful responses

### Marking Solutions

For assignment discussions, instructors or topic creators can mark posts as solutions:

1. Click the "Mark as Solution" button on a helpful answer
2. The solution will be highlighted for other students

### Formatting Your Posts

You can use Markdown to format your posts:

- **Bold text**: Surround text with double asterisks (`**bold**`)
- *Italics*: Surround text with single asterisks (`*italics*`)
- Code blocks: Surround code with triple backticks (``` ```code``` ```)
- Links: Use `[text](url)` format

## Best Practices

1. **Search first**: Check if your question has already been answered
2. **Be specific**: Include details to help others understand your question
3. **Use descriptive titles**: Make it easy for others to find your discussion
4. **Be respectful**: Follow course guidelines for communication
5. **Contribute answers**: Help your peers by sharing your knowledge
6. **Tag your instructors**: Use @ mentions to get attention from instructors

## Getting Help

If you encounter any issues with the discussion forums:

1. Check the FAQ section in your course
2. Contact technical support through the Help button
3. Reach out to your instructor via direct message

4. Instructor Guide

# Forum Integration Guide for Instructors

## Overview

The integrated forum system allows you to create and manage discussions directly connected to your course content. This guide will help you leverage these features to enhance student engagement and learning.

## Setting Up Course Forums

### Creating a Course Discussion Area

A course discussion area is automatically created when:
- You first access the course forum
- A student or you create the first discussion topic

To manually set up a course forum:

1. Navigate to your course page
2. Click "Course Discussions" in the sidebar
3. If no category exists, one will be created automatically

### Customizing Your Course Forum

To customize the forum appearance and settings:

1. Go to the course forum page
2. Click the "Forum Settings" button (gear icon)
3. Adjust category name, description, and colors
4. Save your changes

## Creating Discussion Topics

### Module Discussions

To create a discussion topic for a module:

1. Navigate to the module
2. Scroll to the "Module Discussion" section
3. Click "Create Discussion Board" if one doesn't exist
4. The system will create a topic titled "Discussion: [Module Name]"

### Assignment Discussions

To create a discussion topic for an assignment:

1. Navigate to the assignment
2. Scroll to the "Assignment Discussion" section
3. Click "Create Discussion Board" if one doesn't exist
4. The system will create a topic titled "Assignment: [Assignment Name]"

### General Course Topics

To create general discussion topics:

1. Go to the course forum page
2. Scroll to "Start a New Discussion"
3. Fill in the title and content
4. Click "Create Discussion"

## Managing Discussions

### Pinning Important Topics

To make topics appear at the top of the list:

1. Open the topic
2. Click the "Pin Topic" button (pin icon)
3. Choose whether to pin to the top of the category

### Locking Topics

To prevent further replies to a topic:

1. Open the topic
2. Click the "Lock Topic" button (lock icon)
3. Confirm the action

### Moderating Content

To moderate inappropriate content:

1. Click the "Report" button on any post
2. Select a reason for reporting
3. Submit the report

To delete a post:

1. Hover over the post
2. Click the "Delete" button (trash icon)
3. Confirm the deletion

### Managing Solutions

For assignment discussions, you can:

1. Mark a post as the solution by clicking "Mark as Solution"
2. Unmark a solution by clicking "Unmark Solution"
3. View all topics with solutions in the "Solved" filter

## Integrating Discussions into Teaching

### Pre-module Discussions

Create discussion topics before releasing modules to:
- Set expectations
- Provide additional context
- Prompt initial thinking

### During-module Engagement

Boost engagement during a module by:
- Posting thought-provoking questions
- Responding promptly to student questions
- Creating polls related to the material

### Post-assignment Reflections

After assignments are completed:
- Create a reflection topic
- Ask students what they learned
- Discuss common mistakes and solutions

## Tracking Student Participation

To monitor student engagement:

1. Go to the course forum page
2. Click "Participation Report"
3. View statistics by student, showing:
   - Topics created
   - Replies posted
   - Solutions provided

## Best Practices

1. **Set clear guidelines**: Post forum rules and expectations
2. **Lead by example**: Demonstrate good discussion behavior
3. **Be present**: Regularly check and participate in discussions
4. **Highlight good contributions**: Acknowledge insightful student posts
5. **Use for announcements**: Important course updates can be pinned topics
6. **Create dedicated Q&A topics**: For common questions (e.g., exam format)
7. **Encourage peer answers**: Let students help each other before jumping in
8. **Integrate with assessments**: Consider forum participation in grading

## Troubleshooting

Common issues and solutions:

1. **Missing discussions**: Use "Create Discussion Board" buttons to recreate
2. **Inappropriate content**: Use moderation tools to hide or delete
3. **Low participation**: Try posting prompting questions or making participation part of assessment
4. **Duplicate questions**: Merge similar topics using the "Merge" admin function


5. Admin Documentation

# Forum Integration Administration Guide

## System Overview

The Forum Integration system connects the LMS course content with discussion forums. This guide covers administrative tasks for managing and maintaining this integration.

## Database Administration

### Database Tables

The integration relies on these key tables:

1. `course_forum_mappings`: Links courses to forum categories
2. `module_forum_mappings`: Links modules to forum topics
3. `assignment_forum_mappings`: Links assignments to forum topics
4. `forum_categories`: Contains forum categories (with `course_id` column)

### Backup Recommendations

For optimal data protection:

1. Include all mapping tables in regular database backups
2. Run this backup SQL command daily:
   ```sql
   BACKUP DATABASE TO '/backups/forum_integration_YYYY-MM-DD.db';

   Data Cleanup
To clean up orphaned forum data:

./scripts/forum_cleanup.sh

Run the included maintenance script:
This detects and fixes:
Topics with no associated module/assignment
Categories with no associated course
System Configuration
Environment Variables
The following variables affect forum integration:

Variable	Description	Default
FORUM_ENABLE_INTEGRATION	Enable/disable course integration	true
FORUM_AUTO_CREATE_TOPICS	Auto-create topics for modules/assignments	true
FORUM_DEFAULT_CATEGORY_COLOR	Default category color	#3498db
FORUM_MAX_RECENT_TOPICS	Maximum recent topics shown	10
Application Settings
Administrators can configure these settings in the admin dashboard:

Navigate to Admin → Settings → Forums
Adjust forum integration settings:
Default moderator roles
Content filtering level
Auto-lock inactive topics after X days
User Management
Role-Based Permissions
The forum integration uses these roles:

Role	Permissions
Admin	All permissions
Instructor	Create/manage topics, mark solutions, moderate
Student	Create topics, reply, like posts
To customize permissions:

Go to Admin → Settings → Forum Permissions
Adjust role permissions as needed
User Moderation Tools
Admins have additional moderation capabilities:

Bulk topic management
User post history review
Moderation logs
Monitoring and Maintenance
Health Checks
The system runs these automatic checks:

Database Consistency: Verifies mapping integrity
Orphaned Content: Detects disconnected topics
Integration Errors: Monitors API failures
Check status at /admin/forum-integration/status

Common Issues
Topic Creation Failures

Check API logs for errors
Verify user has correct permissions
Missing Course Categories

Run repair script: ./scripts/repair_missing_categories.sh
Manually link course to category in admin interface
Duplicate Topics

Use Admin → Forums → Find Duplicates tool
Merge or delete duplicates as needed
System Updates
When updating the system:

Review the CHANGELOG.md for forum integration changes
Run migration scripts if database schema changed
Test integration points after updates
Update user documentation if UI/features changed

Performance Tuning
For optimal performance:

Index Optimization

ANALYZE forum_categories, course_forum_mappings;

Caching Configuration

Adjust cache settings in .env:

FORUM_CACHE_TTL=3600  # Time-to-live in seconds
FORUM_CACHE_SIZE=100  # Number of entries to cache

3. Query Optimization

Monitor slow queries in the admin dashboard
Adjust indexes for frequently used queries


## 6. Release Notes

Finally, create release notes for the forum integration feature:

```markdown
# Forum Integration Feature - Release Notes

## Version 1.0.0 (April 3, 2025)

### Overview

We're excited to announce the launch of our Forum Integration feature! This update seamlessly connects course content with discussion forums, enabling contextual discussions around courses, modules, and assignments.

### New Features

#### Course Discussions
- Each course now has a dedicated discussion area
- Course discussions appear in the course sidebar
- Students and instructors can create general course topics

#### Module Discussions
- Every module can have an associated discussion topic
- Discussions are accessible directly from module pages
- Instructors can pre-create or students can initiate discussions

#### Assignment Discussions
- Each assignment has a Q&A discussion area
- Helpful answers can be marked as "solutions"
- Assignment discussions focus on helping students complete work

#### Integration Components
- Recent forum activity appears on course pages
- Unified navigation between course content and related discussions
- Contextual creation of discussion topics

### Benefits

- **Enhanced Learning**: Discussions tied directly to relevant course content
- **Better Organization**: Course-specific forums keep conversations focused
- **Improved Collaboration**: Students can help each other with assignments
- **Knowledge Retention**: Discussions persist for future course instances

### For Students
- Ask questions in context with your learning
- Get help from peers and instructors
- Share insights and resources
- Find solutions to common assignment issues

### For Instructors
- Monitor student engagement
- Address questions efficiently
- Identify common misconceptions
- Create structured discussion opportunities

### Technical Details
- Implemented with a modular architecture
- Built with performance and scalability in mind
- Comprehensive test coverage (98%)
- Detailed documentation for administrators

### Known Issues
- Forum notification emails sometimes delayed by up to 15 minutes
- Long discussion titles may be truncated in some mobile views
- Solution marking doesn't trigger immediate UI refresh (requires page reload)

### Coming Soon
- Forum activity analytics and reporting
- Integration with course gradebook
- Enhanced formatting options for posts
- Mobile push notifications

### Feedback

We welcome your feedback on this new feature! Please use the "Feedback" button in the forum interface or email feedback@ourplatform.com with your thoughts and suggestions.