#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FileHeader {
	pub magic: [u8; 4],   // "JLVL"
	pub version: u16,     // 1
	pub header_size: u16, // sizeof(FileHeader)

	pub width: u16,
	pub height: u16,
	pub tile_width: u16,
	pub tile_height: u16,
	pub layer_count: u8,

	pub entity_count: u16,
	pub trigger_count: u16,

	pub gravity_fixed: i16, // Q7.8
	pub background_id: u8,
	pub gravity: u8,

	pub extra0: u8,
	pub extra1: u8,

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
	pub gravity_multiplier: u8,
	pub reserved1: u8,
	pub reserved2: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EntityRuntime {
	pub kind: u8,
	pub render_style: u8,
	pub gravity_multiplier: u8,
	pub jump_multiplier: u8,
	pub attack_power: u8,
	pub hit_points: u16,
	pub top: u16,
	pub left: u16,
	pub health_regen_rate: i16,
	pub invulnerability_time: i16,
	pub width: u8,
	pub height: u8,
	pub speed: u8,
	pub strength: u8,
	pub luck: u8,
	pub range_min: u16,
	pub range_max: u16,
}

impl EntityRuntime {
	// update to the actual bytes you write
	// pub const BYTE_SIZE: u32 = 68;
	// pub const BYTE_SIZE: u32 = 14;
	// pub const BYTE_SIZE: u32 = 17;
	pub const BYTE_SIZE: u32 = 24;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TriggerRuntime {
	pub kind: u8,
	pub gravity_multiplier: u8,
	pub left: u16,
	pub top: u16,
	pub width: u16,
	pub height: u16,
	pub p0: u16,
	pub p1: u16,
}

impl TriggerRuntime {
	pub const BYTE_SIZE: u32 = 14;
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
#[allow(dead_code)]
pub enum EntityKind {
	Player = 1,
	Slime = 2,
	Imp = 3,
	MovingPlatform = 4,
}

#[repr(u8)]
#[allow(dead_code)]
pub enum TriggerKind {
	LevelExit = 0,
	Message = 1,
	Pickup = 2,
}
