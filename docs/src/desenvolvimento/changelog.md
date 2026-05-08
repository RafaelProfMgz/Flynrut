# Changelog

## v0.2.0 — GUI com Iced (atual)

**Mudanças de breaking change:**
- Substituiu a TUI (ratatui/crossterm) pela GUI Iced 0.14 com wgpu
- `EditorModel` agora usa `String` puro (removeu `tui-textarea`)
- `app::handle_key` substituído por métodos específicos de ação

**Adicionado:**
- `src/gui/` — camada de apresentação Iced completa
- Layout com `pane_grid` — 3 painéis redimensionáveis
- `text_editor::Content` para edição na GUI
- Syntax highlight via `syntect`
- Janela GPU-acelerada (wgpu/Vulkan/Metal)
- `App::save_content()`, `App::launch_tool()`, `App::restart_lsp()`

**60 testes passando, 0 erros clippy**

---

## v0.1.3 — Temas e Extensões

**Adicionado:**
- `src/theme/mod.rs` — 5 temas embutidos, sistema TOML customizável, 22 tokens de cor
- `src/extensions/mod.rs` — manifestos TOML, keybindings, tools, filetypes
- `App` passa a ter `theme: ThemeColors` e `extensions: ExtensionRegistry`
- `examples/config/` — arquivos de exemplo documentados
- Toda a UI usa tokens de tema (sem cores hardcoded)

---

## v0.1.2 — Qualidade e TDD

**Adicionado:**
- 29 testes unitários (config, editor, fs_tree, integrations)
- `impl Default for EditorModel`
- `# Errors` em todas as funções `Result`-retornantes públicas
- `#[must_use]` em getters puros

**Zero erros clippy com lints `pedantic` + `unwrap_used` + `expect_used` + `panic`**

---

## v0.1.1 — Estrutura Profissional

**Adicionado:**
- Módulos em subdiretorios (`src/app/`, `src/config/`, etc.)
- `src/lib.rs` re-exportando todos os módulos
- `CLAUDE.md`, `.claude/skills/tdd.md`, `.claude/skills/rust-ide.md`
- `Cargo.toml` com metadados, `[lib]` + `[[bin]]`, lints pedantic

---

## v0.1.0 — Versão Inicial (TUI)

**Adicionado:**
- TUI com ratatui + crossterm
- Árvore de arquivos, editor, sidebar
- Integração git, lazygit, lazydocker
- Cliente LSP
- Configuração TOML
