pub mod editor;
pub mod file_tree;
pub mod sidebar;
pub mod style;

use std::{cell::RefCell, path::PathBuf, process::Command, time::Duration};

use anyhow::Result;
use iced::{
    Alignment, Element, Length, Size, Subscription, Task, Theme, keyboard,
    widget::{container, pane_grid, row, text, text_editor},
    window,
};

use crate::{
    app::{App, Focus},
    keybindings::KeybindingsConfig,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaneKind {
    Tree,
    Editor,
    Sidebar,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    PaneClicked(pane_grid::Pane),
    PaneResized(pane_grid::ResizeEvent),
    TreeEntryPressed(usize),
    EditorAction(text_editor::Action),
    SaveRequested,
    RefreshRequested,
    LaunchTool(&'static str),
    CommandFinished(String),
    RestartLsp,
    FocusNext,
    FocusPrev,
    /// Jump directly to a specific pane.
    FocusTree,
    FocusEditor,
    FocusSidebar,
    TreeMoveUp,
    TreeMoveDown,
    TreeOpen,
    /// Toggle the settings panel open/closed.
    ToggleSettings,
    /// Open a config file in the editor.
    OpenConfigFile(PathBuf),
    /// Launch mdbook serve with the configured port.
    LaunchDocs,
    QuitRequested,
    /// No-op: produced by unhandled keyboard events.
    Ignored,
}

#[allow(missing_debug_implementations)]
pub struct IdeApp {
    pub app: App,
    pub editor_content: text_editor::Content,
    pane_state: pane_grid::State<PaneKind>,
    /// Whether the settings panel is currently open.
    pub settings_open: bool,
}

impl IdeApp {
    pub fn new(app: App) -> (Self, Task<Message>) {
        let editor_content = text_editor::Content::with_text(app.editor.contents());
        let pane_state = Self::pane_state();

        (
            Self {
                app,
                editor_content,
                pane_state,
                settings_open: false,
            },
            Self::next_tick(),
        )
    }

    /// # Errors
    /// Returns any Iced runtime error while launching the GUI.
    pub fn run(app: App) -> Result<()> {
        let initial = RefCell::new(Some(Self::new(app)));

        iced::application(
            move || {
                if let Some(state) = initial.borrow_mut().take() {
                    state
                } else {
                    std::process::exit(1);
                }
            },
            Self::update,
            Self::view,
        )
        .title(Self::title)
        .subscription(Self::subscription)
        .theme(Self::theme)
        .window(window::Settings {
            size: Size::new(1400.0, 900.0),
            min_size: Some(Size::new(960.0, 640.0)),
            ..window::Settings::default()
        })
        .centered()
        .antialiasing(true)
        .run()?;

        Ok(())
    }

    #[must_use]
    pub fn title(&self) -> String {
        if let Some(path) = self.app.editor.current_file() {
            format!("rust-ide — {}", path.display())
        } else {
            "rust-ide".to_string()
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                if let Err(error) = self.app.on_tick() {
                    self.app.set_status(format!("Erro no tick: {error}"));
                }
                Self::next_tick()
            }
            Message::PaneClicked(pane) => {
                if let Some(kind) = self.pane_state.get(pane).copied() {
                    self.app.focus = focus_for(kind);
                }
                Task::none()
            }
            Message::PaneResized(event) => {
                self.pane_state.resize(event.split, event.ratio);
                Task::none()
            }
            Message::TreeEntryPressed(index) => {
                let previous = self.app.editor.current_file().map(ToOwned::to_owned);
                let current = self.app.tree.selected_index();
                self.app
                    .tree
                    .move_selection(selection_delta(index, current));

                if let Err(error) = self.app.open_tree_selection() {
                    self.app
                        .set_status(format!("Erro ao abrir arquivo: {error}"));
                }

                if self.app.editor.current_file() != previous.as_deref() {
                    self.sync_editor_from_app();
                }

                Task::none()
            }
            Message::EditorAction(action) => {
                let mutates_content = action.is_edit();
                self.editor_content.perform(action);
                if mutates_content {
                    self.app.editor.set_dirty(true);
                }
                Task::none()
            }
            Message::SaveRequested => {
                let contents = self.editor_content.text();
                if let Err(error) = self.app.save_content(&contents) {
                    self.app.set_status(format!("Erro ao salvar: {error}"));
                }
                Task::none()
            }
            Message::RefreshRequested => {
                if let Err(error) = self.app.refresh() {
                    self.app.set_status(format!("Erro ao atualizar: {error}"));
                }
                Task::none()
            }
            Message::LaunchTool(name) => {
                if let Some(command) = self.app.launch_tool(name) {
                    self.app.set_status(format!("Executando: {command}"));
                    Task::perform(
                        async move { spawn_shell_command(&command) },
                        Message::CommandFinished,
                    )
                } else {
                    Task::none()
                }
            }
            Message::CommandFinished(status) => {
                self.app.set_status(status);
                Task::none()
            }
            Message::RestartLsp => {
                self.app.restart_lsp();
                Task::none()
            }
            Message::FocusNext => {
                self.app.cycle_focus_forward();
                Task::none()
            }
            Message::FocusPrev => {
                self.app.cycle_focus_backward();
                Task::none()
            }
            Message::FocusTree => {
                self.app.focus = crate::app::Focus::Tree;
                Task::none()
            }
            Message::FocusEditor => {
                self.app.focus = crate::app::Focus::Editor;
                Task::none()
            }
            Message::FocusSidebar => {
                self.app.focus = crate::app::Focus::Sidebar;
                Task::none()
            }
            Message::ToggleSettings => {
                self.settings_open = !self.settings_open;
                Task::none()
            }
            Message::OpenConfigFile(path) => {
                if let Err(error) = self.app.open_file(&path) {
                    self.app
                        .set_status(format!("Erro ao abrir config: {error}"));
                } else {
                    self.sync_editor_from_app();
                }
                Task::none()
            }
            Message::LaunchDocs => {
                let port = self.app.config.docs.port;
                let command = format!("mdbook serve --port {port} docs/");
                self.app
                    .set_status(format!("Iniciando docs na porta {port}..."));
                Task::perform(
                    async move { spawn_shell_command(&command) },
                    Message::CommandFinished,
                )
            }
            Message::TreeMoveUp => {
                self.app.tree_move_up();
                Task::none()
            }
            Message::TreeMoveDown => {
                self.app.tree_move_down();
                Task::none()
            }
            Message::TreeOpen => {
                let previous = self.app.editor.current_file().map(ToOwned::to_owned);
                if let Err(error) = self.app.open_tree_selection() {
                    self.app
                        .set_status(format!("Erro ao abrir arquivo: {error}"));
                }
                if self.app.editor.current_file() != previous.as_deref() {
                    self.sync_editor_from_app();
                }
                Task::none()
            }
            Message::QuitRequested => iced::exit(),
            Message::Ignored => Task::none(),
        }
    }

    #[must_use]
    pub fn view(&self) -> Element<'_, Message> {
        let theme = &self.app.theme;

        let grid = pane_grid::PaneGrid::new(&self.pane_state, move |_pane, kind, _| {
            let focused = self.app.focus == focus_for(*kind);
            let body = match kind {
                PaneKind::Tree => file_tree::view(&self.app.tree, theme),
                PaneKind::Editor => editor::view(
                    &self.editor_content,
                    self.app.editor.title(),
                    focused,
                    theme,
                ),
                PaneKind::Sidebar => sidebar::view(&self.app, self.settings_open),
            };

            let title = match kind {
                PaneKind::Tree => "Arquivos",
                PaneKind::Editor => "Editor",
                PaneKind::Sidebar => "Sidebar",
            };
            let panel_theme = theme.clone();

            pane_grid::Content::new(body)
                .title_bar(
                    pane_grid::TitleBar::new(
                        text(title)
                            .size(13)
                            .color(style::text_style(theme.sidebar_fg)),
                    )
                    .padding([8, 10])
                    .style(|_| style::title_bar_style(theme.editor_bg, theme)),
                )
                .style(move |_| match kind {
                    PaneKind::Tree => style::panel_style(
                        panel_theme.tree_bg,
                        panel_theme.tree_fg,
                        focused,
                        &panel_theme,
                    ),
                    PaneKind::Editor => style::panel_style(
                        panel_theme.editor_bg,
                        panel_theme.editor_fg,
                        focused,
                        &panel_theme,
                    ),
                    PaneKind::Sidebar => style::panel_style(
                        panel_theme.sidebar_bg,
                        panel_theme.sidebar_fg,
                        focused,
                        &panel_theme,
                    ),
                })
        })
        .spacing(8)
        .on_click(Message::PaneClicked)
        .on_resize(10, Message::PaneResized)
        .style(|_| style::pane_grid_style(theme))
        .width(Length::Fill)
        .height(Length::Fill);

        let status_bar = container(
            row![
                text(format!("status: {}", self.app.status_message))
                    .size(13)
                    .color(style::text_style(theme.status_fg)),
                container(
                    row![
                        action_button("Salvar", Message::SaveRequested, theme),
                        action_button("Atualizar", Message::RefreshRequested, theme),
                        action_button("Configurações", Message::ToggleSettings, theme),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                )
                .width(Length::Shrink),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        )
        .padding([6, 12])
        .width(Length::Fill)
        .style(|_| style::status_bar_style(theme));

        iced::widget::column![grid, status_bar]
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let kb = self.app.keybindings.clone();
        keyboard::listen().with(kb).map(|(kb, event)| match event {
            keyboard::Event::KeyPressed { key, modifiers, .. } => {
                key_to_message(&key, modifiers, &kb).unwrap_or(Message::Ignored)
            }
            _ => Message::Ignored,
        })
    }

    #[must_use]
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn sync_editor_from_app(&mut self) {
        self.editor_content = text_editor::Content::with_text(self.app.editor.contents());
    }

    fn pane_state() -> pane_grid::State<PaneKind> {
        let (mut state, tree) = pane_grid::State::new(PaneKind::Tree);

        if let Some((editor, left_split)) =
            state.split(pane_grid::Axis::Vertical, tree, PaneKind::Editor)
        {
            state.resize(left_split, 0.22);

            if let Some((_sidebar, right_split)) =
                state.split(pane_grid::Axis::Vertical, editor, PaneKind::Sidebar)
            {
                state.resize(right_split, 0.692_307_7);
            }
        }

        state
    }

    fn next_tick() -> Task<Message> {
        Task::perform(
            async {
                std::thread::sleep(Duration::from_millis(250));
            },
            |()| Message::Tick,
        )
    }
}

fn focus_for(kind: PaneKind) -> Focus {
    match kind {
        PaneKind::Tree => Focus::Tree,
        PaneKind::Editor => Focus::Editor,
        PaneKind::Sidebar => Focus::Sidebar,
    }
}

fn action_button<'a>(
    label: &'a str,
    message: Message,
    theme: &'a crate::theme::ThemeColors,
) -> iced::widget::Button<'a, Message> {
    iced::widget::button(
        text(label)
            .size(12)
            .color(style::text_style(theme.status_fg)),
    )
    .padding([4, 8])
    .style(move |_, status| style::tool_button_style(true, theme, status))
    .on_press(message)
}

fn selection_delta(target: usize, current: usize) -> isize {
    if target >= current {
        match isize::try_from(target - current) {
            Ok(delta) => delta,
            Err(_) => isize::MAX,
        }
    } else {
        match isize::try_from(current - target) {
            Ok(delta) => -delta,
            Err(_) => isize::MIN + 1,
        }
    }
}

fn spawn_shell_command(command: &str) -> String {
    let detached_command = format!("({command}) &");
    match Command::new("sh").arg("-lc").arg(&detached_command).spawn() {
        Ok(_) => format!("Comando iniciado: {command}"),
        Err(error) => format!("Falha ao iniciar comando: {error}"),
    }
}

/// Convert an Iced key event into a normalized string like `"Ctrl+S"`, `"Tab"`, `"g"`.
/// Returns an empty string for unrecognized keys.
fn iced_key_str(key: &keyboard::Key, mods: keyboard::Modifiers) -> String {
    let mut parts: Vec<&str> = Vec::new();
    if mods.control() {
        parts.push("Ctrl");
    }
    if mods.shift() {
        parts.push("Shift");
    }
    if mods.alt() {
        parts.push("Alt");
    }

    match key {
        keyboard::Key::Character(c) => parts.push(c.as_str()),
        keyboard::Key::Named(n) => {
            let name = match n {
                keyboard::key::Named::Tab => "Tab",
                keyboard::key::Named::Enter => "Enter",
                keyboard::key::Named::ArrowUp => "Up",
                keyboard::key::Named::ArrowDown => "Down",
                keyboard::key::Named::ArrowLeft => "Left",
                keyboard::key::Named::ArrowRight => "Right",
                keyboard::key::Named::Escape => "Escape",
                keyboard::key::Named::Backspace => "Backspace",
                keyboard::key::Named::Delete => "Delete",
                keyboard::key::Named::Home => "Home",
                keyboard::key::Named::End => "End",
                keyboard::key::Named::PageUp => "PageUp",
                keyboard::key::Named::PageDown => "PageDown",
                keyboard::key::Named::F1 => "F1",
                keyboard::key::Named::F2 => "F2",
                keyboard::key::Named::F3 => "F3",
                keyboard::key::Named::F4 => "F4",
                keyboard::key::Named::F5 => "F5",
                keyboard::key::Named::F6 => "F6",
                keyboard::key::Named::F7 => "F7",
                keyboard::key::Named::F8 => "F8",
                keyboard::key::Named::F9 => "F9",
                keyboard::key::Named::F10 => "F10",
                keyboard::key::Named::F11 => "F11",
                keyboard::key::Named::F12 => "F12",
                _ => return String::new(),
            };
            parts.push(name);
        }
        keyboard::Key::Unidentified => return String::new(),
    }

    parts.join("+")
}

fn key_to_message(
    key: &keyboard::Key,
    mods: keyboard::Modifiers,
    bindings: &KeybindingsConfig,
) -> Option<Message> {
    let key_str = iced_key_str(key, mods);
    if key_str.is_empty() {
        return None;
    }

    // Global bindings (always active)
    if bindings.global.save.matches(&key_str) {
        return Some(Message::SaveRequested);
    }
    if bindings.global.refresh.matches(&key_str) {
        return Some(Message::RefreshRequested);
    }
    if bindings.global.quit.matches(&key_str) {
        return Some(Message::QuitRequested);
    }
    if bindings.global.focus_next.matches(&key_str) {
        return Some(Message::FocusNext);
    }
    if bindings.global.focus_prev.matches(&key_str) {
        return Some(Message::FocusPrev);
    }
    if bindings.global.focus_tree.matches(&key_str) {
        return Some(Message::FocusTree);
    }
    if bindings.global.focus_editor.matches(&key_str) {
        return Some(Message::FocusEditor);
    }
    if bindings.global.focus_sidebar.matches(&key_str) {
        return Some(Message::FocusSidebar);
    }
    if bindings.global.open_settings.matches(&key_str) {
        return Some(Message::ToggleSettings);
    }

    // Tool launch bindings
    if bindings.tools.lazygit.matches(&key_str) {
        return Some(Message::LaunchTool("lazygit"));
    }
    if bindings.tools.lazydocker.matches(&key_str) {
        return Some(Message::LaunchTool("lazydocker"));
    }
    if bindings.tools.ai.matches(&key_str) {
        return Some(Message::LaunchTool("ai"));
    }
    if bindings.tools.mcp.matches(&key_str) {
        return Some(Message::LaunchTool("mcp"));
    }
    if bindings.tools.restart_lsp.matches(&key_str) {
        return Some(Message::RestartLsp);
    }

    // Tree navigation bindings
    if bindings.tree.move_up.matches(&key_str) {
        return Some(Message::TreeMoveUp);
    }
    if bindings.tree.move_down.matches(&key_str) {
        return Some(Message::TreeMoveDown);
    }
    if bindings.tree.open.matches(&key_str) {
        return Some(Message::TreeOpen);
    }

    None
}
