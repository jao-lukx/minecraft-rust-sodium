# Sodium-inspired Rust

Orçamento adaptativo de upload de chunks e oclusão de entidades.

Implementação independente para [Minecraft Rust Offline / Horizon](https://github.com/jao-lukx/minecraft-rust-offline), Minecraft 26.3. Não é o mod Java original nem é afiliado aos seus autores.

## Instalar na versão offline

Este é um **mod de código-fonte**, não um JAR ou plugin carregado pelo executável pronto. Requer recompilar o cliente. O instalador aplica alterações pequenas no código extraído do ZIP `horizon-offline-profiles-source.zip` da distribuição offline. Não baixa outro Horizon e preserva as alterações dos perfis offline.

1. Baixe e extraia o código-fonte da distribuição offline. Instale Rust 1.99.1 ou mais recente e as dependências Linux descritas no guia da distribuição.
2. Clone este repositório e aplique o mod:

```sh
python3 tools/install.py /caminho/horizon-offline-profiles
```

3. Para combinar os quatro mods, execute o instalador de cada repositório sobre a mesma pasta **antes** de compilar. Eles usam IDs e módulos distintos.
4. Na raiz do código do Horizon:

```sh
cargo build --locked --profile fast -p horizon-client --bin horizon-client
./target/fast/horizon-client --offline --name Builder_1 --resources /caminho/recursos/26.3
```

Use os caminhos dos seus próprios recursos preparados e o seu perfil. Não compartilhe recursos, contas ou mundos. Preserve `HORIZON_HOME` e `HORIZON_LAUNCHER_HOME` que usa hoje. Nenhum mundo é aberto pelo instalador.

O mod aparece em **Opções → Mods** e pode ser desligado ali. Na primeira compilação ele está ligado. Preferências gráficas e de controles originais não são regravadas pelo mod.

## Remover

```sh
python3 tools/install.py /caminho/horizon-offline-profiles --remove
```

Recompile o cliente. A remoção preserva os demais mods. O instalador recusa âncoras incompatíveis e arquivos do mod alterados localmente. Não rode `cargo fmt` no código integrado antes de remover: a remoção exige as âncoras exatas. Guarde uma cópia limpa do código original.

## Desenvolvimento

```sh
cargo test
cargo clippy --all-targets -- -D warnings
python3 tools/package.py
```

`core.rs` é a lógica Rust testável; `runtime.rs` integra com o jogo. `patches.json` descreve cada alteração. Python é usado apenas como ferramenta de desenvolvimento. O pacote `target/*-source.zip` contém fontes e instalador, não um `.hnmod`; não o coloque na pasta de mods do launcher.

Base: Horizon `e4472a7` com perfis offline `34ead8e`. Outras revisões não são garantidas. Veja [VALIDATION.md](VALIDATION.md) para os resultados e limites dos testes.

## O que esta versão otimiza

Implementação inicial inspirada na finalidade do Sodium: suavizar a carga de renderização. Usa os uploads priorizados por proximidade já presentes no Horizon e ajusta seu orçamento a cada 30 amostras, com média móvel do tempo de quadro. Varia entre 2–32 seções e 1–16 MiB por quadro; frames de pausa acima de 250 ms são ignorados. Ativa a oclusão de entidades do motor, que já protege packs, espectadores e terreno recém-alterado. Não muda resolução nem distância de renderização.

**Não é um port do renderizador Sodium e não substitui o renderizador wgpu.** Não implementa todo o conjunto de otimizações do original. Reduzir uploads pode atrasar o aparecimento de chunks; a oclusão tem custo de CPU. Compare ligado/desligado no seu hardware. Nenhum ganho de FPS foi medido nesta entrega. As preferências originais são usadas imediatamente ao desligar o mod.

## Outros mods desta coleção

- [minecraft-rust-jei](https://github.com/jao-lukx/minecraft-rust-jei)
- [minecraft-rust-mouse-tweaks](https://github.com/jao-lukx/minecraft-rust-mouse-tweaks)
- [minecraft-rust-zoom](https://github.com/jao-lukx/minecraft-rust-zoom)
