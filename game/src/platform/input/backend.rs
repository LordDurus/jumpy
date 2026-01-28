#[derive(Clone, Copy, Debug, Default)]
pub struct InputState {
	pub quit: bool,
	pub left: bool,
	pub right: bool,
	pub jump: bool,
	pub up: bool,
	pub down: bool,
	pub inventory: bool,
	pub read: bool,
	pub escape: bool,
	pub page_up: bool,
	pub page_down: bool,
	pub copy: bool,
}

#[allow(dead_code)]
pub trait InputBackend {
	fn poll(&mut self) -> InputState;
}
