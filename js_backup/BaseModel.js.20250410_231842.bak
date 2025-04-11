import { convertObjectKeys } from '../../utils/namingConventions';

/**
 * Base class for all unified models
 */
export default class BaseModel {
  constructor(data = {}) {
    this._originalSource = null;
    this._originalData = null;
    this.setData(data);
  }
  
  /**
   * Set data with source tracking
   */
  setData(data, source = null) {
    if (data) {
      this._originalData = { ...data };
      this._originalSource = source;
      
      // Convert snake_case to camelCase for JS
      const camelData = convertObjectKeys(data, 'camel');
      Object.assign(this, camelData);
    }
    return this;
  }
  
  /**
   * Convert model to snake_case for API
   */
  toApiFormat() {
    const plainObj = this.toObject();
    return convertObjectKeys(plainObj, 'snake');
  }
  
  /**
   * Convert model to plain object
   */
  toObject() {
    const obj = {};
    
    // Only include non-private properties
    Object.entries(this).forEach(([key, value]) => {
      if (!key.startsWith('_')) {
        obj[key] = value;
      }
    });
    
    return obj;
  }
  
  /**
   * Get the original source system
   */
  getSource() {
    return this._originalSource;
  }
}