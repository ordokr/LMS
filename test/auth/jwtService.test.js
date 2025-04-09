import { generateJwtToken, verifyJwtToken } from '../../src/auth/jwtService';
import jwt from 'jsonwebtoken';
import { config } from '../../src/config';

// Mock jwt library
jest.mock('jsonwebtoken');

describe('JWT Service', () => {
  const mockUser = {
    id: '123',
    email: 'test@example.com',
    roles: ['student']
  };
  
  beforeEach(() => {
    jest.clearAllMocks();
  });
  
  test('generateJwtToken should call jwt.sign with correct parameters', () => {
    // Mock implementation
    jwt.sign.mockReturnValue('mock-token');
    
    // Execute function
    const token = generateJwtToken(mockUser);
    
    // Assertions
    expect(jwt.sign).toHaveBeenCalledTimes(1);
    expect(jwt.sign.mock.calls[0][0]).toMatchObject({
      id: mockUser.id,
      email: mockUser.email,
      roles: mockUser.roles
    });
    expect(jwt.sign.mock.calls[0][1]).toBe(config.JWT_SECRET);
    expect(token).toBe('mock-token');
  });
  
  test('verifyJwtToken should return decoded token when valid', () => {
    // Mock implementation
    jwt.verify.mockReturnValue({ id: '123', email: 'test@example.com' });
    
    // Execute function
    const result = verifyJwtToken('valid-token');
    
    // Assertions
    expect(jwt.verify).toHaveBeenCalledWith('valid-token', config.JWT_SECRET);
    expect(result).toEqual({ id: '123', email: 'test@example.com' });
  });
  
  test('verifyJwtToken should return null when token is invalid', () => {
    // Mock implementation
    jwt.verify.mockImplementation(() => {
      throw new Error('Invalid token');
    });
    
    // Execute function
    const result = verifyJwtToken('invalid-token');
    
    // Assertions
    expect(jwt.verify).toHaveBeenCalledWith('invalid-token', config.JWT_SECRET);
    expect(result).toBeNull();
  });
});