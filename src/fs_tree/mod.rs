use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct VisibleEntry {
    pub path: PathBuf,
    pub label: String,
    pub depth: usize,
    pub is_dir: bool,
    pub expanded: bool,
}

#[derive(Debug)]
pub struct FileTree {
    root_path: PathBuf,
    root: TreeNode,
    selected: usize,
}

#[derive(Debug)]
struct TreeNode {
    path: PathBuf,
    label: String,
    is_dir: bool,
    expanded: bool,
    children: Vec<TreeNode>,
}

impl FileTree {
    /// # Errors
    /// Returns an error if the root path cannot be read from the filesystem.
    pub fn new(root_path: &Path) -> Result<Self> {
        let root = TreeNode::build(root_path, true)?;
        let mut tree = Self {
            root_path: root_path.to_path_buf(),
            root,
            selected: 0,
        };
        tree.selected = tree.visible_entries().len().saturating_sub(1).min(1);
        Ok(tree)
    }

    /// # Errors
    /// Returns an error if the filesystem cannot be re-read.
    pub fn refresh(&mut self) -> Result<()> {
        let selected_path = self.selected_entry().map(|entry| entry.path);
        self.root = TreeNode::build(&self.root_path, true)?;

        if let Some(path) = selected_path {
            if let Some(index) = self
                .visible_entries()
                .iter()
                .position(|entry| entry.path == path)
            {
                self.selected = index;
            } else {
                self.selected = 0;
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn visible_entries(&self) -> Vec<VisibleEntry> {
        let mut entries = Vec::new();
        self.root.collect_visible(0, &mut entries);
        entries
    }

    #[must_use]
    pub fn selected_entry(&self) -> Option<VisibleEntry> {
        self.visible_entries().get(self.selected).cloned()
    }

    #[must_use]
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn move_selection(&mut self, delta: isize) {
        let total = self.visible_entries().len();
        if total == 0 {
            self.selected = 0;
            return;
        }

        if delta.is_negative() {
            self.selected = self.selected.saturating_sub(delta.unsigned_abs());
        } else {
            self.selected = (self.selected + delta.unsigned_abs()).min(total - 1);
        }
    }

    pub fn activate_selected(&mut self) -> Option<PathBuf> {
        let entry = self.selected_entry()?;
        if entry.is_dir {
            self.toggle(&entry.path);
            None
        } else {
            Some(entry.path)
        }
    }

    fn toggle(&mut self, path: &Path) {
        self.root.toggle(path);
    }
}

impl TreeNode {
    fn build(path: &Path, expanded: bool) -> Result<Self> {
        let metadata = fs::metadata(path)?;
        let is_dir = metadata.is_dir();
        let label = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().to_string(),
        );

        let mut node = Self {
            path: path.to_path_buf(),
            label,
            is_dir,
            expanded,
            children: Vec::new(),
        };

        if is_dir {
            let mut entries = fs::read_dir(path)?
                .flatten()
                .filter(|entry| {
                    let name = entry.file_name().to_string_lossy().to_string();
                    name != ".git" && name != "target"
                })
                .collect::<Vec<_>>();

            entries.sort_by(|left, right| {
                let left_is_dir = left.file_type().is_ok_and(|kind| kind.is_dir());
                let right_is_dir = right.file_type().is_ok_and(|kind| kind.is_dir());
                right_is_dir
                    .cmp(&left_is_dir)
                    .then_with(|| left.file_name().cmp(&right.file_name()))
            });

            for entry in entries {
                node.children.push(TreeNode::build(&entry.path(), false)?);
            }
        }

        Ok(node)
    }

    fn collect_visible(&self, depth: usize, output: &mut Vec<VisibleEntry>) {
        output.push(VisibleEntry {
            path: self.path.clone(),
            label: self.label.clone(),
            depth,
            is_dir: self.is_dir,
            expanded: self.expanded,
        });

        if self.is_dir && self.expanded {
            for child in &self.children {
                child.collect_visible(depth + 1, output);
            }
        }
    }

    fn toggle(&mut self, path: &Path) -> bool {
        if self.path == path && self.is_dir {
            self.expanded = !self.expanded;
            return true;
        }

        for child in &mut self.children {
            if child.toggle(path) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    use std::fs;
    use tempfile::TempDir;

    use super::*;

    fn make_tree() -> (TempDir, FileTree) {
        let dir = TempDir::new().expect("tempdir");
        let root = dir.path();
        fs::create_dir(root.join("src")).unwrap();
        fs::write(root.join("Cargo.toml"), "[package]").unwrap();
        fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
        let tree = FileTree::new(root).expect("FileTree::new");
        (dir, tree)
    }

    // --- visible_entries ---

    #[test]
    fn root_is_expanded_by_default() {
        let (_dir, tree) = make_tree();
        let entries = tree.visible_entries();
        // root + its direct children should be visible
        assert!(
            entries.len() > 1,
            "root should be expanded showing children"
        );
    }

    #[test]
    fn root_entry_is_first() {
        let (dir, tree) = make_tree();
        let entries = tree.visible_entries();
        assert_eq!(entries[0].path, dir.path());
        assert!(entries[0].is_dir);
        assert!(entries[0].expanded);
    }

    #[test]
    fn dirs_sorted_before_files() {
        let (_dir, tree) = make_tree();
        let entries = tree.visible_entries();
        // Skip root at index 0; among direct children, src/ (dir) should come before Cargo.toml
        let children: Vec<_> = entries
            .iter()
            .skip(1)
            .take_while(|e| e.depth == 1)
            .collect();
        if children.len() >= 2 {
            // first child should be a directory
            assert!(children[0].is_dir, "directory should sort before files");
        }
    }

    // --- selected_index ---

    #[test]
    fn initial_selection_is_within_bounds() {
        let (_dir, tree) = make_tree();
        let total = tree.visible_entries().len();
        assert!(tree.selected_index() < total);
    }

    // --- move_selection ---

    #[test]
    fn move_selection_down_increases_index() {
        let (_dir, mut tree) = make_tree();
        tree.move_selection(-isize::try_from(tree.selected_index()).unwrap_or(0)); // move to 0
        let before = tree.selected_index();
        tree.move_selection(1);
        assert_eq!(tree.selected_index(), before + 1);
    }

    #[test]
    fn move_selection_clamps_at_zero() {
        let (_dir, mut tree) = make_tree();
        tree.move_selection(-1000); // go as far left as possible
        assert_eq!(tree.selected_index(), 0);
    }

    #[test]
    fn move_selection_clamps_at_end() {
        let (_dir, mut tree) = make_tree();
        let total = tree.visible_entries().len();
        tree.move_selection(1000);
        assert_eq!(tree.selected_index(), total - 1);
    }

    // --- toggle / activate_selected ---

    #[test]
    fn activate_on_dir_toggles_expansion() {
        let (_dir, mut tree) = make_tree();
        // Find the src dir entry
        let src_index = tree
            .visible_entries()
            .iter()
            .position(|e| e.is_dir && e.label == "src")
            .expect("src dir in entries");

        // Move to it and check its initial state
        let was_expanded = tree.visible_entries()[src_index].expanded;

        // Position selection manually by moving from current
        let current = tree.selected_index() as isize;
        tree.move_selection(src_index as isize - current);

        // Activate should toggle (returns None for dirs)
        let result = tree.activate_selected();
        assert!(result.is_none(), "activating a dir should return None");

        // Expansion state should have flipped
        let new_expanded = tree.visible_entries()[src_index].expanded;
        assert_ne!(was_expanded, new_expanded, "expansion should have toggled");
    }

    #[test]
    fn activate_on_file_returns_path() {
        let (dir, mut tree) = make_tree();
        let expected = dir.path().join("Cargo.toml");

        let cargo_index = tree
            .visible_entries()
            .iter()
            .position(|e| !e.is_dir && e.label == "Cargo.toml")
            .expect("Cargo.toml in entries");

        let current = tree.selected_index() as isize;
        tree.move_selection(cargo_index as isize - current);

        let result = tree.activate_selected();
        assert_eq!(result, Some(expected));
    }

    // --- refresh ---

    #[test]
    fn refresh_picks_up_new_file() {
        let (dir, mut tree) = make_tree();
        let before = tree.visible_entries().len();

        fs::write(dir.path().join("new_file.txt"), "content").unwrap();
        tree.refresh().expect("refresh ok");

        let after = tree.visible_entries().len();
        assert!(after > before, "new file should appear after refresh");
    }
}
