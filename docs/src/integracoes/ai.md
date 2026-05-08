# AI & MCP

## AI (OpenAI-compatível)

A IDE suporta integração com APIs compatíveis com o formato OpenAI Chat Completions.

### Configurar

```toml
# config.toml
[ai]
endpoint = "https://api.openai.com/v1/chat/completions"
model    = "gpt-4o-mini"
enabled  = true
```

A chave de API deve ser passada via variável de ambiente:

```bash
export OPENAI_API_KEY=sk-seu-token
rust-ide
```

### Usar

Pressione **A** para lançar o cliente AI configurado.

### Backends compatíveis

| Serviço | Endpoint |
|---|---|
| OpenAI | `https://api.openai.com/v1/chat/completions` |
| Anthropic (via proxy) | Use um proxy OpenAI-compatível |
| Ollama (local) | `http://localhost:11434/v1/chat/completions` |
| LM Studio | `http://localhost:1234/v1/chat/completions` |
| GitHub Copilot API | `https://api.githubcopilot.com/chat/completions` |

## MCP (Model Context Protocol)

[MCP](https://modelcontextprotocol.io) permite conectar ferramentas de AI que expõem contexto estruturado.

### Configurar

```toml
# config.toml
[tools]
mcp = "/usr/local/bin/meu-servidor-mcp"
```

### Usar

Pressione **M** para lançar o servidor MCP configurado.

> **Status atual**: a integração MCP está preparada na configuração mas a implementação completa do protocolo está no roadmap.
