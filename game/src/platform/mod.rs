pub mod audio;
pub mod input;
pub mod render;
pub mod timer;

use crate::platform::{audio::backend::AudioBackend, input::backend::InputBackend, render::backend::RenderBackend, timer::backend::TimerBackend};

#[allow(dead_code)]
pub struct Platform {
	pub render: Box<dyn RenderBackend>,
	pub input: Box<dyn InputBackend>,
	pub audio: Box<dyn AudioBackend>,
	pub timer: Box<dyn TimerBackend>,
}
