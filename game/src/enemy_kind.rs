#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EnemyKind {
	None = 0,
	Slime = 1,
	Imp = 2,
}

impl EnemyKind {
	pub fn from_u8(v: u8) -> EnemyKind {
		match v {
			1 => EnemyKind::Slime,
			2 => EnemyKind::Imp,
			_ => EnemyKind::None,
		}
	}

	pub fn to_u8(self) -> u8 {
		return self as u8;
	}
}
