# Contribuindo

## Configurar ambiente de desenvolvimento

```bash
git clone https://github.com/seu-usuario/rust-ide
cd rust-ide
source ~/.cargo/env

# Verificar que tudo compila e testa
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Workflow TDD (obrigatório)

Todo novo código deve seguir o ciclo TDD:

```
1. RED    — escrever teste que falha PRIMEIRO
2. GREEN  — implementar o mínimo para o teste passar
3. REFACTOR — limpar o código mantendo testes passando
4. FINALIZE — clippy, fmt, atualizar skills/changelog
```

### Exemplo

```rust
// 1. RED — escreva o teste primeiro
#[test]
fn editor_marks_dirty_after_set() {
    let mut model = EditorModel::new();
    assert!(!model.dirty);
    model.set_dirty(true);
    assert!(model.dirty);
}

// 2. GREEN — implemente
pub fn set_dirty(&mut self, dirty: bool) {
    self.dirty = dirty;
}
```

## Padrões de código

### Lints ativos

```toml
[lints.clippy]
pedantic     = "warn"
unwrap_used  = "warn"
expect_used  = "warn"
panic        = "warn"
```

- **Nunca** use `unwrap()` ou `expect()` fora de `#[cfg(test)]`
- Toda função pública `Result`-retornante deve ter `/// # Errors`
- Getters puros devem ter `#[must_use]`

### Módulos de domínio

Os módulos em `src/` (exceto `gui/`) **não devem** depender de:
- `iced` (nem seus sub-crates)
- `ratatui`, `crossterm` (removidos)
- Qualquer framework de UI

A separação garante testabilidade e facilita futuras trocas de UI.

### Adicionar um novo módulo

1. Criar `src/novo_modulo/mod.rs`
2. Adicionar `pub mod novo_modulo;` em `src/lib.rs`
3. Escrever testes no módulo
4. Documentar a API pública

## Verificações antes de commitar

```bash
cargo test                                          # todos os testes passando
cargo clippy --all-targets -- -D warnings          # zero erros
cargo fmt                                           # código formatado
```

## Estrutura de um commit

```
feat(editor): adicionar busca de texto

- Implementa EditorModel::find(query: &str) -> Vec<usize>
- Testes: find_empty, find_single_match, find_multiple_matches
- Atualiza skills/rust-ide.md v0.2.1

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>
```
