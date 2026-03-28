<!-- Unlicense — cochranblock.org -->

# Section 508 / WCAG 2.1 Compliance Status

> Last updated: 2026-03-27

## Web Application (oakilydokily.com)

### Keyboard Navigation

| Feature | Status | Notes |
|---------|--------|-------|
| Skip-to-content link | Present | `.skip-link` hidden until focused, jumps to `#main` |
| Tab order | Logical | Form fields, links, buttons follow DOM order |
| Hamburger menu | Keyboard accessible | `aria-expanded` toggled on click |
| Form submission | Enter key works | Standard HTML form behavior |
| Focus indicators | Visible | `outline: 2px solid var(--accent)` on `:focus-visible` |

### ARIA Labels

| Element | Implementation |
|---------|---------------|
| Navigation | `<nav>` semantic element with `.nav` class |
| Main content | `<main id="main">` landmark |
| Hero mural canvas | `aria-hidden="true"` (decorative, not interactive content) |
| Form labels | All inputs have associated `<label>` elements |
| Checkboxes | Wrapped in `<label>` for click-to-toggle |

### Color Contrast

| Element | Foreground | Background | Ratio | Pass (AA) |
|---------|-----------|-----------|-------|-----------|
| Body text | #e6edf3 | #0f1419 | ~13:1 | Yes |
| Muted text | #8b949e | #0f1419 | ~5.5:1 | Yes |
| Accent (purple) | #a855f7 | #0f1419 | ~5:1 | Yes |
| Primary (pink) | #ec4899 | #fff (button text) | ~3.5:1 | AA Large |
| Warm (orange) | #fb923c | #0f1419 | ~5.5:1 | Yes |

### Screen Reader Support

- Semantic HTML: `<nav>`, `<main>`, `<footer>`, `<section>`, `<h1>`-`<h3>`
- Form labels explicitly associated with inputs
- Error messages rendered as visible text (not just color)
- Decorative canvas marked `aria-hidden="true"`

### Mobile Responsiveness

- Viewport meta tag: `width=device-width, initial-scale=1`
- Three breakpoints: default (desktop), 768px (tablet), 480px (phone)
- Touch targets: minimum 42px height on buttons (`.btn { min-height: 42px }`)
- Hamburger nav for screens under 768px

### Known Gaps

- WASM mural canvas is not keyboard-navigable (decorative only)
- No `lang` attribute on individual content blocks (only `<html lang="en">`)
- Print stylesheet hides nav but does not add print-specific alt text
- No high-contrast mode toggle
- No text size adjustment controls
