import request from 'supertest';
import express from 'express';
import notificationRoutes from '../../src/routes/notificationRoutes';
import notificationService from '../../src/services/notificationService';
import { requireAuth } from '../../src/middleware/authMiddleware';

// Mock dependencies
jest.mock('../../src/services/notificationService');
jest.mock('../../src/middleware/authMiddleware');

describe('Notification Routes', () => {
  let app;
  
  beforeEach(() => {
    // Create express app for testing
    app = express();
    app.use(express.json());
    
    // Setup mock auth middleware to provide user
    requireAuth.mockImplementation((req, res, next) => {
      req.user = { id: '123', email: 'test@example.com' };
      next();
    });
    
    // Use the notification routes
    app.use('/api/v1/notifications', notificationRoutes);
  });

  // Reset mocks between tests
  beforeEach(() => {
    notificationService.getUserNotifications.mockReset();
    notificationService.markAsRead.mockReset();
  });
  
  test('GET / should return user notifications', async () => {
    const mockNotifications = [
      { id: '1', userId: '123', subject: 'Test 1', read: false },
      { id: '2', userId: '123', subject: 'Test 2', read: true }
    ];
    
    notificationService.getUserNotifications.mockResolvedValue(mockNotifications);
    
    const response = await request(app)
      .get('/api/v1/notifications')
      .query({ read: 'false', limit: '10' });
    
    expect(response.status).toBe(200);
    expect(response.body).toEqual(mockNotifications);
    expect(notificationService.getUserNotifications).toHaveBeenCalledWith('123', {
      read: false,
      type: undefined,
      since: undefined,
      limit: 10
    });
  });
  
  test('POST /:id/read should mark notification as read', async () => {
    const mockNotification = {
      id: '1',
      userId: '123',
      subject: 'Test Notification',
      read: true
    };
    
    notificationService.markAsRead.mockResolvedValue(mockNotification);
    
    const response = await request(app)
      .post('/api/v1/notifications/1/read')
      .send({ source: 'canvas' });
    
    expect(response.status).toBe(200);
    expect(response.body).toEqual(mockNotification);
    expect(notificationService.markAsRead).toHaveBeenCalledWith('1', 'canvas');
  });
  
  it('POST /:id/read should return 400 if source is missing', async () => {
    // Reset mock before test
    notificationService.markAsRead.mockReset();
    
    const response = await request(app)
      .post('/api/v1/notifications/1/read')  // Make sure this matches your route definition
      .set('Authorization', 'Bearer valid-token')
      .send({}); // Empty body with no source
      
    expect(response.status).toBe(400);
    expect(response.body).toHaveProperty('error');
    expect(notificationService.markAsRead).not.toHaveBeenCalled();
  });
});