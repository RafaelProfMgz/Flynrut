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
- **`Focus`** — `Tree | Editor | Sidebar`; cycled with `Tab`/`BackTab`.
- **`AppAction`** — returned by `handle_key`: `Continue | Quit | RunCommand(String)`.
- Key methods: `new()`, `handle_key()`, `on_tick()`, `after_command()`.

### `src/config/mod.rs`
- **`AppConfig`** — loaded once at startup. Sources: env vars > TOML file > defaults.
- Config file: `~/.config/rust-ide/config.toml` (via `directories::ProjectDirs`).
- **`ToolCommands`** — lazygit, lazydocker, ai (optional), mcp (optional).
- **`LspSettings`** — command, enabled.
- Pure/testable: `parse_bool(value: &str) -> Option<bool>` (pub(crate)).

### `src/editor/mod.rs`
- **`EditorModel`** — wraps `tui_textarea::TextArea<'static>`.
- Key methods: `new()`, `open(path)`, `save()`, `contents()`, `handle_key()`, `textarea(focused)`.
- Tracks dirty state; `title()` reflects it.
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

### `src/ui/mod.rs`
- `draw(frame, app)` — main entry point, called every render frame.
- Three-panel layout: file tree (24%) | editor (48%) | sidebar (28%).
- Sidebar sub-panels: Git info | Tool status | LSP diagnostics.
- Status bar: 2-line bar with current message + keybinding hints.

## Keybindings (app layer)

| Key | Action |
|---|---|
| `Tab` / `BackTab` | Cycle focus: Tree → Editor → Sidebar |
| `q` | Quit |
| `Ctrl+S` | Save current file |
| `Ctrl+R` | Refresh tree + integrations |
| `g` | Launch lazygit |
| `d` | Launch lazydocker |
| `a` | Launch configured AI command |
| `m` | Launch configured MCP command |
| `l` | Connect/reconnect LSP |
| `↑/k`, `↓/j` | Move tree selection |
| `Enter/→/o` | Open file or toggle dir |

## Test locations

```
src/config/mod.rs      #[cfg(test)] mod tests  → parse_bool, default loading
src/editor/mod.rs      #[cfg(test)] mod tests  → split_lines, title, dirty state
src/fs_tree/mod.rs     #[cfg(test)] mod tests  → move_selection, toggle, visible_entries
src/integrations/mod.rs #[cfg(test)] mod tests → binary_for_command, tool detection
```

## Dependency notes

- `tui-textarea 0.7` requires `ratatui 0.29` and `crossterm 0.28` (not newer versions).
- Do NOT upgrade ratatui/crossterm without verifying tui-textarea compatibility.
- `git2` requires OpenSSL/libssh2 (installed via system package manager).

## Changelog

- **v0.1.0** — Initial modular implementation: file tree, editor, git, LSP, tool integrations.
- **v0.1.1** — Professional structure: subdirectory modules, unit tests, CLAUDE.md, skills.
- **v0.1.2** — TDD cycle complete: 29 unit tests (config, editor, fs_tree, integrations); zero
  clippy warnings with `pedantic` + `unwrap_used` + `expect_used` + `panic` lints enforced;
  `# Errors` doc-comments on all public `Result`-returning functions; `#[must_use]` on pure
  getters; `impl Default for EditorModel`; `if let` chains replace nested `if/match`.
