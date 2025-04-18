# Quiz Module Theme Customization

This document explains how to customize the quiz module's theme to match your application's design language.

## Overview

The quiz module's theme is built using CSS variables, which allows for easy customization without modifying the original CSS files. The theme system is designed to:

1. Match Quenti's native theme by default
2. Allow for theme changes via the main Ordo app
3. Support dark mode and high contrast mode

## File Structure

The theme system consists of several CSS files:

- `quiz-ordo-integration.css`: Defines Ordo app variables that the quiz module will use
- `quiz-theme.css`: Maps Ordo variables to quiz-specific variables
- `quiz.css`: Contains the base styles for the quiz module
- `quiz-dark.css`: Contains dark mode overrides
- `quiz-animations.css`: Contains animations and transitions
- `quiz-print.css`: Contains print-specific styles

## Customizing the Theme

### Basic Customization

To customize the quiz module's theme, you can define CSS variables in your application's CSS:

```css
:root {
  /* Primary color */
  --primary-color: #ff0000; /* Red */
  
  /* Border radius */
  --radius: 0; /* Square corners */
  
  /* Font family */
  --font-heading: 'Roboto', sans-serif;
  --font-body: 'Roboto', sans-serif;
}
```

### Available Variables

Here are the main variables you can customize:

#### Colors

```css
--primary-color: #1a5fff; /* Primary color */
--primary-color-hover: #4b83ff; /* Primary color hover state */
--primary-color-light: rgba(26, 95, 255, 0.1); /* Light version of primary color */

--secondary-color: #ff8b1a; /* Secondary color */
--secondary-color-hover: #ffa54c; /* Secondary color hover state */
--secondary-color-light: rgba(255, 139, 26, 0.1); /* Light version of secondary color */

--accent-color: #ffc91a; /* Accent color */
--accent-color-hover: #ffd54b; /* Accent color hover state */

--danger-color: #e45735; /* Danger/error color */
--success-color: #1ca551; /* Success color */

--text-color: #1a202c; /* Main text color */
--text-light: #4a5568; /* Secondary text color */
--text-lighter: #718096; /* Tertiary text color */

--background-color: #f7fafc; /* Page background color */
--card-bg: #ffffff; /* Card background color */
--border-color: #e2e8f0; /* Border color */
```

#### Typography

```css
--font-heading: 'Outfit', system-ui, sans-serif; /* Heading font */
--font-body: 'Open Sans', system-ui, sans-serif; /* Body font */

--font-xs: 0.75rem; /* Extra small font size */
--font-sm: 0.875rem; /* Small font size */
--font-base: 1rem; /* Base font size */
--font-md: 1.125rem; /* Medium font size */
--font-lg: 1.25rem; /* Large font size */
--font-xl: 1.5rem; /* Extra large font size */
--font-2xl: 1.875rem; /* 2x extra large font size */

--font-normal: 400; /* Normal font weight */
--font-medium: 500; /* Medium font weight */
--font-semibold: 600; /* Semi-bold font weight */
--font-bold: 700; /* Bold font weight */
```

#### Spacing

```css
--spacing-xs: 0.25rem; /* Extra small spacing */
--spacing-sm: 0.5rem; /* Small spacing */
--spacing: 1rem; /* Base spacing */
--spacing-md: 1.5rem; /* Medium spacing */
--spacing-lg: 2rem; /* Large spacing */
--spacing-xl: 3rem; /* Extra large spacing */
```

#### Border Radius

```css
--radius-sm: 0.375rem; /* Small border radius */
--radius: 0.5rem; /* Base border radius */
--radius-md: 0.75rem; /* Medium border radius */
--radius-lg: 1rem; /* Large border radius */
```

#### Shadows

```css
--shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Base shadow */
--shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.05); /* Small shadow */
--shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1); /* Large shadow */
```

#### Transitions

```css
--transition: all 0.2s ease; /* Base transition */
--transition-slow: all 0.3s ease; /* Slow transition */
```

### Dark Mode Customization

To customize the dark mode theme, use the `[data-theme="dark"]` selector:

```css
[data-theme="dark"] {
  --primary-color-dark: #4b83ff;
  --background-color-dark: #171923;
  --card-bg-dark: #242C3A;
  --text-color-dark: #f7fafc;
}
```

## Examples

### Changing to a Green Theme

```css
:root {
  --primary-color: #10b981;
  --primary-color-hover: #059669;
  --primary-color-light: rgba(16, 185, 129, 0.1);
  
  --secondary-color: #6366f1;
  --secondary-color-hover: #4f46e5;
  --secondary-color-light: rgba(99, 102, 241, 0.1);
}
```

### Using Rounded Corners

```css
:root {
  --radius-sm: 0.5rem;
  --radius: 0.75rem;
  --radius-md: 1rem;
  --radius-lg: 1.5rem;
}
```

### Using a Custom Font

```css
:root {
  --font-heading: 'Poppins', sans-serif;
  --font-body: 'Inter', sans-serif;
}
```

## Advanced Customization

For more advanced customization, you can override specific component styles in your application's CSS. The quiz module uses BEM-like class names, making it easy to target specific components.

Example:

```css
.quiz-container {
  /* Custom styles for quiz container */
}

.choice-button {
  /* Custom styles for choice buttons */
}
```

## Accessibility Considerations

The theme system is designed to work well with:

- Dark mode
- High contrast mode
- Reduced motion preferences
- Screen readers

When customizing the theme, ensure that:

1. Text has sufficient contrast against its background
2. Interactive elements are clearly distinguishable
3. Focus states are visible
4. Color is not the only means of conveying information
