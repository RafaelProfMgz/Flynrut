//! Keybindings configuration loaded from `~/.config/rust-ide/keybindings.toml`.
//!
//! Users can override any binding — all fields are optional in the file.
//! Unset fields inherit the compiled-in defaults.

use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

// ── Public types ────────────────────────────────────────────────────────────

/// One key string or a list of alternative key strings.
///
/// In TOML:
/// ```toml
/// save      = "Ctrl+S"          # single
/// move_up   = ["Up", "k"]       # multiple alternatives
/// ```
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum KeyList {
    Single(String),
    Multiple(Vec<String>),
}

impl KeyList {
    /// Returns all key strings in this list.
    #[must_use]
    pub fn keys(&self) -> Vec<&str> {
        match self {
            Self::Single(k) => vec![k.as_str()],
            Self::Multiple(keys) => keys.iter().map(String::as_str).collect(),
        }
    }

    /// Returns `true` if `key_str` matches any entry (case-insensitive).
    #[must_use]
    pub fn matches(&self, key_str: &str) -> bool {
        self.keys()
            .iter()
            .any(|k| k.eq_ignore_ascii_case(key_str))
    }
}

/// Global keybindings (active regardless of which pane has focus).
#[derive(Debug, Clone, Deserialize, Hash)]
pub struct GlobalKeys {
    /// Save the current file.
    pub save: KeyList,
    /// Refresh workspace (tree + git + LSP).
    pub refresh: KeyList,
    /// Quit the IDE.
    pub quit: KeyList,
    /// Move focus to the next pane (Tree → Editor → Sidebar → ...).
    pub focus_next: KeyList,
    /// Move focus to the previous pane.
    pub focus_prev: KeyList,
    /// Jump directly to the file tree pane.
    pub focus_tree: KeyList,
    /// Jump directly to the editor pane.
    pub focus_editor: KeyList,
    /// Jump directly to the sidebar pane.
    pub focus_sidebar: KeyList,
    /// Open / close the settings panel.
    pub open_settings: KeyList,
}

/// Keybindings that launch external tools.
#[derive(Debug, Clone, Deserialize, Hash)]
pub struct ToolKeys {
    /// Open lazygit.
    pub lazygit: KeyList,
    /// Open lazydocker.
    pub lazydocker: KeyList,
    /// Open AI client.
    pub ai: KeyList,
    /// Open MCP client.
    pub mcp: KeyList,
    /// Restart / reconnect the LSP.
    pub restart_lsp: KeyList,
}

/// Keybindings active when the file-tree pane has focus.
#[derive(Debug, Clone, Deserialize, Hash)]
pub struct TreeKeys {
    /// Move selection up.
    pub move_up: KeyList,
    /// Move selection down.
    pub move_down: KeyList,
    /// Open selected file / expand directory.
    pub open: KeyList,
    /// Go to parent directory.
    pub go_parent: KeyList,
}

/// Complete keybindings configuration.
#[derive(Debug, Clone, Hash)]
pub struct KeybindingsConfig {
    pub global: GlobalKeys,
    pub tools: ToolKeys,
    pub tree: TreeKeys,
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            global: GlobalKeys {
                save: KeyList::Single("Ctrl+S".into()),
                refresh: KeyList::Single("Ctrl+R".into()),
                quit: KeyList::Single("Ctrl+Q".into()),
                focus_next: KeyList::Single("Tab".into()),
                focus_prev: KeyList::Single("Shift+Tab".into()),
                focus_tree: KeyList::Single("Ctrl+1".into()),
                focus_editor: KeyList::Single("Ctrl+2".into()),
                focus_sidebar: KeyList::Single("Ctrl+3".into()),
                open_settings: KeyList::Single("Ctrl+,".into()),
            },
            tools: ToolKeys {
                lazygit: KeyList::Single("g".into()),
                lazydocker: KeyList::Single("d".into()),
                ai: KeyList::Single("a".into()),
                mcp: KeyList::Single("m".into()),
                restart_lsp: KeyList::Single("l".into()),
            },
            tree: TreeKeys {
                move_up: KeyList::Multiple(vec!["Up".into(), "k".into()]),
                move_down: KeyList::Multiple(vec!["Down".into(), "j".into()]),
                open: KeyList::Multiple(vec!["Enter".into(), "Right".into(), "o".into()]),
                go_parent: KeyList::Multiple(vec!["Left".into(), "h".into()]),
            },
        }
    }
}

impl KeybindingsConfig {
    /// Load keybindings from a TOML file, merging overrides onto the defaults.
    ///
    /// Missing fields keep their default values — users only need to specify
    /// what they want to change.
    ///
    /// # Errors
    /// Returns an error only if the file exists but cannot be read.
    /// An invalid TOML or missing fields silently fall back to defaults.
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(e.into()),
        };

        let disk: DiskKeybindings = toml::from_str(&content).unwrap_or_default();
        Ok(Self::merge(Self::default(), &disk))
    }

    fn merge(base: Self, overrides: &DiskKeybindings) -> Self {
        Self {
            global: GlobalKeys {
                save: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.save.clone())
                    .unwrap_or(base.global.save),
                refresh: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.refresh.clone())
                    .unwrap_or(base.global.refresh),
                quit: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.quit.clone())
                    .unwrap_or(base.global.quit),
                focus_next: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.focus_next.clone())
                    .unwrap_or(base.global.focus_next),
                focus_prev: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.focus_prev.clone())
                    .unwrap_or(base.global.focus_prev),
                focus_tree: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.focus_tree.clone())
                    .unwrap_or(base.global.focus_tree),
                focus_editor: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.focus_editor.clone())
                    .unwrap_or(base.global.focus_editor),
                focus_sidebar: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.focus_sidebar.clone())
                    .unwrap_or(base.global.focus_sidebar),
                open_settings: overrides
                    .global
                    .as_ref()
                    .and_then(|g| g.open_settings.clone())
                    .unwrap_or(base.global.open_settings),
            },
            tools: ToolKeys {
                lazygit: overrides
                    .tools
                    .as_ref()
                    .and_then(|t| t.lazygit.clone())
                    .unwrap_or(base.tools.lazygit),
                lazydocker: overrides
                    .tools
                    .as_ref()
                    .and_then(|t| t.lazydocker.clone())
                    .unwrap_or(base.tools.lazydocker),
                ai: overrides
                    .tools
                    .as_ref()
                    .and_then(|t| t.ai.clone())
                    .unwrap_or(base.tools.ai),
                mcp: overrides
                    .tools
                    .as_ref()
                    .and_then(|t| t.mcp.clone())
                    .unwrap_or(base.tools.mcp),
                restart_lsp: overrides
                    .tools
                    .as_ref()
                    .and_then(|t| t.restart_lsp.clone())
                    .unwrap_or(base.tools.restart_lsp),
            },
            tree: TreeKeys {
                move_up: overrides
                    .tree
                    .as_ref()
                    .and_then(|t| t.move_up.clone())
                    .unwrap_or(base.tree.move_up),
                move_down: overrides
                    .tree
                    .as_ref()
                    .and_then(|t| t.move_down.clone())
                    .unwrap_or(base.tree.move_down),
                open: overrides
                    .tree
                    .as_ref()
                    .and_then(|t| t.open.clone())
                    .unwrap_or(base.tree.open),
                go_parent: overrides
                    .tree
                    .as_ref()
                    .and_then(|t| t.go_parent.clone())
                    .unwrap_or(base.tree.go_parent),
            },
        }
    }
}

// ── Internal TOML deserialization structs (all fields optional) ─────────────

#[derive(Debug, Default, Deserialize)]
struct DiskKeybindings {
    global: Option<DiskGlobalKeys>,
    tools: Option<DiskToolKeys>,
    tree: Option<DiskTreeKeys>,
}

#[derive(Debug, Default, Deserialize)]
struct DiskGlobalKeys {
    save: Option<KeyList>,
    refresh: Option<KeyList>,
    quit: Option<KeyList>,
    focus_next: Option<KeyList>,
    focus_prev: Option<KeyList>,
    focus_tree: Option<KeyList>,
    focus_editor: Option<KeyList>,
    focus_sidebar: Option<KeyList>,
    open_settings: Option<KeyList>,
}

#[derive(Debug, Default, Deserialize)]
struct DiskToolKeys {
    lazygit: Option<KeyList>,
    lazydocker: Option<KeyList>,
    ai: Option<KeyList>,
    mcp: Option<KeyList>,
    restart_lsp: Option<KeyList>,
}

#[derive(Debug, Default, Deserialize)]
struct DiskTreeKeys {
    move_up: Option<KeyList>,
    move_down: Option<KeyList>,
    open: Option<KeyList>,
    go_parent: Option<KeyList>,
}

// ── Tests (RED → GREEN) ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // ── KeyList::keys() ─────────────────────────────────────────────────────

    #[test]
    fn key_list_single_returns_one_key() {
        let kl = KeyList::Single("Ctrl+S".into());
        assert_eq!(kl.keys(), vec!["Ctrl+S"]);
    }

    #[test]
    fn key_list_multiple_returns_all_keys() {
        let kl = KeyList::Multiple(vec!["Up".into(), "k".into()]);
        assert_eq!(kl.keys(), vec!["Up", "k"]);
    }

    // ── KeyList::matches() ──────────────────────────────────────────────────

    #[test]
    fn key_list_single_matches_exact() {
        let kl = KeyList::Single("Ctrl+S".into());
        assert!(kl.matches("Ctrl+S"));
    }

    #[test]
    fn key_list_single_no_match() {
        let kl = KeyList::Single("Ctrl+S".into());
        assert!(!kl.matches("Ctrl+A"));
    }

    #[test]
    fn key_list_multiple_matches_any() {
        let kl = KeyList::Multiple(vec!["Up".into(), "k".into()]);
        assert!(kl.matches("Up"));
        assert!(kl.matches("k"));
        assert!(!kl.matches("j"));
    }

    #[test]
    fn key_list_case_insensitive() {
        let kl = KeyList::Single("ctrl+s".into());
        assert!(kl.matches("Ctrl+S"));
        assert!(kl.matches("CTRL+S"));
        assert!(kl.matches("ctrl+s"));
    }

    // ── KeybindingsConfig::default() ────────────────────────────────────────

    #[test]
    fn default_global_save_is_ctrl_s() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.save.matches("Ctrl+S"));
    }

    #[test]
    fn default_global_quit_is_ctrl_q() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.quit.matches("Ctrl+Q"));
    }

    #[test]
    fn default_global_refresh_is_ctrl_r() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.refresh.matches("Ctrl+R"));
    }

    #[test]
    fn default_global_focus_next_is_tab() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.focus_next.matches("Tab"));
    }

    #[test]
    fn default_global_focus_prev_is_shift_tab() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.focus_prev.matches("Shift+Tab"));
    }

    #[test]
    fn default_tool_lazygit_is_g() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tools.lazygit.matches("g"));
    }

    #[test]
    fn default_tool_lazydocker_is_d() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tools.lazydocker.matches("d"));
    }

    #[test]
    fn default_tool_ai_is_a() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tools.ai.matches("a"));
    }

    #[test]
    fn default_tool_mcp_is_m() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tools.mcp.matches("m"));
    }

    #[test]
    fn default_tool_restart_lsp_is_l() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tools.restart_lsp.matches("l"));
    }

    #[test]
    fn default_tree_move_up_has_up_and_k() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tree.move_up.matches("Up"));
        assert!(cfg.tree.move_up.matches("k"));
    }

    #[test]
    fn default_tree_move_down_has_down_and_j() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tree.move_down.matches("Down"));
        assert!(cfg.tree.move_down.matches("j"));
    }

    #[test]
    fn default_tree_open_has_enter_right_o() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tree.open.matches("Enter"));
        assert!(cfg.tree.open.matches("Right"));
        assert!(cfg.tree.open.matches("o"));
    }

    #[test]
    fn default_tree_go_parent_has_left_and_h() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.tree.go_parent.matches("Left"));
        assert!(cfg.tree.go_parent.matches("h"));
    }

    // ── load_from_file() ────────────────────────────────────────────────────

    #[test]
    fn load_nonexistent_file_gives_defaults() {
        let cfg = KeybindingsConfig::load_from_file(
            Path::new("/tmp/nonexistent-keybindings-xyz-99.toml"),
        )
        .unwrap();
        assert!(cfg.global.save.matches("Ctrl+S"));
    }

    #[test]
    fn load_empty_file_gives_defaults() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "").unwrap();
        let cfg = KeybindingsConfig::load_from_file(f.path()).unwrap();
        assert!(cfg.global.save.matches("Ctrl+S"));
        assert!(cfg.tools.lazygit.matches("g"));
    }

    #[test]
    fn load_partial_global_overrides_quit_keeps_others() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "[global]\nquit = \"Ctrl+W\"").unwrap();
        let cfg = KeybindingsConfig::load_from_file(f.path()).unwrap();
        assert!(cfg.global.quit.matches("Ctrl+W"), "quit overridden");
        assert!(!cfg.global.quit.matches("Ctrl+Q"), "old quit gone");
        assert!(cfg.global.save.matches("Ctrl+S"), "save unchanged");
        assert!(cfg.tools.lazygit.matches("g"), "lazygit unchanged");
    }

    #[test]
    fn load_multiple_alternatives_from_file() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "[tree]\nmove_up = [\"Up\", \"k\", \"w\"]").unwrap();
        let cfg = KeybindingsConfig::load_from_file(f.path()).unwrap();
        assert!(cfg.tree.move_up.matches("Up"));
        assert!(cfg.tree.move_up.matches("k"));
        assert!(cfg.tree.move_up.matches("w"), "custom w added");
    }

    #[test]
    fn load_tool_binding_override() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "[tools]\nlazygit = \"Ctrl+G\"").unwrap();
        let cfg = KeybindingsConfig::load_from_file(f.path()).unwrap();
        assert!(cfg.tools.lazygit.matches("Ctrl+G"));
        assert!(!cfg.tools.lazygit.matches("g"));
    }

    #[test]
    fn load_invalid_toml_silently_gives_defaults() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "this is not valid toml ][[[").unwrap();
        let cfg = KeybindingsConfig::load_from_file(f.path()).unwrap();
        assert!(cfg.global.save.matches("Ctrl+S"), "falls back to defaults");
    }

    // ── New direct-focus defaults ────────────────────────────────────────────

    #[test]
    fn default_focus_tree_is_ctrl_1() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.focus_tree.matches("Ctrl+1"));
    }

    #[test]
    fn default_focus_editor_is_ctrl_2() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.focus_editor.matches("Ctrl+2"));
    }

    #[test]
    fn default_focus_sidebar_is_ctrl_3() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.focus_sidebar.matches("Ctrl+3"));
    }

    #[test]
    fn default_open_settings_is_ctrl_comma() {
        let cfg = KeybindingsConfig::default();
        assert!(cfg.global.open_settings.matches("Ctrl+,"));
    }

    #[test]
    fn load_overrides_focus_tree() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "[global]\nfocus_tree = \"Ctrl+F1\"").unwrap();
        let cfg = KeybindingsConfig::load_from_file(f.path()).unwrap();
        assert!(cfg.global.focus_tree.matches("Ctrl+F1"));
        assert!(!cfg.global.focus_tree.matches("Ctrl+1"));
    }
}
