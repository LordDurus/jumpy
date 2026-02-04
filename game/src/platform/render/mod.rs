use crate::platform::input;

pub mod backend;
pub mod common;
pub mod icon_registry;

#[cfg(feature = "pc")]
pub mod pc;

#[cfg(feature = "gba")]
pub mod gba;

#[cfg(feature = "psp")]
pub mod psp;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum BackgroundId {
	None = 0,
	Library = 1,
	ParallaxForest = 2,
}
impl BackgroundId {
	pub fn from_u8(value: u8) -> BackgroundId {
		return match value {
			1 => BackgroundId::Library,
			2 => BackgroundId::ParallaxForest,
			_ => BackgroundId::None,
		};
	}

	pub fn to_u8(&self) -> u8 {
		return match self {
			BackgroundId::Library => 1,
			BackgroundId::ParallaxForest => 2,
			_ => 0,
		};
	}
}

pub struct BackgroundDrawParams {
	pub background_id: BackgroundId,
	pub camera_left: i32,
	pub camera_top: i32,
	pub scale: f32,
}
