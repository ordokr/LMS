#[cfg(test)]
mod tests {
    use super::super::enhanced_vue_analyzer::EnhancedVueAnalyzer;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_extract_options_api_component() {
        let temp_dir = tempdir().unwrap();
        
        // Create a Vue component file
        let component_path = temp_dir.path().join("Button.vue");
        let component_content = r#"
<template>
  <button 
    :class="buttonClasses"
    :disabled="disabled"
    @click="handleClick"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
  >
    <slot></slot>
  </button>
</template>

<script>
export default {
  name: 'Button',
  
  props: {
    variant: {
      type: String,
      default: 'primary',
      validator: function(value) {
        return ['primary', 'secondary', 'tertiary'].indexOf(value) !== -1
      }
    },
    size: {
      type: String,
      default: 'medium'
    },
    disabled: {
      type: Boolean,
      default: false
    }
  },
  
  data() {
    return {
      isHovered: false,
      isPressed: false
    }
  },
  
  computed: {
    buttonClasses() {
      return [
        'button',
        `button--${this.variant}`,
        `button--${this.size}`,
        {
          'button--disabled': this.disabled,
          'button--hovered': this.isHovered,
          'button--pressed': this.isPressed
        }
      ]
    }
  },
  
  methods: {
    handleClick(event) {
      if (!this.disabled) {
        this.$emit('click', event)
      }
    },
    
    handleMouseEnter() {
      if (!this.disabled) {
        this.isHovered = true
      }
    },
    
    handleMouseLeave() {
      this.isHovered = false
      this.isPressed = false
    }
  },
  
  watch: {
    disabled: {
      handler(newValue) {
        if (newValue) {
          this.isHovered = false
          this.isPressed = false
        }
      },
      immediate: true
    }
  },
  
  mounted() {
    console.log('Button component mounted')
  },
  
  beforeDestroy() {
    console.log('Button component will be destroyed')
  }
}
</script>

<style scoped>
.button {
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.button--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.button--primary {
  background-color: #3498db;
  color: white;
}

.button--secondary {
  background-color: #ecf0f1;
  color: #2c3e50;
}

.button--tertiary {
  background-color: transparent;
  color: #3498db;
}

.button--small {
  padding: 4px 8px;
  font-size: 12px;
}

.button--medium {
  padding: 8px 16px;
  font-size: 14px;
}

.button--large {
  padding: 12px 24px;
  font-size: 16px;
}
</style>
        "#;
        fs::write(&component_path, component_content).unwrap();
        
        let mut analyzer = EnhancedVueAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        // Check that we found the component
        assert_eq!(analyzer.components.len(), 1);
        
        // Get the component
        let component = analyzer.components.values().next().unwrap();
        
        // Check component properties
        assert_eq!(component.name, "Button");
        assert_eq!(component.component_type, "options_api");
        
        // Check props
        assert_eq!(component.props.len(), 3);
        assert!(component.props.iter().any(|prop| prop.name == "variant" && prop.prop_type == "String"));
        assert!(component.props.iter().any(|prop| prop.name == "size" && prop.prop_type == "String"));
        assert!(component.props.iter().any(|prop| prop.name == "disabled" && prop.prop_type == "Boolean"));
        
        // Check data
        assert_eq!(component.data.len(), 2);
        assert!(component.data.iter().any(|data| data.name == "isHovered"));
        assert!(component.data.iter().any(|data| data.name == "isPressed"));
        
        // Check computed properties
        assert_eq!(component.computed.len(), 1);
        assert!(component.computed.iter().any(|computed| computed.name == "buttonClasses"));
        
        // Check methods
        assert_eq!(component.methods.len(), 3);
        assert!(component.methods.iter().any(|method| method.name == "handleClick"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseEnter"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseLeave"));
        
        // Check watches
        assert_eq!(component.watches.len(), 1);
        assert!(component.watches.iter().any(|watch| watch.target == "disabled" && watch.immediate));
        
        // Check lifecycle hooks
        assert_eq!(component.lifecycle_hooks.len(), 2);
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "mounted"));
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "beforeDestroy"));
        
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_extract_composition_api_component() {
        let temp_dir = tempdir().unwrap();
        
        // Create a Vue component file
        let component_path = temp_dir.path().join("CompositionButton.vue");
        let component_content = r#"
<template>
  <button 
    :class="buttonClasses"
    :disabled="disabled"
    @click="handleClick"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
  >
    <slot></slot>
  </button>
</template>

<script>
import { defineComponent, ref, computed, watch, onMounted, onBeforeUnmount } from 'vue'

export default defineComponent({
  name: 'CompositionButton',
  
  props: {
    variant: {
      type: String,
      default: 'primary',
      validator: (value) => ['primary', 'secondary', 'tertiary'].includes(value)
    },
    size: {
      type: String,
      default: 'medium'
    },
    disabled: {
      type: Boolean,
      default: false
    }
  },
  
  setup(props, { emit }) {
    const isHovered = ref(false)
    const isPressed = ref(false)
    
    const buttonClasses = computed(() => {
      return [
        'button',
        `button--${props.variant}`,
        `button--${props.size}`,
        {
          'button--disabled': props.disabled,
          'button--hovered': isHovered.value,
          'button--pressed': isPressed.value
        }
      ]
    })
    
    const handleClick = (event) => {
      if (!props.disabled) {
        emit('click', event)
      }
    }
    
    const handleMouseEnter = () => {
      if (!props.disabled) {
        isHovered.value = true
      }
    }
    
    const handleMouseLeave = () => {
      isHovered.value = false
      isPressed.value = false
    }
    
    watch(() => props.disabled, (newValue) => {
      if (newValue) {
        isHovered.value = false
        isPressed.value = false
      }
    }, { immediate: true })
    
    onMounted(() => {
      console.log('Button component mounted')
    })
    
    onBeforeUnmount(() => {
      console.log('Button component will be destroyed')
    })
    
    return {
      isHovered,
      isPressed,
      buttonClasses,
      handleClick,
      handleMouseEnter,
      handleMouseLeave
    }
  }
})
</script>

<style scoped>
.button {
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}
</style>
        "#;
        fs::write(&component_path, component_content).unwrap();
        
        let mut analyzer = EnhancedVueAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        // Check that we found the component
        assert!(analyzer.components.len() > 0);
        
        // Get the composition component
        let component = analyzer.components.values().find(|c| c.name == "CompositionButton").unwrap();
        
        // Check component properties
        assert_eq!(component.component_type, "composition_api");
        
        // Check props
        assert_eq!(component.props.len(), 3);
        assert!(component.props.iter().any(|prop| prop.name == "variant" && prop.prop_type == "String"));
        assert!(component.props.iter().any(|prop| prop.name == "size" && prop.prop_type == "String"));
        assert!(component.props.iter().any(|prop| prop.name == "disabled" && prop.prop_type == "Boolean"));
        
        // Check data (refs)
        assert!(component.data.len() >= 2);
        assert!(component.data.iter().any(|data| data.name == "isHovered"));
        assert!(component.data.iter().any(|data| data.name == "isPressed"));
        
        // Check computed properties
        assert!(component.computed.len() >= 1);
        assert!(component.computed.iter().any(|computed| computed.name == "buttonClasses"));
        
        // Check methods
        assert!(component.methods.len() >= 3);
        assert!(component.methods.iter().any(|method| method.name == "handleClick"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseEnter"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseLeave"));
        
        // Check watches
        assert!(component.watches.len() >= 1);
        
        // Check lifecycle hooks
        assert!(component.lifecycle_hooks.len() >= 2);
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "mounted"));
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "beforeDestroy"));
        
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_extract_class_based_component() {
        let temp_dir = tempdir().unwrap();
        
        // Create a Vue component file
        let component_path = temp_dir.path().join("ClassButton.ts");
        let component_content = r#"
import { Component, Prop, Watch, Vue } from 'vue-property-decorator'

@Component({
  name: 'ClassButton',
})
export default class ClassButton extends Vue {
  @Prop({ type: String, default: 'primary' }) readonly variant!: string
  @Prop({ type: String, default: 'medium' }) readonly size!: string
  @Prop({ type: Boolean, default: false }) readonly disabled!: boolean

  isHovered = false
  isPressed = false

  get buttonClasses() {
    return [
      'button',
      `button--${this.variant}`,
      `button--${this.size}`,
      {
        'button--disabled': this.disabled,
        'button--hovered': this.isHovered,
        'button--pressed': this.isPressed
      }
    ]
  }

  @Watch('disabled', { immediate: true })
  onDisabledChange(newValue: boolean) {
    if (newValue) {
      this.isHovered = false
      this.isPressed = false
    }
  }

  handleClick(event: Event) {
    if (!this.disabled) {
      this.$emit('click', event)
    }
  }

  handleMouseEnter() {
    if (!this.disabled) {
      this.isHovered = true
    }
  }

  handleMouseLeave() {
    this.isHovered = false
    this.isPressed = false
  }

  mounted() {
    console.log('Button component mounted')
  }

  beforeDestroy() {
    console.log('Button component will be destroyed')
  }
}
        "#;
        fs::write(&component_path, component_content).unwrap();
        
        let mut analyzer = EnhancedVueAnalyzer::new();
        analyzer.analyze_directory(temp_dir.path()).unwrap();
        
        // Check that we found the component
        assert!(analyzer.components.len() > 0);
        
        // Get the class-based component
        let component = analyzer.components.values().find(|c| c.name == "ClassButton").unwrap();
        
        // Check component properties
        assert_eq!(component.component_type, "class_based");
        
        // Check props
        assert_eq!(component.props.len(), 3);
        assert!(component.props.iter().any(|prop| prop.name == "variant" && prop.prop_type == "string"));
        assert!(component.props.iter().any(|prop| prop.name == "size" && prop.prop_type == "string"));
        assert!(component.props.iter().any(|prop| prop.name == "disabled" && prop.prop_type == "boolean"));
        
        // Check data
        assert_eq!(component.data.len(), 2);
        assert!(component.data.iter().any(|data| data.name == "isHovered"));
        assert!(component.data.iter().any(|data| data.name == "isPressed"));
        
        // Check computed properties
        assert_eq!(component.computed.len(), 1);
        assert!(component.computed.iter().any(|computed| computed.name == "buttonClasses"));
        
        // Check methods
        assert_eq!(component.methods.len(), 3);
        assert!(component.methods.iter().any(|method| method.name == "handleClick"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseEnter"));
        assert!(component.methods.iter().any(|method| method.name == "handleMouseLeave"));
        
        // Check watches
        assert_eq!(component.watches.len(), 1);
        assert!(component.watches.iter().any(|watch| watch.target == "disabled" && watch.immediate));
        
        // Check lifecycle hooks
        assert_eq!(component.lifecycle_hooks.len(), 2);
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "mounted"));
        assert!(component.lifecycle_hooks.iter().any(|hook| hook.name == "beforeDestroy"));
        
        temp_dir.close().unwrap();
    }
}
