# Referência do Manifesto

Especificação completa do formato TOML de extensões.

## Campos raiz

| Campo | Tipo | Obrigatório | Padrão | Descrição |
|---|---|---|---|---|
| `name` | string | **sim** | — | Identificador único da extensão |
| `version` | string | não | `"0.1.0"` | Versão SemVer |
| `description` | string | não | `""` | Texto descritivo |
| `keybindings` | lista | não | `[]` | Atalhos de teclado |
| `tools` | lista | não | `[]` | Ferramentas externas |
| `filetypes` | mapa | não | `{}` | Mapeamento extensão→linguagem |

## `[[keybindings]]`

| Campo | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `key` | string | **sim** | Combinação de teclas (ex: `"Ctrl-P"`) |
| `command` | string | **sim** | Identificador do comando |
| `description` | string | não | Texto na paleta de comandos |

## `[[tools]]`

| Campo | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `name` | string | **sim** | Nome curto para a sidebar |
| `command` | string | **sim** | Comando shell a executar |
| `description` | string | não | Descrição na paleta |

## `[filetypes]`

Mapa de pares `"extensão" = "linguagem"`. A extensão não deve ter ponto.

```toml
[filetypes]
"rs"   = "rust"
"lock" = "toml"    # Cargo.lock usa TOML
```

## Exemplo mínimo válido

```toml
name = "minimal"
```

Uma extensão só com `name` é válida (não faz nada, mas não causa erro).
