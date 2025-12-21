#[rear(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl TileKind {
	pub fn from_u8(v: u8) -> TileKind {
		match v {
			1 => TileKind::Dirt,
			2 => TileKind::SpikeUp,
			3 => TileKind::Water,
			4 => TileKind::GrassTop,
			5 => TileKind::SpikeDown,
			6 => TileKind::SpikeLeft,
			7 => TileKind::SpikeRight,
			_ => TileKind::Empty,
		}
	}
	pub fn is_solid(self) -> bool {
		match self {
			TileKind::Dirt | TileKind::GrassTop => true,
			_ => false,
		}
	}

	pub fn is_hazard(self) -> bool {
		match self {
			TileKind::SpikeUp | TileKind::SpikeDown | TileKind::SpikeLeft | TileKind::SpikeRight => true,
			_ => false,
		}
	}

	pub fn is_liquid(self) -> bool {
		match self {
			TileKind::Water => true,
			_ => false,
		}
	}
}

pub fn resolve_sheet_tile_id(tile_kind: u8, frame: u32, x: i32, y: i32) -> u16 {
	if tile_kind == 0 {
		return 0; // unused, caller should skip drawing
	}
	if tile_kind == 1 {
		return 24; // dirt
	}
	if tile_kind == 2 {
		return 78; // spike up
	}
	if tile_kind == 3 {
		// water: 14, 17, 38, 40
		let ids: [u16; 4] = [14, 17, 38, 40];
		let idx: usize = (frame as usize) & 3; // cheap animation
		return ids[idx];
	}
	if tile_kind == 4 {
		// grass top: 0,1,2 (variant/animation)
		let ids: [u16; 3] = [0, 1, 2];
		let idx: usize = ((x + y) as usize) % 3; // deterministic variation
		return ids[idx];
	}
	if tile_kind == 5 {
		return 6; // spike down
	}
	if tile_kind == 6 {
		return 30; // spike left
	}
	if tile_kind == 7 {
		return 54; // spike right
	}
	return 0;
}
