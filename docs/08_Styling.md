# Type-Safe Styling

Lithe.rs provides a CSS-in-Rust solution that combines the power of utility classes with the safety of types.

## 1. The Style Builder
```rust
const PRIMARY_BTN: Style = Style::new()
    .display(Display::Flex)
    .bg_color(Color::Blue)
    .padding(px(16))
    .hover(|s| s.bg_color(Color::DarkBlue))
    .transition(Fade::standard());

button().style(PRIMARY_BTN).text("Click Me")
```

## 2. Atomic CSS Generation
At build time, the Lithe compiler extracts all styles into a single, minimal, and scoped CSS file.
- **No Runtime Overhead:** Styles are just classes by the time they hit the browser.
- **Scoped by Default:** Styles defined in a component do not leak to other parts of the app.

## 3. Theming
Theming is handled via a typed `Theme` struct that can be injected via Context.
```rust
let theme = use_theme();
div().bg_color(theme.surface_color)
```
