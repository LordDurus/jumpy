#[cfg(feature != "psp")]
use crate::platform::input::InputState;

pub fn poll() -> InputState {
	let input = InputState::default();
	return input;
}
