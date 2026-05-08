# Variáveis de Ambiente

Variáveis de ambiente têm **prioridade máxima**, sobrescrevendo qualquer valor do `config.toml`.

## Lista de variáveis

| Variável | Equivalente no config | Exemplo |
|---|---|---|
| `RUST_IDE_THEME` | `theme` | `RUST_IDE_THEME=gruvbox` |
| `RUST_IDE_LSP_COMMAND` | `lsp.command` | `RUST_IDE_LSP_COMMAND=/usr/local/bin/rust-analyzer` |
| `RUST_IDE_LSP_ENABLED` | `lsp.enabled` | `RUST_IDE_LSP_ENABLED=false` |
| `OPENAI_API_KEY` | — (chave da API AI) | `OPENAI_API_KEY=sk-...` |

## Exemplos de uso

```bash
# Abrir com tema diferente sem editar o config
RUST_IDE_THEME=nord rust-ide

# Desabilitar LSP temporariamente
RUST_IDE_LSP_ENABLED=false rust-ide

# Usar um rust-analyzer alternativo
RUST_IDE_LSP_COMMAND=~/.cargo/bin/rust-analyzer rust-ide

# Passar chave da API de AI
OPENAI_API_KEY=sk-seu-token rust-ide
```

## Prioridade de configuração

```
Variáveis de ambiente  (prioridade 1 — mais alta)
       ↓
config.toml            (prioridade 2)
       ↓
Valores padrão         (prioridade 3 — mais baixa)
```
