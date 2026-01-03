use std::{cell::RefCell, io, rc::Rc};

use ratzilla::{
    event::KeyCode,
    ratatui::{
        buffer::Buffer,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Paragraph, Widget},
    },
    Theme, WebRenderer,
};

use examples_shared::backend::{BackendType, MultiBackendBuilder};

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let app_state = Rc::new(RefCell::new(AppState::new()));

    let terminal = {
        let app_state = app_state.borrow();
        MultiBackendBuilder::with_fallback(BackendType::WebGl2)
            .theme(app_state.current_theme())
            .build_terminal()?
    };

    terminal.on_key_event({
        let app_state = app_state.clone();
        move |key_event| {
            let mut state = app_state.borrow_mut();
            match key_event.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    state.previous_theme();
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    state.next_theme();
                }
                _ => {}
            }
        }
    });

    terminal.draw_web(move |f| {
        let state = app_state.borrow();
        f.render_widget(ColorShowcase::new(&state), f.area());
    });

    // Note: Dynamic theme switching requires recreating the backend.
    // This example demonstrates theme configurations and color showcasing.
    // To fully switch themes at runtime, you would need to reload the page
    // with different theme parameters or rebuild the terminal with a new backend.

    Ok(())
}

/// Application state
struct AppState {
    themes: Vec<ThemeConfig>,
    current_theme_index: usize,
}

impl AppState {
    fn new() -> Self {
        Self {
            themes: create_themes(),
            current_theme_index: 0,
        }
    }

    fn current_theme(&self) -> Theme {
        self.themes[self.current_theme_index].theme.clone()
    }

    fn current_theme_name(&self) -> &str {
        &self.themes[self.current_theme_index].name
    }

    fn current_theme_description(&self) -> &str {
        &self.themes[self.current_theme_index].description
    }

    fn next_theme(&mut self) {
        self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
    }

    fn previous_theme(&mut self) {
        if self.current_theme_index == 0 {
            self.current_theme_index = self.themes.len() - 1;
        } else {
            self.current_theme_index -= 1;
        }
    }

    fn theme_count(&self) -> usize {
        self.themes.len()
    }
}

struct ThemeConfig {
    name: String,
    description: String,
    theme: Theme,
}

/// Widget to showcase colors and themes
struct ColorShowcase<'a> {
    state: &'a AppState,
}

impl<'a> ColorShowcase<'a> {
    fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl Widget for ColorShowcase<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Theme info
                Constraint::Min(0),    // Color showcase
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header with Reset background
        let header = Paragraph::new("Theme Switcher & Color Showcase")
            .alignment(Alignment::Center)
            .bold()
            .bg(Color::Reset)
            .block(Block::bordered().border_style(Color::Cyan).bg(Color::Reset));
        header.render(chunks[0], buf);

        // Theme info
        let theme_info = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Theme Configuration: ", Style::default().fg(Color::Reset)),
                Span::styled(
                    self.state.current_theme_name(),
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::styled(
                    format!(
                        " ({}/{})",
                        self.state.current_theme_index + 1,
                        self.state.theme_count()
                    ),
                    Style::default().fg(Color::Reset),
                ),
            ]),
            Line::from(Span::styled(
                self.state.current_theme_description(),
                Style::default().fg(Color::Reset),
            )),
        ])
        .alignment(Alignment::Center)
        .bg(Color::Reset)
        .block(Block::bordered().bg(Color::Reset));
        theme_info.render(chunks[1], buf);

        // Color showcase
        self.render_color_showcase(chunks[2], buf);

        // Footer
        let footer = Paragraph::new(vec![
            Line::from("Press ↑/↓ or j/k to cycle through theme configurations"),
            Line::from(Span::styled(
                "Note: Currently showcasing color palettes (backend theme switching requires reload)",
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .alignment(Alignment::Center)
        .bg(Color::Reset)
        .block(Block::bordered().border_style(Color::Green).bg(Color::Reset));
        footer.render(chunks[3], buf);
    }
}

impl ColorShowcase<'_> {
    fn render_theme_palette(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title(format!(
            "{} Theme Colors",
            self.state.current_theme_name()
        ));
        let inner = block.inner(area);
        block.render(area, buf);

        // Show a compact preview of the theme's main colors
        let preview_text = Line::from(vec![
            Span::styled("  FG  ", Style::default().fg(Color::Reset).bg(Color::Reset)),
            Span::raw(" "),
            Span::styled(
                "  BG  ",
                Style::default()
                    .fg(Color::Reset)
                    .bg(Color::Reset)
                    .add_modifier(Modifier::REVERSED),
            ),
            Span::raw(" "),
            Span::styled(
                " Red ",
                Style::default()
                    .bg(Color::Indexed(1))
                    .fg(Color::Black),
            ),
            Span::raw(" "),
            Span::styled(
                " Grn ",
                Style::default()
                    .bg(Color::Indexed(2))
                    .fg(Color::Black),
            ),
            Span::raw(" "),
            Span::styled(
                " Blu ",
                Style::default()
                    .bg(Color::Indexed(4))
                    .fg(Color::Black),
            ),
            Span::raw(" "),
            Span::styled(
                " Yel ",
                Style::default()
                    .bg(Color::Indexed(3))
                    .fg(Color::Black),
            ),
            Span::raw(" "),
            Span::styled(
                " Mag ",
                Style::default()
                    .bg(Color::Indexed(5))
                    .fg(Color::Black),
            ),
            Span::raw(" "),
            Span::styled(
                " Cyn ",
                Style::default()
                    .bg(Color::Indexed(6))
                    .fg(Color::Black),
            ),
        ]);

        buf.set_line(inner.x, inner.y, &preview_text, inner.width);
    }

    fn render_color_showcase(&self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Theme palette preview
                Constraint::Length(3), // Basic colors
                Constraint::Length(3), // Bright colors
                Constraint::Length(3), // RGB colors
                Constraint::Min(0),    // Styles showcase
            ])
            .split(area);

        // Theme palette preview
        self.render_theme_palette(chunks[0], buf);

        // Basic ANSI colors (0-7)
        self.render_basic_colors(chunks[1], buf);

        // Bright ANSI colors (8-15)
        self.render_bright_colors(chunks[2], buf);

        // RGB colors
        self.render_rgb_colors(chunks[3], buf);

        // Text styles
        self.render_text_styles(chunks[4], buf);
    }

    fn render_basic_colors(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Basic ANSI Colors (0-7)");
        let inner = block.inner(area);
        block.render(area, buf);

        let colors = [
            ("Black", Color::Indexed(0)),
            ("Red", Color::Indexed(1)),
            ("Green", Color::Indexed(2)),
            ("Yellow", Color::Indexed(3)),
            ("Blue", Color::Indexed(4)),
            ("Magenta", Color::Indexed(5)),
            ("Cyan", Color::Indexed(6)),
            ("White", Color::Indexed(7)),
        ];

        let mut x = inner.x;
        for (name, color) in colors {
            if x + 10 > inner.right() {
                break;
            }
            let style = Style::default().bg(color).fg(Color::Black);
            let span = Span::styled(format!(" {:<7} ", name), style);
            buf.set_line(x, inner.y, &Line::from(span), 10);
            x += 10;
        }
    }

    fn render_bright_colors(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Bright ANSI Colors (8-15)");
        let inner = block.inner(area);
        block.render(area, buf);

        let colors = [
            ("BrBlack", Color::Indexed(8)),
            ("BrRed", Color::Indexed(9)),
            ("BrGreen", Color::Indexed(10)),
            ("BrYellow", Color::Indexed(11)),
            ("BrBlue", Color::Indexed(12)),
            ("BrMagenta", Color::Indexed(13)),
            ("BrCyan", Color::Indexed(14)),
            ("BrWhite", Color::Indexed(15)),
        ];

        let mut x = inner.x;
        for (name, color) in colors {
            if x + 10 > inner.right() {
                break;
            }
            let style = Style::default().bg(color).fg(Color::Black);
            let span = Span::styled(format!(" {:<7} ", name), style);
            buf.set_line(x, inner.y, &Line::from(span), 10);
            x += 10;
        }
    }

    fn render_rgb_colors(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("RGB Colors");
        let inner = block.inner(area);
        block.render(area, buf);

        let colors = [
            ("Red", Color::Rgb(255, 0, 0)),
            ("Orange", Color::Rgb(255, 165, 0)),
            ("Yellow", Color::Rgb(255, 255, 0)),
            ("Green", Color::Rgb(0, 255, 0)),
            ("Cyan", Color::Rgb(0, 255, 255)),
            ("Blue", Color::Rgb(0, 0, 255)),
            ("Purple", Color::Rgb(128, 0, 128)),
            ("Pink", Color::Rgb(255, 192, 203)),
        ];

        let mut x = inner.x;
        for (name, color) in colors {
            if x + 10 > inner.right() {
                break;
            }
            let style = Style::default().bg(color).fg(Color::Black);
            let span = Span::styled(format!(" {:<7} ", name), style);
            buf.set_line(x, inner.y, &Line::from(span), 10);
            x += 10;
        }
    }

    fn render_text_styles(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title("Text Styles");
        let inner = block.inner(area);
        block.render(area, buf);

        let styles = vec![
            Line::from(vec![
                Span::raw("Normal  "),
                Span::styled("Bold", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled("Italic", Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw("  "),
                Span::styled(
                    "Underline",
                    Style::default().add_modifier(Modifier::UNDERLINED),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Crossed",
                    Style::default().add_modifier(Modifier::CROSSED_OUT),
                ),
                Span::raw("  "),
                Span::styled(
                    "Reversed",
                    Style::default().add_modifier(Modifier::REVERSED),
                ),
                Span::raw("  "),
                Span::styled("Red FG", Style::default().fg(Color::Red)),
                Span::raw("  "),
                Span::styled("Blue BG", Style::default().bg(Color::Blue)),
            ]),
            Line::from(vec![
                Span::styled(
                    "Bold+Italic",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::raw("  "),
                Span::styled(
                    "Bold+Underline",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]),
        ];

        for (i, line) in styles.iter().enumerate() {
            if inner.y + i as u16 >= inner.bottom() {
                break;
            }
            buf.set_line(inner.x, inner.y + i as u16, line, inner.width);
        }
    }
}

/// Create predefined themes
fn create_themes() -> Vec<ThemeConfig> {
    vec![
        ThemeConfig {
            name: "Default".to_string(),
            description: "Standard terminal colors".to_string(),
            theme: Theme::default(),
        },
        ThemeConfig {
            name: "Dracula".to_string(),
            description: "Dark theme with purple accents".to_string(),
            theme: Theme::builder()
                .background(Color::Rgb(40, 42, 54))
                .foreground(Color::Rgb(248, 248, 242))
                .palette_color(0, Color::Rgb(40, 42, 54))    // black
                .palette_color(1, Color::Rgb(255, 85, 85))   // red
                .palette_color(2, Color::Rgb(80, 250, 123))  // green
                .palette_color(3, Color::Rgb(241, 250, 140)) // yellow
                .palette_color(4, Color::Rgb(98, 114, 164))  // blue
                .palette_color(5, Color::Rgb(255, 121, 198)) // magenta
                .palette_color(6, Color::Rgb(139, 233, 253)) // cyan
                .palette_color(7, Color::Rgb(248, 248, 242)) // white
                .palette_color(8, Color::Rgb(68, 71, 90))    // bright black
                .palette_color(9, Color::Rgb(255, 110, 103)) // bright red
                .palette_color(10, Color::Rgb(105, 255, 148)) // bright green
                .palette_color(11, Color::Rgb(255, 255, 153)) // bright yellow
                .palette_color(12, Color::Rgb(123, 139, 189)) // bright blue
                .palette_color(13, Color::Rgb(255, 146, 223)) // bright magenta
                .palette_color(14, Color::Rgb(164, 255, 255)) // bright cyan
                .palette_color(15, Color::Rgb(255, 255, 255)) // bright white
                .cursor_color(Color::Rgb(248, 248, 242))
                .cursor_text_color(Color::Rgb(40, 42, 54))
                .selection_background(Color::Rgb(68, 71, 90))
                .selection_foreground(Color::Rgb(248, 248, 242))
                .build(),
        },
        ThemeConfig {
            name: "Nord".to_string(),
            description: "Arctic, north-bluish color palette".to_string(),
            theme: Theme::builder()
                .background(Color::Rgb(46, 52, 64))
                .foreground(Color::Rgb(216, 222, 233))
                .palette_color(0, Color::Rgb(46, 52, 64))     // black
                .palette_color(1, Color::Rgb(191, 97, 106))   // red
                .palette_color(2, Color::Rgb(163, 190, 140))  // green
                .palette_color(3, Color::Rgb(235, 203, 139))  // yellow
                .palette_color(4, Color::Rgb(129, 161, 193))  // blue
                .palette_color(5, Color::Rgb(180, 142, 173))  // magenta
                .palette_color(6, Color::Rgb(136, 192, 208))  // cyan
                .palette_color(7, Color::Rgb(229, 233, 240))  // white
                .palette_color(8, Color::Rgb(76, 86, 106))    // bright black
                .palette_color(9, Color::Rgb(191, 97, 106))   // bright red
                .palette_color(10, Color::Rgb(163, 190, 140)) // bright green
                .palette_color(11, Color::Rgb(235, 203, 139)) // bright yellow
                .palette_color(12, Color::Rgb(129, 161, 193)) // bright blue
                .palette_color(13, Color::Rgb(180, 142, 173)) // bright magenta
                .palette_color(14, Color::Rgb(143, 188, 187)) // bright cyan
                .palette_color(15, Color::Rgb(236, 239, 244)) // bright white
                .cursor_color(Color::Rgb(216, 222, 233))
                .cursor_text_color(Color::Rgb(46, 52, 64))
                .selection_background(Color::Rgb(76, 86, 106))
                .selection_foreground(Color::Rgb(216, 222, 233))
                .build(),
        },
        ThemeConfig {
            name: "Gruvbox Dark".to_string(),
            description: "Retro groove color scheme".to_string(),
            theme: Theme::builder()
                .background(Color::Rgb(40, 40, 40))
                .foreground(Color::Rgb(235, 219, 178))
                .palette_color(0, Color::Rgb(40, 40, 40))     // black
                .palette_color(1, Color::Rgb(204, 36, 29))    // red
                .palette_color(2, Color::Rgb(152, 151, 26))   // green
                .palette_color(3, Color::Rgb(215, 153, 33))   // yellow
                .palette_color(4, Color::Rgb(69, 133, 136))   // blue
                .palette_color(5, Color::Rgb(177, 98, 134))   // magenta
                .palette_color(6, Color::Rgb(104, 157, 106))  // cyan
                .palette_color(7, Color::Rgb(168, 153, 132))  // white
                .palette_color(8, Color::Rgb(146, 131, 116))  // bright black
                .palette_color(9, Color::Rgb(251, 73, 52))    // bright red
                .palette_color(10, Color::Rgb(184, 187, 38))  // bright green
                .palette_color(11, Color::Rgb(250, 189, 47))  // bright yellow
                .palette_color(12, Color::Rgb(131, 165, 152)) // bright blue
                .palette_color(13, Color::Rgb(211, 134, 155)) // bright magenta
                .palette_color(14, Color::Rgb(142, 192, 124)) // bright cyan
                .palette_color(15, Color::Rgb(235, 219, 178)) // bright white
                .cursor_color(Color::Rgb(235, 219, 178))
                .cursor_text_color(Color::Rgb(40, 40, 40))
                .selection_background(Color::Rgb(60, 56, 54))
                .selection_foreground(Color::Rgb(235, 219, 178))
                .build(),
        },
        ThemeConfig {
            name: "Solarized Dark".to_string(),
            description: "Precision colors for machines and people".to_string(),
            theme: Theme::builder()
                .background(Color::Rgb(0, 43, 54))
                .foreground(Color::Rgb(131, 148, 150))
                .palette_color(0, Color::Rgb(7, 54, 66))      // black
                .palette_color(1, Color::Rgb(220, 50, 47))    // red
                .palette_color(2, Color::Rgb(133, 153, 0))    // green
                .palette_color(3, Color::Rgb(181, 137, 0))    // yellow
                .palette_color(4, Color::Rgb(38, 139, 210))   // blue
                .palette_color(5, Color::Rgb(211, 54, 130))   // magenta
                .palette_color(6, Color::Rgb(42, 161, 152))   // cyan
                .palette_color(7, Color::Rgb(238, 232, 213))  // white
                .palette_color(8, Color::Rgb(0, 43, 54))      // bright black
                .palette_color(9, Color::Rgb(203, 75, 22))    // bright red
                .palette_color(10, Color::Rgb(88, 110, 117))  // bright green
                .palette_color(11, Color::Rgb(101, 123, 131)) // bright yellow
                .palette_color(12, Color::Rgb(131, 148, 150)) // bright blue
                .palette_color(13, Color::Rgb(108, 113, 196)) // bright magenta
                .palette_color(14, Color::Rgb(147, 161, 161)) // bright cyan
                .palette_color(15, Color::Rgb(253, 246, 227)) // bright white
                .cursor_color(Color::Rgb(131, 148, 150))
                .cursor_text_color(Color::Rgb(0, 43, 54))
                .selection_background(Color::Rgb(7, 54, 66))
                .selection_foreground(Color::Rgb(131, 148, 150))
                .build(),
        },
        ThemeConfig {
            name: "One Dark".to_string(),
            description: "Atom's iconic One Dark theme".to_string(),
            theme: Theme::builder()
                .background(Color::Rgb(40, 44, 52))
                .foreground(Color::Rgb(171, 178, 191))
                .palette_color(0, Color::Rgb(40, 44, 52))     // black
                .palette_color(1, Color::Rgb(224, 108, 117))  // red
                .palette_color(2, Color::Rgb(152, 195, 121))  // green
                .palette_color(3, Color::Rgb(229, 192, 123))  // yellow
                .palette_color(4, Color::Rgb(97, 175, 239))   // blue
                .palette_color(5, Color::Rgb(198, 120, 221))  // magenta
                .palette_color(6, Color::Rgb(86, 182, 194))   // cyan
                .palette_color(7, Color::Rgb(171, 178, 191))  // white
                .palette_color(8, Color::Rgb(92, 99, 112))    // bright black
                .palette_color(9, Color::Rgb(190, 80, 70))    // bright red
                .palette_color(10, Color::Rgb(152, 195, 121)) // bright green
                .palette_color(11, Color::Rgb(229, 192, 123)) // bright yellow
                .palette_color(12, Color::Rgb(97, 175, 239))  // bright blue
                .palette_color(13, Color::Rgb(198, 120, 221)) // bright magenta
                .palette_color(14, Color::Rgb(86, 182, 194))  // bright cyan
                .palette_color(15, Color::Rgb(255, 255, 255)) // bright white
                .cursor_color(Color::Rgb(171, 178, 191))
                .cursor_text_color(Color::Rgb(40, 44, 52))
                .selection_background(Color::Rgb(61, 66, 77))
                .selection_foreground(Color::Rgb(171, 178, 191))
                .build(),
        },
        ThemeConfig {
            name: "Tokyo Night".to_string(),
            description: "A clean, dark theme inspired by Tokyo".to_string(),
            theme: Theme::builder()
                .background(Color::Rgb(26, 27, 38))
                .foreground(Color::Rgb(169, 177, 214))
                .palette_color(0, Color::Rgb(26, 27, 38))     // black
                .palette_color(1, Color::Rgb(247, 118, 142))  // red
                .palette_color(2, Color::Rgb(158, 206, 106))  // green
                .palette_color(3, Color::Rgb(224, 175, 104))  // yellow
                .palette_color(4, Color::Rgb(125, 207, 255))  // blue
                .palette_color(5, Color::Rgb(187, 154, 247))  // magenta
                .palette_color(6, Color::Rgb(125, 207, 255))  // cyan
                .palette_color(7, Color::Rgb(169, 177, 214))  // white
                .palette_color(8, Color::Rgb(68, 71, 90))     // bright black
                .palette_color(9, Color::Rgb(247, 118, 142))  // bright red
                .palette_color(10, Color::Rgb(158, 206, 106)) // bright green
                .palette_color(11, Color::Rgb(224, 175, 104)) // bright yellow
                .palette_color(12, Color::Rgb(125, 207, 255)) // bright blue
                .palette_color(13, Color::Rgb(187, 154, 247)) // bright magenta
                .palette_color(14, Color::Rgb(125, 207, 255)) // bright cyan
                .palette_color(15, Color::Rgb(192, 202, 245)) // bright white
                .cursor_color(Color::Rgb(169, 177, 214))
                .cursor_text_color(Color::Rgb(26, 27, 38))
                .selection_background(Color::Rgb(68, 71, 90))
                .selection_foreground(Color::Rgb(169, 177, 214))
                .build(),
        },
    ]
}
