#![cfg(feature = "gba")]

use crate::platform::audio::{Audio, SfxId};

pub struct GbaAudio;

impl Audio for GbaAudio {
	fn new() -> Self {
		return Self;
	}

	fn init(&mut self) {}

	fn play_sfx(&mut self, _id: SfxId) {}

	fn update(&mut self) {}
}
