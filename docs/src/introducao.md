# rust-ide

**rust-ide** é uma IDE leve, modular e extensível construída em Rust, com interface gráfica GPU-acelerada (via [Iced](https://iced.rs) + wgpu), integrações nativas com git, lazygit, lazydocker, LSP e AI.

## Destaques

| Característica | Descrição |
|---|---|
| 🚀 **GPU-acelerado** | Renderização via wgpu (mesmo backend do Zed) |
| 🧩 **Modular** | Cada funcionalidade é um módulo independente |
| 🎨 **Temas** | 5 temas embutidos + suporte a temas TOML customizados |
| 🔌 **Extensões** | Atalhos, ferramentas e mapeamento de tipos via TOML |
| 🔧 **LSP nativo** | rust-analyzer e qualquer language server |
| 🐙 **Git integrado** | Status em tempo real, lazygit em um clique |
| 🐋 **Docker** | lazydocker integrado |
| 🤖 **AI** | Suporte a endpoints OpenAI-compatíveis |

## Layout da Interface

```
┌──────────────┬──────────────────────────────┬──────────────────┐
│  Árvore de   │         Editor               │    Sidebar       │
│  Arquivos    │                              │                  │
│  (22%)       │  Tab: src/main.rs            │  Git: main ●2    │
│              │  ─────────────────────────   │  ─────────────   │
│  📁 src/     │   1 │ use rust_ide::...      │  lazygit   ok    │
│  📄 main.rs  │   2 │                        │  lsp       ok    │
│  📁 gui/     │   3 │ fn main() -> Result    │  ─────────────   │
│  📁 config/  │   4 │     let workspace =    │  L5 [erro]       │
│  📄 lib.rs   │   5 │     let config = ...   │  mismatched type │
│              │   6 │ }                      │                  │
└──────────────┴──────────────────────────────┴──────────────────┘
│  status: IDE pronta. LSP conectado.    [Salvar]  [Atualizar]   │
└────────────────────────────────────────────────────────────────┘
```

## Início Rápido

```bash
# Clonar e compilar
git clone <url-do-repo>
cd rust-ide
cargo build --release

# Executar no diretório de trabalho
./target/release/rust-ide
# ou
cargo run -- /caminho/para/seu/projeto
```

## Requisitos do Sistema

| Componente | Versão mínima |
|---|---|
| Rust | 1.88+ |
| Sistema operacional | Linux, macOS, Windows |
| GPU / driver | Vulkan, Metal ou DirectX 12 (wgpu) |
| GPU (fallback) | OpenGL via tiny-skia (software) |
