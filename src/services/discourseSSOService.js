import crypto from 'crypto';
import { config } from '../config';

/**
 * Generate Discourse SSO payload for a user
 * @param {Object} user - User object
 * @param {string} sso - Base64 encoded SSO payload from Discourse
 * @param {string} sig - Signature from Discourse
 * @returns {string} URL-encoded payload for Discourse SSO
 */
export function generateDiscourseSSOPayload(user, sso, sig) {
  // Verify the signature from Discourse
  const hmac = crypto.createHmac('sha256', config.DISCOURSE_SSO_SECRET);
  hmac.update(sso);
  const computedSig = hmac.digest('hex');
  
  if (computedSig !== sig) {
    throw new Error('Invalid Discourse SSO signature');
  }
  
  // Decode the payload
  const payload = Buffer.from(sso, 'base64').toString('utf-8');
  const nonce = new URLSearchParams(payload).get('nonce');
  
  // Prepare response payload
  const responseParams = {
    nonce: nonce,
    external_id: user.id.toString(),
    email: user.email,
    username: user.name.replace(/\s+/g, '_').toLowerCase(),
    name: user.name,
    // Additional optional parameters
    groups: user.roles.join(',')
  };
  
  // Encode the response
  const returnPayload = new URLSearchParams(responseParams).toString();
  const base64Payload = Buffer.from(returnPayload).toString('base64');
  
  // Generate signature for the response
  const returnHmac = crypto.createHmac('sha256', config.DISCOURSE_SSO_SECRET);
  returnHmac.update(base64Payload);
  const returnSig = returnHmac.digest('hex');
  
  return `sso=${encodeURIComponent(base64Payload)}&sig=${returnSig}`;
}