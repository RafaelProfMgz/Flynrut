# Atalhos de Teclado

## Atalhos Globais

Funcionam independentemente do painel com foco:

| Atalho | Ação |
|---|---|
| **Tab** | Mover foco: Árvore → Editor → Sidebar → Árvore |
| **Shift+Tab** | Mover foco na direção oposta |
| **Ctrl+S** | Salvar arquivo atual |
| **Ctrl+R** | Atualizar workspace (árvore + git + LSP) |
| **G** | Abrir lazygit |
| **D** | Abrir lazydocker |
| **A** | Lançar cliente AI |
| **M** | Lançar cliente MCP |
| **L** | Reiniciar / reconectar LSP |
| **Ctrl+Q** | Sair da IDE |

## Painel: Árvore de Arquivos

| Atalho | Ação |
|---|---|
| ↑ ou `k` | Selecionar item anterior |
| ↓ ou `j` | Selecionar próximo item |
| Enter / → / `o` | Abrir arquivo / expandir diretório |
| ← / `h` | Subir nível |
| **Tab** | Passar foco para o Editor |

## Painel: Editor

O editor segue os atalhos padrão do `iced::widget::text_editor`:

| Atalho | Ação |
|---|---|
| Qualquer tecla | Inserir caractere |
| Backspace | Apagar caractere anterior |
| Delete | Apagar caractere seguinte |
| Enter | Nova linha |
| Tab | Inserir indentação (4 espaços) |
| Ctrl+A | Selecionar tudo |
| Ctrl+C | Copiar seleção |
| Ctrl+X | Recortar seleção |
| Ctrl+V | Colar |
| Ctrl+Z | Desfazer |
| Ctrl+Y | Refazer |
| Home | Início da linha |
| End | Fim da linha |
| Ctrl+Home | Início do arquivo |
| Ctrl+End | Fim do arquivo |
| Shift+setas | Selecionar texto |
| Ctrl+Shift+seta | Selecionar palavra |
| Ctrl+S | Salvar |

## Painel: Sidebar

| Atalho | Ação |
|---|---|
| ↑ ou `k` | Scroll para cima |
| ↓ ou `j` | Scroll para baixo |
| **Tab** | Passar foco para Árvore |

## Personalizar Atalhos via Extensões

Você pode adicionar atalhos customizados criando uma extensão TOML. Veja [Criar uma Extensão](../extensoes/criar.md).

```toml
# ~/.config/rust-ide/extensions/meus-atalhos.toml
name    = "meus-atalhos"
version = "1.0.0"

[[keybindings]]
key         = "Ctrl-P"
command     = "open_file_picker"
description = "Abrir seletor de arquivo"

[[keybindings]]
key         = "F5"
command     = "run_tests"
description = "Executar testes"
```
