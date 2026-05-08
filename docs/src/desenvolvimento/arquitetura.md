# Arquitetura do Projeto

## Visão Geral

```
rust-ide/
├── src/
│   ├── lib.rs              ← re-exporta todos os módulos
│   ├── main.rs             ← entry point: cria App, roda IdeApp
│   ├── app/mod.rs          ← App state (negócio puro, sem deps UI)
│   ├── config/mod.rs       ← leitura de config TOML + env vars
│   ├── editor/mod.rs       ← EditorModel (buffer de texto puro)
│   ├── fs_tree/mod.rs      ← FileTree (walkdir, seleção, navegação)
│   ├── integrations/mod.rs ← git, lazygit, lazydocker, tools
│   ├── lsp/mod.rs          ← cliente LSP stdio
│   ├── theme/mod.rs        ← ThemeColors, temas embutidos, TOML
│   ├── extensions/mod.rs   ← ExtensionRegistry, manifestos TOML
│   └── gui/
│       ├── mod.rs          ← IdeApp (iced::Application), Message, pane_grid
│       ├── editor.rs       ← widget text_editor com line numbers
│       ├── file_tree.rs    ← widget árvore de arquivos
│       ├── sidebar.rs      ← widget sidebar (git, tools, LSP)
│       └── style.rs        ← bridge ThemeColors → iced Colors
├── docs/                   ← manual (mdBook)
├── examples/config/        ← exemplos comentados de config/tema/extensão
└── Cargo.toml
```

## Separação de camadas

```
┌─────────────────────────────────────────────────┐
│  Camada de apresentação (src/gui/)              │
│  Iced 0.14 + wgpu (GPU-acelerado)               │
│  IdeApp, widgets, estilos, bridge de tema       │
├─────────────────────────────────────────────────┤
│  Camada de estado (src/app/)                    │
│  App struct — sem deps de UI, testável          │
│  Métodos de ação: open_file, save, refresh, ... │
├─────────────────────────────────────────────────┤
│  Módulos de domínio (puro Rust, sem UI)         │
│  config  editor  fs_tree  integrations          │
│  lsp     theme   extensions                     │
└─────────────────────────────────────────────────┘
```

## Fluxo de dados (Iced Elm Architecture)

```
Evento do usuário
      │
      ▼
  Message enum
  (EditorAction, TreeEntryPressed, SaveRequested, ...)
      │
      ▼
  IdeApp::update()
  ├── modifica self.editor_content (estado Iced)
  ├── chama self.app.* (lógica de negócio)
  └── retorna Task<Message> (ação assíncrona)
      │
      ▼
  IdeApp::view()
  └── gera Element<Message> com estado atual
      │
      ▼
  wgpu renderiza na GPU
```

## Tick System

A IDE usa um sistema de tick a 250ms para atualizar informações em background:

```
Task::perform(sleep(250ms), |_| Message::Tick)
    │
    ▼ Message::Tick
IdeApp::update
    ├── app.on_tick()
    │   ├── integrations.refresh() → status git
    │   └── lsp.drain()            → diagnósticos LSP
    └── agenda próximo tick
```

## Tecnologias

| Componente | Tecnologia |
|---|---|
| GUI framework | [Iced 0.14](https://iced.rs) |
| Renderização | wgpu (Vulkan/Metal/DX12) + tiny-skia (software fallback) |
| Syntax highlight | [syntect 5](https://github.com/trishume/syntect) |
| Git | [libgit2](https://libgit2.org) via `git2` crate |
| LSP | JSON-RPC sobre stdio (implementação própria) |
| Config/Temas/Extensões | [TOML](https://toml.io) via `toml` + `serde` |
| Caminhos de config | `directories` crate (XDG no Linux) |
| Árvore de arquivos | `walkdir` crate |
