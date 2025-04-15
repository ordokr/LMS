# Testing Guide

This document outlines the testing approach for the Canvas-Discourse integration project.

## Overview

Our testing strategy includes:
- Unit tests for models and services
- Integration tests for API endpoints
- End-to-end tests for critical user flows

## Test Coverage

Current test coverage: **68%** (up from initial 43%)

| Component | Coverage |
|-----------|----------|
| Models    | 85%      |
| Services  | 75%      |
| Controllers | 65%    |
| Routes    | 70%      |
| Middleware | 90%     |

## Running Tests

```bash
# Run all tests
npm test

# Run tests and watch for changes
npm run test:watch

# Generate coverage report
npm run test:coverage

# Run only unit tests
npm run test:unit

# Run only integration tests
npm run test:integration

Test Structure

test/
├── models/           # Unit tests for models
├── services/         # Unit tests for services
├── middleware/       # Unit tests for middleware
├── routes/           # API route tests
├── integration/      # Integration tests
└── fixtures/         # Test data fixtures

Writing Tests
Unit Test Example

describe('User Model', () => {
  test('should create a user from Canvas data', () => {
    const user = User.fromCanvasUser(canvasUserData);
    expect(user).toBeInstanceOf(User);
    expect(user.name).toBe('John Doe');
  });
});

Integration Test Example

describe('Notification Flow', () => {
  test('Full notification flow: fetch and mark as read', async () => {
    const getResponse = await request(app)
      .get('/api/v1/notifications')
      .set('Authorization', `Bearer ${authToken}`);
    
    expect(getResponse.status).toBe(200);
  });
});

Test Best Practices
Isolate tests: Use mocks and stubs to isolate components
Test both happy and error paths: Ensure errors are handled correctly
Use descriptive test names: Tests should document behavior
Avoid test interdependence: Tests shouldn't depend on each other
Keep tests fast: Slow tests discourage running them frequently
CI/CD Integration
Tests are automatically run in the CI pipeline on every pull request.

Unit tests must pass before PR review
Integration tests must pass before merge
End-to-end tests run nightly on the staging environment

Mocking External Services

We use Jest's mocking capabilities to mock Canvas and Discourse APIs:

jest.mock('../../src/api/canvasApi');
jest.mock('../../src/api/discourseApi');

This prevents tests from making actual API calls while still validating integration logic.

