use iced::{
    Element, Font, Length,
    widget::{column, container, row, text, text_editor},
};

use crate::theme::ThemeColors;

use super::{Message, style};

#[must_use]
pub fn view<'a>(
    content: &'a text_editor::Content,
    title: String,
    focused: bool,
    theme: &'a ThemeColors,
) -> Element<'a, Message> {
    let tab_bar = container(
        text(title)
            .font(Font::MONOSPACE)
            .size(14)
            .color(style::text_style(theme.editor_fg)),
    )
    .padding([8, 12])
    .width(Length::Fill)
    .style(move |_| style::editor_tab_style(focused, theme));

    let line_numbers = (1..=content.line_count().max(1))
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let gutter = container(
        text(line_numbers)
            .font(Font::MONOSPACE)
            .size(13)
            .color(style::text_style(theme.editor_line_number_fg)),
    )
    .padding([10, 8])
    .width(56)
    .height(Length::Fill)
    .style(|_| style::editor_gutter_style(theme));

    let editor_theme = theme.clone();
    let editor = text_editor(content)
        .height(Length::Fill)
        .font(Font::MONOSPACE)
        .on_action(Message::EditorAction)
        .placeholder("Abra um arquivo para começar")
        .style(move |_, status| style::editor_style(focused, &editor_theme, status));

    column![
        tab_bar,
        row![gutter, editor].height(Length::Fill).spacing(8)
    ]
    .height(Length::Fill)
    .into()
}
