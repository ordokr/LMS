import notificationService from '../../src/services/notificationService';
import canvasApi from '../../src/api/canvasApi';
import discourseApi from '../../src/api/discourseApi';
import { Notification } from '../../src/models/unifiedModels';

// Mock dependencies
jest.mock('../../src/api/canvasApi');
jest.mock('../../src/api/discourseApi');

describe('Notification Service', () => {
  // Sample data
  const userId = '123';
  const canvasNotifications = [
    { id: 'c1', user_id: '123', subject: 'Canvas Note 1', message: 'Hello Canvas', read: false, created_at: '2025-04-06T10:00:00Z' },
    { id: 'c2', user_id: '123', subject: 'Canvas Note 2', message: 'Hello Again', read: true, created_at: '2025-04-05T10:00:00Z' }
  ];
  
  const discourseNotifications = [
    { id: 'd1', user_id: '123', data: { excerpt: 'New reply' }, read: false, created_at: '2025-04-07T10:00:00Z' },
    { id: 'd2', user_id: '123', data: { excerpt: 'Topic mentioned' }, read: false, created_at: '2025-04-04T10:00:00Z' }
  ];
  
  beforeEach(() => {
    // Reset mocks before each test
    jest.clearAllMocks();
    
    // Setup mock implementations
    canvasApi.getUserNotifications.mockResolvedValue(canvasNotifications);
    discourseApi.getUserNotifications.mockResolvedValue(discourseNotifications);
    
    // Mock user mapping method
    notificationService._getUserMapping = jest.fn().mockResolvedValue({
      internalId: userId,
      canvasId: userId,
      discourseId: userId
    });
  });
  
  test('should fetch and combine notifications from both systems', async () => {
    const notifications = await notificationService.getUserNotifications(userId);
    
    // Verify API calls
    expect(canvasApi.getUserNotifications).toHaveBeenCalledWith(userId);
    expect(discourseApi.getUserNotifications).toHaveBeenCalledWith(userId);
    
    // Verify results
    expect(notifications).toHaveLength(4);
    
    // Should be sorted by date (newest first)
    expect(notifications[0].discourseId).toBe('d1');  // April 7
    expect(notifications[1].canvasId).toBe('c1');     // April 6
    expect(notifications[2].canvasId).toBe('c2');     // April 5
    expect(notifications[3].discourseId).toBe('d2');  // April 4
  });
  
  test('should filter notifications by read status', async () => {
    const unreadNotifications = await notificationService.getUserNotifications(userId, { read: false });
    
    expect(unreadNotifications).toHaveLength(3);
    expect(unreadNotifications.every(n => n.read === false)).toBe(true);
  });
  
  test('should mark notification as read in Canvas', async () => {
    const notificationId = 'c1';
    const updatedNotification = { ...canvasNotifications[0], read: true };
    
    canvasApi.markNotificationAsRead.mockResolvedValue(updatedNotification);
    
    const result = await notificationService.markAsRead(notificationId, 'canvas');
    
    expect(canvasApi.markNotificationAsRead).toHaveBeenCalledWith(notificationId);
    expect(result).toBeInstanceOf(Notification);
    expect(result.read).toBe(true);
  });
  
  test('should mark notification as read in Discourse', async () => {
    const notificationId = 'd1';
    const updatedNotification = { ...discourseNotifications[0], read: true };
    
    discourseApi.markNotificationAsRead.mockResolvedValue(updatedNotification);
    
    const result = await notificationService.markAsRead(notificationId, 'discourse');
    
    expect(discourseApi.markNotificationAsRead).toHaveBeenCalledWith(notificationId);
    expect(result).toBeInstanceOf(Notification);
    expect(result.read).toBe(true);
  });
  
  test('should create a notification in both systems', async () => {
    const notificationData = {
      userId: '123',
      subject: 'Test Notification',
      message: 'This is a test notification'
    };
    
    canvasApi.createNotification.mockResolvedValue({ id: 'cn1', ...notificationData });
    discourseApi.createNotification.mockResolvedValue({ id: 'dn1', ...notificationData });
    
    const result = await notificationService.createNotification(notificationData);
    
    expect(canvasApi.createNotification).toHaveBeenCalled();
    expect(discourseApi.createNotification).toHaveBeenCalled();
    expect(result).toBeInstanceOf(Notification);
    expect(result.canvasId).toBe('cn1');
    expect(result.discourseId).toBe('dn1');
  });
});