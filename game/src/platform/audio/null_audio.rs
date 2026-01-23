use crate::platform::audio::{
	AudioEngine, SfxId,
	backend::{AudioHandle, MusicId},
};

pub struct NullAudio;

impl NullAudio {
	pub fn new() -> NullAudio {
		return NullAudio;
	}
}

impl AudioEngine for NullAudio {
	fn new() -> Self {
		return Self {};
	}

	fn init(&mut self) {}

	fn play_sfx(&mut self, _id: SfxId) -> Option<AudioHandle> {
		return None;
	}

	fn play_sfx_and_wait(&mut self, _id: SfxId) {}

	fn play_music(&mut self, _id: MusicId, _looped: bool) {}

	fn stop_music(&mut self) {}
	fn stop(&mut self, _handle: AudioHandle) {}
	fn update(&mut self) {}
}
