# rust-ide

IDE terminal em Rust, modular e rápida, com:

- árvore de arquivos estilo VS Code/Telescope
- editor embutido
- integração nativa com git, lazygit e lazydocker
- cliente LSP enxuto para `rust-analyzer`
- pontos de integração para AI e MCP por configuração

## Rodando

```bash
cargo run
```

## Atalhos

- `Tab` alterna foco entre árvore, editor e sidebar
- `Enter` abre arquivo ou expande/recolhe diretório
- `Ctrl+S` salva o arquivo atual
- `Ctrl+R` atualiza a árvore e o estado das integrações
- `g` abre `lazygit`
- `d` abre `lazydocker`
- `a` executa o comando configurado de AI
- `m` executa o comando configurado de MCP
- `l` reconecta o LSP
- `q` sai

## Configuração

O arquivo de configuração fica em `~/.config/rust-ide/config.toml`.

Exemplo:

```toml
[lsp]
command = "rust-analyzer"
enabled = true

[tools]
lazygit = "lazygit"
lazydocker = "lazydocker"
ai = "claude"
mcp = "npx @modelcontextprotocol/inspector"
```

Também é possível sobrescrever por variáveis de ambiente:

- `RUST_IDE_LSP_COMMAND`
- `RUST_IDE_LSP_ENABLED`
- `RUST_IDE_LAZYGIT_COMMAND`
- `RUST_IDE_LAZYDOCKER_COMMAND`
- `RUST_IDE_AI_COMMAND`
- `RUST_IDE_MCP_COMMAND`
