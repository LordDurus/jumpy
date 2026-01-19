#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MusicId {
	World1,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SfxId {
	Jump,
	Stomp,
	Hit,
	Player1Died,
	Player2Died,
	BackendDebugSound,
	BackendLevel1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioHandle(u32);

impl AudioHandle {
	pub fn new(value: u32) -> AudioHandle {
		return AudioHandle(value);
	}
}

#[allow(dead_code)]
pub trait AudioEngine {
	fn new() -> Self
	where
		Self: Sized;
	fn init(&mut self);
	fn play_sfx(&mut self, id: SfxId) -> Option<AudioHandle>;
	fn play_sfx_and_wait(&mut self, id: SfxId);
	fn play_music(&mut self, id: MusicId, loop_forever: bool);
	fn update(&mut self);
	fn stop(&mut self, handle: AudioHandle);
	fn stop_music(&mut self);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LocomotionAnim {
	Walk,
	Run,
}
