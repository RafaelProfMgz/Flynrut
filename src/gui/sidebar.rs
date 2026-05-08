use iced::{
    Alignment, Element, Font, Length,
    widget::{button, column, container, row, text},
};

use crate::{app::App, lsp::DiagnosticItem, theme::ThemeColors};

use super::{Message, style};

#[must_use]
pub fn view(app: &App) -> Element<'_, Message> {
    column![
        git_section(app),
        tool_section(app),
        diagnostics_section(
            app.lsp.diagnostics_for(app.editor.current_file()),
            &app.theme
        ),
    ]
    .spacing(10)
    .into()
}

fn git_section(app: &App) -> Element<'_, Message> {
    let theme = &app.theme;
    let git = app.integrations.git.as_ref();

    section_container(
        column![
            section_title("Git", theme),
            git_line(
                "branch",
                git.map_or("não disponível", |state| state.branch.as_str()),
                theme.git_added,
            ),
            git_line(
                "stage",
                &git.map_or_else(|| "0".to_string(), |state| state.staged.to_string()),
                theme.git_added,
            ),
            git_line(
                "unstg",
                &git.map_or_else(|| "0".to_string(), |state| state.unstaged.to_string()),
                theme.git_modified,
            ),
            git_line(
                "untrk",
                &git.map_or_else(|| "0".to_string(), |state| state.untracked.to_string()),
                theme.git_deleted,
            ),
        ]
        .spacing(4),
        theme,
    )
}

fn tool_section(app: &App) -> Element<'_, Message> {
    let theme = &app.theme;
    let tools = app.integrations.tools.iter().fold(
        column![section_title("Ferramentas", theme)].spacing(6),
        |column, tool| {
            let button_theme = theme.clone();
            column.push(
                row![
                    button(
                        text(tool.name)
                            .font(Font::MONOSPACE)
                            .size(13)
                            .color(style::text_style(theme.sidebar_fg)),
                    )
                    .width(Length::Fill)
                    .padding([6, 8])
                    .style(move |_, status| {
                        style::tool_button_style(tool.available, &button_theme, status)
                    })
                    .on_press_maybe(tool.available.then_some(Message::LaunchTool(tool.name))),
                    text(if tool.available { "ok" } else { "off" })
                        .size(12)
                        .color(style::text_style(if tool.available {
                            theme.git_added
                        } else {
                            theme.diag_error
                        })),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
        },
    );

    section_container(
        column![
            tools,
            button(
                text("Reiniciar LSP")
                    .size(13)
                    .color(style::text_style(theme.sidebar_fg))
            )
            .padding([6, 8])
            .style(move |_, status| style::tool_button_style(true, theme, status))
            .on_press(Message::RestartLsp),
            text(format!("Status LSP: {}", app.lsp.status))
                .size(12)
                .color(style::text_style(theme.sidebar_fg)),
        ]
        .spacing(8),
        theme,
    )
}

fn diagnostics_section<'a>(
    diagnostics: &'a [DiagnosticItem],
    theme: &'a ThemeColors,
) -> Element<'a, Message> {
    let content = diagnostics.iter().take(8).fold(
        column![section_title("Diagnósticos", theme)].spacing(4),
        |column, item| {
            let color = match item.severity.as_str() {
                "error" => theme.diag_error,
                "warning" => theme.diag_warning,
                _ => theme.diag_info,
            };

            column.push(
                text(format!(
                    "L{} [{}] {}",
                    item.line, item.severity, item.message
                ))
                .size(12)
                .color(style::text_style(color)),
            )
        },
    );

    section_container(content, theme)
}

fn section_container<'a>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a ThemeColors,
) -> Element<'a, Message> {
    container(content)
        .padding(10)
        .width(Length::Fill)
        .style(|_| style::panel_style(theme.sidebar_bg, theme.sidebar_fg, false, theme))
        .into()
}

fn section_title<'a>(label: &'a str, theme: &ThemeColors) -> iced::widget::Text<'a> {
    text(label)
        .size(14)
        .color(style::text_style(theme.sidebar_fg))
}

fn git_line(
    label: &str,
    value: &str,
    color: crate::theme::ThemeColor,
) -> iced::widget::Text<'static> {
    text(format!("{label:>6} {value}"))
        .font(Font::MONOSPACE)
        .size(12)
        .color(style::text_style(color))
}
