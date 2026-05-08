# Solução de Problemas

## A IDE não abre / janela em branco

**Causa provável**: driver GPU ou Vulkan não suportado.

```bash
# Verificar suporte Vulkan
vulkaninfo 2>&1 | head -5

# Forçar backend OpenGL (menos rápido, mas funciona em mais hardwares)
WGPU_BACKEND=gl rust-ide

# Forçar backend software (tiny-skia, sem GPU)
WGPU_BACKEND=cpu rust-ide
```

## LSP não conecta

**1. Verificar se o binário está no PATH:**
```bash
which rust-analyzer
rust-analyzer --version
```

**2. Verificar configuração:**
```bash
cat ~/.config/rust-ide/config.toml
# [lsp]
# command = "rust-analyzer"
```

**3. Logs do LSP:**
```bash
# Capturar stderr do LSP (inicie o server manualmente)
rust-analyzer 2>/tmp/ra-stderr.log
cat /tmp/ra-stderr.log
```

**4. Desabilitar temporariamente:**
```bash
RUST_IDE_LSP_ENABLED=false rust-ide
```

## Git não aparece na sidebar

**1. Verificar se é um repositório git:**
```bash
git status
# deve retornar info do repositório
```

**2. Verificar se libgit2 está instalado:**
```bash
pkg-config --modversion libgit2
```

**3. A IDE não detecta git:**
- O workspace precisa ser a raiz do repositório (ou subdiretório)
- Submodules podem não ser detectados na v0.2.0

## lazygit / lazydocker não abrem

**1. Verificar instalação:**
```bash
which lazygit
lazygit --version
```

**2. Verificar configuração:**
```toml
# config.toml
[tools]
lazygit = "lazygit"  # ou caminho absoluto
```

**3. Status na sidebar mostra `off`:**
- Ferramenta não está no PATH
- Configure o caminho completo no `config.toml`

## Extensão não é carregada

**1. Verificar sintaxe TOML:**
```bash
# Validar o arquivo
cat ~/.config/rust-ide/extensions/minha-ext.toml | python3 -c "import sys, toml; toml.load(sys.stdin)"
```

**2. Verificar campo obrigatório:**
```toml
name = "minha-ext"  # obrigatório!
```

**3. Ver erros no terminal:**
```
rust-ide: skipping extension minha-ext.toml: ...
```

## Tema customizado não aplica

**1. Verificar localização:**
```bash
ls ~/.config/rust-ide/themes/
# deve listar: meu-tema.toml
```

**2. Verificar referência no config:**
```toml
# config.toml
theme = "meu-tema"  # nome SEM .toml
```

**3. Verificar formato de cor:**
```toml
# Correto:
status_bg = "#005f87"
status_bg = "#058"    # abreviado

# Incorreto:
status_bg = "rgb(0, 95, 135)"  # não suportado
status_bg = "blue"              # não suportado
```

## Performance lenta

**1. Usar build release (não debug):**
```bash
cargo build --release
./target/release/rust-ide  # muito mais rápido que `cargo run`
```

**2. Verificar backend GPU:**
```bash
RUST_LOG=wgpu_core=info rust-ide 2>&1 | grep "backend"
```

**3. Workspace muito grande:**
- Árvores de arquivos com >10.000 arquivos podem ser lentas
- Adicione exclusões no `config.toml` (roadmap)

## Erro de compilação: `libgit2` não encontrado

```bash
# Ubuntu/Debian
sudo apt install libgit2-dev

# Arch
sudo pacman -S libgit2

# macOS
brew install libgit2

# Usar versão bundled (mais lento para compilar, sem deps do sistema)
cargo add git2 --features vendored
```

## Reportar um bug

Abra uma issue com:
1. Versão: `rust-ide --version`
2. OS e versão: `uname -a`
3. GPU/driver: `vulkaninfo 2>&1 | head -3`
4. Passos para reproduzir
5. Saída de erro: `RUST_LOG=debug rust-ide 2>&1`
