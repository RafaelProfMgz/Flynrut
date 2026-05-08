# Glossário

| Termo | Definição |
|---|---|
| **LSP** | Language Server Protocol — padrão para comunicação entre editores e servidores de análise de código |
| **MCP** | Model Context Protocol — protocolo para expor contexto estruturado a modelos de linguagem |
| **wgpu** | Biblioteca Rust para renderização GPU cross-platform (Vulkan, Metal, DirectX, WebGPU) |
| **Iced** | Framework GUI Rust inspirado em Elm, usa wgpu para renderização GPU-acelerada |
| **GPUI** | Framework GUI do Zed, GPU-acelerado — inspiração de design para o rust-ide |
| **pane_grid** | Widget Iced para layout de painéis redimensionáveis com drag-and-drop |
| **text_editor** | Widget Iced embutido para edição de texto com histórico, seleção e clipboard |
| **syntect** | Biblioteca Rust para syntax highlighting usando gramáticas TextMate |
| **rust-analyzer** | Language server oficial do Rust — fornece autocomplete, diagnósticos, go-to-definition |
| **lazygit** | TUI Git interativa para terminais |
| **lazydocker** | TUI Docker interativa para terminais |
| **libgit2** | Biblioteca C para operações Git, usada via crate `git2` |
| **TOML** | Tom's Obvious Minimal Language — formato de configuração usado no Cargo.toml |
| **XDG** | X Desktop Group — padrão de diretórios no Linux (`~/.config/`, `~/.local/share/`) |
| **TDD** | Test-Driven Development — desenvolvimento guiado por testes (RED → GREEN → REFACTOR) |
| **SemVer** | Semantic Versioning — versionamento `MAJOR.MINOR.PATCH` |
| **tick** | Intervalo periódico (250ms) em que a IDE atualiza status git e drena mensagens LSP |
| **workspace** | Diretório raiz do projeto aberto na IDE |
| **staging area** | Área de preparação do git (arquivos prontos para commit) |
| **dirty** | Estado do editor quando há alterações não salvas |
