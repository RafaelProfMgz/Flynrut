use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::Deserialize;

/// A custom keybinding defined by an extension.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ExtKeybinding {
    /// The key character, e.g. `"e"` or `"F5"`.
    pub key: String,
    /// Shell command to run when the key is pressed in Normal mode.
    pub command: String,
    /// Optional description shown in the sidebar help.
    #[serde(default)]
    pub description: String,
}

/// A custom tool/integration defined by an extension.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ExtTool {
    /// Display name shown in the sidebar.
    pub name: String,
    /// Shell command to execute.
    pub command: String,
    /// Optional description.
    #[serde(default)]
    pub description: String,
}

/// Parsed content of a single extension TOML file.
#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionManifest {
    /// Human-readable extension name.
    pub name: String,
    /// SemVer version string, e.g. `"1.0.0"`.
    #[serde(default = "default_version")]
    pub version: String,
    /// Optional description.
    #[serde(default)]
    pub description: String,
    /// Custom keybindings added by this extension.
    #[serde(default)]
    pub keybindings: Vec<ExtKeybinding>,
    /// Custom tools/integrations added by this extension.
    #[serde(default)]
    pub tools: Vec<ExtTool>,
    /// File extension → LSP language ID mapping, e.g. `{ "ts" = "typescript" }`.
    #[serde(default)]
    pub filetypes: HashMap<String, String>,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

/// Aggregated state from all loaded extensions.
#[derive(Debug, Clone, Default)]
pub struct ExtensionRegistry {
    /// All loaded manifests, in load order.
    pub extensions: Vec<ExtensionManifest>,
}

impl ExtensionRegistry {
    /// Load all `*.toml` extension files from `dir`.
    /// Files that fail to parse are skipped with a warning (non-fatal).
    ///
    /// # Errors
    /// Only returns an error if the directory itself cannot be read.
    /// Individual file parse errors are skipped.
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        let mut extensions = Vec::new();

        if !dir.exists() {
            return Ok(Self::default());
        }

        let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("toml"))
            .collect();
        paths.sort();

        for path in paths {
            match load_manifest(&path) {
                Ok(manifest) => extensions.push(manifest),
                Err(e) => {
                    eprintln!(
                        "rust-ide: skipping extension {}: {e}",
                        path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                    );
                }
            }
        }

        Ok(Self { extensions })
    }

    /// All keybindings from all loaded extensions.
    #[must_use]
    pub fn all_keybindings(&self) -> Vec<&ExtKeybinding> {
        self.extensions
            .iter()
            .flat_map(|ext| ext.keybindings.iter())
            .collect()
    }

    /// All tools from all loaded extensions.
    #[must_use]
    pub fn all_tools(&self) -> Vec<&ExtTool> {
        self.extensions
            .iter()
            .flat_map(|ext| ext.tools.iter())
            .collect()
    }

    /// Merged filetype map (later extensions override earlier ones).
    #[must_use]
    pub fn filetype_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for ext in &self.extensions {
            map.extend(ext.filetypes.clone());
        }
        map
    }
}

fn load_manifest(path: &Path) -> Result<ExtensionManifest> {
    let content = std::fs::read_to_string(path)?;
    let manifest: ExtensionManifest = toml::from_str(&content)?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_toml(dir: &Path, name: &str, content: &str) {
        fs::write(dir.join(format!("{name}.toml")), content).unwrap();
    }

    // ── load_from_dir ────────────────────────────────────────────────────────

    #[test]
    fn empty_dir_returns_empty_registry() {
        let dir = TempDir::new().unwrap();
        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        assert!(reg.extensions.is_empty());
    }

    #[test]
    fn nonexistent_dir_returns_empty_registry() {
        let reg =
            ExtensionRegistry::load_from_dir(Path::new("/tmp/__no_such_extensions_dir__")).unwrap();
        assert!(reg.extensions.is_empty());
    }

    #[test]
    fn loads_valid_extension() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "my-ext",
            r#"
name = "my-ext"
version = "1.0.0"
description = "A test extension"

[[keybindings]]
key = "F5"
command = "cargo run"
description = "Run project"

[[tools]]
name = "fmt"
command = "cargo fmt"

[filetypes]
rs = "rust"
ts = "typescript"
"#,
        );

        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        assert_eq!(reg.extensions.len(), 1);
        let ext = &reg.extensions[0];
        assert_eq!(ext.name, "my-ext");
        assert_eq!(ext.version, "1.0.0");
        assert_eq!(ext.keybindings.len(), 1);
        assert_eq!(ext.keybindings[0].key, "F5");
        assert_eq!(ext.tools.len(), 1);
        assert_eq!(ext.tools[0].command, "cargo fmt");
        assert_eq!(ext.filetypes["rs"], "rust");
    }

    #[test]
    fn invalid_toml_file_is_skipped() {
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "bad", "this is not valid toml [[[");
        write_toml(dir.path(), "good", r#"name = "good""#);

        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        // only the valid one loaded
        assert_eq!(reg.extensions.len(), 1);
        assert_eq!(reg.extensions[0].name, "good");
    }

    #[test]
    fn non_toml_files_are_ignored() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("README.md"), "# not a toml").unwrap();
        fs::write(dir.path().join("config.json"), "{}").unwrap();
        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        assert!(reg.extensions.is_empty());
    }

    #[test]
    fn multiple_extensions_sorted_by_filename() {
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "z-ext", r#"name = "z-ext""#);
        write_toml(dir.path(), "a-ext", r#"name = "a-ext""#);
        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        assert_eq!(reg.extensions[0].name, "a-ext");
        assert_eq!(reg.extensions[1].name, "z-ext");
    }

    // ── all_keybindings ──────────────────────────────────────────────────────

    #[test]
    fn all_keybindings_merges_across_extensions() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "a",
            r#"
name = "a"
[[keybindings]]
key = "F1"
command = "echo a"
"#,
        );
        write_toml(
            dir.path(),
            "b",
            r#"
name = "b"
[[keybindings]]
key = "F2"
command = "echo b"
"#,
        );
        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        let keys: Vec<_> = reg
            .all_keybindings()
            .iter()
            .map(|k| k.key.as_str())
            .collect();
        assert!(keys.contains(&"F1"));
        assert!(keys.contains(&"F2"));
    }

    // ── filetype_map ─────────────────────────────────────────────────────────

    #[test]
    fn filetype_map_later_extension_wins() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "a",
            r#"
name = "a"
[filetypes]
ts = "typescript"
"#,
        );
        write_toml(
            dir.path(),
            "b",
            r#"
name = "b"
[filetypes]
ts = "typescript-override"
"#,
        );
        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        assert_eq!(reg.filetype_map()["ts"], "typescript-override");
    }

    // ── default version ──────────────────────────────────────────────────────

    #[test]
    fn missing_version_gets_default() {
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "no-ver", r#"name = "no-ver""#);
        let reg = ExtensionRegistry::load_from_dir(dir.path()).unwrap();
        assert_eq!(reg.extensions[0].version, "0.1.0");
    }
}
