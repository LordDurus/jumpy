#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileCollision {
	None,
	Solid,
	OneWay,
}

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
	WaterBody = 8,
	PlatformLeft = 9,
	PlatformMiddle = 10,
	PlatformRight = 11,
}

impl TileKind {
	#[inline(always)]
	pub fn get_collision_kind(self) -> TileCollision {
		match self {
			TileKind::Empty => {
				return TileCollision::None;
			}

			TileKind::PlatformLeft | TileKind::PlatformMiddle | TileKind::PlatformRight => {
				return TileCollision::OneWay;
			}

			_ => return self.is_solid().then(|| TileCollision::Solid).unwrap_or(TileCollision::None),
		}
	}

	pub fn from_u8(v: u8) -> TileKind {
		match v {
			1 => TileKind::Dirt,
			2 => TileKind::SpikeUp,
			3 => TileKind::Water,
			4 => TileKind::GrassTop,
			5 => TileKind::SpikeDown,
			6 => TileKind::SpikeLeft,
			7 => TileKind::SpikeRight,
			8 => TileKind::WaterBody,
			9 => TileKind::PlatformLeft,
			10 => TileKind::PlatformMiddle,
			11 => TileKind::PlatformRight,
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
