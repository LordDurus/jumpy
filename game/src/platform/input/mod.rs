pub mod backend;

#[cfg(feature = "gba")]
pub mod gba;

#[cfg(feature = "pc")]
pub mod pc;

#[cfg(feature = "psp")]
pub mod psp;

pub use backend::InputState;

#[derive(Clone, Copy, Default)]
pub struct TriggerPresses {
	pub action_pressed: bool,
	pub up_pressed: bool,
	pub down_pressed: bool,
	pub left_pressed: bool,
	pub right_pressed: bool,
}
