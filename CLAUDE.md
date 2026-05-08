# CLAUDE.md — rust-ide

Guia de desenvolvimento para agentes de IA (Claude, Copilot, etc.) trabalhando neste repositório.

---

## Visão geral

`rust-ide` é uma IDE terminal modular escrita em Rust.  
TUI com `ratatui` + `tui-textarea`, LSP nativo, integração com `lazygit`, `lazydocker`, AI e MCP.

## Convenções de código

- **Linguagem**: Rust, edição 2024
- **Idioma de mensagens e logs**: Português (BR)
- **Idioma de identificadores**: Inglês (funções, tipos, variáveis)
- **Idioma de comentários**: Inglês ou Português (BR), consistente por arquivo
- **Erros**: sempre usar `anyhow::Result` nas fronteiras públicas; erros internos com `?`
- **Panic proibido**: `clippy::unwrap_used` e `clippy::expect_used` são warnings; prefira `?` ou `ok_or_else`
- **Formatação**: `cargo fmt` antes de qualquer commit
- **Linting**: `cargo clippy --all-targets -- -D warnings` deve passar

## Arquitetura de módulos

```
src/
  lib.rs           ← re-exporta módulos públicos; usado nos testes de integração
  main.rs          ← entrypoint; loop de eventos e terminal TUI
  app/mod.rs       ← estado global da aplicação, roteamento de teclas, ações
  config/mod.rs    ← carregamento de config (TOML + envvars)
  editor/mod.rs    ← EditorModel: abertura, edição, salvamento de arquivos
  fs_tree/mod.rs   ← FileTree: árvore de arquivos com navegação e toggle
  integrations/mod.rs  ← git status nativo (git2), detecção de ferramentas
  lsp/mod.rs       ← cliente LSP enxuto (didOpen/didChange/didSave, diagnósticos)
  ui/mod.rs        ← layout e renderização dos painéis ratatui
```

## Fluxo TDD obrigatório

**Todo código novo ou modificado deve seguir o ciclo TDD:**

1. **RED** — Escreva o(s) teste(s) que definem o comportamento esperado ANTES de qualquer implementação. Rode `cargo test` e confirme que o(s) teste(s) falha(m) ou não compilam.
2. **GREEN** — Escreva a implementação mínima para fazer os testes passarem. Rode `cargo test` novamente e confirme 100% de aprovação.
3. **REFACTOR** — Limpe o código sem quebrar os testes. Rode `cargo test` uma última vez.
4. **FINALIZE** — Atualize os arquivos de skill em `.claude/skills/` para refletir mudanças relevantes.

Nunca finalize uma tarefa com testes falhando.

## Comandos úteis

```bash
# Compilar
cargo build

# Rodar a IDE (abrir no diretório atual)
cargo run

# Executar todos os testes
cargo test

# Testes de um módulo específico
cargo test fs_tree
cargo test config

# Lint
cargo clippy --all-targets -- -D warnings

# Formatar
cargo fmt --check        # verificar
cargo fmt                # aplicar
```

## Configuração

Arquivo: `~/.config/rust-ide/config.toml`

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

Variáveis de ambiente sobrescrevem o arquivo:
- `RUST_IDE_LSP_COMMAND`, `RUST_IDE_LSP_ENABLED`
- `RUST_IDE_LAZYGIT_COMMAND`, `RUST_IDE_LAZYDOCKER_COMMAND`
- `RUST_IDE_AI_COMMAND`, `RUST_IDE_MCP_COMMAND`

## Skills do Claude

Skills específicas de workflow ficam em `.claude/skills/`.  
Ao finalizar qualquer mudança significativa, **atualize o skill relevante**.

- `.claude/skills/tdd.md` — fluxo TDD detalhado
- `.claude/skills/rust-ide.md` — detalhes arquiteturais do projeto
