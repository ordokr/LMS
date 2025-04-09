# User Model Documentation

The User model provides a unified representation of users across different LMS platforms (Canvas and Discourse).

## Properties

| Property | Type | Description |
|----------|------|-------------|
| id | String | Unique identifier for the unified user |
| name | String | User's full name |
| email | String | User's email address |
| username | String | Username for authentication (generated from email if not provided) |
| avatar | String | URL to user's profile image |
| canvasId | String | User's ID in Canvas LMS |
| discourseId | String | User's ID in Discourse forum |
| lastLogin | Date | Timestamp of last login activity |
| sourceSystem | String | Origin system ("canvas" or "discourse") |
| roles | Array<String> | User roles (e.g., "student", "teacher", "moderator") |
| metadata | Object | Original user data from source system |

## Methods

### Static Methods

#### `fromCanvasUser(canvasUser)`
Converts a Canvas user object to a unified User model.

#### `fromDiscourseUser(discourseUser)`
Converts a Discourse user object to a unified User model.

### Instance Methods

#### `toCanvasUser()`
Converts the unified user back to Canvas format.

#### `toDiscourseUser()`
Converts the unified user back to Discourse format.

#### `_generateUsername(email)`
Internal method to generate a username from an email address if not provided.

## Usage Example

```javascript
// Create a user from Canvas data
const canvasUser = {
  id: '123',
  name: 'John Doe',
  email: 'john.doe@example.com',
  login_id: 'johndoe',
  avatar_url: 'https://example.com/avatar.jpg'
};
const user = User.fromCanvasUser(canvasUser);

// Convert to Discourse format
const discourseUser = user.toDiscourseUser();
```