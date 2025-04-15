# Testing Documentation

## Overview

The LMS Integration platform uses Jest for unit and integration testing. We've implemented comprehensive tests for core components with special configuration for testing in isolation from external dependencies.

## Test Structure

- **Unit Tests**: Located in `test/` directory, test individual components
- **Integration Tests**: Located in `tests/integration/`, test interactions between components
- **Sample Tests**: Basic examples in `tests/unit/`

## Setup for Tests

### Configuration

Jest is configured to use the following:
- Environment: Node.js
- Setup files: `jest.setup.js` (sets NODE_ENV to 'test')
- Coverage reporting enabled

### MongoDB Handling

For tests, MongoDB connections are conditionally created:
```javascript
// Only connect to MongoDB in non-test environment
if (process.env.NODE_ENV !== 'test') {
  mongoose.connect(MONGODB_URI)
    .then(() => console.log('Connected to MongoDB'))
    .catch(err => console.error('MongoDB connection error:', err));
}
```

### Test Server

The application server is conditionally started:
```javascript
// Create server only if not in test environment
let server;
if (process.env.NODE_ENV !== 'test') {
  server = app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
  });
}
```

## Running Tests

```bash
# Run all tests
npm test

# Run with open handle detection
npm run test:debug

# Run specific test file
npm test -- test/models/User.test.js

# Run with coverage report
npm test -- --coverage
```

## Current Coverage

| Component | % Statements | % Branch | % Functions | % Lines |
|-----------|--------------|----------|-------------|---------|
| Auth | 100 | 100 | 100 | 100 |
| Models (User) | 89 | 78 | 86 | 92 |
| Models (Notification) | 100 | 89 | 100 | 100 |
| Services (Notification) | 79 | 85 | 82 | 82 |
| Routes | 70 | 100 | 75 | 70 |
| Overall | 24 | 17 | 26 | 24 |

## Mocking

Most tests use Jest's mocking capabilities to isolate the tested component:

```javascript
// Example of service mocking
jest.mock('../../src/services/notificationService', () => ({
  getUserNotifications: jest.fn(),
  markAsRead: jest.fn()
}));
```

## Improving Test Coverage

Priority areas for test coverage improvements:
1. API client implementations (Canvas, Discourse)
2. Remaining models (Assignment, Course, Discussion)
3. Auth controllers and services
4. Webhook handlers