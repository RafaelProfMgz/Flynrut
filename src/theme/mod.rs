use ratatui::style::Color;
use serde::Deserialize;

/// TOML-serialisable RGB color (`"#RRGGBB"` or `"#RGB"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeColor(pub u8, pub u8, pub u8);

impl ThemeColor {
    #[must_use]
    pub fn to_ratatui(self) -> Color {
        Color::Rgb(self.0, self.1, self.2)
    }
}

impl<'de> Deserialize<'de> for ThemeColor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        parse_hex_color(&s).ok_or_else(|| {
            serde::de::Error::custom(format!(
                "invalid color {s:?}: expected \"#RRGGBB\" or \"#RGB\""
            ))
        })
    }
}

/// All color tokens used by the IDE UI.
#[derive(Debug, Clone, Deserialize)]
pub struct ThemeColors {
    // Editor
    pub editor_bg: ThemeColor,
    pub editor_fg: ThemeColor,
    pub editor_cursor_line_bg: ThemeColor,
    pub editor_selection_bg: ThemeColor,
    pub editor_line_number_fg: ThemeColor,

    // File tree
    pub tree_bg: ThemeColor,
    pub tree_fg: ThemeColor,
    pub tree_selected_bg: ThemeColor,
    pub tree_selected_fg: ThemeColor,
    pub tree_dir_fg: ThemeColor,

    // Sidebar / status
    pub sidebar_bg: ThemeColor,
    pub sidebar_fg: ThemeColor,
    pub status_bg: ThemeColor,
    pub status_fg: ThemeColor,

    // Borders
    pub border_normal: ThemeColor,
    pub border_focused: ThemeColor,

    // Diagnostics
    pub diag_error: ThemeColor,
    pub diag_warning: ThemeColor,
    pub diag_info: ThemeColor,

    // Git status colors
    pub git_added: ThemeColor,
    pub git_modified: ThemeColor,
    pub git_deleted: ThemeColor,
}

/// TOML file on disk — all fields optional, missing ones fall back to the base theme.
#[derive(Debug, Default, Deserialize)]
struct ThemeFile {
    editor_bg: Option<ThemeColor>,
    editor_fg: Option<ThemeColor>,
    editor_cursor_line_bg: Option<ThemeColor>,
    editor_selection_bg: Option<ThemeColor>,
    editor_line_number_fg: Option<ThemeColor>,
    tree_bg: Option<ThemeColor>,
    tree_fg: Option<ThemeColor>,
    tree_selected_bg: Option<ThemeColor>,
    tree_selected_fg: Option<ThemeColor>,
    tree_dir_fg: Option<ThemeColor>,
    sidebar_bg: Option<ThemeColor>,
    sidebar_fg: Option<ThemeColor>,
    status_bg: Option<ThemeColor>,
    status_fg: Option<ThemeColor>,
    border_normal: Option<ThemeColor>,
    border_focused: Option<ThemeColor>,
    diag_error: Option<ThemeColor>,
    diag_warning: Option<ThemeColor>,
    diag_info: Option<ThemeColor>,
    git_added: Option<ThemeColor>,
    git_modified: Option<ThemeColor>,
    git_deleted: Option<ThemeColor>,
}

impl ThemeColors {
    /// Load a named built-in theme.  Falls back to `dark` for unknown names.
    #[must_use]
    pub fn builtin(name: &str) -> Self {
        match name {
            "gruvbox" => Self::gruvbox(),
            "nord" => Self::nord(),
            "dracula" => Self::dracula(),
            "light" => Self::light(),
            _ => Self::dark(),
        }
    }

    /// Load from a TOML file, merging with the built-in base theme.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from_file(path: &std::path::Path, base: Self) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let file: ThemeFile = toml::from_str(&content)?;
        Ok(Self::merge(base, file))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn merge(base: Self, overrides: ThemeFile) -> Self {
        macro_rules! ov {
            ($field:ident) => {
                overrides.$field.unwrap_or(base.$field)
            };
        }
        Self {
            editor_bg: ov!(editor_bg),
            editor_fg: ov!(editor_fg),
            editor_cursor_line_bg: ov!(editor_cursor_line_bg),
            editor_selection_bg: ov!(editor_selection_bg),
            editor_line_number_fg: ov!(editor_line_number_fg),
            tree_bg: ov!(tree_bg),
            tree_fg: ov!(tree_fg),
            tree_selected_bg: ov!(tree_selected_bg),
            tree_selected_fg: ov!(tree_selected_fg),
            tree_dir_fg: ov!(tree_dir_fg),
            sidebar_bg: ov!(sidebar_bg),
            sidebar_fg: ov!(sidebar_fg),
            status_bg: ov!(status_bg),
            status_fg: ov!(status_fg),
            border_normal: ov!(border_normal),
            border_focused: ov!(border_focused),
            diag_error: ov!(diag_error),
            diag_warning: ov!(diag_warning),
            diag_info: ov!(diag_info),
            git_added: ov!(git_added),
            git_modified: ov!(git_modified),
            git_deleted: ov!(git_deleted),
        }
    }

    // ──────────────────────────── built-in themes ────────────────────────────

    #[must_use]
    pub fn dark() -> Self {
        Self {
            editor_bg: ThemeColor(30, 30, 30),
            editor_fg: ThemeColor(212, 212, 212),
            editor_cursor_line_bg: ThemeColor(45, 45, 45),
            editor_selection_bg: ThemeColor(60, 60, 60),
            editor_line_number_fg: ThemeColor(90, 90, 90),
            tree_bg: ThemeColor(25, 25, 25),
            tree_fg: ThemeColor(200, 200, 200),
            tree_selected_bg: ThemeColor(55, 55, 100),
            tree_selected_fg: ThemeColor(255, 255, 255),
            tree_dir_fg: ThemeColor(130, 180, 255),
            sidebar_bg: ThemeColor(28, 28, 28),
            sidebar_fg: ThemeColor(180, 180, 180),
            status_bg: ThemeColor(0, 90, 180),
            status_fg: ThemeColor(255, 255, 255),
            border_normal: ThemeColor(60, 60, 60),
            border_focused: ThemeColor(0, 120, 215),
            diag_error: ThemeColor(240, 80, 80),
            diag_warning: ThemeColor(230, 180, 80),
            diag_info: ThemeColor(80, 180, 240),
            git_added: ThemeColor(80, 200, 120),
            git_modified: ThemeColor(220, 180, 80),
            git_deleted: ThemeColor(220, 80, 80),
        }
    }

    #[must_use]
    pub fn gruvbox() -> Self {
        Self {
            editor_bg: ThemeColor(40, 40, 40),
            editor_fg: ThemeColor(235, 219, 178),
            editor_cursor_line_bg: ThemeColor(60, 56, 54),
            editor_selection_bg: ThemeColor(80, 73, 69),
            editor_line_number_fg: ThemeColor(146, 131, 116),
            tree_bg: ThemeColor(32, 32, 32),
            tree_fg: ThemeColor(235, 219, 178),
            tree_selected_bg: ThemeColor(69, 133, 136),
            tree_selected_fg: ThemeColor(251, 241, 199),
            tree_dir_fg: ThemeColor(131, 165, 152),
            sidebar_bg: ThemeColor(29, 32, 33),
            sidebar_fg: ThemeColor(189, 174, 147),
            status_bg: ThemeColor(215, 153, 33),
            status_fg: ThemeColor(29, 32, 33),
            border_normal: ThemeColor(80, 73, 69),
            border_focused: ThemeColor(215, 153, 33),
            diag_error: ThemeColor(251, 73, 52),
            diag_warning: ThemeColor(250, 189, 47),
            diag_info: ThemeColor(131, 165, 152),
            git_added: ThemeColor(184, 187, 38),
            git_modified: ThemeColor(250, 189, 47),
            git_deleted: ThemeColor(251, 73, 52),
        }
    }

    #[must_use]
    pub fn nord() -> Self {
        Self {
            editor_bg: ThemeColor(46, 52, 64),
            editor_fg: ThemeColor(216, 222, 233),
            editor_cursor_line_bg: ThemeColor(59, 66, 82),
            editor_selection_bg: ThemeColor(67, 76, 94),
            editor_line_number_fg: ThemeColor(76, 86, 106),
            tree_bg: ThemeColor(36, 41, 51),
            tree_fg: ThemeColor(216, 222, 233),
            tree_selected_bg: ThemeColor(67, 76, 94),
            tree_selected_fg: ThemeColor(236, 239, 244),
            tree_dir_fg: ThemeColor(129, 161, 193),
            sidebar_bg: ThemeColor(39, 44, 54),
            sidebar_fg: ThemeColor(196, 204, 219),
            status_bg: ThemeColor(94, 129, 172),
            status_fg: ThemeColor(236, 239, 244),
            border_normal: ThemeColor(67, 76, 94),
            border_focused: ThemeColor(136, 192, 208),
            diag_error: ThemeColor(191, 97, 106),
            diag_warning: ThemeColor(235, 203, 139),
            diag_info: ThemeColor(136, 192, 208),
            git_added: ThemeColor(163, 190, 140),
            git_modified: ThemeColor(235, 203, 139),
            git_deleted: ThemeColor(191, 97, 106),
        }
    }

    #[must_use]
    pub fn dracula() -> Self {
        Self {
            editor_bg: ThemeColor(40, 42, 54),
            editor_fg: ThemeColor(248, 248, 242),
            editor_cursor_line_bg: ThemeColor(68, 71, 90),
            editor_selection_bg: ThemeColor(68, 71, 90),
            editor_line_number_fg: ThemeColor(98, 114, 164),
            tree_bg: ThemeColor(33, 34, 44),
            tree_fg: ThemeColor(248, 248, 242),
            tree_selected_bg: ThemeColor(68, 71, 90),
            tree_selected_fg: ThemeColor(255, 255, 255),
            tree_dir_fg: ThemeColor(139, 233, 253),
            sidebar_bg: ThemeColor(33, 34, 44),
            sidebar_fg: ThemeColor(189, 147, 249),
            status_bg: ThemeColor(98, 114, 164),
            status_fg: ThemeColor(248, 248, 242),
            border_normal: ThemeColor(68, 71, 90),
            border_focused: ThemeColor(189, 147, 249),
            diag_error: ThemeColor(255, 85, 85),
            diag_warning: ThemeColor(241, 250, 140),
            diag_info: ThemeColor(139, 233, 253),
            git_added: ThemeColor(80, 250, 123),
            git_modified: ThemeColor(241, 250, 140),
            git_deleted: ThemeColor(255, 85, 85),
        }
    }

    #[must_use]
    pub fn light() -> Self {
        Self {
            editor_bg: ThemeColor(255, 255, 255),
            editor_fg: ThemeColor(30, 30, 30),
            editor_cursor_line_bg: ThemeColor(235, 235, 235),
            editor_selection_bg: ThemeColor(180, 210, 250),
            editor_line_number_fg: ThemeColor(160, 160, 160),
            tree_bg: ThemeColor(245, 245, 245),
            tree_fg: ThemeColor(50, 50, 50),
            tree_selected_bg: ThemeColor(180, 210, 250),
            tree_selected_fg: ThemeColor(0, 0, 0),
            tree_dir_fg: ThemeColor(0, 100, 200),
            sidebar_bg: ThemeColor(240, 240, 240),
            sidebar_fg: ThemeColor(60, 60, 60),
            status_bg: ThemeColor(0, 120, 212),
            status_fg: ThemeColor(255, 255, 255),
            border_normal: ThemeColor(200, 200, 200),
            border_focused: ThemeColor(0, 120, 212),
            diag_error: ThemeColor(200, 0, 0),
            diag_warning: ThemeColor(180, 130, 0),
            diag_info: ThemeColor(0, 130, 200),
            git_added: ThemeColor(0, 160, 60),
            git_modified: ThemeColor(150, 100, 0),
            git_deleted: ThemeColor(200, 0, 0),
        }
    }
}

/// Parse `"#RRGGBB"` or `"#RGB"` hex strings.
pub(crate) fn parse_hex_color(s: &str) -> Option<ThemeColor> {
    let s = s.trim().strip_prefix('#')?;
    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some(ThemeColor(r, g, b))
        }
        3 => {
            let r = u8::from_str_radix(&s[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&s[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&s[2..3], 16).ok()? * 17;
            Some(ThemeColor(r, g, b))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // ── parse_hex_color ──────────────────────────────────────────────────────

    #[test]
    fn parse_rrggbb() {
        assert_eq!(
            parse_hex_color("#1e1e1e"),
            Some(ThemeColor(0x1e, 0x1e, 0x1e))
        );
    }

    #[test]
    fn parse_rgb_short() {
        assert_eq!(parse_hex_color("#fff"), Some(ThemeColor(255, 255, 255)));
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert_eq!(parse_hex_color("1e1e1e"), None);
        assert_eq!(parse_hex_color("#gg0000"), None);
        assert_eq!(parse_hex_color("#12345"), None);
    }

    #[test]
    fn parse_with_whitespace() {
        assert_eq!(parse_hex_color("  #ff0000  "), Some(ThemeColor(255, 0, 0)));
    }

    // ── builtin themes ───────────────────────────────────────────────────────

    #[test]
    fn dark_theme_loads() {
        let t = ThemeColors::builtin("dark");
        assert_eq!(t.editor_bg, ThemeColor(30, 30, 30));
    }

    #[test]
    fn gruvbox_theme_loads() {
        let t = ThemeColors::builtin("gruvbox");
        assert_ne!(
            t.editor_bg,
            ThemeColor(30, 30, 30),
            "gruvbox differs from dark"
        );
    }

    #[test]
    fn nord_theme_loads() {
        let t = ThemeColors::builtin("nord");
        assert_eq!(t.editor_bg, ThemeColor(46, 52, 64));
    }

    #[test]
    fn dracula_theme_loads() {
        let t = ThemeColors::builtin("dracula");
        assert_eq!(t.editor_bg, ThemeColor(40, 42, 54));
    }

    #[test]
    fn light_theme_loads() {
        let t = ThemeColors::builtin("light");
        assert_eq!(t.editor_bg, ThemeColor(255, 255, 255));
    }

    #[test]
    fn unknown_theme_falls_back_to_dark() {
        let t = ThemeColors::builtin("does-not-exist");
        assert_eq!(t.editor_bg, ThemeColor(30, 30, 30));
    }

    // ── load_from_file ───────────────────────────────────────────────────────

    #[test]
    fn file_overrides_only_specified_fields() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, r##"editor_bg = "#ff0000""##).unwrap();
        let base = ThemeColors::dark();
        let orig_fg = base.editor_fg;
        let result = ThemeColors::load_from_file(f.path(), base).unwrap();
        assert_eq!(result.editor_bg, ThemeColor(255, 0, 0));
        assert_eq!(result.editor_fg, orig_fg, "unset fields keep base value");
    }

    #[test]
    fn empty_file_keeps_base_theme() {
        let f = NamedTempFile::new().unwrap();
        let base = ThemeColors::gruvbox();
        let orig_bg = base.editor_bg;
        let result = ThemeColors::load_from_file(f.path(), base).unwrap();
        assert_eq!(result.editor_bg, orig_bg);
    }

    #[test]
    fn invalid_file_path_returns_error() {
        let result = ThemeColors::load_from_file(
            std::path::Path::new("/tmp/nonexistent_theme_xyz.toml"),
            ThemeColors::dark(),
        );
        assert!(result.is_err());
    }

    // ── ThemeColor::to_ratatui ───────────────────────────────────────────────

    #[test]
    fn to_ratatui_returns_rgb() {
        use ratatui::style::Color;
        let c = ThemeColor(10, 20, 30);
        assert_eq!(c.to_ratatui(), Color::Rgb(10, 20, 30));
    }
}
