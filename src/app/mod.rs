use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    config::AppConfig, editor::EditorModel, extensions::ExtensionRegistry, fs_tree::FileTree,
    integrations::IntegrationState, lsp::LspClient, theme::ThemeColors,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Tree,
    Editor,
    Sidebar,
}

#[derive(Debug)]
pub enum AppAction {
    Continue,
    Quit,
    RunCommand(String),
}

#[derive(Debug)]
pub struct App {
    pub workspace_root: PathBuf,
    pub config: AppConfig,
    pub tree: FileTree,
    pub editor: EditorModel,
    pub integrations: IntegrationState,
    pub lsp: LspClient,
    pub focus: Focus,
    pub status_message: String,
    pub theme: ThemeColors,
    pub extensions: ExtensionRegistry,
}

impl App {
    /// # Errors
    /// Returns an error if the file tree or LSP cannot be initialized.
    pub fn new(workspace_root: PathBuf, config: AppConfig) -> Result<Self> {
        let tree = FileTree::new(&workspace_root)?;
        let integrations = IntegrationState::discover(&workspace_root, &config);
        let mut lsp = LspClient::new(workspace_root.clone(), config.lsp.command.clone());

        // Load theme: try custom file, fall back to built-in.
        let base = ThemeColors::builtin(&config.theme);
        let custom_theme_file = config.themes_dir.join(format!("{}.toml", config.theme));
        let theme = if custom_theme_file.exists() {
            ThemeColors::load_from_file(&custom_theme_file, base.clone()).unwrap_or(base)
        } else {
            base
        };

        // Load extensions (non-fatal).
        let extensions =
            ExtensionRegistry::load_from_dir(&config.extensions_dir).unwrap_or_default();

        let status_message = if config.lsp.enabled {
            if let Err(error) = lsp.start() {
                format!("IDE pronta. LSP indisponível: {error}")
            } else {
                "IDE pronta. LSP conectado.".to_string()
            }
        } else {
            "IDE pronta. LSP desabilitado por configuração.".to_string()
        };

        Ok(Self {
            workspace_root,
            config,
            tree,
            editor: EditorModel::new(),
            integrations,
            lsp,
            focus: Focus::Tree,
            status_message,
            theme,
            extensions,
        })
    }

    /// # Errors
    /// Returns an error if a save or refresh operation fails.
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<AppAction> {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            return match key.code {
                KeyCode::Char('s') => {
                    if let Err(error) = self.save_current_file() {
                        self.status_message = format!("Erro ao salvar: {error}");
                    }
                    Ok(AppAction::Continue)
                }
                KeyCode::Char('r') => {
                    if let Err(error) = self.refresh() {
                        self.status_message = format!("Erro ao atualizar: {error}");
                    }
                    Ok(AppAction::Continue)
                }
                _ => Ok(AppAction::Continue),
            };
        }

        match key.code {
            KeyCode::Char('q') => return Ok(AppAction::Quit),
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Tree => Focus::Editor,
                    Focus::Editor => Focus::Sidebar,
                    Focus::Sidebar => Focus::Tree,
                };
                return Ok(AppAction::Continue);
            }
            KeyCode::BackTab => {
                self.focus = match self.focus {
                    Focus::Tree => Focus::Sidebar,
                    Focus::Editor => Focus::Tree,
                    Focus::Sidebar => Focus::Editor,
                };
                return Ok(AppAction::Continue);
            }
            KeyCode::Char('g') => return Ok(self.launch_named_command("lazygit")),
            KeyCode::Char('d') => return Ok(self.launch_named_command("lazydocker")),
            KeyCode::Char('a') => return Ok(self.launch_named_command("ai")),
            KeyCode::Char('m') => return Ok(self.launch_named_command("mcp")),
            KeyCode::Char('l') => {
                if self.lsp.is_running() {
                    self.status_message = format!("LSP ativo: {}", self.lsp.status);
                } else if let Err(error) = self.lsp.start() {
                    self.status_message = format!("Falha ao iniciar LSP: {error}");
                } else {
                    self.status_message = "LSP reconectado.".to_string();
                }
                return Ok(AppAction::Continue);
            }
            _ => {}
        }

        match self.focus {
            Focus::Tree => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.tree.move_selection(-1),
                KeyCode::Down | KeyCode::Char('j') => self.tree.move_selection(1),
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('o') => {
                    if let Err(error) = self.open_tree_selection() {
                        self.status_message = format!("Erro ao abrir arquivo: {error}");
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.tree.move_selection(-1);
                }
                _ => {}
            },
            Focus::Editor => self.editor.handle_key(key),
            Focus::Sidebar => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.tree.move_selection(-1),
                KeyCode::Down | KeyCode::Char('j') => self.tree.move_selection(1),
                _ => {}
            },
        }

        Ok(AppAction::Continue)
    }

    /// # Errors
    /// Propagates tick-level errors (currently infallible).
    pub fn on_tick(&mut self) -> Result<()> {
        self.integrations.refresh(&self.workspace_root);
        self.lsp.drain();
        Ok(())
    }

    /// # Errors
    /// Returns any error from the command result or subsequent refresh.
    pub fn after_command(&mut self, status: Result<String>) -> Result<()> {
        self.status_message = status?;
        self.refresh()
    }

    fn refresh(&mut self) -> Result<()> {
        self.tree.refresh()?;
        self.integrations
            .refresh_now(&self.workspace_root, &self.config.tools);
        self.lsp.drain();
        self.status_message = format!("Workspace atualizada: {}", self.workspace_root.display());
        Ok(())
    }

    fn open_tree_selection(&mut self) -> Result<()> {
        if let Some(path) = self.tree.activate_selected() {
            self.editor.open(&path)?;
            self.lsp.sync_document(&path, &self.editor.contents())?;
            self.focus = Focus::Editor;
            self.status_message = format!("Arquivo aberto: {}", path.display());
        }
        Ok(())
    }

    fn save_current_file(&mut self) -> Result<()> {
        let path = self.editor.save()?;
        let contents = self.editor.contents();
        self.lsp.save_document(&path, &contents)?;
        self.tree.refresh()?;
        self.status_message = format!("Arquivo salvo: {}", path.display());
        Ok(())
    }

    fn launch_named_command(&mut self, name: &str) -> AppAction {
        let command = self
            .integrations
            .tools
            .iter()
            .find(|tool| tool.name == name)
            .and_then(|tool| tool.command.clone());

        if let Some(command) = command {
            AppAction::RunCommand(command)
        } else {
            self.status_message = format!(
                "Ferramenta {name} não configurada. Ajuste {}",
                self.config.config_path.display()
            );
            AppAction::Continue
        }
    }
}
