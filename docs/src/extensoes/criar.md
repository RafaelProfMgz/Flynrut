# Criar uma Extensão

## Template completo

```toml
# ~/.config/rust-ide/extensions/minha-extensao.toml

# ── Metadados ────────────────────────────────────────────────────
name        = "minha-extensao"
version     = "1.0.0"
description = "Minha extensão personalizada"

# ── Atalhos de teclado ───────────────────────────────────────────
[[keybindings]]
key         = "Ctrl-P"
command     = "open_file_picker"
description = "Abrir seletor de arquivos"

[[keybindings]]
key         = "Ctrl-Shift-G"
command     = "open_lazygit"
description = "Abrir lazygit"

[[keybindings]]
key         = "F5"
command     = "run_tests"
description = "Executar testes"

# ── Ferramentas externas ─────────────────────────────────────────
[[tools]]
name        = "cargo-fmt"
command     = "cargo fmt --all"
description = "Formatar com rustfmt"

[[tools]]
name        = "cargo-clippy"
command     = "cargo clippy --all-targets -- -D warnings"
description = "Linter estrito"

[[tools]]
name        = "just"
command     = "just"
description = "Executar Justfile"

# ── Mapeamento de tipos de arquivo ───────────────────────────────
[filetypes]
"rs"    = "rust"
"toml"  = "toml"
"md"    = "markdown"
"json"  = "json"
"yaml"  = "yaml"
"yml"   = "yaml"
"sh"    = "bash"
"py"    = "python"
"ts"    = "typescript"
"tsx"   = "typescript"
```

## Guia de campos

### Metadados

```toml
name        = "identificador-unico"   # sem espaços, use hífens
version     = "1.0.0"                 # semver
description = "Texto livre"           # opcional
```

### Keybindings

```toml
[[keybindings]]
key         = "Ctrl-P"        # Ctrl, Alt, Shift + tecla
command     = "meu_comando"   # identificador snake_case
description = "Texto"         # exibido na paleta de comandos
```

**Modificadores suportados**: `Ctrl`, `Alt`, `Shift`, `Ctrl-Shift`, `Ctrl-Alt`

**Teclas especiais**: `F1`-`F12`, `Enter`, `Escape`, `Tab`, `Space`, `Up`, `Down`, `Left`, `Right`

### Tools

```toml
[[tools]]
name        = "nome-curto"        # exibido na sidebar (máx ~15 chars)
command     = "shell command"     # executado com sh -lc em background
description = "Texto"
```

**Variáveis disponíveis no command** (roadmap):
- `$WORKSPACE` — raiz do workspace
- `$FILE` — caminho do arquivo aberto
- `$LINE` — linha atual do cursor

### Filetypes

```toml
[filetypes]
"extensao" = "linguagem"   # sem ponto na extensão
```

Linguagens comuns: `rust`, `python`, `javascript`, `typescript`, `go`, `c`, `cpp`, `java`, `json`, `yaml`, `toml`, `markdown`, `bash`, `html`, `css`
