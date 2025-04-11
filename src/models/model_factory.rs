// Auto-generated from ModelFactory.js
// Source: src/models/ModelFactory.js

use serde::{Deserialize, Serialize};
use thiserror::Error;

// Import unified models
use crate::models::canvas::user::User;
use crate::models::canvas::course::Course;
use crate::models::canvas::discussion::Discussion;
use crate::models::canvas::assignment::Assignment;

/// Errors that can occur when using the ModelFactory
#[derive(Error, Debug)]
pub enum ModelFactoryError {
    #[error("Data is required to create a model")]
    MissingData,
    
    #[error("Source must be either \"canvas\" or \"discourse\"")]
    InvalidSource,
    
    #[error("Unsupported model type: {0}")]
    UnsupportedModelType(String),
    
    #[error("Failed to convert model: {0}")]
    ConversionError(String),
}

/// Supported source systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceSystem {
    Canvas,
    Discourse,
}

impl SourceSystem {
    pub fn from_str(source: &str) -> Result<Self, ModelFactoryError> {
        match source.to_lowercase().as_str() {
            "canvas" => Ok(SourceSystem::Canvas),
            "discourse" => Ok(SourceSystem::Discourse),
            _ => Err(ModelFactoryError::InvalidSource),
        }
    }
}

/// Factory for creating and converting between unified models
pub struct ModelFactory;

impl ModelFactory {
    /// Create a unified model from source system data
    ///
    /// # Arguments
    /// * `model_type` - Type of model to create (user, course, discussion, assignment)
    /// * `data` - Data from source system
    /// * `source` - Source system (canvas or discourse)
    ///
    /// # Returns
    /// Unified model instance
    pub fn create<T: Serialize + for<'de> Deserialize<'de>>(
        model_type: &str, 
        data: Option<T>, 
        source: &str
    ) -> Result<Box<dyn std::any::Any>, ModelFactoryError> {
        // Validate inputs
        let data = match data {
            Some(d) => d,
            None => return Err(ModelFactoryError::MissingData),
        };
        
        let source = SourceSystem::from_str(source)?;
        
        // Create the appropriate model
        match model_type.to_lowercase().as_str() {
            "user" => {
                let user = match source {
                    SourceSystem::Canvas => User::from_canvas_user(&data),
                    SourceSystem::Discourse => User::from_discourse_user(&data),
                };
                match user {
                    Ok(u) => Ok(Box::new(u)),
                    Err(e) => Err(ModelFactoryError::ConversionError(e.to_string())),
                }
            },
            
            "course" => {
                let course = match source {
                    SourceSystem::Canvas => Course::from_canvas_course(&data),
                    SourceSystem::Discourse => Course::from_discourse_category(&data),
                };
                match course {
                    Ok(c) => Ok(Box::new(c)),
                    Err(e) => Err(ModelFactoryError::ConversionError(e.to_string())),
                }
            },
            
            "discussion" => {
                let discussion = match source {
                    SourceSystem::Canvas => Discussion::from_canvas_discussion(&data),
                    SourceSystem::Discourse => Discussion::from_discourse_topic(&data),
                };
                match discussion {
                    Ok(d) => Ok(Box::new(d)),
                    Err(e) => Err(ModelFactoryError::ConversionError(e.to_string())),
                }
            },
            
            "assignment" => {
                let assignment = match source {
                    SourceSystem::Canvas => Assignment::from_canvas_assignment(&data),
                    SourceSystem::Discourse => Assignment::from_discourse_topic(&data),
                };
                match assignment {
                    Ok(a) => Ok(Box::new(a)),
                    Err(e) => Err(ModelFactoryError::ConversionError(e.to_string())),
                }
            },
            
            _ => Err(ModelFactoryError::UnsupportedModelType(model_type.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_source_system_from_str() {
        assert_eq!(SourceSystem::from_str("canvas").unwrap(), SourceSystem::Canvas);
        assert_eq!(SourceSystem::from_str("Canvas").unwrap(), SourceSystem::Canvas);
        assert_eq!(SourceSystem::from_str("CANVAS").unwrap(), SourceSystem::Canvas);
        assert_eq!(SourceSystem::from_str("discourse").unwrap(), SourceSystem::Discourse);
        assert!(SourceSystem::from_str("invalid").is_err());
    }
}
