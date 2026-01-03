# Theme Switcher Example

This example demonstrates:
- How to build custom color themes using the `Theme` API
- Showcasing different color types (ANSI indexed, RGB, named colors)
- Displaying various text styles (bold, italic, underline, etc.)
- Creating an interactive TUI with keyboard controls

## Features

### Included Themes

The example includes 7 pre-configured themes:
1. **Default** - Standard terminal colors
2. **Dracula** - Dark theme with purple accents
3. **Nord** - Arctic, north-bluish color palette
4. **Gruvbox Dark** - Retro groove color scheme
5. **Solarized Dark** - Precision colors for machines and people
6. **One Dark** - Atom's iconic One Dark theme
7. **Tokyo Night** - A clean, dark theme inspired by Tokyo

### Color Showcases

- **Theme Palette Preview** - Shows the active theme's foreground, background, and primary colors
- **Basic ANSI Colors (0-7)** - The standard 8 terminal colors
- **Bright ANSI Colors (8-15)** - Bright variants of the basic colors
- **RGB Colors** - Custom RGB color examples
- **Text Styles** - Demonstrations of bold, italic, underline, crossed-out, and reversed text

## Controls

- **↑ / k** - Switch to previous theme configuration
- **↓ / j** - Switch to next theme configuration

## Running the Example

### Build and serve locally:

```bash
# Build for WASM
trunk build

# Or build and serve
trunk serve
```

Then open `http://localhost:8080` in your browser.

### Build only:

```bash
cargo build --target wasm32-unknown-unknown
```

## Note on Theme Switching

This example demonstrates theme configurations and how to build themes using the `Theme::builder()` API. The color showcases use explicit color values (indexed and RGB) which remain consistent across theme switches.

For true dynamic theme switching that affects the entire backend rendering (including Reset colors), you would need to:
1. Recreate the backend with the new theme, or
2. Reload the page with different theme parameters

The current implementation showcases the theme definitions and demonstrates the color rendering capabilities of Ratzilla.

## Code Highlights

### Building a Custom Theme

```rust
Theme::builder()
    .background(Color::Rgb(40, 42, 54))
    .foreground(Color::Rgb(248, 248, 242))
    .palette_color(0, Color::Rgb(40, 42, 54))    // black
    .palette_color(1, Color::Rgb(255, 85, 85))   // red
    // ... more colors
    .cursor_color(Color::Rgb(248, 248, 242))
    .selection_background(Color::Rgb(68, 71, 90))
    .build()
```

### Using Themes with Backend

```rust
let terminal = MultiBackendBuilder::with_fallback(BackendType::WebGl2)
    .theme(my_theme)
    .build_terminal()?;
```
