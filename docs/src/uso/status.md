# Barra de Status

A barra de status fica na parte inferior da janela e exibe informações contextuais.

## Estrutura

```
│  status: IDE pronta. LSP conectado.           [Salvar]  [Atualizar]  │
```

- **Esquerda**: mensagem de status
- **Direita**: botões de ação rápida

## Mensagens comuns

| Mensagem | Significado |
|---|---|
| `IDE pronta. LSP conectado.` | Inicialização bem-sucedida |
| `IDE pronta. LSP desabilitado por configuração.` | LSP está desabilitado no config.toml |
| `IDE pronta. LSP indisponível: ...` | Binário do LSP não encontrado |
| `Arquivo aberto: /caminho/arquivo.rs` | Arquivo foi aberto no editor |
| `Arquivo salvo: /caminho/arquivo.rs` | Arquivo foi salvo com sucesso |
| `Workspace atualizada: /caminho/` | Ctrl+R executado com sucesso |
| `Erro ao salvar: ...` | Falha ao salvar (permissão, disco cheio, etc.) |
| `Executando: lazygit` | Ferramenta externa sendo iniciada |
| `LSP reconectado.` | LSP reiniciado com sucesso |

## Botões

| Botão | Equivalente | Ação |
|---|---|---|
| **Salvar** | Ctrl+S | Salva o arquivo aberto |
| **Atualizar** | Ctrl+R | Atualiza árvore, git e LSP |
