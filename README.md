# Jumpy

Jumpy is a cross-platform game engine and game project targeting both constrained hardware
(Game Boy Advance, PSP) and modern PCs.

The core design goal is **platform-agnostic game logic** with **platform-specific backends**
for rendering, audio, input, and threading. The same game code runs on PC and embedded targets,
with differences handled behind feature-gated implementations.

PC is the primary development platform. Embedded targets are treated as first-class,
but with stricter memory and API constraints.

## supported platforms

- **PC (Windows / Linux / macOS)**  
  - SDL2-based rendering, input, and audio
  - wav + ogg audio
  - threaded or single-threaded game loop
- **Game Boy Advance**
  - OAM / VRAM rendering
  - no standard library
  - strict memory limits
- **PlayStation Portable (PSP)**
  - `rust-psp` backend
  - feature-gated platform support

## features

- **Platform-agnostic game logic**
- **Feature-gated platform backends**
  - `pc`
  - `gba`
  - `psp`
- **Custom ECS**
  - Lightweight and data-oriented
- **Physics systems**
  - Gravity and collision handling
- **Deterministic math**
  - Custom vector math
  - Optional PC-only integrations
- **Audio system**
  - Unified trait
  - Platform-specific implementations
- **Asset pipeline**
  - Shared logical assets
  - Platform-specific formats where required

## project structure

High-level layout (simplified):

```text
src/
├── main.rs
├── game/              # game logic (platform-agnostic)
│   ├── level/
│   ├── triggers/
│   ├── ecs/
│   └── render/        # renderer abstractions
├── physics/
├── platform/          # platform backends
│   ├── render/
│   │   ├── pc.rs
│   │   ├── gba.rs
│   │   └── psp.rs
│   ├── audio/
│   └── input/
├── assets/
└── common/
