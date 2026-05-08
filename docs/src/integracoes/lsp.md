# LSP (Language Server Protocol)

O rust-ide implementa um cliente LSP enxuto que se comunica com qualquer language server compatível.

## Configurar

```toml
# config.toml
[lsp]
command = "rust-analyzer"
# args = []
# enabled = true
```

## Language servers suportados

Qualquer LSP compatível com o protocolo `stdio` funciona:

| Linguagem | Language Server | Instalar |
|---|---|---|
| Rust | `rust-analyzer` | `rustup component add rust-analyzer` |
| TypeScript/JS | `typescript-language-server` | `npm i -g typescript-language-server` |
| Python | `pylsp` | `pip install python-lsp-server` |
| Go | `gopls` | `go install golang.org/x/tools/gopls@latest` |
| C/C++ | `clangd` | `sudo apt install clangd` |
| JSON | `vscode-json-languageserver` | `npm i -g vscode-langservers-extracted` |
| YAML | `yaml-language-server` | `npm i -g yaml-language-server` |

## Recursos do cliente LSP

| Recurso | Status |
|---|---|
| `textDocument/didOpen` | ✅ Notifica ao abrir arquivo |
| `textDocument/didChange` | ✅ Notifica ao salvar |
| `textDocument/publishDiagnostics` | ✅ Exibe erros/avisos na sidebar |
| `textDocument/completion` | 🔲 Roadmap |
| `textDocument/hover` | 🔲 Roadmap |
| `textDocument/definition` | 🔲 Roadmap |
| `textDocument/formatting` | 🔲 Roadmap |

## Diagnósticos na Sidebar

Quando o LSP detecta problemas no arquivo aberto, eles aparecem na seção **LSP diagnósticos** da sidebar:

```
L12 [error] mismatched types: expected `&str`, found `String`
L34 [warning] unused variable: `x`
L56 [info] consider using `?` operator
```

Cores:
- 🔴 **error** — cor `diag_error` do tema
- 🟡 **warning** — cor `diag_warning`
- 🔵 **info** / **hint** — cor `diag_info` / `diag_hint`

## Reiniciar o LSP

Se o LSP travar ou apresentar problemas:

- Pressione **L** para reconectar
- Ou clique em **Atualizar** na barra de status (Ctrl+R)

## Desabilitar

```toml
[lsp]
enabled = false
```

Ou: `RUST_IDE_LSP_ENABLED=false rust-ide`
