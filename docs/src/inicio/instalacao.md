# Instalação

## Pré-requisitos

### Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustc --version  # deve ser >= 1.88
```

### Dependências do sistema (Linux)

```bash
# Ubuntu / Debian
sudo apt install -y \
  build-essential pkg-config libssl-dev libgit2-dev \
  libvulkan-dev libwayland-dev libxkbcommon-dev \
  libx11-dev libxcb1-dev

# Arch Linux
sudo pacman -S base-devel openssl libgit2 vulkan-icd-loader wayland

# Fedora / RHEL
sudo dnf install openssl-devel libgit2-devel vulkan-loader-devel
```

### macOS

```bash
xcode-select --install
brew install libgit2
```

### Ferramentas opcionais (recomendadas)

| Ferramenta | Instalação | Para quê |
|---|---|---|
| [rust-analyzer](https://rust-analyzer.github.io) | `rustup component add rust-analyzer` | LSP para Rust |
| [lazygit](https://github.com/jesseduffield/lazygit) | `brew install lazygit` / pacote do sistema | UI Git TUI |
| [lazydocker](https://github.com/jesseduffield/lazydocker) | `brew install lazydocker` | UI Docker TUI |

## Compilar do código-fonte

```bash
git clone https://github.com/seu-usuario/rust-ide
cd rust-ide

# Debug (desenvolvimento)
cargo build

# Release (uso diário — muito mais rápido)
cargo build --release
```

O binário compilado fica em `target/release/rust-ide`.

## Instalar no sistema

```bash
cargo install --path .
# Isso coloca `rust-ide` no PATH via ~/.cargo/bin/
```

## Verificar instalação

```bash
rust-ide --version  # saída: rust-ide 0.2.0
```

## Resolução de problemas na compilação

### Erro: `libgit2` não encontrado

```bash
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
cargo build --release
```

### Erro: driver Vulkan ausente (Linux)

O Iced usa wgpu com fallback automático para OpenGL/software. Se mesmo assim falhar:

```bash
# Forçar backend de software (tiny-skia)
WGPU_BACKEND=gl cargo run
```
