# Primeiro Uso

## Abrir a IDE

```bash
# Abrir no diretório atual
rust-ide

# Abrir em um projeto específico
rust-ide /home/usuario/meu-projeto

# Variáveis de ambiente úteis
RUST_IDE_THEME=gruvbox rust-ide ~/projetos/rust-app
```

## O que acontece na inicialização

1. **Carregar configuração** — lê `~/.config/rust-ide/config.toml` (ou cria padrão)
2. **Detectar workspace** — usa o diretório passado como argumento, ou `$PWD`
3. **Carregar tema** — aplica o tema configurado (padrão: `dark`)
4. **Carregar extensões** — lê todos os `.toml` em `~/.config/rust-ide/extensions/`
5. **Iniciar LSP** — inicia o language server configurado (se habilitado)
6. **Detectar ferramentas** — verifica quais de git/lazygit/lazydocker estão no PATH
7. **Renderizar GUI** — abre a janela com wgpu

## Navegar pela primeira vez

### 1. Selecionar um arquivo

- Use as **setas ↑↓** (ou `j`/`k`) na árvore de arquivos para navegar
- Pressione **Enter** ou clique no arquivo para abri-lo no editor

### 2. Editar

- O editor recebe foco automaticamente ao abrir um arquivo
- Digite normalmente — é um editor de texto completo
- **Ctrl+S** para salvar

### 3. Ver status git

- A sidebar direita exibe branch, arquivos staged/unstaged
- Pressione **G** para abrir o lazygit (se instalado)

### 4. Fechar

- **Ctrl+Q** fecha a IDE (com confirmação se houver alterações não salvas)

## Dica: configuração inicial recomendada

Crie o arquivo de configuração padrão copiando o exemplo:

```bash
mkdir -p ~/.config/rust-ide
cp /caminho/para/rust-ide/examples/config/config.toml ~/.config/rust-ide/config.toml
```

Edite para ativar o LSP:

```toml
# ~/.config/rust-ide/config.toml
theme = "dark"

[lsp]
command = "rust-analyzer"
# args = []
```
