use std::io::{Error, ErrorKind};
use unified_analyzer::generators::error::GeneratorError;

#[test]
fn test_generator_error_display() {
    // Test DirectoryCreation error
    let error = GeneratorError::DirectoryCreation("Failed to create directory".to_string());
    assert_eq!(error.to_string(), "Directory creation error: Failed to create directory");
    
    // Test FileWrite error
    let error = GeneratorError::FileWrite("Failed to write file".to_string());
    assert_eq!(error.to_string(), "File write error: Failed to write file");
    
    // Test FileRead error
    let error = GeneratorError::FileRead("Failed to read file".to_string());
    assert_eq!(error.to_string(), "File read error: Failed to read file");
    
    // Test DataParsing error
    let error = GeneratorError::DataParsing("Failed to parse data".to_string());
    assert_eq!(error.to_string(), "Data parsing error: Failed to parse data");
    
    // Test ContentGeneration error
    let error = GeneratorError::ContentGeneration("Failed to generate content".to_string());
    assert_eq!(error.to_string(), "Content generation error: Failed to generate content");
    
    // Test Other error
    let error = GeneratorError::Other("Some other error".to_string());
    assert_eq!(error.to_string(), "Other error: Some other error");
}

#[test]
fn test_generator_error_from_io_error() {
    // Test NotFound error
    let io_error = Error::new(ErrorKind::NotFound, "File not found");
    let generator_error = GeneratorError::from(io_error);
    assert!(matches!(generator_error, GeneratorError::FileRead(_)));
    
    // Test PermissionDenied error
    let io_error = Error::new(ErrorKind::PermissionDenied, "Permission denied");
    let generator_error = GeneratorError::from(io_error);
    assert!(matches!(generator_error, GeneratorError::FileWrite(_)));
    
    // Test other error
    let io_error = Error::new(ErrorKind::Other, "Some other error");
    let generator_error = GeneratorError::from(io_error);
    assert!(matches!(generator_error, GeneratorError::Other(_)));
}
