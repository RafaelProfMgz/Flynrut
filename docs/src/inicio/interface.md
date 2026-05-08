# Interface Geral

A interface é dividida em quatro regiões principais:

```
┌──────────────┬──────────────────────────────┬──────────────────┐
│  [1] Árvore  │        [2] Editor            │   [3] Sidebar    │
│  de Arquivos │                              │                  │
│              │                              │                  │
└──────────────┴──────────────────────────────┴──────────────────┘
│                    [4] Barra de Status                         │
└────────────────────────────────────────────────────────────────┘
```

## [1] Árvore de Arquivos (File Tree)

- Exibe todos os arquivos e diretórios do workspace
- Ícone 📁 para diretórios, 📄 para arquivos
- Indentação visual para mostrar hierarquia
- Item selecionado destacado com a cor `sidebar_selected` do tema
- Clique ou **Enter** abre o arquivo no editor

**Atalhos neste painel:**

| Tecla | Ação |
|---|---|
| ↑ / `k` | Mover seleção para cima |
| ↓ / `j` | Mover seleção para baixo |
| Enter / → / `o` | Abrir arquivo selecionado |
| Tab | Mover foco para o Editor |

## [2] Editor

- Editor de texto completo baseado em `iced::widget::text_editor`
- Suporte a Unicode, múltiplas linhas, scroll
- Números de linha na margem esquerda
- Tab bar com o nome do arquivo aberto e indicador `*` se há alterações não salvas
- Syntax highlight via `syntect` (suporte a >300 linguagens)

**Atalhos neste painel:**

| Tecla | Ação |
|---|---|
| Ctrl+S | Salvar arquivo |
| Ctrl+R | Atualizar workspace |
| Tab (sem texto selecionado, fora da edição) | Mover foco para Sidebar |
| Ctrl+Z | Desfazer |
| Ctrl+Y | Refazer |
| Ctrl+A | Selecionar tudo |

## [3] Sidebar

Painel informativo à direita, dividido em três seções:

### Git Status
- Nome do repositório
- Branch atual com indicador de commits à frente/atrás do remoto
- Contagem de arquivos: staged (✚), unstaged (●), não rastreados (?)

### Ferramentas
| Linha | Informação |
|---|---|
| `lazygit  ok` | Ferramenta disponível no PATH |
| `lazydocker off` | Ferramenta não encontrada |
| `lsp       ok` | LSP rodando |

### Diagnósticos LSP
- Últimas 8 mensagens do language server para o arquivo aberto
- Cores diferenciadas: vermelho = erro, amarelo = aviso, azul = informação

## [4] Barra de Status

- **Esquerda**: mensagem de status atual (operação mais recente, erros, etc.)
- **Direita**: botões de ação rápida — **Salvar** e **Atualizar**

## Painéis Redimensionáveis

Os três painéis principais são redimensionáveis. **Arraste as bordas** entre eles para ajustar o tamanho. O estado não é salvo entre sessões (roadmap: salvar layout no config).

## Tema Visual

A paleta de cores inteira é controlada pelo tema ativo. Veja [Temas](../temas/embutidos.md).
