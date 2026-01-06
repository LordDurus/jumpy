# Build & Runtime Notes (PC / SDL2)

This document captures the steps, decisions, and fixes required to get the **PC (SDL2) build compiling and running** as of this point in the project.

It is intentionally practical and chronological.  
This document **will be updated** once the **GBA build** is wired up.

---

## 1. Asset & File Layout Decisions

### Source vs Runtime Assets
We separate **source assets** from **runtime assets**.

```
src/
  gfx/
    tileset.png        # source-of-truth artwork
    pc/
      tileset.png      # runtime copy for PC
```

- `src/gfx/tileset.png` is edited by hand
- `src/gfx/pc/tileset.png` is copied manually for now
- build/clean automation will handle this later

The PC runtime loads assets **relative to the executable**, not the repo root.

---

## 2. Tile System Decisions

### TileKind (Semantic Tiles)
Levels store **semantic tile kinds**, not sprite indices.

```rust
#[repr(u8)]
pub enum TileKind {
    Empty = 0,
    Dirt = 1,
    SpikeUp = 2,
    Water = 3,
    GrassTop = 4,
    SpikeDown = 5,
    SpikeLeft = 6,
    SpikeRight = 7,
}
```

Levels compile to `.lvlb` files storing `u8` tile kinds.

### Sprite Sheet Mapping
All sprite index mapping lives in **one place**:

```rust
resolve_sheet_tile_id(TileKind, frame, tile_x, tile_y) -> u16
```

---

## 3. Level File Extension

Binary level files use:

```
.lvlb
```

---

## 4. World / Level Access

### Tile Queries
The `Level` API exposes **semantic queries**, not raw bytes.

---

## 5. Movement & Jump Architecture

```rust
pub fn try_jump(world: &mut GameState, entity_id: EntityId) -> bool
```

---

## 6. SDL2 on Windows (MSVC)

### vcpkg Setup

```powershell
git clone https://github.com/microsoft/vcpkg C:\dev\vcpkg
cd C:\dev\vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg.exe install sdl2 sdl2-image --triplet x64-windows
```

---

## 7. Runtime DLLs (PC)

Copy required DLLs from:

```
%VCPKG_ROOT%\installed\x64-windows\bin
```

Into:

```
target\debug
```

---

## 8. Current State (Checkpoint)

- PC build compiles
- SDL2 + SDL2_image linked
- Falling block visible

---

## 9. GBA Build (Pending)
