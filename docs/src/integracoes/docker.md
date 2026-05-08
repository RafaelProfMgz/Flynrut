# lazydocker & Docker

## lazydocker

[lazydocker](https://github.com/jesseduffield/lazydocker) é uma TUI para gerenciar containers Docker.

### Instalar

```bash
# macOS
brew install lazydocker

# Linux (script)
curl https://raw.githubusercontent.com/jesseduffield/lazydocker/master/scripts/install_update_linux.sh | bash

# Via Go
go install github.com/jesseduffield/lazydocker@latest
```

### Uso

Pressione **D** para abrir o lazydocker. Ele abre em background e gerencia containers enquanto você continua codificando.

### Configurar

```toml
# config.toml
[tools]
lazydocker = "/usr/local/bin/lazydocker"
docker     = "docker"
```

## Docker CLI

O `docker` CLI é detectado automaticamente. Ferramentas de extensão podem usá-lo:

```toml
# extensao.toml
[[tools]]
name    = "docker-ps"
command = "docker ps"
description = "Listar containers"

[[tools]]
name    = "docker-compose-up"
command = "docker-compose up -d"
description = "Subir serviços"
```
