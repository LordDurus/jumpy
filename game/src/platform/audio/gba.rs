#![cfg(feature = "gba")]

use crate::{
	MusicId,
	platform::audio::{AudioEngine, SfxId, backend::AudioHandle},
};

pub struct GbaAudio;

impl AudioEngine for GbaAudio {
	fn new() -> Self {
		return Self;
	}

	fn init(&mut self) {}

	fn play_sfx(&mut self, _id: SfxId) -> Option<AudioHandle> {
		return None;
	}

	fn play_sfx_and_wait(&mut self, _id: SfxId) {}

	fn play_music(&mut self, _id: MusicId, _loop_forever: bool) {}

	fn update(&mut self) {}

	fn stop(&mut self, _handle: AudioHandle) {}

	fn stop_music(&mut self) {}
}
