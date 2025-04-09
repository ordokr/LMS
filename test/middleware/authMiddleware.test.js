import { requireAuth } from '../../src/middleware/authMiddleware';
import { verifyJwtToken } from '../../src/auth/jwtService';

// Mock the JWT service
jest.mock('../../src/auth/jwtService');

describe('Auth Middleware', () => {
  let req, res, next;
  
  beforeEach(() => {
    req = {
      headers: {}
    };
    res = {
      status: jest.fn().mockReturnThis(),
      json: jest.fn()
    };
    next = jest.fn();
  });
  
  test('should return 401 when no token is provided', () => {
    requireAuth(req, res, next);
    
    expect(res.status).toHaveBeenCalledWith(401);
    expect(res.json).toHaveBeenCalledWith({ error: 'Authentication required' });
    expect(next).not.toHaveBeenCalled();
  });
  
  test('should return 401 when token is invalid', () => {
    req.headers.authorization = 'Bearer invalid-token';
    verifyJwtToken.mockReturnValue(null);
    
    requireAuth(req, res, next);
    
    expect(verifyJwtToken).toHaveBeenCalledWith('invalid-token');
    expect(res.status).toHaveBeenCalledWith(401);
    expect(res.json).toHaveBeenCalledWith({ error: 'Invalid or expired token' });
    expect(next).not.toHaveBeenCalled();
  });
  
  test('should call next() when token is valid', () => {
    const user = { id: '123', email: 'test@example.com' };
    req.headers.authorization = 'Bearer valid-token';
    verifyJwtToken.mockReturnValue(user);
    
    requireAuth(req, res, next);
    
    expect(verifyJwtToken).toHaveBeenCalledWith('valid-token');
    expect(req.user).toEqual(user);
    expect(next).toHaveBeenCalled();
    expect(res.status).not.toHaveBeenCalled();
  });
});