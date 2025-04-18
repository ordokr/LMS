# Quiz Module Collaborative Quizzes

This document describes the collaborative quizzes feature implemented for the Quiz Module, allowing multiple users to work together on creating and editing quizzes.

## 1. Overview

The Quiz Module now includes a comprehensive collaboration system that:

- Allows multiple users to collaborate on quiz creation and editing
- Provides different roles with varying permissions (Owner, Editor, Viewer)
- Supports inviting users by ID or email
- Includes a commenting system for discussions about quizzes and questions
- Tracks edit history for accountability and transparency

This system enables teams of educators to work together efficiently on quiz creation, improving quality and reducing duplication of effort.

## 2. Collaboration Roles

The following roles are defined for quiz collaboration:

```rust
pub enum CollaborationRole {
    Owner,    // Full control over the quiz, including managing collaborators
    Editor,   // Can edit the quiz content but cannot manage collaborators
    Viewer,   // Can only view the quiz but not edit it
}
```

## 3. Data Models

### Quiz Collaborator

The `QuizCollaborator` represents a user's role in a quiz:

```rust
pub struct QuizCollaborator {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub role: CollaborationRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Collaboration Invitation

The `CollaborationInvitation` represents an invitation to collaborate on a quiz:

```rust
pub struct CollaborationInvitation {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub inviter_id: Uuid,
    pub invitee_id: Option<Uuid>,
    pub invitee_email: Option<String>,
    pub role: CollaborationRole,
    pub status: InvitationStatus,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Quiz Edit History

The `QuizEditHistory` tracks changes made to a quiz:

```rust
pub struct QuizEditHistory {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub edit_type: String,
    pub description: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### Quiz Comment

The `QuizComment` represents a comment on a quiz or question:

```rust
pub struct QuizComment {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question_id: Option<Uuid>,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 4. Database Schema

The collaboration system uses several tables:

### Quiz Collaborators Table

```sql
CREATE TABLE IF NOT EXISTS quiz_collaborators (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);
```

### Quiz Collaboration Invitations Table

```sql
CREATE TABLE IF NOT EXISTS quiz_collaboration_invitations (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    inviter_id TEXT NOT NULL,
    invitee_id TEXT,
    invitee_email TEXT,
    role TEXT NOT NULL,
    status TEXT NOT NULL,
    token TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);
```

### Quiz Edit History Table

```sql
CREATE TABLE IF NOT EXISTS quiz_edit_history (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    edit_type TEXT NOT NULL,
    description TEXT NOT NULL,
    details TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);
```

### Quiz Comments Table

```sql
CREATE TABLE IF NOT EXISTS quiz_comments (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    question_id TEXT,
    user_id TEXT NOT NULL,
    parent_id TEXT,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE,
    FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES quiz_comments (id) ON DELETE CASCADE
);
```

## 5. Core Functionality

### Managing Collaborators

Collaborators can be added, updated, and removed:

```rust
// Add a collaborator to a quiz
let collaborator = collaboration_service.add_collaborator(quiz_id, user_id, role).await?;

// Update a collaborator's role
let collaborator = collaboration_service.update_collaborator_role(quiz_id, user_id, role, updated_by).await?;

// Remove a collaborator from a quiz
collaboration_service.remove_collaborator(quiz_id, user_id, removed_by).await?;
```

### Managing Invitations

Users can be invited to collaborate on a quiz:

```rust
// Invite a user by ID
let invitation = collaboration_service.invite_user(quiz_id, inviter_id, invitee_id, role).await?;

// Invite a user by email
let invitation = collaboration_service.invite_by_email(quiz_id, inviter_id, email, role).await?;

// Accept an invitation
let collaborator = collaboration_service.accept_invitation(invitation_id, user_id).await?;

// Decline an invitation
collaboration_service.decline_invitation(invitation_id, user_id).await?;

// Cancel an invitation
collaboration_service.cancel_invitation(invitation_id, user_id).await?;
```

### Managing Comments

Comments can be added, updated, and removed:

```rust
// Add a comment to a quiz
let comment = collaboration_service.add_comment(quiz_id, user_id, content).await?;

// Add a comment to a question
let comment = collaboration_service.add_question_comment(quiz_id, question_id, user_id, content).await?;

// Add a reply to a comment
let comment = collaboration_service.add_reply(parent_id, user_id, content).await?;

// Update a comment
let comment = collaboration_service.update_comment(comment_id, user_id, content).await?;

// Delete a comment
collaboration_service.delete_comment(comment_id, user_id).await?;
```

## 6. Tauri Commands

The following Tauri commands are available for the collaboration system:

### Collaborator Commands

```typescript
// Add a collaborator to a quiz
const collaborator = await invoke('add_collaborator', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  userId: '550e8400-e29b-41d4-a716-446655440001',
  role: 'Editor',
});

// Update a collaborator's role
const collaborator = await invoke('update_collaborator_role', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  userId: '550e8400-e29b-41d4-a716-446655440001',
  role: 'Viewer',
  updatedBy: '550e8400-e29b-41d4-a716-446655440002',
});

// Remove a collaborator from a quiz
await invoke('remove_collaborator', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  userId: '550e8400-e29b-41d4-a716-446655440001',
  removedBy: '550e8400-e29b-41d4-a716-446655440002',
});

// Get a collaborator
const collaborator = await invoke('get_collaborator', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  userId: '550e8400-e29b-41d4-a716-446655440001',
});

// Get all collaborators for a quiz
const collaborators = await invoke('get_collaborators', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
});
```

### Invitation Commands

```typescript
// Invite a user by ID
const invitation = await invoke('invite_user', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  inviterId: '550e8400-e29b-41d4-a716-446655440001',
  inviteeId: '550e8400-e29b-41d4-a716-446655440002',
  role: 'Editor',
});

// Invite a user by email
const invitation = await invoke('invite_by_email', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  inviterId: '550e8400-e29b-41d4-a716-446655440001',
  email: 'user@example.com',
  role: 'Editor',
});

// Accept an invitation
const collaborator = await invoke('accept_invitation', {
  invitationId: '550e8400-e29b-41d4-a716-446655440003',
  userId: '550e8400-e29b-41d4-a716-446655440002',
});

// Decline an invitation
await invoke('decline_invitation', {
  invitationId: '550e8400-e29b-41d4-a716-446655440003',
  userId: '550e8400-e29b-41d4-a716-446655440002',
});

// Cancel an invitation
await invoke('cancel_invitation', {
  invitationId: '550e8400-e29b-41d4-a716-446655440003',
  userId: '550e8400-e29b-41d4-a716-446655440001',
});

// Get an invitation
const invitation = await invoke('get_invitation', {
  invitationId: '550e8400-e29b-41d4-a716-446655440003',
});

// Get all invitations for a quiz
const invitations = await invoke('get_invitations_for_quiz', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
});

// Get all pending invitations for a user
const invitations = await invoke('get_pending_invitations_for_user', {
  userId: '550e8400-e29b-41d4-a716-446655440002',
});
```

### Comment Commands

```typescript
// Add a comment to a quiz
const comment = await invoke('add_comment', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  userId: '550e8400-e29b-41d4-a716-446655440001',
  content: 'This is a comment',
});

// Add a comment to a question
const comment = await invoke('add_question_comment', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
  questionId: '550e8400-e29b-41d4-a716-446655440004',
  userId: '550e8400-e29b-41d4-a716-446655440001',
  content: 'This is a question comment',
});

// Add a reply to a comment
const comment = await invoke('add_reply', {
  parentId: '550e8400-e29b-41d4-a716-446655440005',
  userId: '550e8400-e29b-41d4-a716-446655440001',
  content: 'This is a reply',
});

// Update a comment
const comment = await invoke('update_comment', {
  commentId: '550e8400-e29b-41d4-a716-446655440005',
  userId: '550e8400-e29b-41d4-a716-446655440001',
  content: 'This is an updated comment',
});

// Delete a comment
await invoke('delete_comment', {
  commentId: '550e8400-e29b-41d4-a716-446655440005',
  userId: '550e8400-e29b-41d4-a716-446655440001',
});

// Get a comment
const comment = await invoke('get_comment', {
  commentId: '550e8400-e29b-41d4-a716-446655440005',
});

// Get all comments for a quiz
const comments = await invoke('get_comments_for_quiz', {
  quizId: '550e8400-e29b-41d4-a716-446655440000',
});

// Get all comments for a question
const comments = await invoke('get_comments_for_question', {
  questionId: '550e8400-e29b-41d4-a716-446655440004',
});

// Get all replies to a comment
const replies = await invoke('get_replies', {
  commentId: '550e8400-e29b-41d4-a716-446655440005',
});
```

## 7. Frontend Integration

The collaboration system can be integrated into the frontend to provide a collaborative quiz editing experience:

### Collaborator Management

```tsx
// In a collaborator management component
const [collaborators, setCollaborators] = useState([]);
const [role, setRole] = useState('Editor');
const [userId, setUserId] = useState('');

useEffect(() => {
  const fetchCollaborators = async () => {
    const collaborators = await invoke('get_collaborators', {
      quizId: quizId,
    });
    setCollaborators(collaborators);
  };
  
  fetchCollaborators();
}, [quizId]);

const addCollaborator = async () => {
  try {
    const collaborator = await invoke('add_collaborator', {
      quizId: quizId,
      userId: userId,
      role: role,
    });
    
    setCollaborators([...collaborators, collaborator]);
    setUserId('');
  } catch (error) {
    console.error('Error adding collaborator:', error);
  }
};

const updateRole = async (userId, newRole) => {
  try {
    const collaborator = await invoke('update_collaborator_role', {
      quizId: quizId,
      userId: userId,
      role: newRole,
      updatedBy: currentUser.id,
    });
    
    setCollaborators(collaborators.map(c => 
      c.user_id === userId ? collaborator : c
    ));
  } catch (error) {
    console.error('Error updating role:', error);
  }
};

const removeCollaborator = async (userId) => {
  try {
    await invoke('remove_collaborator', {
      quizId: quizId,
      userId: userId,
      removedBy: currentUser.id,
    });
    
    setCollaborators(collaborators.filter(c => c.user_id !== userId));
  } catch (error) {
    console.error('Error removing collaborator:', error);
  }
};

return (
  <div className="collaborator-management">
    <h2>Collaborators</h2>
    
    <div className="add-collaborator">
      <input
        type="text"
        placeholder="User ID"
        value={userId}
        onChange={e => setUserId(e.target.value)}
      />
      
      <select value={role} onChange={e => setRole(e.target.value)}>
        <option value="Owner">Owner</option>
        <option value="Editor">Editor</option>
        <option value="Viewer">Viewer</option>
      </select>
      
      <button onClick={addCollaborator}>Add Collaborator</button>
    </div>
    
    <ul className="collaborator-list">
      {collaborators.map(collaborator => (
        <li key={collaborator.id} className="collaborator-item">
          <div className="collaborator-info">
            <span className="collaborator-name">{collaborator.user_id}</span>
            <span className="collaborator-role">{collaborator.role}</span>
          </div>
          
          <div className="collaborator-actions">
            <select
              value={collaborator.role}
              onChange={e => updateRole(collaborator.user_id, e.target.value)}
            >
              <option value="Owner">Owner</option>
              <option value="Editor">Editor</option>
              <option value="Viewer">Viewer</option>
            </select>
            
            <button onClick={() => removeCollaborator(collaborator.user_id)}>
              Remove
            </button>
          </div>
        </li>
      ))}
    </ul>
  </div>
);
```

### Invitation Management

```tsx
// In an invitation management component
const [invitations, setInvitations] = useState([]);
const [email, setEmail] = useState('');
const [role, setRole] = useState('Editor');

useEffect(() => {
  const fetchInvitations = async () => {
    const invitations = await invoke('get_invitations_for_quiz', {
      quizId: quizId,
    });
    setInvitations(invitations);
  };
  
  fetchInvitations();
}, [quizId]);

const inviteByEmail = async () => {
  try {
    const invitation = await invoke('invite_by_email', {
      quizId: quizId,
      inviterId: currentUser.id,
      email: email,
      role: role,
    });
    
    setInvitations([...invitations, invitation]);
    setEmail('');
  } catch (error) {
    console.error('Error inviting user:', error);
  }
};

const cancelInvitation = async (invitationId) => {
  try {
    await invoke('cancel_invitation', {
      invitationId: invitationId,
      userId: currentUser.id,
    });
    
    setInvitations(invitations.filter(i => i.id !== invitationId));
  } catch (error) {
    console.error('Error canceling invitation:', error);
  }
};

return (
  <div className="invitation-management">
    <h2>Invitations</h2>
    
    <div className="invite-user">
      <input
        type="email"
        placeholder="Email"
        value={email}
        onChange={e => setEmail(e.target.value)}
      />
      
      <select value={role} onChange={e => setRole(e.target.value)}>
        <option value="Owner">Owner</option>
        <option value="Editor">Editor</option>
        <option value="Viewer">Viewer</option>
      </select>
      
      <button onClick={inviteByEmail}>Invite</button>
    </div>
    
    <ul className="invitation-list">
      {invitations.map(invitation => (
        <li key={invitation.id} className="invitation-item">
          <div className="invitation-info">
            <span className="invitation-email">{invitation.invitee_email}</span>
            <span className="invitation-role">{invitation.role}</span>
            <span className="invitation-status">{invitation.status}</span>
          </div>
          
          <div className="invitation-actions">
            <button onClick={() => cancelInvitation(invitation.id)}>
              Cancel
            </button>
          </div>
        </li>
      ))}
    </ul>
  </div>
);
```

### Comment System

```tsx
// In a comment system component
const [comments, setComments] = useState([]);
const [content, setContent] = useState('');

useEffect(() => {
  const fetchComments = async () => {
    const comments = await invoke('get_comments_for_quiz', {
      quizId: quizId,
    });
    setComments(comments);
  };
  
  fetchComments();
}, [quizId]);

const addComment = async () => {
  try {
    const comment = await invoke('add_comment', {
      quizId: quizId,
      userId: currentUser.id,
      content: content,
    });
    
    setComments([comment, ...comments]);
    setContent('');
  } catch (error) {
    console.error('Error adding comment:', error);
  }
};

const updateComment = async (commentId, newContent) => {
  try {
    const comment = await invoke('update_comment', {
      commentId: commentId,
      userId: currentUser.id,
      content: newContent,
    });
    
    setComments(comments.map(c => 
      c.id === commentId ? comment : c
    ));
  } catch (error) {
    console.error('Error updating comment:', error);
  }
};

const deleteComment = async (commentId) => {
  try {
    await invoke('delete_comment', {
      commentId: commentId,
      userId: currentUser.id,
    });
    
    setComments(comments.filter(c => c.id !== commentId));
  } catch (error) {
    console.error('Error deleting comment:', error);
  }
};

return (
  <div className="comment-system">
    <h2>Comments</h2>
    
    <div className="add-comment">
      <textarea
        placeholder="Add a comment..."
        value={content}
        onChange={e => setContent(e.target.value)}
      />
      
      <button onClick={addComment}>Add Comment</button>
    </div>
    
    <ul className="comment-list">
      {comments.map(comment => (
        <li key={comment.id} className="comment-item">
          <div className="comment-header">
            <span className="comment-author">{comment.user_id}</span>
            <span className="comment-date">
              {new Date(comment.created_at).toLocaleString()}
            </span>
          </div>
          
          <div className="comment-content">{comment.content}</div>
          
          <div className="comment-actions">
            <button onClick={() => updateComment(comment.id, prompt('Edit comment:', comment.content))}>
              Edit
            </button>
            
            <button onClick={() => deleteComment(comment.id)}>
              Delete
            </button>
          </div>
        </li>
      ))}
    </ul>
  </div>
);
```

## 8. Security Considerations

- **Access Control**: Only authorized users can view and edit quizzes.
- **Role-Based Permissions**: Different roles have different permissions.
- **Invitation Expiry**: Invitations expire after a certain period.
- **Edit History**: All changes are tracked for accountability.
- **Comment Moderation**: Comments can be deleted by the author or quiz owner.

## 9. Performance Considerations

- **Indexing**: The database schema includes indexes on foreign keys to improve query performance.
- **Pagination**: When retrieving large sets of data, consider implementing pagination to improve performance.
- **Caching**: Consider caching frequently accessed data, such as collaborators and comments.

## 10. Future Enhancements

- **Real-Time Collaboration**: Implement WebSockets for real-time updates when multiple users are editing a quiz.
- **Conflict Resolution**: Add mechanisms to resolve conflicts when multiple users edit the same content.
- **Version History**: Allow viewing and restoring previous versions of a quiz.
- **Advanced Permissions**: Add more granular permissions for specific actions.
- **Comment Reactions**: Allow users to react to comments with emojis.
- **Mention System**: Allow mentioning users in comments.
- **Notification Integration**: Send notifications when users are mentioned or when comments are added to a quiz.
