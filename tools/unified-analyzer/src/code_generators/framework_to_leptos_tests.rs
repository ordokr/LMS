#[cfg(test)]
mod tests {
    use crate::analyzers::modules::enhanced_vue_analyzer::{EnhancedVueAnalyzer, VueComponent};
    use crate::analyzers::modules::enhanced_angular_analyzer::{EnhancedAngularAnalyzer, AngularComponent};
    use crate::code_generators::{VueToLeptosGenerator, AngularToLeptosGenerator};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_vue_to_leptos_generator() {
        let temp_dir = tempdir().unwrap();
        
        // Create a Vue component
        let mut component = VueComponent {
            name: "ButtonComponent".to_string(),
            file_path: "components/Button.vue".to_string(),
            component_type: "options_api".to_string(),
            ..Default::default()
        };
        
        // Add props
        component.props.push(crate::analyzers::modules::enhanced_vue_analyzer::VueProp {
            name: "variant".to_string(),
            prop_type: "String".to_string(),
            required: false,
            default_value: Some("'primary'".to_string()),
            validator: None,
        });
        
        component.props.push(crate::analyzers::modules::enhanced_vue_analyzer::VueProp {
            name: "disabled".to_string(),
            prop_type: "Boolean".to_string(),
            required: false,
            default_value: Some("false".to_string()),
            validator: None,
        });
        
        // Add data
        component.data.push(crate::analyzers::modules::enhanced_vue_analyzer::VueData {
            name: "isHovered".to_string(),
            data_type: "boolean".to_string(),
            initial_value: Some("false".to_string()),
        });
        
        // Add methods
        component.methods.push(crate::analyzers::modules::enhanced_vue_analyzer::VueMethod {
            name: "handleClick".to_string(),
            parameters: vec!["event".to_string()],
            code: "if (!this.disabled) { this.$emit('click', event); }".to_string(),
        });
        
        // Add template
        component.template_content = Some(r#"
<button 
  :class="['button', `button--${variant}`, { 'button--disabled': disabled }]"
  :disabled="disabled"
  @click="handleClick"
>
  <slot></slot>
</button>
        "#.to_string());
        
        // Create output directory
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&output_dir).unwrap();
        
        // Generate Leptos component
        let generator = VueToLeptosGenerator::new(&output_dir);
        generator.generate_component(&component).unwrap();
        
        // Check that the file was created
        let output_file = output_dir.join("components").join("button_component.rs");
        assert!(output_file.exists());
        
        // Check file content
        let content = fs::read_to_string(output_file).unwrap();
        assert!(content.contains("#[component]"));
        assert!(content.contains("pub fn ButtonComponent"));
        assert!(content.contains("pub variant: Option<String>"));
        assert!(content.contains("pub disabled: Option<bool>"));
        assert!(content.contains("let (isHovered, set_isHovered) = create_signal"));
        assert!(content.contains("let handleClick = move |event|"));
        
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_angular_to_leptos_generator() {
        let temp_dir = tempdir().unwrap();
        
        // Create an Angular component
        let mut component = AngularComponent {
            name: "ButtonComponent".to_string(),
            file_path: "src/app/components/button/button.component.ts".to_string(),
            selector: "app-button".to_string(),
            ..Default::default()
        };
        
        // Add inputs
        component.inputs.push(crate::analyzers::modules::enhanced_angular_analyzer::AngularInput {
            name: "variant".to_string(),
            input_type: "string".to_string(),
            required: false,
            default_value: Some("'primary'".to_string()),
            decorator_options: Default::default(),
        });
        
        component.inputs.push(crate::analyzers::modules::enhanced_angular_analyzer::AngularInput {
            name: "disabled".to_string(),
            input_type: "boolean".to_string(),
            required: false,
            default_value: Some("false".to_string()),
            decorator_options: Default::default(),
        });
        
        // Add outputs
        component.outputs.push(crate::analyzers::modules::enhanced_angular_analyzer::AngularOutput {
            name: "clicked".to_string(),
            event_type: "MouseEvent".to_string(),
            decorator_options: Default::default(),
        });
        
        // Add properties
        component.properties.push(crate::analyzers::modules::enhanced_angular_analyzer::AngularProperty {
            name: "isHovered".to_string(),
            property_type: "boolean".to_string(),
            initial_value: Some("false".to_string()),
            is_private: true,
            is_readonly: false,
        });
        
        // Add methods
        component.methods.push(crate::analyzers::modules::enhanced_angular_analyzer::AngularMethod {
            name: "handleClick".to_string(),
            parameters: vec!["event: MouseEvent".to_string()],
            return_type: Some("void".to_string()),
            code: "if (!this.disabled) { this.clicked.emit(event); }".to_string(),
            is_private: false,
        });
        
        // Add template
        component.template_content = Some(r#"
<button 
  [ngClass]="['button', 'button--' + variant, { 'button--disabled': disabled }]"
  [disabled]="disabled"
  (click)="handleClick($event)"
>
  <ng-content></ng-content>
</button>
        "#.to_string());
        
        // Create output directory
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&output_dir).unwrap();
        
        // Generate Leptos component
        let generator = AngularToLeptosGenerator::new(&output_dir);
        generator.generate_component(&component).unwrap();
        
        // Check that the file was created
        let output_file = output_dir.join("components").join("button_component.rs");
        assert!(output_file.exists());
        
        // Check file content
        let content = fs::read_to_string(output_file).unwrap();
        assert!(content.contains("#[component]"));
        assert!(content.contains("pub fn ButtonComponent"));
        assert!(content.contains("pub variant: Option<String>"));
        assert!(content.contains("pub disabled: Option<bool>"));
        assert!(content.contains("let (on_clicked, set_on_clicked) = create_signal"));
        assert!(content.contains("let handleClick = move |event"));
        
        temp_dir.close().unwrap();
    }
}
