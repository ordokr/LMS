import { User, Course, Discussion, Assignment } from './unifiedModels';

/**
 * Factory class for creating and converting between unified models
 */
class ModelFactory {
  /**
   * Create a unified model from source system data
   * @param {string} modelType - Type of model to create (user, course, discussion, assignment)
   * @param {Object} data - Data from source system
   * @param {string} source - Source system (canvas or discourse)
   * @returns {Object} Unified model instance
   */
  static create(modelType, data, source) {
    modelType = modelType.toLowerCase();
    source = source.toLowerCase();
    
    // Validate inputs
    if (!data) {
      throw new Error('Data is required to create a model');
    }
    
    if (!['canvas', 'discourse'].includes(source)) {
      throw new Error('Source must be either "canvas" or "discourse"');
    }
    
    // Create the appropriate model
    switch (modelType) {
      case 'user':
        return source === 'canvas' 
          ? User.fromCanvasUser(data) 
          : User.fromDiscourseUser(data);
        
      case 'course':
        return source === 'canvas' 
          ? Course.fromCanvasCourse(data) 
          : Course.fromDiscourseCategory(data);
        
      case 'discussion':
        return source === 'canvas' 
          ? Discussion.fromCanvasDiscussion(data) 
          : Discussion.fromDiscourseTopic(data);
        
      case 'assignment':
        return source === 'canvas' 
          ? Assignment.fromCanvasAssignment(data) 
          : Assignment.fromDiscourseTopic(data);
        
      default:
        throw new Error(`Unsupported model type: ${modelType}`);
    }
  }
  
  /**
   * Convert a unified model to a source system format
   * @param {Object} model - Unified model instance
   * @param {string} target - Target system (canvas or discourse)
   * @returns {Object} Data in target system format
   */
  static convertToSource(model, target) {
    target = target.toLowerCase();
    
    // Validate inputs
    if (!model) {
      throw new Error('Model is required for conversion');
    }
    
    if (!['canvas', 'discourse'].includes(target)) {
      throw new Error('Target must be either "canvas" or "discourse"');
    }
    
    // Convert to the appropriate format
    if (model instanceof User) {
      return target === 'canvas' ? model.toCanvasUser() : model.toDiscourseUser();
    }
    
    if (model instanceof Course) {
      return target === 'canvas' ? model.toCanvasCourse() : model.toDiscourseCategory();
    }
    
    if (model instanceof Discussion) {
      return target === 'canvas' ? model.toCanvasDiscussion() : model.toDiscourseTopic();
    }
    
    if (model instanceof Assignment) {
      return target === 'canvas' ? model.toCanvasAssignment() : model.toDiscourseCustomFields();
    }
    
    throw new Error('Unsupported model type for conversion');
  }
}

export default ModelFactory;