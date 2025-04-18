#[cfg(test)]
mod tests {
    use super::super::enhanced_ember_analyzer::EnhancedEmberAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_ember_component() {
        let temp_dir = tempdir().unwrap();
        
        // Create directory structure
        let components_dir = temp_dir.path().join("app").join("components");
        fs::create_dir_all(&components_dir).unwrap();
        
        // Create a classic Ember component file
        let component_path = components_dir.join("button.js");
        let component_content = r#"
import Component from '@ember/component';
import { computed } from '@ember/object';
import { inject as service } from '@ember/service';

export default Component.extend({
  tagName: 'button',
  classNames: ['btn'],
  classNameBindings: ['variantClass', 'sizeClass', 'isDisabled:disabled'],
  
  // Properties
  variant: 'primary',
  size: 'medium',
  isDisabled: false,
  
  // Services
  i18n: service(),
  
  // Computed properties
  variantClass: computed('variant', function() {
    return `btn-${this.variant}`;
  }),
  
  sizeClass: computed('size', function() {
    return `btn-${this.size}`;
  }),
  
  // Lifecycle hooks
  init() {
    this._super(...arguments);
    this.set('clickCount', 0);
  },
  
  didInsertElement() {
    this._super(...arguments);
    this.element.focus();
  },
  
  willDestroy() {
    this._super(...arguments);
    // Clean up any event listeners
  },
  
  // Actions
  actions: {
    click() {
      if (!this.isDisabled) {
        this.incrementProperty('clickCount');
        if (this.onClick) {
          this.onClick();
        }
      }
    },
    
    mouseEnter() {
      this.set('isHovered', true);
    },
    
    mouseLeave() {
      this.set('isHovered', false);
    }
  }
});
        "#;
        fs::write(&component_path, component_content).unwrap();
        
        // Create a template file
        let templates_dir = temp_dir.path().join("app").join("templates").join("components");
        fs::create_dir_all(&templates_dir).unwrap();
        
        let template_path = templates_dir.join("button.hbs");
        let template_content = r#"
{{#if hasBlock}}
  {{yield}}
{{else}}
  {{label}}
{{/if}}
        "#;
        fs::write(&template_path, template_content).unwrap();
        
        // Create a Glimmer component file
        let glimmer_component_path = components_dir.join("g-button.js");
        let glimmer_component_content = r#"
import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';
import { inject as service } from '@ember/service';

export default class GButtonComponent extends Component {
  @service i18n;
  
  @tracked isHovered = false;
  @tracked clickCount = 0;
  
  get variant() {
    return this.args.variant || 'primary';
  }
  
  get size() {
    return this.args.size || 'medium';
  }
  
  get variantClass() {
    return `btn-${this.variant}`;
  }
  
  get sizeClass() {
    return `btn-${this.size}`;
  }
  
  get isDisabled() {
    return this.args.disabled || false;
  }
  
  constructor() {
    super(...arguments);
    // Initialization code
  }
  
  willDestroy() {
    super.willDestroy();
    // Clean up code
  }
  
  @action
  click() {
    if (!this.isDisabled) {
      this.clickCount++;
      if (this.args.onClick) {
        this.args.onClick();
      }
    }
  }
  
  @action
  mouseEnter() {
    this.isHovered = true;
  }
  
  @action
  mouseLeave() {
    this.isHovered = false;
  }
}
        "#;
        fs::write(&glimmer_component_path, glimmer_component_content).unwrap();
        
        // Create a Glimmer component template file
        let glimmer_template_path = components_dir.join("g-button.hbs");
        let glimmer_template_content = r#"
<button 
  class="btn {{this.variantClass}} {{this.sizeClass}} {{if this.isDisabled 'disabled'}}"
  disabled={{this.isDisabled}}
  {{on "click" this.click}}
  {{on "mouseenter" this.mouseEnter}}
  {{on "mouseleave" this.mouseLeave}}
>
  {{#if (has-block)}}
    {{yield}}
  {{else}}
    {{@label}}
  {{/if}}
</button>
        "#;
        fs::write(&glimmer_template_path, glimmer_template_content).unwrap();
        
        let mut analyzer = EnhancedEmberAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        // Check that we found both components
        assert_eq!(analyzer.components.len(), 2);
        
        // Check classic component
        let classic_component = analyzer.components.values().find(|c| c.name == "button").unwrap();
        assert_eq!(classic_component.component_type, "classic");
        
        // Check classic component properties
        assert!(classic_component.properties.iter().any(|p| p.name == "variant" && p.default_value == Some("'primary'".to_string())));
        assert!(classic_component.properties.iter().any(|p| p.name == "size" && p.default_value == Some("'medium'".to_string())));
        assert!(classic_component.properties.iter().any(|p| p.name == "isDisabled" && p.default_value == Some("false".to_string())));
        
        // Check classic component computed properties
        assert_eq!(classic_component.computed_properties.len(), 2);
        assert!(classic_component.computed_properties.iter().any(|cp| cp.name == "variantClass" && cp.dependencies.contains(&"variant".to_string())));
        assert!(classic_component.computed_properties.iter().any(|cp| cp.name == "sizeClass" && cp.dependencies.contains(&"size".to_string())));
        
        // Check classic component services
        assert_eq!(classic_component.services.len(), 1);
        assert!(classic_component.services.iter().any(|s| s.name == "i18n"));
        
        // Check classic component actions
        assert_eq!(classic_component.actions.len(), 3);
        assert!(classic_component.actions.iter().any(|a| a.name == "click"));
        assert!(classic_component.actions.iter().any(|a| a.name == "mouseEnter"));
        assert!(classic_component.actions.iter().any(|a| a.name == "mouseLeave"));
        
        // Check classic component lifecycle hooks
        assert_eq!(classic_component.lifecycle_hooks.len(), 3);
        assert!(classic_component.lifecycle_hooks.iter().any(|h| h.name == "init"));
        assert!(classic_component.lifecycle_hooks.iter().any(|h| h.name == "didInsertElement"));
        assert!(classic_component.lifecycle_hooks.iter().any(|h| h.name == "willDestroy"));
        
        // Check Glimmer component
        let glimmer_component = analyzer.components.values().find(|c| c.name == "g-button").unwrap();
        assert_eq!(glimmer_component.component_type, "glimmer");
        
        // Check Glimmer component properties
        assert!(glimmer_component.properties.iter().any(|p| p.name == "isHovered" && p.is_tracked));
        assert!(glimmer_component.properties.iter().any(|p| p.name == "clickCount" && p.is_tracked));
        assert!(glimmer_component.properties.iter().any(|p| p.name == "variant" && p.property_type == "argument"));
        assert!(glimmer_component.properties.iter().any(|p| p.name == "size" && p.property_type == "argument"));
        
        // Check Glimmer component computed properties
        assert!(glimmer_component.computed_properties.iter().any(|cp| cp.name == "variantClass"));
        assert!(glimmer_component.computed_properties.iter().any(|cp| cp.name == "sizeClass"));
        
        // Check Glimmer component services
        assert_eq!(glimmer_component.services.len(), 1);
        assert!(glimmer_component.services.iter().any(|s| s.name == "i18n"));
        
        // Check Glimmer component actions
        assert_eq!(glimmer_component.actions.len(), 3);
        assert!(glimmer_component.actions.iter().any(|a| a.name == "click"));
        assert!(glimmer_component.actions.iter().any(|a| a.name == "mouseEnter"));
        assert!(glimmer_component.actions.iter().any(|a| a.name == "mouseLeave"));
        
        // Check Glimmer component lifecycle hooks
        assert_eq!(glimmer_component.lifecycle_hooks.len(), 2);
        assert!(glimmer_component.lifecycle_hooks.iter().any(|h| h.name == "constructor"));
        assert!(glimmer_component.lifecycle_hooks.iter().any(|h| h.name == "willDestroy"));
        
        temp_dir.close().unwrap();
    }
}
