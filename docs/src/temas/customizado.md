# Criar um Tema Customizado

## 1. Criar o arquivo de tema

Crie um arquivo `.toml` no diretório de temas:

```bash
mkdir -p ~/.config/rust-ide/themes
nano ~/.config/rust-ide/themes/meu-tema.toml
```

## 2. Escrever o tema

Todos os campos são **opcionais** — apenas sobrescreva os que quiser alterar. O restante herda do tema base `dark`.

```toml
# ~/.config/rust-ide/themes/meu-tema.toml

# ── Barra de status ───────────────────────────────────────
status_bg = "#005f87"
status_fg = "#ffffff"

# ── Árvore de arquivos ───────────────────────────────────
sidebar_bg       = "#1c1c1c"
sidebar_fg       = "#d0d0d0"
sidebar_selected = "#005f87"
sidebar_dir      = "#87afff"
sidebar_file     = "#d0d0d0"

# ── Editor ───────────────────────────────────────────────
editor_bg       = "#121212"
editor_fg       = "#e4e4e4"
editor_cursor   = "#ffff00"
editor_selection = "#005f87"

# ── Bordas dos painéis ───────────────────────────────────
border_focused  = "#87afff"
border_normal   = "#4e4e4e"

# ── Diagnósticos LSP ─────────────────────────────────────
diag_error   = "#ff5f5f"
diag_warning = "#ffaf00"
diag_info    = "#87afff"
diag_hint    = "#5faf5f"

# ── Git ───────────────────────────────────────────────────
git_added     = "#5faf5f"
git_modified  = "#ffaf00"
git_deleted   = "#ff5f5f"
git_renamed   = "#87afff"
git_untracked = "#d0d0d0"
```

## 3. Ativar o tema

Referencie pelo nome do arquivo (sem a extensão `.toml`):

```toml
# ~/.config/rust-ide/config.toml
theme = "meu-tema"
```

## 4. Testar sem reiniciar

Use a variável de ambiente para testar:

```bash
RUST_IDE_THEME=meu-tema rust-ide
```

## Formato de cores

Suporte a dois formatos hexadecimais:

| Formato | Exemplo | Equivalente |
|---|---|---|
| `#RRGGBB` | `#ff5f5f` | vermelho |
| `#RGB` | `#f55` | vermelho (abreviado) |

> Cores inválidas causam erro de parse e o tema base `dark` é usado como fallback.
