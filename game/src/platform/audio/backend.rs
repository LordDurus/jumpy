#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfxId {
	Jump,
	Land,
	Hit,
	Player1Died,
	Player2Died,
	BackendDebugSound,
	BackendLevel1,
}

#[allow(dead_code)]
pub trait AudioEngine {
	fn new() -> Self
	where
		Self: Sized;
	fn init(&mut self);
	fn play_sfx(&mut self, id: SfxId);
	fn play_sfx_and_wait(&mut self, id: SfxId);
	fn update(&mut self);
	fn stop(&mut self, _id: SfxId) {}
}
