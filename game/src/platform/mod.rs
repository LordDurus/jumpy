pub mod audio;
pub mod input;
pub mod level_loader;
pub mod memory;
pub mod render;
pub mod timer;

use crate::platform::{audio::backend::AudioEngine, input::backend::InputBackend, render::backend::RenderBackend, timer::backend::TimerBackend};

#[cfg(feature = "gba")]
use alloc::boxed::Box;

#[allow(dead_code)]
pub struct Platform {
	pub render: Box<dyn RenderBackend>,
	pub input: Box<dyn InputBackend>,
	pub audio: Box<dyn AudioEngine>,
	pub timer: Box<dyn TimerBackend>,
}
