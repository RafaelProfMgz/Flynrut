# Arquivo de Configuração

## Localização

| Sistema | Caminho |
|---|---|
| Linux | `~/.config/rust-ide/config.toml` |
| macOS | `~/Library/Application Support/rust-ide/config.toml` |
| Windows | `%APPDATA%\rust-ide\config.toml` |

Se o arquivo não existir, a IDE usa valores padrão e cria o diretório automaticamente.

## Exemplo completo

```toml
# ~/.config/rust-ide/config.toml

# ── Tema ────────────────────────────────────────────────────────
# Valores: "dark" | "gruvbox" | "nord" | "dracula" | "light"
# Ou nome de arquivo em ~/.config/rust-ide/themes/<nome>.toml
theme = "dark"

# ── LSP ─────────────────────────────────────────────────────────
[lsp]
# Binário do language server (deve estar no PATH ou ser caminho absoluto)
command = "rust-analyzer"

# Argumentos extras (opcional)
# args = ["--log-file", "/tmp/ra.log"]

# Desabilitar completamente (padrão: habilitado se command estiver no PATH)
# enabled = false

# ── Ferramentas externas ─────────────────────────────────────────
[tools]
git        = "git"
lazygit    = "lazygit"
lazydocker = "lazydocker"
docker     = "docker"

# ── AI / MCP ────────────────────────────────────────────────────
[ai]
# Endpoint compatível com OpenAI Chat API
# endpoint = "https://api.openai.com/v1/chat/completions"
model   = "gpt-4o-mini"
enabled = false
```

## Referência de campos

### Raiz

| Campo | Tipo | Padrão | Descrição |
|---|---|---|---|
| `theme` | string | `"dark"` | Nome do tema embutido ou customizado |

### `[lsp]`

| Campo | Tipo | Padrão | Descrição |
|---|---|---|---|
| `command` | string | `"rust-analyzer"` | Caminho/nome do binário LSP |
| `args` | lista de strings | `[]` | Argumentos extras |
| `enabled` | bool | `true` se command existe no PATH | Habilitar/desabilitar LSP |

### `[tools]`

| Campo | Tipo | Padrão | Descrição |
|---|---|---|---|
| `git` | string | `"git"` | Binário do git |
| `lazygit` | string | `"lazygit"` | Binário do lazygit |
| `lazydocker` | string | `"lazydocker"` | Binário do lazydocker |
| `docker` | string | `"docker"` | Binário do docker |

### `[ai]`

| Campo | Tipo | Padrão | Descrição |
|---|---|---|---|
| `endpoint` | string | — | URL da API (OpenAI-compatível) |
| `model` | string | `"gpt-4o-mini"` | Modelo a usar |
| `enabled` | bool | `false` | Habilitar integração AI |

## Diretórios derivados

A partir do diretório de configuração, a IDE deriva automaticamente:

| Diretório | Conteúdo |
|---|---|
| `~/.config/rust-ide/themes/` | Arquivos TOML de temas customizados |
| `~/.config/rust-ide/extensions/` | Manifestos TOML de extensões |
