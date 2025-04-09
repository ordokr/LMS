/**
 * Standardized naming conventions for the integrated system
 * This addresses the naming inconsistency issue
 */

/**
 * Convert camelCase to snake_case (Canvas style)
 * @param {string} str - String to convert
 * @returns {string} Converted string
 */
export function toSnakeCase(str) {
  return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

/**
 * Convert snake_case to camelCase (Discourse/JS style)
 * @param {string} str - String to convert
 * @returns {string} Converted string
 */
export function toCamelCase(str) {
  return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

/**
 * Convert object keys based on naming convention
 * @param {Object} obj - Object to convert
 * @param {string} toStyle - Target style ('camel' or 'snake')
 * @returns {Object} Converted object
 */
export function convertObjectKeys(obj, toStyle = 'camel') {
  if (!obj || typeof obj !== 'object') return obj;
  
  const converter = toStyle === 'camel' ? toCamelCase : toSnakeCase;
  
  if (Array.isArray(obj)) {
    return obj.map(item => convertObjectKeys(item, toStyle));
  }
  
  return Object.entries(obj).reduce((result, [key, value]) => {
    const newKey = converter(key);
    const newValue = (value && typeof value === 'object') 
      ? convertObjectKeys(value, toStyle) 
      : value;
    
    result[newKey] = newValue;
    return result;
  }, {});
}

/**
 * Apply consistent naming to a model instance
 * @param {Object} model - Model to standardize
 * @param {string} modelName - Name of the model type
 * @returns {Object} Standardized model
 */
export function standardizeModel(model, modelName) {
  // Define standard field names for common concepts
  const standardFields = {
    user: {
      id: 'id',
      email: 'email',
      name: 'name',
      username: 'username',
      created_at: 'createdAt',
      updated_at: 'updatedAt'
    },
    course: {
      id: 'id',
      title: 'title',
      description: 'description',
      start_date: 'startDate',
      end_date: 'endDate'
    },
    discussion: {
      id: 'id',
      title: 'title',
      body: 'body',
      created_at: 'createdAt',
      updated_at: 'updatedAt'
    },
    assignment: {
      id: 'id',
      title: 'title',
      description: 'description',
      due_date: 'dueDate',
      points_possible: 'pointsPossible'
    },
    submission: {
      id: 'id',
      user_id: 'userId',
      assignment_id: 'assignmentId',
      submitted_at: 'submittedAt',
      score: 'score'
    },
    attachment: {
      id: 'id',
      filename: 'filename',
      content_type: 'contentType',
      size: 'size',
      url: 'url'
    }
  };
  
  const fieldMap = standardFields[modelName];
  if (!fieldMap) return model;
  
  // Create a standardized version
  const standardized = {};
  
  // Map known fields
  Object.entries(fieldMap).forEach(([originalField, standardField]) => {
    if (model[originalField] !== undefined) {
      standardized[standardField] = model[originalField];
    } else if (model[standardField] !== undefined) {
      standardized[standardField] = model[standardField];
    }
  });
  
  // Copy any other fields
  Object.entries(model).forEach(([key, value]) => {
    if (standardized[key] === undefined) {
      standardized[key] = value;
    }
  });
  
  return standardized;
}