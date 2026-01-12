#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
	MovingPlatformLeft = 9,
	MovingPlatformMiddle = 10,
	MovingPlatformRight = 11,
	SignBegin = 13,
	Stone = 12,
	SignEnd = 14,
	PlatformLeft = 15,
	PlatformMiddle = 16,
	PlatformRight = 17,
}

impl TileKind {
	#[inline(always)]
	pub fn get_collision_kind(self) -> TileCollision {
		match self {
			TileKind::Empty => {
				return TileCollision::None;
			}

			TileKind::MovingPlatformLeft
			| TileKind::MovingPlatformMiddle
			| TileKind::MovingPlatformRight
			| TileKind::PlatformLeft
			| TileKind::PlatformMiddle
			| TileKind::PlatformRight => {
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
			9 => TileKind::MovingPlatformLeft,
			10 => TileKind::MovingPlatformMiddle,
			11 => TileKind::MovingPlatformRight,
			12 => TileKind::Stone,
			13 => TileKind::SignBegin,
			14 => TileKind::SignEnd,
			15 => TileKind::PlatformLeft,
			16 => TileKind::PlatformMiddle,
			17 => TileKind::PlatformRight,

			_ => TileKind::Empty,
		}
	}

	pub fn is_solid(self) -> bool {
		match self {
			TileKind::Dirt | TileKind::GrassTop | TileKind::Stone => true,
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
