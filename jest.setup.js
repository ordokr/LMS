// Global setup for Jest tests
process.env.NODE_ENV = 'test';

// Add proper error handlers for unhandled promise rejections
process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled Rejection at:', promise, 'reason:', reason);
});

// Don't use beforeEach/afterEach here as they're not available in this context