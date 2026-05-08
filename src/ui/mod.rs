use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{
    app::{App, Focus},
    fs_tree::VisibleEntry,
    theme::ThemeColors,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(frame.area());
    let body = layout[0];
    let status_bar = layout[1];

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(24),
            Constraint::Percentage(48),
            Constraint::Percentage(28),
        ])
        .split(body);
    let tree_area = panels[0];
    let editor_area = panels[1];
    let side_area = panels[2];

    render_tree(frame, app, tree_area);
    frame.render_widget(app.editor.textarea(app.focus == Focus::Editor), editor_area);
    render_sidebar(frame, app, side_area);
    render_status(frame, app, status_bar);
}

fn render_tree(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let theme = &app.theme;
    let entries = app.tree.visible_entries();
    let lines = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| entry_line(entry, index == app.tree.selected_index(), theme))
        .collect::<Vec<_>>();

    frame.render_widget(
        Paragraph::new(lines)
            .block(panel_block("Arquivos", app.focus == Focus::Tree, theme))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn entry_line<'a>(entry: &VisibleEntry, selected: bool, theme: &ThemeColors) -> Line<'a> {
    let indent = "  ".repeat(entry.depth);
    let icon = if entry.is_dir {
        if entry.expanded { "▾" } else { "▸" }
    } else {
        "•"
    };

    let style = if selected {
        Style::default()
            .fg(theme.tree_selected_fg.to_ratatui())
            .bg(theme.tree_selected_bg.to_ratatui())
            .add_modifier(Modifier::BOLD)
    } else if entry.is_dir {
        Style::default().fg(theme.tree_dir_fg.to_ratatui())
    } else {
        Style::default().fg(theme.tree_fg.to_ratatui())
    };

    Line::from(Span::styled(
        format!("{indent}{icon} {}", entry.label),
        style,
    ))
}

fn render_sidebar(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let theme = &app.theme;
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(9),
            Constraint::Min(0),
        ])
        .split(area);
    let git_area = sections[0];
    let tools_area = sections[1];
    let diag_area = sections[2];

    let git_lines = if let Some(git) = &app.integrations.git {
        vec![
            Line::from(format!("repo  {}", git.repository_root.display())),
            Line::from(Span::styled(
                format!("branch {}", git.branch),
                Style::default().fg(theme.git_added.to_ratatui()),
            )),
            Line::from(Span::styled(
                format!("stage {}", git.staged),
                Style::default().fg(theme.git_added.to_ratatui()),
            )),
            Line::from(Span::styled(
                format!("unstg {}", git.unstaged),
                Style::default().fg(theme.git_modified.to_ratatui()),
            )),
            Line::from(Span::styled(
                format!("untrk {}", git.untracked),
                Style::default().fg(theme.git_deleted.to_ratatui()),
            )),
            Line::from(format!("sync  +{} / -{}", git.ahead, git.behind)),
        ]
    } else {
        vec![Line::from("repositório git não encontrado")]
    };

    let tool_lines = app
        .integrations
        .tools
        .iter()
        .map(|tool| {
            let status = if tool.available { "ok" } else { "off" };
            let command = tool.command.as_deref().unwrap_or("não configurado");
            let color = if tool.available {
                theme.git_added.to_ratatui()
            } else {
                theme.diag_error.to_ratatui()
            };
            Line::from(vec![
                Span::styled(
                    format!("{:10} ", tool.name),
                    Style::default().fg(theme.sidebar_fg.to_ratatui()),
                ),
                Span::styled(format!("{status:3} "), Style::default().fg(color)),
                Span::raw(command.to_string()),
            ])
        })
        .chain([
            Line::from(format!("lsp        {}", app.lsp.status)),
            Line::from(format!(
                "config     {}",
                app.integrations.config_path.display()
            )),
        ])
        .collect::<Vec<_>>();

    let diagnostics = app.lsp.diagnostics_for(app.editor.current_file());
    let diagnostic_lines = if diagnostics.is_empty() {
        vec![Line::from("sem diagnósticos para o arquivo atual")]
    } else {
        diagnostics
            .iter()
            .take(8)
            .map(|item| {
                let color = match item.severity.as_str() {
                    "error" => theme.diag_error.to_ratatui(),
                    "warning" => theme.diag_warning.to_ratatui(),
                    _ => theme.diag_info.to_ratatui(),
                };
                Line::from(Span::styled(
                    format!("L{} [{}] {}", item.line, item.severity, item.message),
                    Style::default().fg(color),
                ))
            })
            .collect::<Vec<_>>()
    };

    frame.render_widget(
        Paragraph::new(git_lines)
            .block(panel_block("Git", app.focus == Focus::Sidebar, theme))
            .wrap(Wrap { trim: true }),
        git_area,
    );
    frame.render_widget(
        Paragraph::new(tool_lines)
            .block(panel_block("Integrações", false, theme))
            .wrap(Wrap { trim: false }),
        tools_area,
    );
    frame.render_widget(
        Paragraph::new(diagnostic_lines)
            .block(panel_block("LSP diagnósticos", false, theme))
            .wrap(Wrap { trim: false }),
        diag_area,
    );
}

fn render_status(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let theme = &app.theme;
    let left = Span::styled(
        app.status_message.as_str(),
        Style::default()
            .fg(theme.status_fg.to_ratatui())
            .bg(theme.status_bg.to_ratatui()),
    );
    let right = Span::styled(
        "q sair | tab foco | enter abrir | ctrl+s salvar | g/d/a/m ferramentas | l lsp",
        Style::default().fg(theme.border_normal.to_ratatui()),
    );

    frame.render_widget(
        Paragraph::new(vec![Line::from(vec![left]), Line::from(vec![right])])
            .block(Block::default().borders(Borders::TOP)),
        area,
    );
}

fn panel_block<'a>(title: &str, focused: bool, theme: &ThemeColors) -> Block<'a> {
    let border_style = if focused {
        Style::default()
            .fg(theme.border_focused.to_ratatui())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.border_normal.to_ratatui())
    };

    Block::default()
        .title(title.to_string())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style)
}
