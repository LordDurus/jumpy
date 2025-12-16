// runtime.rs

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FileHeader {
    pub magic: [u8; 4],   // "JLVL"
    pub version: u16,     // 1
    pub header_size: u16, // sizeof(FileHeader)

    pub width: u16,
    pub height: u16,
    pub tile_size: u8,
    pub layer_count: u8,

    pub entity_count: u16,
    pub trigger_count: u16,

    pub gravity_fixed: i16, // Q7.8
    pub background_id: u8,
    pub reserved0: u8,

    pub tiles_per_layer: u32,
    pub tile_count_total: u32,

    pub offset_layers: u32,
    pub offset_entities: u32,
    pub offset_triggers: u32,
    pub offset_tiles: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LayerRuntime {
    pub collision: u8,
    pub reserved0: u8,
    pub reserved1: u8,
    pub reserved2: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EntityRuntime {
    pub kind: u8,
    pub reserved0: u8,
    pub x: u16,
    pub y: u16,
    pub a: i16,
    pub b: i16,
    pub extra_id: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TriggerRuntime {
    pub kind: u8,
    pub reserved0: u8,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub p0: u16,
    pub p1: u16,
}

#[derive(Debug)]
pub struct CompiledLevel {
    pub header: FileHeader,
    pub layers: Vec<LayerRuntime>,
    pub entities: Vec<EntityRuntime>,
    pub triggers: Vec<TriggerRuntime>,
    pub tiles: Vec<u8>, // TileId = u8
}

#[repr(u8)]
pub enum EntityKind {
    PlayerStart = 0,
    Enemy = 1,
    Pickup = 2,
}

#[repr(u8)]
pub enum TriggerKind {
    LevelExit = 0,
    Message = 1,
}
