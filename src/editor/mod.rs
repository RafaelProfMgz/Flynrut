use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};

#[derive(Debug)]
pub struct EditorModel {
    content: String,
    current_file: Option<PathBuf>,
    pub dirty: bool,
}

impl Default for EditorModel {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorModel {
    #[must_use]
    pub fn new() -> Self {
        Self {
            content: String::new(),
            current_file: None,
            dirty: false,
        }
    }

    #[must_use]
    pub fn title(&self) -> String {
        match &self.current_file {
            Some(path) if self.dirty => format!("Editor * {}", path.display()),
            Some(path) => format!("Editor {}", path.display()),
            None => "Editor".to_string(),
        }
    }

    /// # Errors
    /// Returns an error if the file cannot be read.
    pub fn open(&mut self, path: &Path) -> Result<()> {
        self.content = fs::read_to_string(path)
            .with_context(|| format!("não foi possível abrir {}", path.display()))?;
        self.current_file = Some(path.to_path_buf());
        self.dirty = false;
        Ok(())
    }

    /// # Errors
    /// Returns an error if no file is open or writing fails.
    pub fn save(&mut self) -> Result<PathBuf> {
        let contents = self.content.clone();
        self.save_content(&contents)
    }

    /// # Errors
    /// Returns an error if no file is open or writing fails.
    pub fn save_content(&mut self, text: &str) -> Result<PathBuf> {
        let path = self
            .current_file
            .clone()
            .ok_or_else(|| anyhow!("nenhum arquivo aberto"))?;
        fs::write(&path, text)
            .with_context(|| format!("não foi possível salvar {}", path.display()))?;
        self.content.clear();
        self.content.push_str(text);
        self.dirty = false;
        Ok(path)
    }

    #[must_use]
    pub fn current_file(&self) -> Option<&Path> {
        self.current_file.as_deref()
    }

    #[must_use]
    pub fn contents(&self) -> &str {
        &self.content
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn split_lines(content: &str) -> Vec<String> {
    if content.is_empty() {
        return vec![String::new()];
    }

    content.lines().map(ToOwned::to_owned).collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    // --- split_lines ---

    #[test]
    fn split_lines_empty_gives_one_empty_line() {
        assert_eq!(split_lines(""), vec![String::new()]);
    }

    #[test]
    fn split_lines_single_line() {
        assert_eq!(split_lines("hello"), vec!["hello".to_string()]);
    }

    #[test]
    fn split_lines_multiple_lines() {
        let result = split_lines("line1\nline2\nline3");
        assert_eq!(result, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn split_lines_preserves_trailing_newline_content() {
        let result = split_lines("a\n");
        assert_eq!(result, vec!["a"]);
    }

    // --- EditorModel title ---

    #[test]
    fn title_without_file_is_editor() {
        let model = EditorModel::new();
        assert_eq!(model.title(), "Editor");
    }

    #[test]
    fn title_with_dirty_file_includes_marker() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "fn main() {}\n").unwrap();

        let mut model = EditorModel::new();
        model.open(file.path()).unwrap();
        model.set_dirty(true);

        assert_eq!(model.title(), format!("Editor * {}", file.path().display()));
    }

    // --- contents ---

    #[test]
    fn contents_new_editor_is_empty() {
        let model = EditorModel::new();
        assert_eq!(model.contents(), "");
    }

    #[test]
    fn open_loads_file_contents() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "fn main() {}\n").unwrap();

        let mut model = EditorModel::new();
        model.open(file.path()).unwrap();

        assert_eq!(model.contents(), "fn main() {}\n");
        assert_eq!(model.current_file(), Some(file.path()));
        assert!(!model.dirty);
    }

    #[test]
    fn save_content_updates_disk_and_model() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), "old\n").unwrap();

        let mut model = EditorModel::new();
        model.open(file.path()).unwrap();
        model.set_dirty(true);

        let saved_path = model.save_content("new\ncontent\n").unwrap();

        assert_eq!(saved_path, file.path());
        assert_eq!(fs::read_to_string(file.path()).unwrap(), "new\ncontent\n");
        assert_eq!(model.contents(), "new\ncontent\n");
        assert!(!model.dirty);
    }

    #[test]
    fn save_without_open_file_returns_error() {
        let mut model = EditorModel::new();
        let result = model.save_content("irrelevant");
        assert!(result.is_err());
    }

    // --- current_file ---

    #[test]
    fn current_file_is_none_initially() {
        let model = EditorModel::new();
        assert!(model.current_file().is_none());
    }
}
