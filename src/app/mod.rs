use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
    config::AppConfig, editor::EditorModel, extensions::ExtensionRegistry, fs_tree::FileTree,
    integrations::IntegrationState, keybindings::KeybindingsConfig, lsp::LspClient,
    theme::ThemeColors,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Tree,
    Editor,
    Sidebar,
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
    pub keybindings: KeybindingsConfig,
}

impl App {
    /// # Errors
    /// Returns an error if the file tree cannot be initialized.
    pub fn new(workspace_root: PathBuf, config: AppConfig) -> Result<Self> {
        let tree = FileTree::new(&workspace_root)?;
        let integrations = IntegrationState::discover(&workspace_root, &config);
        let mut lsp = LspClient::new(workspace_root.clone(), config.lsp.command.clone());

        let base = ThemeColors::builtin(&config.theme);
        let custom_theme_file = config.themes_dir.join(format!("{}.toml", config.theme));
        let theme = if custom_theme_file.exists() {
            ThemeColors::load_from_file(&custom_theme_file, base.clone()).unwrap_or(base)
        } else {
            base
        };

        let extensions =
            ExtensionRegistry::load_from_dir(&config.extensions_dir).unwrap_or_default();

        let keybindings =
            KeybindingsConfig::load_from_file(&config.keybindings_path).unwrap_or_default();

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
            keybindings,
        })
    }

    /// # Errors
    /// Propagates tick-level errors (currently infallible).
    pub fn on_tick(&mut self) -> Result<()> {
        self.integrations.refresh(&self.workspace_root);
        self.lsp.drain();
        Ok(())
    }

    pub fn cycle_focus_forward(&mut self) {
        self.focus = match self.focus {
            Focus::Tree => Focus::Editor,
            Focus::Editor => Focus::Sidebar,
            Focus::Sidebar => Focus::Tree,
        };
    }

    pub fn cycle_focus_backward(&mut self) {
        self.focus = match self.focus {
            Focus::Tree => Focus::Sidebar,
            Focus::Editor => Focus::Tree,
            Focus::Sidebar => Focus::Editor,
        };
    }

    pub fn tree_move_up(&mut self) {
        self.tree.move_selection(-1);
    }

    pub fn tree_move_down(&mut self) {
        self.tree.move_selection(1);
    }

    /// # Errors
    /// Returns any error while opening the selected entry.
    pub fn open_tree_selection(&mut self) -> Result<()> {
        if let Some(path) = self.tree.activate_selected() {
            self.open_file(&path)?;
        }
        Ok(())
    }

    /// # Errors
    /// Returns any error while opening or synchronizing the file.
    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        self.editor.open(path)?;
        self.lsp.sync_document(path, self.editor.contents())?;
        self.focus = Focus::Editor;
        self.status_message = format!("Arquivo aberto: {}", path.display());
        Ok(())
    }

    /// # Errors
    /// Returns any error while saving the current file.
    pub fn save_file(&mut self) -> Result<()> {
        let contents = self.editor.contents().to_owned();
        let path = self.editor.save()?;
        self.lsp.save_document(&path, &contents)?;
        self.tree.refresh()?;
        self.status_message = format!("Arquivo salvo: {}", path.display());
        Ok(())
    }

    /// # Errors
    /// Returns any error while saving GUI-provided content.
    pub fn save_content(&mut self, text: &str) -> Result<()> {
        let path = self.editor.save_content(text)?;
        self.lsp.save_document(&path, self.editor.contents())?;
        self.tree.refresh()?;
        self.status_message = format!("Arquivo salvo: {}", path.display());
        Ok(())
    }

    /// # Errors
    /// Returns any refresh error from the tree.
    pub fn refresh(&mut self) -> Result<()> {
        self.tree.refresh()?;
        self.integrations
            .refresh_now(&self.workspace_root, &self.config.tools);
        self.lsp.drain();
        self.status_message = format!("Workspace atualizada: {}", self.workspace_root.display());
        Ok(())
    }

    #[must_use]
    pub fn launch_tool(&mut self, name: &str) -> Option<String> {
        let command = self
            .integrations
            .tools
            .iter()
            .find(|tool| tool.name == name)
            .and_then(|tool| tool.command.clone());

        if command.is_none() {
            self.status_message = format!(
                "Ferramenta {name} não configurada. Ajuste {}",
                self.config.config_path.display()
            );
        }

        command
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_message = msg;
    }

    pub fn restart_lsp(&mut self) {
        if self.lsp.is_running() {
            self.status_message = format!("LSP ativo: {}", self.lsp.status);
        } else if let Err(error) = self.lsp.start() {
            self.status_message = format!("Falha ao iniciar LSP: {error}");
        } else {
            self.status_message = "LSP reconectado.".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_app() -> (TempDir, App, PathBuf) {
        let dir = TempDir::new().expect("tempdir");
        let root = dir.path();
        let src_dir = root.join("src");
        fs::create_dir(&src_dir).expect("create src");
        let main_rs = src_dir.join("main.rs");
        fs::write(&main_rs, "fn main() {}\n").expect("write file");

        let mut config = AppConfig::load(root).expect("load config");
        config.lsp.enabled = false;

        let app = App::new(root.to_path_buf(), config).expect("create app");
        (dir, app, main_rs)
    }

    #[test]
    fn cycle_focus_forward_wraps() {
        let (_dir, mut app, _file) = make_app();
        assert_eq!(app.focus, Focus::Tree);
        app.cycle_focus_forward();
        assert_eq!(app.focus, Focus::Editor);
        app.cycle_focus_forward();
        assert_eq!(app.focus, Focus::Sidebar);
        app.cycle_focus_forward();
        assert_eq!(app.focus, Focus::Tree);
    }

    #[test]
    fn cycle_focus_backward_wraps() {
        let (_dir, mut app, _file) = make_app();
        app.cycle_focus_backward();
        assert_eq!(app.focus, Focus::Sidebar);
        app.cycle_focus_backward();
        assert_eq!(app.focus, Focus::Editor);
        app.cycle_focus_backward();
        assert_eq!(app.focus, Focus::Tree);
    }

    #[test]
    fn launch_tool_returns_git_command() {
        let (_dir, mut app, _file) = make_app();
        assert_eq!(app.launch_tool("git"), Some("git".to_string()));
    }

    #[test]
    fn launch_tool_sets_status_for_unknown_tool() {
        let (_dir, mut app, _file) = make_app();
        assert!(app.launch_tool("missing-tool").is_none());
        assert!(
            app.status_message
                .contains("Ferramenta missing-tool não configurada")
        );
    }

    #[test]
    fn save_content_updates_disk_and_editor_state() {
        let (_dir, mut app, main_rs) = make_app();
        app.open_file(&main_rs).expect("open file");
        app.editor.set_dirty(true);

        app.save_content("fn changed() {}\n").expect("save content");

        assert_eq!(
            fs::read_to_string(&main_rs).expect("read file"),
            "fn changed() {}\n"
        );
        assert_eq!(app.editor.contents(), "fn changed() {}\n");
        assert!(!app.editor.dirty);
    }
}
