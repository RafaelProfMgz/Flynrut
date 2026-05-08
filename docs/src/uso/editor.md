# Editor de Código

## Recursos

- **Edição completa**: inserção, exclusão, seleção, clipboard (via Ctrl+C/V/X)
- **Múltiplas linhas**: scroll vertical e horizontal
- **Números de linha**: exibidos na margem esquerda
- **Indicador de modificação**: título do painel mostra `*` quando há alterações não salvas
- **Syntax highlight**: via `syntect`, suporte a >300 linguagens com temas TextMate
- **Desfazer/Refazer**: histórico completo de edições (Ctrl+Z / Ctrl+Y)

## Abrindo Arquivos

1. Navegue na árvore de arquivos com ↑↓
2. Pressione **Enter** ou clique no arquivo
3. O editor abre o arquivo e o LSP é notificado

Ou via programático (extensão/tool):
```bash
# Uma extensão pode invocar:
command = "open_file $FILE"
```

## Salvando

- **Ctrl+S** — salva o arquivo atual no disco
- O indicador `*` desaparece após salvar
- O LSP é notificado do novo conteúdo automaticamente

## Syntax Highlight

O highlight é determinado pela extensão do arquivo:

| Extensão | Linguagem detectada |
|---|---|
| `.rs` | Rust |
| `.toml` | TOML |
| `.md` | Markdown |
| `.json` | JSON |
| `.yaml`, `.yml` | YAML |
| `.sh`, `.bash` | Shell |
| `.py` | Python |
| `.js`, `.ts` | JavaScript / TypeScript |

> Extensões de tipo de arquivo podem ser adicionadas ou sobrescritas via [extensões](../extensoes/manifesto.md#filetypes).

## Integração com LSP

Quando um arquivo é aberto:
1. O editor sincroniza o conteúdo com o LSP (`textDocument/didOpen`)
2. O LSP analisa o código em background
3. Diagnósticos (erros, avisos) aparecem na Sidebar em tempo real

## Limitações atuais

- Sem busca/substituição em arquivo (roadmap)
- Sem múltiplos cursores (roadmap)
- Sem autocompletar inline (roadmap — requer widget customizado)
- Sem folding (roadmap)
