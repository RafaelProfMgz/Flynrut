---
name: tdd
description: >
  Mandatory TDD workflow for all code changes in this repository.
  Invoke this skill before writing ANY implementation code.
type: rigid
---

# TDD Workflow — rust-ide

Este é um skill **rígido**. Ele se aplica a TODA mudança de código neste repositório,
sem exceção. Nunca implemente código antes de ter testes cobrindo o comportamento esperado.

---

## O Ciclo Obrigatório

```
RED → GREEN → REFACTOR → FINALIZE
```

### Passo 1 — RED (Teste primeiro)

1. Identifique a unidade de comportamento a ser adicionada ou modificada.
2. Escreva o(s) teste(s) unitários dentro do módulo afetado (`#[cfg(test)]`) ou em `tests/`.
3. Execute `cargo test` e confirme que:
   - O teste **falha** (ou nem compila), provando que o comportamento ainda não existe.
4. **NÃO escreva nenhum código de implementação antes desta etapa.**

```rust
// Exemplo: teste antes da função existir
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nova_funcionalidade() {
        // Define o contrato esperado
        assert_eq!(minha_funcao("entrada"), "resultado esperado");
    }
}
```

### Passo 2 — GREEN (Implementação mínima)

1. Escreva a implementação **mínima** necessária para fazer os testes passarem.
2. Execute `cargo test` e confirme que **todos os testes passam** (incluindo os preexistentes).
3. Se algum teste preexistente quebrar, resolva antes de continuar.

### Passo 3 — REFACTOR (Limpeza)

1. Melhore a legibilidade, nomeação, deduplicação de código — **sem mudar comportamento**.
2. Execute `cargo test` novamente para confirmar que tudo ainda passa.
3. Execute `cargo fmt` e `cargo clippy --all-targets -- -D warnings`.

### Passo 4 — FINALIZE

1. Confirme: `cargo test` ✓, `cargo fmt --check` ✓, `cargo clippy` ✓
2. Atualize `.claude/skills/rust-ide.md` se a arquitetura ou API pública mudou.
3. Documente funções públicas novas com `///` doc comments.
4. Só então considere a tarefa concluída.

---

## Localização dos testes

- **Testes unitários**: dentro do arquivo do módulo, em `#[cfg(test)] mod tests { ... }`
- **Testes de integração**: em `tests/<nome>.rs` (use `rust_ide::` para acessar a lib)
- Funções privadas que precisam de testes: use `pub(crate)` ou teste no mesmo arquivo

## Red flags — Você está violando o TDD se:

| Pensamento | Problema |
|---|---|
| "Vou só criar a função e depois escrevo o teste" | Viola RED |
| "O teste é óbvio, posso pular" | Viola RED |
| "Já sei que vai funcionar" | Viola RED |
| "Só uma mudança pequena, não precisa de teste" | Viola RED |
| "Deixa eu terminar e aí rodo os testes" | Viola todos os passos |

## Execução rápida

```bash
# RED: rodar testes (esperar falha)
cargo test nome_do_teste

# GREEN: rodar todos após implementar
cargo test

# REFACTOR: lint + format
cargo fmt && cargo clippy --all-targets -- -D warnings

# FINALIZE: verificação completa
cargo test && cargo fmt --check && cargo clippy --all-targets -- -D warnings
```
