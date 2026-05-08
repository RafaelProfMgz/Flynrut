# Temas Embutidos

A IDE vem com 5 temas embutidos. Para ativá-los:

```toml
# config.toml
theme = "dark"      # ou: gruvbox, nord, dracula, light
```

Ou via variável de ambiente:

```bash
RUST_IDE_THEME=gruvbox rust-ide
```

---

## dark (padrão)

Tema escuro minimalista, inspirado em editores modernos como Zed.

| Área | Cor |
|---|---|
| Background geral | `#1a1a2e` (azul-preto) |
| Editor | `#12121e` |
| File tree | `#151520` |
| Sidebar | `#0f0f1a` |
| Borda ativa | `#7aa2f7` (azul) |
| Cursor | `#e0af68` |
| Erro LSP | `#f7768e` (vermelho) |
| Aviso LSP | `#e0af68` (laranja) |

---

## gruvbox

Tema quente com tons terrosos, baseado no popular Gruvbox.

| Área | Cor |
|---|---|
| Background | `#282828` |
| Editor | `#1d2021` |
| Borda ativa | `#fabd2f` (amarelo) |
| Texto | `#ebdbb2` |
| Erro | `#fb4934` |

---

## nord

Tema frio baseado na paleta Nord (paleta ártica).

| Área | Cor |
|---|---|
| Background | `#2e3440` |
| Editor | `#242933` |
| Borda ativa | `#88c0d0` (ciano) |
| Texto | `#d8dee9` |
| Erro | `#bf616a` |

---

## dracula

Tema roxo/rosa, baseado no popular Dracula.

| Área | Cor |
|---|---|
| Background | `#282a36` |
| Editor | `#1e1f29` |
| Borda ativa | `#bd93f9` (roxo) |
| Texto | `#f8f8f2` |
| Erro | `#ff5555` |
| Seleção | `#44475a` |

---

## light

Tema claro para uso em ambientes com bastante luz.

| Área | Cor |
|---|---|
| Background | `#f5f5f5` |
| Editor | `#ffffff` |
| Borda ativa | `#005fd4` (azul) |
| Texto | `#1a1a1a` |
| Erro | `#cc0000` |
