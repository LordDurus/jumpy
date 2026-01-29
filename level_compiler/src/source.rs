#[derive(Debug)]
pub struct LevelSource {
	pub header: LevelHeader,
	pub layers: Vec<LayerSource>,
	pub entities: Vec<EntitySource>,
	pub triggers: Vec<TriggerSource>,
}

#[derive(Debug)]
pub struct LevelHeader {
	pub version: u32,
	pub name: String,
	pub author: String,
	pub width: u32,
	pub height: u32,
	pub tile_width: u32,
	pub tile_height: u32,
	pub gravity: f32,
	pub background: String,
	pub music: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LayerSource {
	pub name: String,
	pub collision: bool,
	pub rows: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EntitySource {
	pub top: i32,
	pub left: i32,
	pub render_style: u8,
	pub jump_multiplier: f32,
	pub attack_power: i32,
	pub hit_points: i32,
	pub gravity_multiplier: f32,
	pub kind: EntityKindSource,
	pub width: f32, // if width is in tiles (like 0.25)
	pub height: f32,
	pub speed: u8,
	pub strength: u8,
	pub luck: u8,
	pub range_min: i32,
	pub range_max: i32,
	pub health_regen_rate: i16,
	pub invulnerability_time: i16,
}

#[derive(Debug)]
pub enum EntityKindSource {
	PlayerStart,
	Enemy {
		enemy_kind: String,
		range_min: i32,
		range_max: i32,
	},
	MovingPlatform {
		platform_kind: String, // "horizontal" | "vertical"
		size: i32,             // tiles
		speed: i32,            // small int for now
		range_min: i32,        // bound in tiles
		range_max: i32,        // bound in tiles
	},
}

#[derive(Debug)]
pub struct TriggerSource {
	pub top: i32,
	pub left: i32,
	pub width: i32,
	pub height: i32,
	pub kind: TriggerKindSource,
	pub icon_id: i32,
}

#[derive(Debug)]
pub enum TriggerKindSource {
	LevelExit { target: String, level: String, activation_mode: u8 },
	Message { text_id: String, activation_mode: u8 },
	Pickup { pickup: String, amount: u16, activation_mode: u8 },
}
