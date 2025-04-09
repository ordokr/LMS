import jwt from 'jsonwebtoken';
import { config } from '../config';

/**
 * Generates a JWT token for authenticated users
 * @param {Object} user - User object with authentication details
 * @returns {string} JWT token
 */
export function generateJwtToken(user) {
  const payload = {
    id: user.id,
    email: user.email,
    roles: user.roles,
    // Include any other necessary user information
    // but be careful not to include sensitive data
    exp: Math.floor(Date.now() / 1000) + (60 * 60 * 24) // 24 hour expiration
  };
  
  // Sign the token with your secret key
  return jwt.sign(payload, config.JWT_SECRET);
}

/**
 * Verify and decode a JWT token
 * @param {string} token - JWT token to verify
 * @returns {Object|null} Decoded token or null if invalid
 */
export function verifyJwtToken(token) {
  try {
    return jwt.verify(token, config.JWT_SECRET);
  } catch (error) {
    console.error('JWT verification error:', error);
    return null;
  }
}