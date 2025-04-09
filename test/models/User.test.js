import { User } from '../../src/models/unifiedModels';

describe('User Model', () => {
  const canvasUserData = {
    id: '123',
    name: 'John Doe',
    email: 'john.doe@example.com',
    created_at: '2025-01-15T10:00:00Z',
    updated_at: '2025-01-15T10:00:00Z',
    avatar_url: 'https://canvas.example.com/avatar.png',
    time_zone: 'America/New_York',
    enrollments: [
      { role: 'student', type: 'StudentEnrollment', course_id: '101' },
      { role: 'teacher', type: 'TeacherEnrollment', course_id: '102' }
    ]
  };
  
  const discourseUserData = {
    id: '456',
    name: 'John Doe',
    username: 'johndoe',
    email: 'john.doe@example.com',
    created_at: '2025-01-15T10:00:00Z',
    trust_level: 2,
    title: 'Professor',
    avatar_template: '/avatar/{size}.png',
    admin: false,
    moderator: true
  };

  test('should create a user from Canvas data', () => {
    const user = User.fromCanvasUser(canvasUserData);
    
    expect(user).toBeInstanceOf(User);
    expect(user.name).toBe('John Doe');
    expect(user.email).toBe('john.doe@example.com');
    expect(user.canvasId).toBe('123');
    expect(user.sourceSystem).toBe('canvas');
    expect(user.roles).toContain('student');
    expect(user.roles).toContain('teacher');
  });

  test('should create a user from Discourse data', () => {
    const user = User.fromDiscourseUser(discourseUserData);
    
    expect(user).toBeInstanceOf(User);
    expect(user.name).toBe('John Doe');
    expect(user.email).toBe('john.doe@example.com');
    expect(user.username).toBe('johndoe');
    expect(user.discourseId).toBe('456');
    expect(user.sourceSystem).toBe('discourse');
    expect(user.roles).toContain('moderator');
  });

  test('should convert to Canvas user format', () => {
    const user = new User({
      id: '789',
      name: 'Jane Smith',
      email: 'jane@example.com',
      canvasId: '345',
      avatarUrl: 'https://example.com/avatar.jpg',
      enrollments: [{ role: 'student', course_id: '101' }]
    });
    
    const canvasUser = user.toCanvasUser();
    
    expect(canvasUser).toHaveProperty('id', '345');
    expect(canvasUser).toHaveProperty('name', 'Jane Smith');
    expect(canvasUser).toHaveProperty('email', 'jane@example.com');
    expect(canvasUser).toHaveProperty('avatar_url', 'https://example.com/avatar.jpg');
    expect(canvasUser.enrollments).toHaveLength(1);
  });

  test('should convert to Discourse user format', () => {
    const user = new User({
      id: '789',
      name: 'Jane Smith',
      email: 'jane@example.com',
      username: 'janesmith',
      discourseId: '567',
      trustLevel: 3
    });
    
    const discourseUser = user.toDiscourseUser();
    
    expect(discourseUser).toHaveProperty('id', '567');
    expect(discourseUser).toHaveProperty('name', 'Jane Smith');
    expect(discourseUser).toHaveProperty('username', 'janesmith');
    expect(discourseUser).toHaveProperty('email', 'jane@example.com');
    expect(discourseUser).toHaveProperty('trust_level', 3);
  });

  test('should generate username from email when not provided', () => {
    const user = new User({
      name: 'Test User',
      email: 'test.user@example.com'
    });
    
    expect(user._generateUsername()).toBe('testuser');
    expect(user.toDiscourseUser().username).toBe('testuser');
  });
});