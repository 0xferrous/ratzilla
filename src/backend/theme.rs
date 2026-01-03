use ratatui::prelude::Color;

/// Internal representation of RGB color as packed u32 (0xRRGGBB).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RgbColor(u32);

impl RgbColor {
    #[inline]
    pub(super) const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    #[inline]
    pub(super) const fn from_u32(rgb: u32) -> Self {
        Self(rgb)
    }

    #[inline]
    pub(super) const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub(super) fn to_color(self) -> Color {
        let r = ((self.0 >> 16) & 0xFF) as u8;
        let g = ((self.0 >> 8) & 0xFF) as u8;
        let b = (self.0 & 0xFF) as u8;
        Color::Rgb(r, g, b)
    }
}

/// Terminal color theme configuration.
///
/// Defines the color palette for terminal rendering, including ANSI colors,
/// default colors, cursor colors, and selection colors.
///
/// # Example
/// ```no_run
/// use ratzilla::Theme;
/// use ratatui::style::Color;
///
/// let theme = Theme::builder()
///     .palette_color(0, Color::Rgb(16, 18, 22))    // black
///     .palette_color(1, Color::Rgb(247, 129, 102)) // red
///     .foreground(Color::Rgb(139, 148, 158))
///     .background(Color::Rgb(16, 18, 22))
///     .cursor_color(Color::Rgb(201, 209, 217))
///     .cursor_text_color(Color::Rgb(16, 18, 22))
///     .selection_background(Color::Rgb(59, 80, 112))
///     .selection_foreground(Color::Rgb(255, 255, 255))
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// 16 ANSI color palette (indices 0-15)
    /// Corresponds to standard terminal colors:
    /// 0-7: black, red, green, yellow, blue, magenta, cyan, white
    /// 8-15: bright variants
    palette: [RgbColor; 16],

    /// Default foreground color (used for Color::Reset in foreground position)
    default_fg: RgbColor,

    /// Default background color (used for Color::Reset in background position)
    default_bg: RgbColor,

    /// Cursor background color
    cursor_color: RgbColor,

    /// Cursor text color (text under cursor)
    cursor_text_color: RgbColor,

    /// Selection background color (for text selection in WebGL2 backend)
    selection_bg: RgbColor,

    /// Selection foreground color (for text selection in WebGL2 backend)
    selection_fg: RgbColor,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Use existing hardcoded BASIC_COLORS from color.rs
            palette: [
                RgbColor::from_u32(0x000000), // 0: black
                RgbColor::from_u32(0xCD0000), // 1: red
                RgbColor::from_u32(0x00CD00), // 2: green
                RgbColor::from_u32(0xCDCD00), // 3: yellow
                RgbColor::from_u32(0x0000EE), // 4: blue
                RgbColor::from_u32(0xCD00CD), // 5: magenta
                RgbColor::from_u32(0x00CDCD), // 6: cyan
                RgbColor::from_u32(0xE5E5E5), // 7: white
                RgbColor::from_u32(0x7F7F7F), // 8: bright black
                RgbColor::from_u32(0xFF0000), // 9: bright red
                RgbColor::from_u32(0x00FF00), // 10: bright green
                RgbColor::from_u32(0xFFFF00), // 11: bright yellow
                RgbColor::from_u32(0x5C5CFF), // 12: bright blue
                RgbColor::from_u32(0xFF00FF), // 13: bright magenta
                RgbColor::from_u32(0x00FFFF), // 14: bright cyan
                RgbColor::from_u32(0xFFFFFF), // 15: bright white
            ],
            default_fg: RgbColor::from_u32(0xFFFFFF),
            default_bg: RgbColor::from_u32(0x000000),
            cursor_color: RgbColor::from_u32(0xFFFFFF),
            cursor_text_color: RgbColor::from_u32(0x000000),
            selection_bg: RgbColor::from_u32(0x3B5070),
            selection_fg: RgbColor::from_u32(0xFFFFFF),
        }
    }
}

impl Theme {
    /// Creates a new [`ThemeBuilder`] starting with default colors.
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder {
            theme: Theme::default(),
        }
    }

    /// Converts a [`Color`] to a 24-bit RGB value using this theme's palette.
    ///
    /// The `is_foreground` parameter determines which default color to use for `Color::Reset`:
    /// - `true`: Uses theme's default foreground color
    /// - `false`: Uses theme's default background color
    ///
    /// Named colors (Black, Red, etc.) and indexed colors 0-15 use the theme palette.
    /// RGB colors and indexed colors 16-255 are converted algorithmically.
    pub(crate) fn to_rgb(&self, color: Color, is_foreground: bool) -> RgbColor {
        /// Default theme used as fallback for palette color lookups.
        static DEFAULT_THEME: std::sync::LazyLock<Theme> = std::sync::LazyLock::new(Theme::default);

        match color {
            Color::Rgb(r, g, b) => RgbColor::from_rgb(r, g, b),
            Color::Reset => {
                if is_foreground {
                    self.default_fg
                } else {
                    self.default_bg
                }
            }
            Color::Black => self
                .palette_color(0)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(0).unwrap()),
            Color::Red => self
                .palette_color(1)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(1).unwrap()),
            Color::Green => self
                .palette_color(2)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(2).unwrap()),
            Color::Yellow => self
                .palette_color(3)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(3).unwrap()),
            Color::Blue => self
                .palette_color(4)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(4).unwrap()),
            Color::Magenta => self
                .palette_color(5)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(5).unwrap()),
            Color::Cyan => self
                .palette_color(6)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(6).unwrap()),
            Color::Gray => self
                .palette_color(7)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(7).unwrap()),
            Color::DarkGray => self
                .palette_color(8)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(8).unwrap()),
            Color::LightRed => self
                .palette_color(9)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(9).unwrap()),
            Color::LightGreen => self
                .palette_color(10)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(10).unwrap()),
            Color::LightYellow => self
                .palette_color(11)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(11).unwrap()),
            Color::LightBlue => self
                .palette_color(12)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(12).unwrap()),
            Color::LightMagenta => self
                .palette_color(13)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(13).unwrap()),
            Color::LightCyan => self
                .palette_color(14)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(14).unwrap()),
            Color::White => self
                .palette_color(15)
                .unwrap_or_else(|| DEFAULT_THEME.palette_color(15).unwrap()),
            Color::Indexed(code) => self.indexed_color_to_rgb(code),
        }
    }

    /// Converts an indexed color (0-255) to an RGB value, using theme for 0-15.
    fn indexed_color_to_rgb(&self, index: u8) -> RgbColor {
        /// Default theme used as fallback for palette color lookups.
        static DEFAULT_THEME: std::sync::LazyLock<Theme> = std::sync::LazyLock::new(Theme::default);

        match index {
            // Use theme palette for basic 16 colors
            0..=15 => self.palette_color(index).unwrap_or_else(|| {
                // Fallback to default theme if lookup fails
                DEFAULT_THEME.palette_color(index).unwrap()
            }),

            // 216-color cube (16-231) - algorithmic, not themed
            16..=231 => {
                let cube_index = index - 16;
                let r = cube_index / 36;
                let g = (cube_index % 36) / 6;
                let b = cube_index % 6;

                // Convert 0-5 range to 0-255 RGB
                let to_rgb = |n: u8| -> u8 {
                    if n == 0 {
                        0
                    } else {
                        55 + 40 * n
                    }
                };

                RgbColor::from_rgb(to_rgb(r), to_rgb(g), to_rgb(b))
            }

            // 24 grayscale colors (232-255) - algorithmic, not themed
            232..=255 => {
                let gray_index = index - 232;
                let gray = 8 + gray_index * 10;
                RgbColor::from_rgb(gray, gray, gray)
            }
        }
    }

    /// Returns the RGB value for a palette color (0-15).
    pub(crate) fn palette_color(&self, index: u8) -> Option<RgbColor> {
        if index < 16 {
            Some(self.palette[index as usize])
        } else {
            None
        }
    }

    /// Returns the default foreground color.
    pub(crate) fn default_fg(&self) -> RgbColor {
        self.default_fg
    }

    /// Returns the default background color.
    pub(crate) fn default_bg(&self) -> RgbColor {
        self.default_bg
    }

    /// Returns the cursor color.
    pub(crate) fn cursor_color(&self) -> RgbColor {
        self.cursor_color
    }

    /// Returns the cursor text color.
    pub(crate) fn cursor_text_color(&self) -> RgbColor {
        self.cursor_text_color
    }

    /// Returns the selection background color.
    pub(crate) fn selection_bg(&self) -> RgbColor {
        self.selection_bg
    }

    /// Returns the selection foreground color.
    pub(crate) fn selection_fg(&self) -> RgbColor {
        self.selection_fg
    }
}

/// Builder for constructing a [`Theme`] with customized colors.
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// Sets a palette color (index 0-15).
    ///
    /// Indices outside the range 0-15 are ignored.
    pub fn palette_color(mut self, index: u8, color: Color) -> Self {
        if index < 16 {
            self.theme.palette[index as usize] = color_to_rgb(color);
        }
        self
    }

    /// Sets the default foreground color.
    pub fn foreground(mut self, color: Color) -> Self {
        self.theme.default_fg = color_to_rgb(color);
        self
    }

    /// Sets the default background color.
    pub fn background(mut self, color: Color) -> Self {
        self.theme.default_bg = color_to_rgb(color);
        self
    }

    /// Sets the cursor background color.
    pub fn cursor_color(mut self, color: Color) -> Self {
        self.theme.cursor_color = color_to_rgb(color);
        self
    }

    /// Sets the cursor text color.
    pub fn cursor_text_color(mut self, color: Color) -> Self {
        self.theme.cursor_text_color = color_to_rgb(color);
        self
    }

    /// Sets the selection background color.
    pub fn selection_background(mut self, color: Color) -> Self {
        self.theme.selection_bg = color_to_rgb(color);
        self
    }

    /// Sets the selection foreground color.
    pub fn selection_foreground(mut self, color: Color) -> Self {
        self.theme.selection_fg = color_to_rgb(color);
        self
    }

    /// Builds the final [`Theme`].
    pub fn build(self) -> Theme {
        self.theme
    }
}

/// Converts a ratatui Color to u32 RGB (without fallback).
fn color_to_rgb(color: Color) -> RgbColor {
    match color {
        Color::Rgb(r, g, b) => RgbColor::from_rgb(r, g, b),
        // For named colors, use the standard ANSI values
        Color::Black => RgbColor::from_u32(0x000000),
        Color::Red => RgbColor::from_u32(0x800000),
        Color::Green => RgbColor::from_u32(0x008000),
        Color::Yellow => RgbColor::from_u32(0x808000),
        Color::Blue => RgbColor::from_u32(0x000080),
        Color::Magenta => RgbColor::from_u32(0x800080),
        Color::Cyan => RgbColor::from_u32(0x008080),
        Color::Gray => RgbColor::from_u32(0xc0c0c0),
        Color::DarkGray => RgbColor::from_u32(0x808080),
        Color::LightRed => RgbColor::from_u32(0xFF0000),
        Color::LightGreen => RgbColor::from_u32(0x00FF00),
        Color::LightYellow => RgbColor::from_u32(0xFFFF00),
        Color::LightBlue => RgbColor::from_u32(0x0000FF),
        Color::LightMagenta => RgbColor::from_u32(0xFF00FF),
        Color::LightCyan => RgbColor::from_u32(0x00FFFF),
        Color::White => RgbColor::from_u32(0xFFFFFF),
        Color::Indexed(idx) => {
            // For indexed colors in theme definition, use hardcoded lookup
            // This is a limitation - user should use Rgb() for custom themes
            RgbColor::from_u32(indexed_color_to_rgb(idx))
        }
        Color::Reset => RgbColor::from_u32(0x000000), // Shouldn't be used in theme definition
    }
}

/// Converts an indexed color (0-255) to an RGB value using xterm 256-color palette.
fn indexed_color_to_rgb(index: u8) -> u32 {
    match index {
        // Basic 16 colors (0-15)
        0..=15 => {
            const BASIC_COLORS: [u32; 16] = [
                0x000000, // 0: black
                0xCD0000, // 1: red
                0x00CD00, // 2: green
                0xCDCD00, // 3: yellow
                0x0000EE, // 4: blue
                0xCD00CD, // 5: magenta
                0x00CDCD, // 6: cyan
                0xE5E5E5, // 7: white
                0x7F7F7F, // 8: bright Black
                0xFF0000, // 9: bright Red
                0x00FF00, // 10: bright Green
                0xFFFF00, // 11: bright Yellow
                0x5C5CFF, // 12: bright Blue
                0xFF00FF, // 13: bright Magenta
                0x00FFFF, // 14: bright Cyan
                0xFFFFFF, // 15: bright White
            ];
            BASIC_COLORS[index as usize]
        }

        // 216-color cube (16-231)
        16..=231 => {
            let cube_index = index - 16;
            let r = cube_index / 36;
            let g = (cube_index % 36) / 6;
            let b = cube_index % 6;

            // Convert 0-5 range to 0-255 RGB
            let to_rgb = |n: u8| -> u32 {
                if n == 0 {
                    0
                } else {
                    55 + 40 * n as u32
                }
            };

            to_rgb(r) << 16 | to_rgb(g) << 8 | to_rgb(b)
        }

        // 24 grayscale colors (232-255)
        232..=255 => {
            let gray_index = index - 232;
            let gray = (8 + gray_index * 10) as u32;
            (gray << 16) | (gray << 8) | gray
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.palette_color(0), Some(RgbColor::from_u32(0x000000)));
        assert_eq!(theme.palette_color(1), Some(RgbColor::from_u32(0xCD0000)));
        assert_eq!(theme.palette_color(15), Some(RgbColor::from_u32(0xFFFFFF)));
        assert_eq!(theme.default_fg(), RgbColor::from_u32(0xFFFFFF));
        assert_eq!(theme.default_bg(), RgbColor::from_u32(0x000000));
        assert_eq!(theme.cursor_color(), RgbColor::from_u32(0xFFFFFF));
        assert_eq!(theme.cursor_text_color(), RgbColor::from_u32(0x000000));
        assert_eq!(theme.selection_bg(), RgbColor::from_u32(0x3B5070));
        assert_eq!(theme.selection_fg(), RgbColor::from_u32(0xFFFFFF));
    }

    #[test]
    fn test_theme_builder() {
        let theme = Theme::builder()
            .palette_color(0, Color::Rgb(16, 18, 22))
            .palette_color(1, Color::Rgb(247, 129, 102))
            .foreground(Color::Rgb(139, 148, 158))
            .background(Color::Rgb(16, 18, 22))
            .cursor_color(Color::Rgb(201, 209, 217))
            .cursor_text_color(Color::Rgb(16, 18, 22))
            .selection_background(Color::Rgb(59, 80, 112))
            .selection_foreground(Color::Rgb(255, 255, 255))
            .build();

        assert_eq!(theme.palette_color(0), Some(RgbColor::from_u32(0x101216)));
        assert_eq!(theme.palette_color(1), Some(RgbColor::from_u32(0xF78166)));
        assert_eq!(theme.default_fg(), RgbColor::from_u32(0x8B949E));
        assert_eq!(theme.default_bg(), RgbColor::from_u32(0x101216));
        assert_eq!(theme.cursor_color(), RgbColor::from_u32(0xC9D1D9));
        assert_eq!(theme.cursor_text_color(), RgbColor::from_u32(0x101216));
    }

    #[test]
    fn test_palette_bounds() {
        let theme = Theme::builder()
            .palette_color(16, Color::Rgb(255, 0, 0)) // Should be ignored
            .build();

        assert_eq!(theme.palette_color(16), None);
        // Should still have default value for index 15
        assert_eq!(theme.palette_color(15), Some(RgbColor::from_u32(0xFFFFFF)));
    }

    #[test]
    fn test_color_to_rgb() {
        assert_eq!(
            color_to_rgb(Color::Rgb(255, 128, 64)),
            RgbColor::from_u32(0xFF8040)
        );
        assert_eq!(color_to_rgb(Color::Black), RgbColor::from_u32(0x000000));
        assert_eq!(color_to_rgb(Color::LightRed), RgbColor::from_u32(0xFF0000));
        assert_eq!(color_to_rgb(Color::White), RgbColor::from_u32(0xFFFFFF));
    }

    #[test]
    fn test_indexed_color_to_rgb() {
        // Test basic colors
        assert_eq!(indexed_color_to_rgb(0), 0x000000);
        assert_eq!(indexed_color_to_rgb(1), 0xCD0000);
        assert_eq!(indexed_color_to_rgb(15), 0xFFFFFF);

        // Test 216-color cube
        assert_eq!(indexed_color_to_rgb(16), 0x000000);
        assert_eq!(indexed_color_to_rgb(68), 0x5F87D7);

        // Test grayscale
        assert_eq!(indexed_color_to_rgb(232), 0x080808);
        assert_eq!(indexed_color_to_rgb(255), 0xEEEEEE);
    }
}
