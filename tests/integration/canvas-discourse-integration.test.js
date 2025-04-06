const { expect } = require('chai');
const { CanvasClient } = require('../../src/clients/canvas');
const { DiscourseClient } = require('../../src/clients/discourse');
const { IntegrationService } = require('../../src/services/integration');

describe('Canvas-Discourse Integration Tests', function() {
  let canvasClient;
  let discourseClient;
  let integrationService;
  
  // Use beforeEach instead of before
  beforeEach(function() {
    canvasClient = new CanvasClient({
      baseUrl: process.env.CANVAS_API_URL || 'http://localhost:3000/api/v1',
      token: process.env.CANVAS_API_TOKEN || 'test-token'
    });
    
    discourseClient = new DiscourseClient({
      baseUrl: process.env.DISCOURSE_API_URL || 'http://localhost:4000',
      apiKey: process.env.DISCOURSE_API_KEY || 'test-key',
      apiUsername: process.env.DISCOURSE_API_USERNAME || 'system'
    });
    
    integrationService = new IntegrationService(canvasClient, discourseClient);
  });
  
  it('should verify basic integration setup', function() {
    expect(true).to.equal(true);
    expect(canvasClient).to.be.an('object');
    expect(discourseClient).to.be.an('object');
    expect(integrationService).to.be.an('object');
  });
  
  it('should demonstrate a test with async/await', async function() {
    // This would be replaced with actual API calls in real tests
    const result = await Promise.resolve({
      success: true,
      discourseTopicId: '12345'
    });
    
    expect(result).to.have.property('success', true);
    expect(result).to.have.property('discourseTopicId');
  });
  
  describe('Forum Topic Creation', function() {
    it('should create a Discourse topic when a Canvas announcement is published', async function() {
      const announcement = {
        id: 'ann123',
        title: 'Test Announcement',
        message: 'This is a test announcement',
        courseId: 'course123'
      };
      
      const result = await integrationService.syncAnnouncementToForum(announcement);
      
      expect(result).to.have.property('success', true);
      expect(result).to.have.property('discourseTopicId');
      expect(result).to.have.property('canvasAnnouncementId', 'ann123');
    });
  });
  
  describe('User Authentication Flow', function() {
    it('should authenticate a Canvas user with Discourse', async function() {
      const canvasUser = {
        id: 'user123',
        email: 'test@example.com',
        name: 'Test User'
      };
      
      const result = await integrationService.authenticateUserWithDiscourse(canvasUser);
      
      expect(result).to.have.property('success', true);
      expect(result).to.have.property('canvasUserId', 'user123');
      expect(result).to.have.property('discourseUserId');
      expect(result).to.have.property('ssoToken');
    });
  });
});