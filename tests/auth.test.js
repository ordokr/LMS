const { generateJwt, verifyJwt } = require('../src/services/auth');

test('JWT generation and verification', () => {
  const payload = { userId: 123, role: 'teacher' };
  const token = generateJwt(payload);
  const decoded = verifyJwt(token);
  
  expect(decoded.userId).toBe(payload.userId);
  expect(decoded.role).toBe(payload.role);
});