use iced::{
    Element, Font, Length,
    widget::{button, column, container, scrollable, text},
};

use crate::{fs_tree::FileTree, theme::ThemeColors};

use super::{Message, style};

#[must_use]
pub fn view<'a>(tree: &'a FileTree, theme: &'a ThemeColors) -> Element<'a, Message> {
    let entries = tree.visible_entries();
    let selected = tree.selected_index();

    let items = entries.iter().enumerate().fold(
        column!().spacing(2).padding([4, 6]),
        |column, (index, entry)| {
            let icon = if entry.is_dir {
                if entry.expanded { "📂" } else { "📁" }
            } else {
                "📄"
            };
            let label = format!("{}{} {}", "  ".repeat(entry.depth), icon, entry.label);
            let is_selected = index == selected;
            let button_theme = theme.clone();

            column.push(
                button(
                    text(label)
                        .font(Font::MONOSPACE)
                        .size(15)
                        .color(if entry.is_dir {
                            style::text_style(theme.tree_dir_fg)
                        } else if is_selected {
                            style::text_style(theme.tree_selected_fg)
                        } else {
                            style::text_style(theme.tree_fg)
                        }),
                )
                .width(Length::Fill)
                .padding([6, 8])
                .style(move |_, status| {
                    style::file_button_style(is_selected, &button_theme, status)
                })
                .on_press(Message::TreeEntryPressed(index)),
            )
        },
    );

    scrollable(container(items).width(Length::Fill))
        .style(move |_, _status| style::scrollable_style(theme, theme.tree_bg))
        .into()
}
