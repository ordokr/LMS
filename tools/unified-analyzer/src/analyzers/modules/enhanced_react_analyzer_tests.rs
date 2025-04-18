#[cfg(test)]
mod tests {
    use super::super::enhanced_react_analyzer::EnhancedReactAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_react_component() {
        let temp_dir = tempdir().unwrap();
        
        // Create a React component file
        let component_path = temp_dir.path().join("Button.jsx");
        let component_content = r#"
import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';

/**
 * Button component for user interactions
 */
const Button = ({ 
    children, 
    variant = 'primary', 
    size = 'medium', 
    disabled = false, 
    onClick 
}) => {
    const [isHovered, setIsHovered] = useState(false);
    const [isPressed, setIsPressed] = useState(false);
    
    useEffect(() => {
        if (disabled) {
            setIsHovered(false);
            setIsPressed(false);
        }
    }, [disabled]);
    
    const handleMouseEnter = () => {
        if (!disabled) {
            setIsHovered(true);
        }
    };
    
    const handleMouseLeave = () => {
        setIsHovered(false);
        setIsPressed(false);
    };
    
    const handleMouseDown = () => {
        if (!disabled) {
            setIsPressed(true);
        }
    };
    
    const handleMouseUp = () => {
        setIsPressed(false);
    };
    
    const handleClick = (event) => {
        if (!disabled && onClick) {
            onClick(event);
        }
    };
    
    const buttonClasses = classNames(
        'button',
        `button--${variant}`,
        `button--${size}`,
        {
            'button--disabled': disabled,
            'button--hovered': isHovered,
            'button--pressed': isPressed
        }
    );
    
    return (
        <button
            className={buttonClasses}
            disabled={disabled}
            onClick={handleClick}
            onMouseEnter={handleMouseEnter}
            onMouseLeave={handleMouseLeave}
            onMouseDown={handleMouseDown}
            onMouseUp={handleMouseUp}
        >
            {children}
        </button>
    );
};

Button.propTypes = {
    children: PropTypes.node.isRequired,
    variant: PropTypes.oneOf(['primary', 'secondary', 'tertiary']),
    size: PropTypes.oneOf(['small', 'medium', 'large']),
    disabled: PropTypes.bool,
    onClick: PropTypes.func
};

export default Button;
        "#;
        fs::write(&component_path, component_content).unwrap();
        
        let mut analyzer = EnhancedReactAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        // Check that we found the component
        assert_eq!(analyzer.components.len(), 1);
        
        // Get the component
        let component = analyzer.components.values().next().unwrap();
        
        // Check component properties
        assert_eq!(component.name, "Button");
        assert_eq!(component.component_type, "functional");
        
        // Check props
        assert_eq!(component.props.len(), 5);
        assert!(component.props.iter().any(|prop| prop.name == "children" && prop.required));
        assert!(component.props.iter().any(|prop| prop.name == "variant"));
        assert!(component.props.iter().any(|prop| prop.name == "size"));
        assert!(component.props.iter().any(|prop| prop.name == "disabled"));
        assert!(component.props.iter().any(|prop| prop.name == "onClick"));
        
        // Check state
        assert_eq!(component.state.len(), 2);
        assert!(component.state.iter().any(|state| state.name == "isHovered"));
        assert!(component.state.iter().any(|state| state.name == "isPressed"));
        
        // Check effects
        assert_eq!(component.effects.len(), 1);
        assert!(component.effects[0].dependencies.contains(&"disabled".to_string()));
        
        // Check handlers
        assert_eq!(component.handlers.len(), 5);
        assert!(component.handlers.iter().any(|handler| handler.name == "handleMouseEnter"));
        assert!(component.handlers.iter().any(|handler| handler.name == "handleMouseLeave"));
        assert!(component.handlers.iter().any(|handler| handler.name == "handleMouseDown"));
        assert!(component.handlers.iter().any(|handler| handler.name == "handleMouseUp"));
        assert!(component.handlers.iter().any(|handler| handler.name == "handleClick"));
        
        // Check hooks used
        assert!(component.hooks_used.contains(&"useState".to_string()));
        assert!(component.hooks_used.contains(&"useEffect".to_string()));
        
        // Check imports
        assert!(component.imports.values().any(|import| import == "react"));
        assert!(component.imports.values().any(|import| import == "prop-types"));
        assert!(component.imports.values().any(|import| import == "classnames"));
        
        temp_dir.close().unwrap();
    }
}
