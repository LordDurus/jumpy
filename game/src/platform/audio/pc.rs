#![cfg(feature = "pc")]
use crate::platform::audio::{AudioEngine, SfxId};
// use std::collections::HashMap;

#[allow(dead_code)]
pub struct PcAudio;

impl AudioEngine for PcAudio {
	fn new() -> Self {
		return Self;
	}

	fn init(&mut self) {}
	fn play_sfx(&mut self, _id: SfxId) {}

	fn play_sfx_and_wait(&mut self, _id: SfxId) {
		// Stop background music if it is playing and play sound effect
		/*

		// get the channel for the sound effect
		let channel = self.sfx_libary.get(&id);

		while channel.is_playing() {
			std::thread::sleep(std::time::Duration::from_millis(10));
		}
		*/
		// resume background music if it was playing before
	}
	fn update(&mut self) {}
	fn stop(&mut self, _id: SfxId) {}
}
