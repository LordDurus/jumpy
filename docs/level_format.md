# level format and pipeline

this document defines the `.level` text format, the `.lvlb` binary format, and the full build/runtime process.

---

## overview

- `.level` — human-readable source format
- `.lvlb` — compiled binary format for pc and gba
- `levelc` — rust command-line compiler

---

## process flow

```text
WinUI3 Editor
    ↓
.level (text)
    ↓
levelc (rust)
    ↓
.lvlb (binary)
    ↓
PC / GBA runtime
```

---

## .level text format (v1)

### top-level structure

```text
header { ... }
layers { ... }
entities { ... }
triggers { ... }
```

order is mandatory.

---

## header block

```text
header
{
    version = 1
    name = "first jump"
    author = "tom"
    width = 64
    height = 16
    tile_size = 16
    gravity = 0.35
    background = "sky_blue"
}
```

| field | type | notes |
|------|------|------|
| version | int | text format version |
| name | string | editor/debug only |
| author | string | editor/debug only |
| width | int | tiles |
| height | int | tiles |
| tile_size | int | pixels |
| gravity | float | per-level gravity |
| background | string | resolved to id |

---

## layers

each layer defines collision and tiles.

```text
layer "main"
{
    collision = true
    tiles =
    [
        "####",
        "#..#",
        "#..#",
        "####"
    ]
}
```

tile legend (v1):

| char | id | meaning |
|----|----|----|
| . | 0 | empty |
| # | 1 | solid |
| ^ | 2 | spike |
| ~ | 3 | water |

---

## entities

```text
player_start { x = 2 y = 13 }

enemy "slime"
{
    x = 20
    y = 13
    patrol_min = 18
    patrol_max = 24
}

pickup "coin"
{
    x = 24
    y = 12
    value = 1
}
```

entities compile to fixed-size records.

---

## triggers

```text
trigger "message"
{
    x = 5
    y = 11
    width = 1
    height = 1
    text_id = "tutorial_press_jump"
}
```

text is always referenced by id.

---

## gravity

gravity is stored as fixed-point Q7.8.

```text
gravity_fixed = round(gravity * 256)
```

runtime usage is deterministic across pc and gba.

---

## .lvlb binary format (v1)

layout:

```text
[FileHeader]
[LayerRuntime[]]
[EntityRuntime[]]
[TriggerRuntime[]]
[TileId[]]
```

little-endian.

---

## fileheader

```rust
magic = "JLVL"
version = 1
gravity_fixed = i16
offsets = u32
```

offsets allow direct access without parsing.

---

## text localization

levels store only numeric message ids.

localization tables are built separately per language.

---

## guarantees

- deterministic binary layout
- no runtime parsing on gba
- versioned formats
- forward-compatible extensions

