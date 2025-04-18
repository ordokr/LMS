-- Quiz Collaboration Schema

-- Quiz collaborators table
CREATE TABLE IF NOT EXISTS quiz_collaborators (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);

-- Quiz collaboration invitations table
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

-- Quiz edit history table
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

-- Quiz comments table
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

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_quiz_collaborators_quiz_id ON quiz_collaborators (quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_collaborators_user_id ON quiz_collaborators (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_collaboration_invitations_quiz_id ON quiz_collaboration_invitations (quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_collaboration_invitations_inviter_id ON quiz_collaboration_invitations (inviter_id);
CREATE INDEX IF NOT EXISTS idx_quiz_collaboration_invitations_invitee_id ON quiz_collaboration_invitations (invitee_id);
CREATE INDEX IF NOT EXISTS idx_quiz_collaboration_invitations_invitee_email ON quiz_collaboration_invitations (invitee_email);
CREATE INDEX IF NOT EXISTS idx_quiz_collaboration_invitations_token ON quiz_collaboration_invitations (token);
CREATE INDEX IF NOT EXISTS idx_quiz_edit_history_quiz_id ON quiz_edit_history (quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_edit_history_user_id ON quiz_edit_history (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_comments_quiz_id ON quiz_comments (quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_comments_question_id ON quiz_comments (question_id);
CREATE INDEX IF NOT EXISTS idx_quiz_comments_user_id ON quiz_comments (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_comments_parent_id ON quiz_comments (parent_id);
