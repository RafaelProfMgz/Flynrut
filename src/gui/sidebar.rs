use iced::{
    Alignment, Element, Font, Length,
    widget::{button, column, container, row, scrollable, text},
};

use crate::{app::App, lsp::DiagnosticItem, theme::ThemeColors};

use super::{Message, style};

#[must_use]
pub fn view(app: &App, settings_open: bool) -> Element<'_, Message> {
    if settings_open {
        settings_view(app)
    } else {
        normal_view(app)
    }
}

fn normal_view(app: &App) -> Element<'_, Message> {
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

fn settings_view(app: &App) -> Element<'_, Message> {
    let theme = &app.theme;

    let content = column![
        // Header
        row![
            text("⚙ Configurações")
                .size(15)
                .color(style::text_style(theme.sidebar_fg)),
            iced::widget::Space::new().width(Length::Fill),
            button(
                text("✕")
                    .size(13)
                    .color(style::text_style(theme.sidebar_fg))
            )
            .padding([4, 8])
            .style(move |_, status| style::tool_button_style(true, theme, status))
            .on_press(Message::ToggleSettings),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        
        // IDE Config
        config_section(
            "IDE — config.toml",
            format!("{}", app.config.config_path.display()),
            Message::OpenConfigFile(app.config.config_path.clone()),
            theme,
        ),
        
        // Keybindings
        config_section(
            "Atalhos — keybindings.toml",
            format!("{}", app.config.keybindings_path.display()),
            Message::OpenConfigFile(app.config.keybindings_path.clone()),
            theme,
        ),
        
        // Themes
        config_section(
            "Temas",
            format!("Tema ativo: {}  |  Dir: {}", app.config.theme, app.config.themes_dir.display()),
            Message::OpenConfigFile(app.config.themes_dir.clone()),
            theme,
        ),
        
        // Extensions
        config_section(
            "Extensões",
            format!("Dir: {}", app.config.extensions_dir.display()),
            Message::OpenConfigFile(app.config.extensions_dir.clone()),
            theme,
        ),
        
        // Docs
        docs_section(app, theme),
        
        // Integrations
        integrations_section(app, theme),
    ]
    .spacing(10)
    .padding(10);

    scrollable(content).height(Length::Fill).into()
}

// Corrigido: info agora recebe String para evitar erro de referência temporária
fn config_section<'a>(
    title: &'a str,
    info: String,
    open_msg: Message,
    theme: &'a ThemeColors,
) -> Element<'a, Message> {
    let button_theme = theme.clone();
    section_container(
        column![
            section_title(title, theme),
            text(info)
                .size(11)
                .color(style::text_style(theme.sidebar_fg)),
            button(
                text("Abrir no editor")
                    .size(12)
                    .color(style::text_style(theme.sidebar_fg))
            )
            .padding([4, 8])
            .style(move |_, status| style::tool_button_style(true, &button_theme, status))
            .on_press(open_msg),
        ]
        .spacing(6),
        theme,
    )
}

fn docs_section<'a>(app: &'a App, theme: &'a ThemeColors) -> Element<'a, Message> {
    let port = app.config.docs.port;
    let button_theme = theme.clone();
    section_container(
        column![
            section_title("Documentação (mdBook)", theme),
            text(format!("Porta configurada: {port}"))
                .size(12)
                .color(style::text_style(theme.sidebar_fg)),
            text("Altere [docs] port no config.toml")
                .size(11)
                .color(style::text_style(theme.sidebar_fg)),
            button(
                text(format!("Servir docs (porta {port})"))
                    .size(12)
                    .color(style::text_style(theme.sidebar_fg))
            )
            .padding([4, 8])
            .style(move |_, status| style::tool_button_style(true, &button_theme, status))
            .on_press(Message::LaunchDocs),
        ]
        .spacing(6),
        theme,
    )
}

fn integrations_section<'a>(app: &'a App, theme: &'a ThemeColors) -> Element<'a, Message> {
    let tools_info = app
        .integrations
        .tools
        .iter()
        .map(|t| {
            format!(
                "{}: {}",
                t.name,
                if t.available { "disponível" } else { "não encontrado" }
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    section_container(
        column![
            section_title("Integrações", theme),
            text(tools_info)
                .size(11)
                .color(style::text_style(theme.sidebar_fg)),
        ]
        .spacing(4),
        theme,
    )
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
                        text(tool.name.clone()) // Clone para garantir ownership
                            .font(Font::MONOSPACE)
                            .size(13)
                            .color(style::text_style(theme.sidebar_fg)),
                    )
                    .width(Length::Fill)
                    .padding([6, 8])
                    .style(move |_, status| {
                        style::tool_button_style(tool.available, &button_theme, status)
                    })
                    .on_press_maybe(tool.available.then_some(Message::LaunchTool(tool.name.clone()))),
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