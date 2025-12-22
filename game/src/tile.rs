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

	#[allow(dead_code)]
	pub fn is_hazard(self) -> bool {
		match self {
			TileKind::SpikeUp | TileKind::SpikeDown | TileKind::SpikeLeft | TileKind::SpikeRight => true,
			_ => false,
		}
	}

	#[allow(dead_code)]
	pub fn is_liquid(self) -> bool {
		match self {
			TileKind::Water => true,
			_ => false,
		}
	}
}

pub fn resolve_sheet_tile_id(kind: TileKind, frame_index: u32, tile_x: i32, tile_y: i32) -> u16 {
	match kind {
		TileKind::Empty => return 0,
		TileKind::Dirt => return 24,

		TileKind::GrassTop => {
			let ids: [u16; 3] = [0, 1, 2];
			let idx: usize = ((tile_x + tile_y) as usize) % 3;
			return ids[idx];
		}

		TileKind::Water => {
			let ids: [u16; 4] = [14, 17, 38, 40];
			let idx: usize = (frame_index as usize) & 3;
			return ids[idx];
		}

		TileKind::SpikeUp => return 78,
		TileKind::SpikeDown => return 6,
		TileKind::SpikeLeft => return 30,
		TileKind::SpikeRight => return 54,
	}
}
