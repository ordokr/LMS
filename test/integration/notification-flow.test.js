import request from 'supertest';
const { app } = require('../../src/app');
import { generateJwtToken } from '../../src/auth/jwtService';
import canvasApi from '../../src/api/canvasApi';
import discourseApi from '../../src/api/discourseApi';

// Mock APIs but leave the rest of the system intact for integration testing
jest.mock('../../src/api/canvasApi');
jest.mock('../../src/api/discourseApi');

// Setup for integration tests
beforeAll(async () => {
  // Any setup needed
});

afterAll(async () => {
  // Close any open connections
  if (app.server) {
    await new Promise(resolve => app.server.close(resolve));
  }
});

// Add beforeAll and afterAll hooks

// Setup test
beforeAll(() => {
  // Any setup code
  jest.setTimeout(10000); // Increase timeout for integration tests
});

// Clean up test resources
afterAll(async () => {
  // Any cleanup code
  jest.clearAllMocks();
  
  // If you have any open servers
  if (global.server) {
    await new Promise(resolve => global.server.close(resolve));
  }
});

describe('Notification Flow Integration Tests', () => {
  let authToken;
  
  beforeAll(() => {
    // Generate a real token for the test user
    const testUser = { id: 'test123', email: 'test@example.com', roles: ['student'] };
    authToken = generateJwtToken(testUser);
    
    // Setup API mocks
    canvasApi.getUserNotifications.mockResolvedValue([
      { id: 'c1', user_id: 'test123', subject: 'Canvas Note', read: false, created_at: new Date().toISOString() }
    ]);
    
    discourseApi.getUserNotifications.mockResolvedValue([
      { id: 'd1', user_id: 'test123', data: { excerpt: 'Discourse Note' }, read: false, created_at: new Date().toISOString() }
    ]);
  });
  
  test('Full notification flow: fetch and mark as read', async () => {
    // Step 1: Fetch notifications
    const getResponse = await request(app)
      .get('/api/v1/notifications')
      .set('Authorization', `Bearer ${authToken}`);
    
    expect(getResponse.status).toBe(200);
    expect(getResponse.body.length).toBeGreaterThanOrEqual(2);
    
    // Find a Canvas notification from the response
    const canvasNotification = getResponse.body.find(n => n.canvasId === 'c1');
    expect(canvasNotification).toBeTruthy();
    
    // Step 2: Mark the Canvas notification as read
    canvasApi.markNotificationAsRead.mockResolvedValue({
      ...canvasNotification,
      read: true
    });
    
    const markResponse = await request(app)
      .post(`/api/v1/notifications/${canvasNotification.id}/read`)
      .set('Authorization', `Bearer ${authToken}`)
      .send({ source: 'canvas' });
    
    expect(markResponse.status).toBe(200);
    expect(markResponse.body.read).toBe(true);
    expect(canvasApi.markNotificationAsRead).toHaveBeenCalledWith(canvasNotification.id);
  });
  
  test('Webhook creates notification', async () => {
    // Mock the notification creation
    const createdNotification = {
      id: 'new1',
      user_id: 'test123',
      subject: 'New notification',
      read: false
    };
    
    canvasApi.createNotification.mockResolvedValue(createdNotification);
    discourseApi.createNotification.mockResolvedValue({ id: 'dnew1', ...createdNotification });
    
    // Send webhook
    const webhookResponse = await request(app)
      .post('/api/v1/webhooks/canvas')
      .send({
        event_type: 'submission_created',
        submission: {
          id: 's123',
          assignment: { id: 'a123', name: 'Test Assignment' }
        },
        user: { id: 'test123' },
        course: { id: 'c123' }
      });
    
    expect(webhookResponse.status).toBe(200);
    expect(webhookResponse.body.status).toBe('success');
    
    // Verify APIs were called
    expect(canvasApi.createNotification).toHaveBeenCalled();
    expect(discourseApi.createNotification).toHaveBeenCalled();
  });
});

// Add proper cleanup:

afterAll(async () => {
  // Close any open server instances
  if (global.server) {
    await new Promise(resolve => global.server.close(resolve));
  }
  
  // Reset mocks and database connections if applicable
  jest.clearAllMocks();
});