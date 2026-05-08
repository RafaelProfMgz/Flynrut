# Módulos

## `src/app/mod.rs`

Estado central da aplicação. Agrega todos os módulos de domínio.

**Structs principais:**
- `App` — estado completo: workspace, config, tree, editor, integrations, lsp, theme, extensions
- `Focus` — painel com foco: `Tree | Editor | Sidebar`

**Métodos públicos:**
```rust
App::new(workspace_root: PathBuf, config: AppConfig) -> Result<Self>
app.on_tick()                       // refresh periódico
app.cycle_focus_forward()           // Tab
app.cycle_focus_backward()          // Shift+Tab
app.tree_move_up()                  // ↑ na árvore
app.tree_move_down()                // ↓ na árvore
app.open_tree_selection()           // Enter na árvore
app.open_file(path: &Path)          // abrir arquivo diretamente
app.save_file()                     // salvar via EditorModel
app.save_content(text: &str)        // salvar texto da GUI
app.refresh()                       // Ctrl+R
app.launch_tool(name: &str) -> Option<String>  // lançar ferramenta
app.set_status(msg: String)         // atualizar barra de status
app.restart_lsp()                   // reconectar LSP
```

## `src/config/mod.rs`

Carrega configuração de múltiplas fontes com prioridade.

**Structs:**
- `AppConfig` — configuração resolvida final
- `DiskConfig` — struct serde para o TOML
- `LspSettings`, `ToolCommands` — subconfigurações

**Função principal:**
```rust
AppConfig::load(workspace_root: &Path) -> Result<AppConfig>
```

## `src/editor/mod.rs`

Buffer de texto puro, sem dependências de UI.

```rust
EditorModel::new() -> Self
model.open(path: &Path) -> Result<()>
model.save() -> Result<PathBuf>
model.save_content(text: &str) -> Result<PathBuf>
model.contents() -> &str
model.current_file() -> Option<&Path>
model.title() -> String
model.set_dirty(dirty: bool)
```

## `src/fs_tree/mod.rs`

Árvore de arquivos baseada em `walkdir`.

```rust
FileTree::new(root: &Path) -> Result<Self>
tree.visible() -> &[VisibleEntry]   // entradas visíveis para renderizar
tree.move_selection(delta: isize)   // navegar
tree.activate_selected() -> Option<PathBuf>  // abrir selecionado
tree.selected_index() -> usize
tree.refresh() -> Result<()>
```

## `src/theme/mod.rs`

Sistema de temas com 22 tokens de cor.

```rust
ThemeColors::builtin(name: &str) -> ThemeColors  // dark/gruvbox/nord/dracula/light
ThemeColors::load_from_file(path, base) -> Result<ThemeColors>
ThemeColor(r, g, b)                              // cor RGB
color.to_ratatui() -> ratatui::style::Color     // conversão (legado)
color.components() -> (u8, u8, u8)              // para iced::Color
```

## `src/extensions/mod.rs`

Carrega e agrega manifestos de extensões.

```rust
ExtensionRegistry::load_from_dir(dir: &Path) -> Result<Self>
registry.all_keybindings() -> Vec<&ExtKeybinding>
registry.all_tools() -> Vec<&ExtTool>
registry.filetype_map() -> HashMap<&str, &str>
```

## `src/integrations/mod.rs`

Detecta e monitora ferramentas externas.

```rust
IntegrationState::discover(root: &Path, config: &AppConfig) -> Self
state.refresh(root: &Path)
state.refresh_now(root: &Path, tools: &ToolCommands)
// campos públicos: git, tools, config_path
```

## `src/lsp/mod.rs`

Cliente LSP via stdin/stdout JSON-RPC.

```rust
LspClient::new(workspace: PathBuf, command: String) -> Self
lsp.start() -> Result<()>
lsp.is_running() -> bool
lsp.drain()                           // processar respostas pendentes
lsp.sync_document(path, content)      // didOpen/didChange
lsp.diagnostics_for(file) -> Vec<Diagnostic>
```

## `src/gui/`

Camada de apresentação Iced.

- `mod.rs` — `IdeApp`, `Message`, `PaneKind`, pane_grid
- `style.rs` — conversões de cor e estilos Iced
- `file_tree.rs` — `view(tree, theme)` → `Element<Message>`
- `editor.rs` — `view(content, title, focused, theme)` → `Element<Message>`
- `sidebar.rs` — `view(app)` → `Element<Message>`
