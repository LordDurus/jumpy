# Jumpy

Jumpy is a cross-platform game engine and project framework designed for both embedded devices (like the Game Boy Advance) and modern PCs. The goal of the project is to provide a modular and scalable structure that allows game logic to remain platform-agnostic while supporting platform-specific rendering and input.

## Features

- **Cross-Platform Rendering:**
  - GBA rendering using OAM and VRAM.
  - PC rendering via SDL or similar libraries (future enhancements planned).
- **Entity Component System (ECS):**
  - Lightweight and custom-designed ECS for managing entities and components.
- **Physics Module:**
  - Gravity and other physics-related systems.
- **Vector2 Math:**
  - Custom `Vector2` implementation with optional integration for `nalgebra` (PC-specific).

## Project Structure

The project is organized into modular components for ease of development and scalability:

```plaintext
src/
├── main.rs            # Main entry point
├── game/              # Core game logic
│   ├── movement.rs    # Movement system
│   ├── mod.rs         # Game module entry point
├── physics/           # Physics systems
│   ├── gravity.rs     # Gravity system
│   ├── mod.rs         # Physics module entry point
├── render/            # Platform-specific rendering
│   ├── mod.rs         # Render module entry point
│   ├── gba.rs         # GBA rendering implementation
│   ├── pc.rs          # PC rendering implementation
├── vector2.rs         # Vector2 math implementation
└── world.rs           # ECS world and entity management
```

## Getting Started
Prerequisites

    Rust Toolchain: Install from rustup.rs.
    For PC Builds:
        SDL2 or equivalent libraries for rendering (future planned).
    For GBA Builds:
        Cross-compilation target: thumbv4t-none-eabi.
        A GBA emulator or flash cart for testing (e.g., mGBA, VisualBoy Advance).

## Build Commands

To build for your desired platform, use the following commands:

### For PC Builds:
cargo build --features pc  
cargo run --features pc  

### For GBA Builds:
cargo build --features gba --target thumbv4t-none-eabi

You can test the GBA build using an emulator (e.g., mGBA):
mgba target/thumbv4t-none-eabi/debug/jumpy.gba

## Goals and Future Enhancements
  Expand rendering capabilities for PC, using a robust library like SDL2 or winit + pixels.  
  Add support for collision detection and advanced physics.  
  Explore compatibility with other platforms (e.g., SNES, PSP).

Here's the updated README.md with the build commands included:

# Jumpy

Jumpy is a cross-platform game engine and project framework designed for both embedded devices (like the Game Boy Advance) and modern PCs. The goal of the project is to provide a modular and scalable structure that allows game logic to remain platform-agnostic while supporting platform-specific rendering and input.

## Features

- **Cross-Platform Rendering:**
  - GBA rendering using OAM and VRAM.
  - PC rendering via SDL or similar libraries (future enhancements planned).
- **Entity Component System (ECS):**
  - Lightweight and custom-designed ECS for managing entities and components.
- **Physics Module:**
  - Gravity and other physics-related systems.
- **Vector2 Math:**
  - Custom `Vector2` implementation with optional integration for `nalgebra` (PC-specific).

## Project Structure

The project is organized into modular components for ease of development and scalability:

```plaintext
src/
├── main.rs            # Main entry point
├── game/              # Core game logic
│   ├── movement.rs    # Movement system
│   ├── mod.rs         # Game module entry point
├── physics/           # Physics systems
│   ├── gravity.rs     # Gravity system
│   ├── mod.rs         # Physics module entry point
├── render/            # Platform-specific rendering
│   ├── mod.rs         # Render module entry point
│   ├── gba.rs         # GBA rendering implementation
│   ├── pc.rs          # PC rendering implementation
├── vector2.rs         # Vector2 math implementation
└── world.rs           # ECS world and entity management
```

Getting Started
Prerequisites

    Rust Toolchain: Install from rustup.rs.
    For PC Builds:
        SDL2 or equivalent libraries for rendering (future planned).
    For GBA Builds:
        Cross-compilation target: thumbv4t-none-eabi.
        A GBA emulator or flash cart for testing (e.g., mGBA, VisualBoy Advance).

Build Commands

To build for your desired platform, use the following commands:

For PC Builds:

cargo build --features pc
cargo run --features pc

For GBA Builds:

cargo build --features gba --target thumbv4t-none-eabi

You can test the GBA build using an emulator (e.g., mGBA):

mgba target/thumbv4t-none-eabi/debug/jumpy.gba

### Goals and Future Enhancements
  -- Expand rendering capabilities for PC, using a robust library like SDL2 or winit + pixels.  
  -- Add support for collision detection and advanced physics.  
  -- Explore compatibility with other platforms (e.g., SNES, PSP).

### Contributing
Contributions are welcome!  
Feel free to fork this repository, make changes, and submit a pull request.  
If you encounter issues or have suggestions, please open an issue.  