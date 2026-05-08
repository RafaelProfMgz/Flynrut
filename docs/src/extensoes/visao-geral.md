# O Sistema de Extensões

O rust-ide suporta extensões via arquivos TOML simples — sem código de plugin, sem Lua, sem script. Uma extensão define **atalhos de teclado**, **ferramentas externas** e **mapeamentos de tipo de arquivo**.

## Como funciona

1. Na inicialização, a IDE lê todos os arquivos `*.toml` do diretório de extensões
2. Os manifestos são carregados em ordem alfabética
3. Atalhos, ferramentas e filetypes são agregados de todas as extensões
4. Arquivos inválidos são ignorados com um aviso (a IDE não falha)

## Diretório de extensões

```
~/.config/rust-ide/extensions/
  minha-extensao.toml
  git-extras.toml
  python-tools.toml
  ...
```

## O que uma extensão pode fazer

| Recurso | Descrição |
|---|---|
| `keybindings` | Registrar atalhos de teclado com comandos nomeados |
| `tools` | Adicionar ferramentas externas na sidebar e lançar via `Ctrl+tecla` |
| `filetypes` | Mapear extensões de arquivo para linguagens (para highlight e LSP) |

## Limitações atuais

- Extensões não podem executar código Rust (sem FFI/WASM por enquanto)
- Comandos de keybinding são identificadores — a lógica de execução está no core
- Comandos de tools são executados via `sh -c` em background

> **Roadmap**: suporte a plugins WASM para lógica customizada.
