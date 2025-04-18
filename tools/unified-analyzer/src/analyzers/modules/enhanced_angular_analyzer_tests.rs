#[cfg(test)]
mod tests {
    use super::super::enhanced_angular_analyzer::EnhancedAngularAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_angular_component() {
        let temp_dir = tempdir().unwrap();
        
        // Create directory structure
        let components_dir = temp_dir.path().join("src").join("app").join("components");
        fs::create_dir_all(&components_dir).unwrap();
        
        // Create an Angular component file
        let component_path = components_dir.join("button.component.ts");
        let component_content = r#"
import { Component, Input, Output, EventEmitter, OnInit, OnDestroy } from '@angular/core';
import { ButtonService } from '../../services/button.service';

@Component({
  selector: 'app-button',
  templateUrl: './button.component.html',
  styleUrls: ['./button.component.scss']
})
export class ButtonComponent implements OnInit, OnDestroy {
  @Input() variant: 'primary' | 'secondary' | 'tertiary' = 'primary';
  @Input() size: 'small' | 'medium' | 'large' = 'medium';
  @Input() disabled = false;
  @Input('buttonLabel') label: string;
  
  @Output() clicked = new EventEmitter<MouseEvent>();
  
  private isHovered = false;
  private isPressed = false;
  
  constructor(
    private buttonService: ButtonService
  ) {}
  
  ngOnInit(): void {
    console.log('Button component initialized');
    this.buttonService.registerButton(this);
  }
  
  ngOnDestroy(): void {
    console.log('Button component destroyed');
    this.buttonService.unregisterButton(this);
  }
  
  get buttonClasses(): string[] {
    return [
      'button',
      `button--${this.variant}`,
      `button--${this.size}`,
      this.disabled ? 'button--disabled' : '',
      this.isHovered ? 'button--hovered' : '',
      this.isPressed ? 'button--pressed' : ''
    ].filter(Boolean);
  }
  
  handleClick(event: MouseEvent): void {
    if (!this.disabled) {
      this.clicked.emit(event);
    }
  }
  
  handleMouseEnter(): void {
    if (!this.disabled) {
      this.isHovered = true;
    }
  }
  
  handleMouseLeave(): void {
    this.isHovered = false;
    this.isPressed = false;
  }
}
        "#;
        fs::write(&component_path, component_content).unwrap();
        
        // Create a template file
        let template_path = components_dir.join("button.component.html");
        let template_content = r#"
<button 
  [ngClass]="buttonClasses"
  [disabled]="disabled"
  (click)="handleClick($event)"
  (mouseenter)="handleMouseEnter()"
  (mouseleave)="handleMouseLeave()"
>
  <ng-content *ngIf="!label"></ng-content>
  <span *ngIf="label">{{ label | uppercase }}</span>
</button>
        "#;
        fs::write(&template_path, template_content).unwrap();
        
        // Create a style file
        let style_path = components_dir.join("button.component.scss");
        let style_content = r#"
.button {
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  
  &--disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  &--primary {
    background-color: #3498db;
    color: white;
  }
  
  &--secondary {
    background-color: #ecf0f1;
    color: #2c3e50;
  }
  
  &--tertiary {
    background-color: transparent;
    color: #3498db;
  }
  
  &--small {
    padding: 4px 8px;
    font-size: 12px;
  }
  
  &--medium {
    padding: 8px 16px;
    font-size: 14px;
  }
  
  &--large {
    padding: 12px 24px;
    font-size: 16px;
  }
}
        "#;
        fs::write(&style_path, style_content).unwrap();
        
        let mut analyzer = EnhancedAngularAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        // Check that we found the component
        assert_eq!(analyzer.components.len(), 1);
        
        // Get the component
        let component = analyzer.components.values().next().unwrap();
        
        // Check component properties
        assert_eq!(component.name, "ButtonComponent");
        assert_eq!(component.selector, "app-button");
        
        // Check inputs
        assert_eq!(component.inputs.len(), 4);
        assert!(component.inputs.iter().any(|input| input.name == "variant"));
        assert!(component.inputs.iter().any(|input| input.name == "size"));
        assert!(component.inputs.iter().any(|input| input.name == "disabled"));
        assert!(component.inputs.iter().any(|input| input.name == "label" && 
                                           input.decorator_options.get("alias") == Some(&"buttonLabel".to_string())));
        
        // Check outputs
        assert_eq!(component.outputs.len(), 1);
        assert!(component.outputs.iter().any(|output| output.name == "clicked" && 
                                            output.event_type == "MouseEvent"));
        
        // Check properties
        assert!(component.properties.iter().any(|prop| prop.name == "isHovered" && prop.is_private));
        assert!(component.properties.iter().any(|prop| prop.name == "isPressed" && prop.is_private));
        
        // Check methods
        assert!(component.methods.iter().any(|method| method.name == "buttonClasses"));
        assert!(component.methods.iter().any(|method| method.name == "handleClick"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseEnter"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseLeave"));
        
        // Check lifecycle hooks
        assert_eq!(component.lifecycle_hooks.len(), 2);
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "ngOnInit"));
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "ngOnDestroy"));
        
        // Check dependencies
        assert_eq!(component.dependencies.len(), 1);
        assert!(component.dependencies.iter().any(|dep| dep.name == "buttonService" && 
                                                dep.dependency_type == "ButtonService"));
        
        // Check template dependencies
        assert!(component.directives.contains(&"ngClass".to_string()));
        assert!(component.directives.contains(&"ngIf".to_string()));
        assert!(component.pipes.contains(&"uppercase".to_string()));
        
        temp_dir.close().unwrap();
    }
}
