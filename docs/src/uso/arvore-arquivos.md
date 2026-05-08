# Árvore de Arquivos

A árvore de arquivos exibe todos os arquivos e diretórios do workspace em ordem hierárquica.

## Navegação

```
📁 src/          ← diretório
  📁 gui/
    📄 mod.rs    ← arquivo (indentação = profundidade)
    📄 style.rs
  📄 lib.rs
📄 Cargo.toml
📄 README.md
```

### Com teclado
- **↑** / **k** — item anterior
- **↓** / **j** — próximo item
- **Enter** — abrir arquivo
- **→** / **o** — expandir diretório ou abrir arquivo

### Com mouse
- **Clique** em qualquer item — abre o arquivo no editor

## Atualização Automática

A árvore é atualizada a cada **250ms** via tick interno. Se você criar, renomear ou excluir arquivos externamente, eles aparecem automaticamente.

Para forçar atualização imediata: **Ctrl+R**.

## Arquivos Ignorados

Por padrão, `walkdir` lista todos os arquivos. Arquivos em `target/`, `.git/`, `node_modules/` são filtrados automaticamente para manter a árvore limpa.

## Itens Especiais

| Ícone | Tipo |
|---|---|
| 📁 | Diretório |
| 📄 | Arquivo comum |
| Cor `sidebar_dir` | Diretórios |
| Cor `sidebar_file` | Arquivos comuns |
| Fundo `sidebar_selected` | Item selecionado |
