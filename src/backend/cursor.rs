use crate::backend::utils::CssAttribute;
use ratatui::style::Style;

use super::theme::RgbColor;

/// Supported cursor shapes.
#[derive(Debug, Default)]
pub enum CursorShape {
    /// A non blinking block cursor shape (█).
    #[default]
    SteadyBlock,
    /// A non blinking underscore cursor shape (_).
    SteadyUnderScore,
    /// This variant is only used to clear cursor.
    None,
}

impl CursorShape {
    /// Transforms the given style to hide the cursor.
    pub fn hide(&self, style: Style) -> Style {
        match self {
            CursorShape::SteadyBlock => style.not_reversed(),
            CursorShape::SteadyUnderScore => style.not_underlined(),
            CursorShape::None => style,
        }
    }

    /// Transforms the given style to show the cursor.
    pub fn show(&self, style: Style) -> Style {
        match self {
            CursorShape::SteadyBlock => style.reversed(),
            CursorShape::SteadyUnderScore => style.underlined(),
            CursorShape::None => style,
        }
    }

    /// Returns a list of css fields and their values for this cursor shape.
    pub fn get_css_attribute(&self) -> CssAttribute {
        match self {
            CursorShape::SteadyBlock => CssAttribute {
                field: "text-decoration",
                value: Some("none"),
            },
            CursorShape::SteadyUnderScore => CssAttribute {
                field: "text-decoration",
                value: Some("underline"),
            },
            CursorShape::None => CssAttribute {
                field: "text-decoration",
                value: None,
            },
        }
    }

    /// Transforms the given style to show the cursor, optionally using theme cursor colors.
    ///
    /// If theme colors are provided (as 0xRRGGBB u32 values), they will be used for the cursor appearance.
    /// Otherwise, falls back to the REVERSED or UNDERLINED modifier.
    pub(crate) fn show_with_colors(
        &self,
        style: Style,
        cursor_bg: Option<u32>,
        cursor_fg: Option<u32>,
    ) -> Style {
        match self {
            CursorShape::SteadyBlock => {
                if let (Some(bg), Some(fg)) = (cursor_bg, cursor_fg) {
                    // Use theme cursor colors
                    style
                        .bg(RgbColor::from_u32(bg).to_color())
                        .fg(RgbColor::from_u32(fg).to_color())
                } else {
                    // Fallback to reversed
                    style.reversed()
                }
            }
            CursorShape::SteadyUnderScore => style.underlined(),
            CursorShape::None => style,
        }
    }
}
