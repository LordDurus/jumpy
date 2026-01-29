#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MusicId {
	None = 0,
	World1 = 1,
	World2 = 2,
	World3 = 3,
	World4 = 4,
	Library = 99,
}

impl MusicId {
	pub fn from_u8(id: u8) -> MusicId {
		return match id {
			0 => MusicId::None,
			1 => MusicId::World1,
			2 => MusicId::World2,
			3 => MusicId::World3,
			4 => MusicId::World4,
			99 => MusicId::Library,
			_ => MusicId::None,
		};
	}
}
