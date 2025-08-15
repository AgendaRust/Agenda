# Static Assets

This folder contains static assets for the frontend application.

## Structure

- `images/` - Static images (logos, backgrounds, photos)
- `icons/` - Icon files (SVG, PNG icons)
- `fonts/` - Custom font files

## Usage

Static files are served by Trunk and can be referenced in your CSS or HTML:

```css
/* In CSS */
background-image: url("/static/images/background.jpg");

/* For icons */
background-image: url("/static/icons/logo.svg");
```

```html
<!-- In HTML -->
<img src="/static/images/logo.png" alt="Logo" />
```

## File Organization

### Images

- Use WebP format for photos when possible (better compression)
- Use SVG for simple graphics and logos
- Keep file sizes reasonable for web performance

### Icons

- Prefer SVG icons for scalability
- Use consistent naming convention (e.g., `icon-name.svg`)
- Consider using an icon font for common UI icons

### Fonts

- Include WOFF2 format for modern browsers
- Provide fallbacks for older browsers
- Add appropriate font-display properties in CSS
