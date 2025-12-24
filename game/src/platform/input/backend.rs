#[derive(Clone, Copy, Debug, Default)]
pub struct InputState {
	pub quit: bool,
	pub left: bool,
	pub right: bool,
	pub jump: bool,
}

#[allow(dead_code)]
pub trait InputBackend {
	fn poll(&mut self) -> InputState;
}
