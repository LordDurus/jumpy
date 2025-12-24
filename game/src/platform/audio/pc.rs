#![cfg(feature = "pc")]
use crate::platform::audio::{AudioBackend, SfxId};

#[allow(dead_code)]
pub struct PcAudio;

impl AudioBackend for PcAudio {
	fn new() -> Self {
		return Self;
	}

	fn init(&mut self) {}

	fn play_sfx(&mut self, _id: SfxId) {}

	fn update(&mut self) {}
}
