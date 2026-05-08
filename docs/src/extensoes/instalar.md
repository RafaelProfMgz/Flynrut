# Instalar Extensões

## Instalação manual

Copie o arquivo `.toml` da extensão para o diretório de extensões:

```bash
# Criar o diretório (se não existir)
mkdir -p ~/.config/rust-ide/extensions

# Copiar a extensão
cp minha-extensao.toml ~/.config/rust-ide/extensions/

# Reiniciar a IDE para carregar
```

## Verificar carregamento

Extensões carregadas com sucesso não exibem mensagem. Extensões com TOML inválido exibem um aviso no terminal:

```
rust-ide: skipping extension minha-extensao.toml: expected table, found string at line 3
```

## Desabilitar uma extensão

Renomeie o arquivo para não ter a extensão `.toml`, ou mova-o para fora do diretório:

```bash
mv ~/.config/rust-ide/extensions/minha-extensao.toml \
   ~/.config/rust-ide/extensions/minha-extensao.toml.disabled
```

## Ordem de carregamento

Extensões são carregadas em **ordem alfabética** por nome de arquivo. Para controlar a ordem, prefixe com números:

```
~/.config/rust-ide/extensions/
  01-base.toml
  02-git-extras.toml
  03-python.toml
```

> Em caso de conflito de filetype (mesma extensão de arquivo em duas extensões), a última carregada vence.
