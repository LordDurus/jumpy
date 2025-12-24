#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfxId {
	Jump,
	Land,
	Hit,
}

#[allow(dead_code)]
pub trait AudioBackend {
	fn new() -> Self
	where
		Self: Sized;
	fn init(&mut self);
	fn play_sfx(&mut self, id: SfxId);
	fn update(&mut self);
}
