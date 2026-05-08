use iced::{
    Background, Border, Color,
    widget::{button, container, pane_grid, scrollable, text_editor},
};

use crate::theme::{ThemeColor, ThemeColors};

#[must_use]
pub fn color(value: ThemeColor) -> Color {
    let (r, g, b) = value.components();
    Color::from_rgb8(r, g, b)
}

#[must_use]
pub fn text_style(value: ThemeColor) -> Color {
    color(value)
}

#[must_use]
pub fn border_color(focused: bool, theme: &ThemeColors) -> Color {
    if focused {
        color(theme.border_focused)
    } else {
        color(theme.border_normal)
    }
}

#[must_use]
pub fn panel_style(
    background: ThemeColor,
    foreground: ThemeColor,
    focused: bool,
    theme: &ThemeColors,
) -> container::Style {
    container::Style::default()
        .background(color(background))
        .color(color(foreground))
        .border(Border {
            color: border_color(focused, theme),
            width: if focused { 1.0 } else { 0.5 },
            radius: 8.0.into(),
        })
}

#[must_use]
pub fn title_bar_style(background: ThemeColor, theme: &ThemeColors) -> container::Style {
    container::Style::default()
        .background(color(background))
        .color(color(theme.sidebar_fg))
        .border(Border {
            color: color(theme.border_normal),
            width: 0.0,
            radius: 8.0.into(),
        })
}

#[must_use]
pub fn status_bar_style(theme: &ThemeColors) -> container::Style {
    container::Style::default()
        .background(color(theme.status_bg))
        .color(color(theme.status_fg))
        .border(Border {
            color: color(theme.status_bg),
            width: 0.0,
            radius: 0.0.into(),
        })
}

#[must_use]
pub fn file_button_style(
    selected: bool,
    theme: &ThemeColors,
    status: button::Status,
) -> button::Style {
    let mut background = if selected {
        color(theme.tree_selected_bg)
    } else {
        color(theme.tree_bg)
    };

    if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        background = if selected {
            color(theme.tree_selected_bg)
        } else {
            color(theme.editor_cursor_line_bg)
        };
    }

    button::Style {
        background: Some(Background::Color(background)),
        text_color: if selected {
            color(theme.tree_selected_fg)
        } else {
            color(theme.tree_fg)
        },
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        ..button::Style::default()
    }
}

#[must_use]
pub fn tool_button_style(
    enabled: bool,
    theme: &ThemeColors,
    status: button::Status,
) -> button::Style {
    let mut background = if enabled {
        color(theme.editor_cursor_line_bg)
    } else {
        color(theme.sidebar_bg)
    };

    if enabled && matches!(status, button::Status::Hovered | button::Status::Pressed) {
        background = color(theme.tree_selected_bg);
    }

    button::Style {
        background: Some(Background::Color(background)),
        text_color: if enabled {
            color(theme.sidebar_fg)
        } else {
            color(theme.border_normal)
        },
        border: Border {
            color: if enabled {
                color(theme.border_normal)
            } else {
                Color::TRANSPARENT
            },
            width: if enabled { 0.5 } else { 0.0 },
            radius: 6.0.into(),
        },
        ..button::Style::default()
    }
}

#[must_use]
pub fn editor_tab_style(focused: bool, theme: &ThemeColors) -> container::Style {
    panel_style(theme.editor_cursor_line_bg, theme.editor_fg, focused, theme)
}

#[must_use]
pub fn editor_gutter_style(theme: &ThemeColors) -> container::Style {
    container::Style::default()
        .background(color(theme.editor_bg))
        .color(color(theme.editor_line_number_fg))
        .border(Border {
            color: color(theme.border_normal),
            width: 0.0,
            radius: 0.0.into(),
        })
}

#[must_use]
pub fn editor_style(
    focused: bool,
    theme: &ThemeColors,
    status: text_editor::Status,
) -> text_editor::Style {
    let border = match status {
        text_editor::Status::Focused { .. } => border_color(true, theme),
        _ => border_color(focused, theme),
    };

    text_editor::Style {
        background: Background::Color(color(theme.editor_bg)),
        border: Border {
            color: border,
            width: if focused { 1.0 } else { 0.5 },
            radius: 8.0.into(),
        },
        placeholder: color(theme.editor_line_number_fg),
        value: color(theme.editor_fg),
        selection: color(theme.editor_selection_bg),
    }
}

#[must_use]
pub fn scrollable_style(theme: &ThemeColors, background: ThemeColor) -> scrollable::Style {
    let scroller = scrollable::Scroller {
        background: Background::Color(color(theme.border_normal)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
    };

    scrollable::Style {
        container: container::Style::default()
            .background(color(background))
            .color(color(theme.sidebar_fg)),
        vertical_rail: scrollable::Rail {
            background: Some(Background::Color(color(background))),
            border: Border::default(),
            scroller,
        },
        horizontal_rail: scrollable::Rail {
            background: Some(Background::Color(color(background))),
            border: Border::default(),
            scroller,
        },
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(color(theme.editor_cursor_line_bg)),
            border: Border::default(),
            shadow: iced::Shadow::default(),
            icon: color(theme.sidebar_fg),
        },
    }
}

#[must_use]
pub fn pane_grid_style(theme: &ThemeColors) -> pane_grid::Style {
    let mut hover = color(theme.editor_selection_bg);
    hover.a = 0.18;

    pane_grid::Style {
        hovered_region: pane_grid::Highlight {
            background: Background::Color(hover),
            border: Border {
                color: color(theme.border_focused),
                width: 1.0,
                radius: 8.0.into(),
            },
        },
        picked_split: pane_grid::Line {
            color: color(theme.border_focused),
            width: 1.0,
        },
        hovered_split: pane_grid::Line {
            color: color(theme.border_normal),
            width: 1.0,
        },
    }
}
