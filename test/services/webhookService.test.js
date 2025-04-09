import webhookService from '../../src/services/webhookService';
import notificationService from '../../src/services/notificationService';

// Mock dependencies
jest.mock('../../src/services/notificationService');

describe('Webhook Service', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    notificationService.createNotification.mockResolvedValue({ id: 'test-notification' });
  });
  
  describe('Canvas Webhooks', () => {
    test('should process submission_created webhook', async () => {
      const payload = {
        event_type: 'submission_created',
        submission: {
          id: 's123',
          assignment: { 
            id: 'a123',
            name: 'Test Assignment'
          }
        },
        user: { id: 'u123' },
        course: { id: 'c123' }
      };
      
      const result = await webhookService.handleCanvasWebhook(payload);
      
      expect(notificationService.createNotification).toHaveBeenCalledTimes(1);
      expect(notificationService.createNotification.mock.calls[0][0]).toHaveProperty('notificationType', 'submission_created');
      expect(result).toHaveProperty('status', 'processed');
    });
    
    test('should process discussion_entry_created webhook', async () => {
      const payload = {
        event_type: 'discussion_entry_created',
        discussion_entry: {
          id: 'de123',
          message: 'Test reply'
        },
        discussion_topic: {
          id: 'dt123',
          user_id: 'u456'
        },
        user: { id: 'u123' },
        course: { id: 'c123' }
      };
      
      const result = await webhookService.handleCanvasWebhook(payload);
      
      expect(notificationService.createNotification).toHaveBeenCalledTimes(1);
      expect(notificationService.createNotification.mock.calls[0][0]).toHaveProperty('notificationType', 'discussion_entry_created');
      expect(result).toHaveProperty('userId', 'u456');
    });
    
    test('should ignore unhandled webhook events', async () => {
      const payload = {
        event_type: 'unknown_event',
        data: {}
      };
      
      const result = await webhookService.handleCanvasWebhook(payload);
      
      expect(notificationService.createNotification).not.toHaveBeenCalled();
      expect(result).toHaveProperty('status', 'ignored');
    });
  });
  
  describe('Discourse Webhooks', () => {
    test('should process post_created webhook', async () => {
      const payload = {
        event_name: 'post_created',
        post: {
          id: 'p123',
          raw: 'Test post content',
          topic_id: 't123'
        },
        topic: {
          id: 't123',
          title: 'Test Topic'
        },
        user: { id: 'u123' }
      };
      
      const result = await webhookService.handleDiscourseWebhook(payload);
      
      expect(result).toHaveProperty('status', 'processed');
    });
  });
});