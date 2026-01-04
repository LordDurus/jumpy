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
