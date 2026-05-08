# Git & lazygit

## Git nativo

A IDE detecta repositórios git automaticamente via `libgit2`. Nenhuma configuração necessária — funciona no diretório do workspace.

### Informações exibidas na Sidebar

```
Git: main ●2 ✚1 ?3
─────────────────────────────
repo   /home/usuario/projeto
branch main
stage  1    ← arquivos staged
unstg  2    ← arquivos unstaged
untrk  3    ← arquivos não rastreados
sync   +0 / -0
```

| Indicador | Significado |
|---|---|
| `stage N` | N arquivos no staging area |
| `unstg N` | N arquivos modificados não staged |
| `untrk N` | N arquivos não rastreados |
| `sync +A / -B` | A commits à frente, B atrás do remoto |

### Atualização

O status git é atualizado a cada **250ms** automaticamente. Para forçar: **Ctrl+R**.

## lazygit

[lazygit](https://github.com/jesseduffield/lazygit) é uma TUI Git poderosa que pode ser aberta diretamente da IDE.

### Pré-requisito

```bash
# Instalar lazygit
# macOS
brew install lazygit

# Ubuntu/Debian (snap)
sudo snap install lazygit

# Arch
sudo pacman -S lazygit

# Via cargo (mais lento)
cargo install lazygit
```

### Uso

Pressione **G** em qualquer painel para abrir o lazygit no workspace atual.

O lazygit abre em background como processo separado. Quando fechado, a IDE atualiza o status git automaticamente.

### Configurar um binário alternativo

```toml
# config.toml
[tools]
lazygit = "/usr/local/bin/lazygit"
# ou
lazygit = "~/.local/bin/lazygit"
```

### Status na Sidebar

```
lazygit   ok     ← binário encontrado no PATH
lazygit   off    ← não encontrado
```
