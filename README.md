# rust-ide

IDE gráfica modular escrita em Rust — rápida, configurável e com integrações nativas.

**Stack:** [Iced](https://iced.rs/) · git2 · LSP nativo · lazygit · lazydocker · AI · MCP

---

## Instalação e execução

### Pré-requisitos

- Rust 1.82+ (`rustup update stable`)
- Sistema Linux com ambiente gráfico (X11 ou Wayland)
- Dependências do sistema: `libxkbcommon`, `libwayland` (normalmente já presentes)

### Compilar e rodar

```bash
# Clonar o repositório
git clone <repo-url>
cd rust-ide

# Compilar em modo release (recomendado)
cargo build --release
./target/release/rust_ide

# Ou rodar diretamente (mais lento, sem otimizações)
cargo run

# Abrir um diretório específico
cd /caminho/para/projeto && cargo run
```

> A IDE abre sempre no diretório de trabalho atual.

---

## Atalhos de teclado

Todos os atalhos são configuráveis em `~/.config/rust-ide/keybindings.toml`.

### Globais (sempre ativos)

| Atalho      | Ação                                 |
| ----------- | ------------------------------------ |
| `Ctrl+S`    | Salvar arquivo                       |
| `Ctrl+R`    | Atualizar árvore + git + LSP         |
| `Ctrl+Q`    | Sair da IDE                          |
| `Tab`       | Mover foco para o próximo painel     |
| `Shift+Tab` | Mover foco para o painel anterior    |
| `Ctrl+1`    | Foco direto na Árvore de Arquivos    |
| `Ctrl+2`    | Foco direto no Editor                |
| `Ctrl+3`    | Foco direto na Sidebar               |
| `Ctrl+,`    | Abrir/fechar painel de Configurações |

### Ferramentas externas

| Atalho | Ação                 |
| ------ | -------------------- |
| `g`    | Abrir lazygit        |
| `d`    | Abrir lazydocker     |
| `a`    | Executar cliente AI  |
| `m`    | Executar cliente MCP |
| `l`    | Reconectar o LSP     |

### Árvore de arquivos (quando em foco)

| Atalho              | Ação                                  |
| ------------------- | ------------------------------------- |
| `↑` / `k`           | Mover seleção para cima (estilo vim)  |
| `↓` / `j`           | Mover seleção para baixo (estilo vim) |
| `Enter` / `→` / `o` | Abrir arquivo / expandir pasta        |
| `←` / `h`           | Subir ao diretório pai                |

---

## Configuração

### Arquivo principal

Localização: `~/.config/rust-ide/config.toml`

```toml
# Tema: "dark" | "gruvbox" | "nord" | "dracula" | "light"
theme = "dark"

[lsp]
command = "rust-analyzer"
enabled = true

[tools]
lazygit    = "lazygit"
lazydocker = "lazydocker"
ai         = "claude"
mcp        = "npx @modelcontextprotocol/inspector"

[docs]
# Porta do servidor mdbook serve (mude se a 3000 já estiver ocupada)
port = 3000
```

### Atalhos customizados

Localização: `~/.config/rust-ide/keybindings.toml`

```toml
# Todos os campos são opcionais — só especifique o que quiser mudar
[global]
quit          = "Ctrl+W"     # Trocar Ctrl+Q por Ctrl+W
focus_tree    = "Ctrl+1"
focus_editor  = "Ctrl+2"
focus_sidebar = "Ctrl+3"
open_settings = "Ctrl+,"

[tools]
lazygit = "Ctrl+G"           # Atalho alternativo para lazygit

[tree]
move_up   = ["Up", "k"]
move_down = ["Down", "j"]
open      = ["Enter", "Right", "o"]
go_parent = ["Left", "h"]
```

### Variáveis de ambiente

As variáveis sobrescrevem o arquivo TOML:

| Variável                      | Descrição                 |
| ----------------------------- | ------------------------- |
| `RUST_IDE_THEME`              | Nome do tema              |
| `RUST_IDE_LSP_COMMAND`        | Binário do LSP            |
| `RUST_IDE_LSP_ENABLED`        | `true` / `false`          |
| `RUST_IDE_LAZYGIT_COMMAND`    | Comando lazygit           |
| `RUST_IDE_LAZYDOCKER_COMMAND` | Comando lazydocker        |
| `RUST_IDE_AI_COMMAND`         | Comando AI                |
| `RUST_IDE_MCP_COMMAND`        | Comando MCP               |
| `RUST_IDE_DOCS_PORT`          | Porta para `mdbook serve` |

---

## Menu de Configurações

Pressione **`Ctrl+,`** ou clique no botão **"Configurações"** na barra inferior para abrir o painel de configurações na sidebar. Nele você pode:

- Ver o caminho de cada arquivo de configuração
- Abrir `config.toml`, `keybindings.toml`, temas e extensões diretamente no editor
- Iniciar o servidor de documentação (`mdbook serve`) na porta configurada
- Ver o status de todas as integrações

---

## Documentação do projeto

A documentação completa está em `docs/` e usa [mdBook](https://rust-lang.github.io/mdBook/).

### Rodar localmente

```bash
# Instalar mdbook (se ainda não tiver)
cargo install mdbook

# Servir na porta padrão (3000)
mdbook serve docs/

# Servir em outra porta (se a 3000 estiver ocupada)
mdbook serve --port 3001 docs/

# Ou configure a porta no config.toml e use o botão na IDE:
# [docs]
# port = 3001
```

Acesse em: `http://localhost:<porta>`

### Gerar HTML estático

```bash
mdbook build docs/
# Saída em: docs/book/
```

---

## Desenvolvimento

```bash
# Executar testes
cargo test

# Testes de um módulo específico
cargo test keybindings
cargo test config
cargo test fs_tree

# Lint
cargo clippy --all-targets -- -D warnings

# Formatar
cargo fmt

# Verificar formatação sem aplicar
cargo fmt --check
```

### Arquitetura

```text
src/
  main.rs          ← entrypoint (carrega config, inicia GUI)
  app/mod.rs       ← estado global, ciclo de foco, operações de arquivo
  config/mod.rs    ← AppConfig: TOML + env vars
  editor/mod.rs    ← EditorModel: abertura, edição, salvamento
  fs_tree/mod.rs   ← FileTree: árvore com navegação e toggle de pastas
  gui/
    mod.rs         ← IdeApp (Iced): layout 3 painéis, messages, keybindings
    editor.rs      ← widget do editor
    file_tree.rs   ← widget da árvore
    sidebar.rs     ← sidebar (normal + painel de configurações)
    style.rs       ← estilos visuais
  integrations/    ← git2 nativo, detecção de ferramentas
  keybindings/     ← configuração de atalhos via TOML
  lsp/             ← cliente LSP enxuto
  theme/           ← temas embutidos e customizados
  extensions/      ← registro de extensões
```
