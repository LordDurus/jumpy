#![cfg(feature = "psp")]

use crate::platform::audio::{Audio, SfxId};

pub struct PspAudio;

impl Audio for PspAudio {
	fn new() -> Self {
		return Self;
	}

	fn init(&mut self) {}

	fn play_sfx(&mut self, _id: SfxId) {}

	fn update(&mut self) {}
}
