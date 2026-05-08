use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub config_path: PathBuf,
    pub tools: ToolCommands,
    pub lsp: LspSettings,
    /// Name of the active theme (built-in or custom TOML file stem).
    pub theme: String,
    /// Directory where extension `.toml` files live.
    pub extensions_dir: PathBuf,
    /// Directory where custom theme `.toml` files live.
    pub themes_dir: PathBuf,
    /// Path to the user's `keybindings.toml` file.
    pub keybindings_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ToolCommands {
    pub lazygit: String,
    pub lazydocker: String,
    pub ai: Option<String>,
    pub mcp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LspSettings {
    pub command: String,
    pub enabled: bool,
}

#[derive(Debug, Default, Deserialize)]
struct DiskConfig {
    tools: Option<DiskToolCommands>,
    lsp: Option<DiskLspSettings>,
    theme: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct DiskToolCommands {
    lazygit: Option<String>,
    lazydocker: Option<String>,
    ai: Option<String>,
    mcp: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct DiskLspSettings {
    command: Option<String>,
    enabled: Option<bool>,
}

impl AppConfig {
    /// # Errors
    /// Infallible in practice; errors in env-var parsing are silently ignored.
    pub fn load(workspace_root: &Path) -> Result<Self> {
        let config_path = config_path(workspace_root);
        let from_disk = fs::read_to_string(&config_path)
            .ok()
            .and_then(|content| toml::from_str::<DiskConfig>(&content).ok())
            .unwrap_or_default();

        let tools = ToolCommands {
            lazygit: std::env::var("RUST_IDE_LAZYGIT_COMMAND")
                .ok()
                .or(from_disk
                    .tools
                    .as_ref()
                    .and_then(|tools| tools.lazygit.clone()))
                .unwrap_or_else(|| "lazygit".to_string()),
            lazydocker: std::env::var("RUST_IDE_LAZYDOCKER_COMMAND")
                .ok()
                .or(from_disk
                    .tools
                    .as_ref()
                    .and_then(|tools| tools.lazydocker.clone()))
                .unwrap_or_else(|| "lazydocker".to_string()),
            ai: std::env::var("RUST_IDE_AI_COMMAND")
                .ok()
                .or(from_disk.tools.as_ref().and_then(|tools| tools.ai.clone())),
            mcp: std::env::var("RUST_IDE_MCP_COMMAND")
                .ok()
                .or(from_disk.tools.as_ref().and_then(|tools| tools.mcp.clone())),
        };

        let lsp = LspSettings {
            command: std::env::var("RUST_IDE_LSP_COMMAND")
                .ok()
                .or(from_disk.lsp.as_ref().and_then(|lsp| lsp.command.clone()))
                .unwrap_or_else(|| "rust-analyzer".to_string()),
            enabled: std::env::var("RUST_IDE_LSP_ENABLED")
                .ok()
                .and_then(|value| parse_bool(&value))
                .or(from_disk.lsp.as_ref().and_then(|lsp| lsp.enabled))
                .unwrap_or(true),
        };

        let theme = std::env::var("RUST_IDE_THEME")
            .ok()
            .or(from_disk.theme.clone())
            .unwrap_or_else(|| "dark".to_string());

        let config_dir = config_path
            .parent()
            .map_or_else(|| workspace_root.join(".rust-ide"), Path::to_path_buf);

        Ok(Self {
            config_path,
            tools,
            lsp,
            theme,
            extensions_dir: config_dir.join("extensions"),
            themes_dir: config_dir.join("themes"),
            keybindings_path: config_dir.join("keybindings.toml"),
        })
    }
}

fn config_path(workspace_root: &Path) -> PathBuf {
    if let Some(project_dirs) = ProjectDirs::from("dev", "angel", "rust-ide") {
        return project_dirs.config_dir().join("config.toml");
    }

    workspace_root.join(".rust-ide.toml")
}

pub(crate) fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    // --- parse_bool ---

    #[test]
    fn parse_bool_truthy_values() {
        for val in ["1", "true", "True", "TRUE", "yes", "YES", "on", "ON"] {
            assert_eq!(parse_bool(val), Some(true), "expected true for {val:?}");
        }
    }

    #[test]
    fn parse_bool_falsy_values() {
        for val in ["0", "false", "False", "FALSE", "no", "NO", "off", "OFF"] {
            assert_eq!(parse_bool(val), Some(false), "expected false for {val:?}");
        }
    }

    #[test]
    fn parse_bool_invalid_returns_none() {
        for val in ["", "maybe", "2", "nope", "tru"] {
            assert_eq!(parse_bool(val), None, "expected None for {val:?}");
        }
    }

    #[test]
    fn parse_bool_trims_whitespace() {
        assert_eq!(parse_bool("  true  "), Some(true));
        assert_eq!(parse_bool("  false  "), Some(false));
    }

    // --- AppConfig defaults ---

    #[test]
    fn config_defaults_without_file() {
        // When no config file exists and no env vars are set, defaults kick in.
        // We cannot guarantee env is clean here, so we just confirm load() succeeds.
        use std::path::Path;
        let result = AppConfig::load(Path::new("/tmp"));
        assert!(result.is_ok(), "config load should not fail: {result:?}");
        let cfg = result.unwrap();
        // LSP defaults
        assert!(
            !cfg.lsp.command.is_empty(),
            "lsp.command should have a default"
        );
        // Tools defaults
        assert!(
            !cfg.tools.lazygit.is_empty(),
            "lazygit should have a default"
        );
        assert!(
            !cfg.tools.lazydocker.is_empty(),
            "lazydocker should have a default"
        );
    }
}
