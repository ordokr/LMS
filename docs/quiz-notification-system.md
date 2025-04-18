# Quiz Module Notification System

This document describes the notification system implemented for the Quiz Module, allowing users to receive notifications about quiz-related events.

## 1. Overview

The Quiz Module now includes a comprehensive notification system that:

- Notifies students when quizzes are assigned to them
- Alerts students when quizzes are due soon or overdue
- Informs students when their quizzes have been graded
- Notifies students when feedback is available
- Alerts students when quizzes are updated or removed
- Provides a notification center for viewing and managing notifications

This system ensures that students stay informed about their quiz assignments and deadlines, improving engagement and completion rates.

## 2. Notification Types

The following notification types are defined:

```rust
pub enum QuizNotificationType {
    QuizAssigned,         // A new quiz has been assigned
    QuizDueSoon,          // A quiz is due within 24 hours
    QuizOverdue,          // A quiz is past its due date
    QuizCompleted,        // A quiz has been completed
    QuizGraded,           // A quiz has been graded
    QuizFeedbackAvailable, // Feedback is available for a quiz
    QuizUpdated,          // A quiz has been updated
    QuizRemoved,          // A quiz has been removed
}
```

## 3. Data Model

### Quiz Notification

The `QuizNotification` represents a notification sent to a user:

```rust
pub struct QuizNotification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: QuizNotificationType,
    pub quiz_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
    pub mapping_id: Option<Uuid>,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub read: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 4. Database Schema

The notification system uses a dedicated table:

```sql
CREATE TABLE IF NOT EXISTS quiz_notifications (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    quiz_id TEXT,
    course_id TEXT,
    mapping_id TEXT,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    link TEXT,
    read INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

## 5. Core Functionality

### Sending Notifications

The notification system can send various types of notifications:

#### Quiz Assigned Notification

```rust
// Send a notification when a quiz is assigned to a student
notification_service.notify_quiz_assigned(student_id, &quiz, &mapping).await?;
```

#### Quiz Due Soon Notification

```rust
// Send a notification when a quiz is due soon
notification_service.notify_quiz_due_soon(student_id, &quiz, &mapping, &assignment).await?;
```

#### Quiz Overdue Notification

```rust
// Send a notification when a quiz is overdue
notification_service.notify_quiz_overdue(student_id, &quiz, &mapping, &assignment).await?;
```

#### Quiz Completed Notification

```rust
// Send a notification when a quiz is completed
notification_service.notify_quiz_completed(student_id, &quiz, &mapping, &attempt).await?;
```

#### Quiz Graded Notification

```rust
// Send a notification when a quiz is graded
notification_service.notify_quiz_graded(student_id, &quiz, &mapping, &attempt).await?;
```

#### Feedback Available Notification

```rust
// Send a notification when feedback is available
notification_service.notify_feedback_available(student_id, &quiz, &mapping).await?;
```

#### Quiz Updated Notification

```rust
// Send a notification when a quiz is updated
notification_service.notify_quiz_updated(student_ids, &quiz, &mapping).await?;
```

#### Quiz Removed Notification

```rust
// Send a notification when a quiz is removed
notification_service.notify_quiz_removed(student_ids, quiz_title, course_id).await?;
```

### Retrieving Notifications

Notifications can be retrieved for a user:

```rust
// Get notifications for a user
let notifications = notification_service.get_notifications_for_user(user_id, limit, offset).await?;
```

### Managing Notifications

Notifications can be marked as read or deleted:

```rust
// Mark a notification as read
notification_service.mark_notification_as_read(notification_id).await?;

// Mark all notifications as read for a user
notification_service.mark_all_notifications_as_read(user_id).await?;

// Delete a notification
notification_service.delete_notification(notification_id).await?;

// Delete all notifications for a user
notification_service.delete_all_notifications_for_user(user_id).await?;
```

### Automatic Notification Checks

The system can automatically check for quizzes that are due soon or overdue:

```rust
// Check for due soon quizzes
notification_service.check_due_soon_quizzes().await?;

// Check for overdue quizzes
notification_service.check_overdue_quizzes().await?;
```

## 6. Tauri Commands

The following Tauri commands are available for the notification system:

### Retrieving Notifications

```typescript
// Get notifications for a user
const notifications = await invoke('get_quiz_notifications', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  limit: 20,
  offset: 0,
});
```

### Getting Unread Count

```typescript
// Get unread notification count for a user
const unreadCount = await invoke('get_unread_notification_count', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
});
```

### Managing Notifications

```typescript
// Mark a notification as read
await invoke('mark_notification_as_read', {
  notificationId: '550e8400-e29b-41d4-a716-446655440001',
});

// Mark all notifications as read for a user
await invoke('mark_all_notifications_as_read', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
});

// Delete a notification
await invoke('delete_notification', {
  notificationId: '550e8400-e29b-41d4-a716-446655440001',
});

// Delete all notifications for a user
await invoke('delete_all_notifications', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
});
```

### Checking for Notifications

```typescript
// Check for due soon and overdue quizzes
await invoke('check_quiz_notifications');
```

## 7. Frontend Integration

The notification system can be integrated into the frontend to provide a notification center:

### Notification Bell Component

```tsx
// In a notification bell component
const [unreadCount, setUnreadCount] = useState(0);

useEffect(() => {
  const fetchUnreadCount = async () => {
    if (currentUser) {
      const count = await invoke('get_unread_notification_count', {
        userId: currentUser.id,
      });
      setUnreadCount(count);
    }
  };
  
  fetchUnreadCount();
  
  // Set up an interval to check for new notifications
  const interval = setInterval(fetchUnreadCount, 60000); // Check every minute
  
  return () => clearInterval(interval);
}, [currentUser]);

return (
  <div className="notification-bell">
    <button onClick={toggleNotificationCenter}>
      <BellIcon />
      {unreadCount > 0 && (
        <span className="notification-badge">{unreadCount}</span>
      )}
    </button>
  </div>
);
```

### Notification Center Component

```tsx
// In a notification center component
const [notifications, setNotifications] = useState([]);
const [loading, setLoading] = useState(true);

useEffect(() => {
  const fetchNotifications = async () => {
    if (currentUser) {
      setLoading(true);
      const notifs = await invoke('get_quiz_notifications', {
        userId: currentUser.id,
        limit: 20,
        offset: 0,
      });
      setNotifications(notifs);
      setLoading(false);
    }
  };
  
  fetchNotifications();
}, [currentUser]);

const markAsRead = async (notificationId) => {
  await invoke('mark_notification_as_read', {
    notificationId,
  });
  
  // Update the notification in the list
  setNotifications(notifications.map(n => 
    n.id === notificationId ? { ...n, read: true } : n
  ));
};

const markAllAsRead = async () => {
  await invoke('mark_all_notifications_as_read', {
    userId: currentUser.id,
  });
  
  // Update all notifications in the list
  setNotifications(notifications.map(n => ({ ...n, read: true })));
};

const deleteNotification = async (notificationId) => {
  await invoke('delete_notification', {
    notificationId,
  });
  
  // Remove the notification from the list
  setNotifications(notifications.filter(n => n.id !== notificationId));
};

return (
  <div className="notification-center">
    <div className="notification-header">
      <h3>Notifications</h3>
      <button onClick={markAllAsRead}>Mark All as Read</button>
    </div>
    
    {loading ? (
      <div className="loading">Loading notifications...</div>
    ) : notifications.length === 0 ? (
      <div className="empty-state">No notifications</div>
    ) : (
      <ul className="notification-list">
        {notifications.map(notification => (
          <li 
            key={notification.id} 
            className={`notification-item ${notification.read ? 'read' : 'unread'}`}
          >
            <div className="notification-content">
              <h4>{notification.title}</h4>
              <p>{notification.message}</p>
              <span className="notification-time">
                {new Date(notification.created_at).toLocaleString()}
              </span>
            </div>
            <div className="notification-actions">
              {!notification.read && (
                <button onClick={() => markAsRead(notification.id)}>
                  Mark as Read
                </button>
              )}
              {notification.link && (
                <a href={notification.link}>View</a>
              )}
              <button onClick={() => deleteNotification(notification.id)}>
                Delete
              </button>
            </div>
          </li>
        ))}
      </ul>
    )}
  </div>
);
```

### Automatic Notification Checks

```tsx
// In the app initialization
useEffect(() => {
  // Check for notifications when the app starts
  invoke('check_quiz_notifications');
  
  // Set up an interval to check for notifications
  const interval = setInterval(() => {
    invoke('check_quiz_notifications');
  }, 3600000); // Check every hour
  
  return () => clearInterval(interval);
}, []);
```

## 8. Integration with Global Notification System

The quiz notification system integrates with the application's global notification system, if available:

```rust
// If the notification service is available, send the notification through it
if let Some(notification_service) = &self.notification_service {
    notification_service.send_notification(
        user_id,
        &title,
        &message,
        link.as_deref(),
    ).await?;
}
```

This allows notifications to be delivered through various channels, such as:

- In-app notifications
- Email notifications
- Push notifications
- SMS notifications

## 9. Performance Considerations

- **Indexing**: The database schema includes indexes on frequently queried columns to improve performance.
- **Pagination**: Notifications are retrieved with pagination to avoid loading too many at once.
- **Batch Processing**: Notifications for multiple users are processed in batches.
- **Background Processing**: Notification checks are performed in the background to avoid blocking the UI.

## 10. Future Enhancements

- **Notification Preferences**: Allow users to configure which notifications they want to receive.
- **Notification Channels**: Support for additional notification channels, such as mobile push notifications.
- **Rich Notifications**: Support for rich content in notifications, such as images and formatted text.
- **Notification Templates**: Customizable notification templates for different types of notifications.
- **Notification Analytics**: Track notification open rates and engagement.
- **Scheduled Notifications**: Support for scheduling notifications in advance.
