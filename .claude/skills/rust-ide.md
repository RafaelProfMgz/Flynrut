---
name: rust-ide
description: >
  Architecture, module contracts, public APIs, and coding conventions for rust-ide.
  Invoke this skill when working on any module of this project.
type: flexible
---

# rust-ide — Architecture Reference

## Module Map

### `src/lib.rs`
Re-exports all public modules. Required for integration tests and external tooling.

### `src/app/mod.rs`
- **`App`** — main state container. Owns `FileTree`, `EditorModel`, `IntegrationState`, `LspClient`.
- **`Focus`** — `Tree | Editor | Sidebar`; now driven by GUI pane selection instead of terminal key events.
- Key methods: `new()`, `on_tick()`, `cycle_focus_forward()`, `cycle_focus_backward()`,
  `open_file()`, `open_tree_selection()`, `save_file()`, `save_content()`, `refresh()`,
  `launch_tool()`, `restart_lsp()`.

### `src/config/mod.rs`
- **`AppConfig`** — loaded once at startup. Sources: env vars > TOML file > defaults.
- Config file: `~/.config/rust-ide/config.toml` (via `directories::ProjectDirs`).
- **`ToolCommands`** — lazygit, lazydocker, ai (optional), mcp (optional).
- **`LspSettings`** — command, enabled.
- Pure/testable: `parse_bool(value: &str) -> Option<bool>` (pub(crate)).

### `src/editor/mod.rs`
- **`EditorModel`** — pure editor state without UI dependencies.
- Stores `content: String`, `current_file: Option<PathBuf>`, and `dirty: bool`.
- Key methods: `new()`, `open(path)`, `save()`, `save_content(text)`, `contents()`, `set_dirty()`.
- `title()` reflects current file + dirty marker.
- Pure/testable: `split_lines(content: &str) -> Vec<String>` (pub(crate)).

### `src/fs_tree/mod.rs`
- **`FileTree`** — tree of directory entries with lazy children.
- **`VisibleEntry`** — flattened visible node (path, label, depth, is_dir, expanded).
- Key methods: `new(root)`, `refresh()`, `visible_entries()`, `move_selection(delta)`,
  `activate_selected()`, `selected_index()`.
- Skips `.git/` and `target/` directories automatically.
- Sort order: dirs before files, then lexicographic.

### `src/integrations/mod.rs`
- **`IntegrationState`** — git snapshot + tool availability. Refreshed on tick (≥2s).
- **`GitStatusSnapshot`** — branch, staged, unstaged, untracked, ahead, behind.
- **`ToolStatus`** — name, command (Option), available (bool, detected via PATH).
- Tools detected: git, lazygit, lazydocker, ai, mcp.
- `refresh()` uses 2-second debounce; `refresh_now()` forces immediate update.

### `src/lsp/mod.rs`
- **`LspClient`** — spawns language server subprocess; communicates over stdin/stdout.
- Protocol: JSON-RPC 2.0, `Content-Length: N\r\n\r\n{payload}`.
- Notifications sent: `initialize`, `initialized`, `textDocument/didOpen`,
  `textDocument/didChange`, `textDocument/didSave`.
- Events received: `textDocument/publishDiagnostics` → parsed to `DiagnosticItem`.
- `drain()` must be called on each tick to process incoming messages non-blockingly.
- Only syncs `.rs` files.

### `src/gui/mod.rs`
- **`IdeApp`** — Iced application wrapper around `App`.
- Uses `pane_grid` for a 3-pane layout: file tree (~22%) | editor (~54%) | sidebar (~24%).
- Keeps `text_editor::Content` as GUI state and syncs it with `App::editor` on open/save.
- `Message` handles pane clicks/resizes, editor actions, file tree selection, tool launch,
  refresh/save requests, LSP restart, and periodic tick updates.
- Submodules:
  - `gui/file_tree.rs` — scrollable tree buttons with indentation and icons.
  - `gui/editor.rs` — tab bar + line numbers + `iced::widget::text_editor`.
  - `gui/sidebar.rs` — Git snapshot, tools, and LSP diagnostics.
  - `gui/style.rs` — converts `ThemeColors` tokens into Iced styles.

## Interaction model

- GUI-first flow with Iced 0.14 + wgpu.
- Focus is derived from pane clicks and preserved in `App::focus`.
- File tree entries are clickable buttons; selecting a directory toggles it, selecting a file opens it.
- Save/refresh actions live in the status bar; tool launch + LSP restart live in the sidebar.
- `IdeApp` schedules a 250ms tick task that calls `App::on_tick()`.

## Test locations

```
src/config/mod.rs      #[cfg(test)] mod tests  → parse_bool, default loading
src/editor/mod.rs      #[cfg(test)] mod tests  → split_lines, title, dirty state
src/fs_tree/mod.rs     #[cfg(test)] mod tests  → move_selection, toggle, visible_entries
src/integrations/mod.rs #[cfg(test)] mod tests → binary_for_command, tool detection
```

## Dependency notes

- GUI stack: `iced 0.14` with `advanced`, `svg`, and `wgpu` features.
- Editor widget: `iced::widget::text_editor`; business editor state stays in `EditorModel`.
- `syntect 5.x` is available for future syntax highlighting work.
- `git2` requires OpenSSL/libssh2 (installed via system package manager).

## Changelog

- **v0.1.0** — Initial modular implementation: file tree, editor, git, LSP, tool integrations.
- **v0.1.1** — Professional structure: subdirectory modules, unit tests, CLAUDE.md, skills.
- **v0.1.2** — TDD cycle complete: 29 unit tests; zero clippy warnings with pedantic lints.
- **v0.1.3** — Theme + extension system: `src/theme/mod.rs` (5 built-in themes, TOML merge),
  `src/extensions/mod.rs` (manifest registry, keybindings, tools, filetypes); `App` now owns
  `theme: ThemeColors` and `extensions: ExtensionRegistry`; all hardcoded colors replaced with
  theme tokens; `render_sidebar` split into helper fns; 52 tests, 0 clippy errors;
  `examples/config/` with annotated config.toml, my-theme.toml, my-extension.toml.
- **v0.2.0** — Migration from terminal TUI to Iced GUI: removed `src/ui/`, rewrote
  `EditorModel` as pure state, simplified `App` for GUI-driven actions, added `src/gui/`
  (`mod`, `style`, `file_tree`, `editor`, `sidebar`), and updated `main.rs` to launch
  the Iced application.
