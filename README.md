# Minecraft 1.8.9 — Rust Reimplementation

An independent, from-scratch Rust project aiming to reproduce the gameplay and client behavior of **Minecraft Java Edition 1.8.9** on Windows without requiring a JVM at runtime.

> **Status: early prototype.** This repository is not yet a complete Minecraft client. The current milestone is a native window, a first-person camera, and a small procedural voxel terrain prototype. Gameplay parity, save compatibility, assets, and multiplayer are future milestones.

## Principles
- Rust-native runtime; no Java VM required by the finished client.
- Reimplement systems independently; do not copy Mojang's proprietary source code.
- Use resources from the user's own legally obtained Minecraft installation where permitted; do not redistribute Mojang assets or the original client JAR.
- Treat Minecraft 1.8.9 behavior and protocol 47 as compatibility targets.

## Build on Windows
Install the stable Rust toolchain with [rustup](https://rustup.rs/), then run:

```powershell
cargo run
```

Release build:

```powershell
cargo build --release
```

Executable: `target/release/minecraft189-rust.exe`.

## Controls (prototype)
- WASD: move
- Mouse: look
- Space: move up
- Left Shift: move down
- Escape: release mouse cursor

Click the game window to capture the mouse.

## Roadmap
1. Native window, input, camera, and voxel prototype.
2. Chunk data model, meshing, frustum culling, and lighting.
3. Minecraft 1.8.9 terrain generation and block registry.
4. Player physics, collision, interaction, inventory, and UI.
5. Entities, mobs, crafting, redstone, audio, and world persistence.
6. Protocol 47 multiplayer and authentication integration.
7. Compatibility testing against legal 1.8.9 reference behavior.

## Legal
Minecraft is a trademark of Mojang Studios. This is an independent, unofficial compatibility project and is not endorsed by or affiliated with Mojang or Microsoft. Minecraft code, trademarks, and assets are not included. Users are responsible for obtaining and using game resources lawfully.
