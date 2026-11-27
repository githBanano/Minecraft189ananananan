# Minecraft 1.8.9 — Rust Port

Projeto de reimplementação/portabilidade do **Minecraft: Java Edition 1.8.9** para Rust e Windows.

> **Estado real:** ainda não é um port jogável. O protótipo anterior de terreno genérico não representava Minecraft e foi retirado. Este repositório passa a ser dedicado à compatibilidade com a versão 1.8.9, não a criar um jogo voxel inspirado nela.

## Objetivo

Construir um cliente Rust que reproduza o Minecraft 1.8.9, incluindo:
- menus, opções e HUD da versão;
- carregamento dos recursos da instalação legítima do jogador;
- mundo em chunks, blocos, iluminação e renderização;
- controles, câmera, colisão, física e interação com blocos;
- inventário, hotbar, crafting e entidades;
- mundos singleplayer e salvamento;
- cliente multiplayer compatível com o protocolo 47 (1.8.x).

## Abordagem técnica

O código original completo do Minecraft Java não é fornecido neste repositório. Portanto, isto não é uma conversão automática de arquivos Java para Rust. A implementação será uma reimplementação independente baseada no comportamento observável e nas especificações públicas, sem incluir código-fonte descompilado ou redistribuir assets proprietários.

Os recursos gráficos/sons devem ser obtidos da instalação legítima do próprio jogador, respeitando os termos aplicáveis. O objetivo é evitar que o jogador tenha de selecionar manualmente um arquivo `data.win` ou fornecer recursos a cada execução: o launcher/cliente deverá localizar uma instalação configurada e validar os recursos uma vez.

## Estrutura planejada

- `src/main.rs`: inicialização do cliente.
- `src/game/`: estado, tick e ciclo de vida.
- `src/render/`: renderer, câmera, atlas e chunks.
- `src/world/`: blocos, chunks, geração e salvamento.
- `src/player/`: input, movimento, colisão e inventário.
- `src/network/`: protocolo 47 e conexão multiplayer.
- `src/resources/`: descoberta e leitura dos recursos locais.

## Compilar (Windows)

Instale Rust stable e execute:

```powershell
cargo check
cargo run --release
```

O workflow do GitHub Actions valida a compilação para Windows.

## Marcos

1. Base de cliente Rust e descoberta de recursos 1.8.9.
2. Renderer de blocos/chunks e câmera em primeira pessoa.
3. Movimento, colisão, quebrar/colocar blocos e HUD.
4. Mundos locais, geração e salvamento.
5. Inventário, crafting, entidades e demais mecânicas.
6. Protocolo 47, multiplayer e compatibilidade de servidor.
7. Testes de paridade visual e funcional com Minecraft 1.8.9.

Cada marco só será marcado como concluído depois de implementado e testado. Não será anunciado como Minecraft completo enquanto não for.
