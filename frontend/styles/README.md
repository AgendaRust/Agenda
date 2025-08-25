# CSS Architecture Documentation

This document describes the new modular CSS structure for the Agenda frontend application.

## Overview

The CSS has been refactored from a single monolithic `style.css` file into a modular, organized structure under the `styles/` directory. This improves maintainability, readability, and scalability.

## File Structure

```
frontend/styles/
├── main.css         # Main entry point that imports all other CSS files
├── variables.css    # CSS custom properties and theme definitions
├── base.css         # Reset styles, typography, and fundamental elements
├── components.css   # Reusable component styles (buttons, containers, etc.)
├── login.css        # Login page specific styles
└── utilities.css    # Utility classes, animations, and responsive design
```

## File Descriptions

### `main.css`

- **Purpose**: Main entry point for all CSS
- **Content**: Import statements for all other CSS modules
- **Usage**: Referenced by `index.html` via `data-trunk`

### `variables.css`

- **Purpose**: CSS custom properties and theme system
- **Content**:
  - Light theme color variables
  - Dark theme color variables
  - Theme switching classes
- **Features**: Orange anime princess theme with Crunchyroll-inspired colors

### `base.css`

- **Purpose**: Foundation styles and typography
- **Content**:
  - CSS reset (\*{margin:0;padding:0;box-sizing:border-box})
  - Body base styles with gradient background
  - Heading typography (h1)
  - Smooth transitions

### `components.css`

- **Purpose**: Reusable UI component styles
- **Content**:
  - Main container with glass morphism effect
  - Theme toggle button
  - Standard button styles with gradients and animations
  - Counter display
  - Button container layouts
  - Dark/light theme specific overrides

### `login.css`

- **Purpose**: Login page specific styles
- **Content**:
  - Login container layouts (split-screen design)
  - Form styling (inputs, labels, buttons)
  - Image positioning
  - Dark theme adjustments for login elements
  - Registration link styling

### `utilities.css`

- **Purpose**: Utility classes, animations, and responsive design
- **Content**:
  - Keyframe animations (pulse, themeSwitch)
  - Animation utility classes (.updated, .dark-theme, .light-theme)
  - Responsive breakpoints for mobile and tablet
  - Media queries for different screen sizes

## Theme System

The CSS uses CSS custom properties for a robust theming system:

- **Light Theme**: Purple/blue gradients with white glass morphism containers
- **Dark Theme**: Dark blue/gray gradients with darker translucent containers
- **Orange Accents**: Pink/orange gradient counters maintaining anime aesthetic

## Build Integration

The new structure integrates with Trunk build system:

- `index.html` imports `styles/main.css`
- Trunk processes the import chain and bundles all CSS
- All assets are served correctly in development and production

## Migration Notes

- Original `style.css` backed up as `style.css.backup`
- All functionality preserved - no visual changes
- Modular structure enables easier team development
- Page-specific styles can be easily added as new CSS files

## Adding New Styles

1. **Page-specific styles**: Create new file like `home.css`, `dashboard.css`
2. **New components**: Add to `components.css` or create component-specific file
3. **Theme colors**: Add to `variables.css` custom properties
4. **Utilities**: Add to `utilities.css` for reusable classes
5. **Import**: Add import statement to `main.css`

## Benefits

- **Maintainability**: Easier to find and modify specific styles
- **Scalability**: New pages/components can have dedicated CSS files
- **Team Development**: Multiple developers can work on different CSS files
- **Performance**: Unchanged (Trunk bundles everything)
- **Organization**: Logical separation of concerns
- **Debugging**: Easier to locate style issues
