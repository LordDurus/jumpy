#![cfg(feature = "psp")]

use crate::{
	game::runtime::State,
	platform::render::{InputState, Renderer},
};

pub struct PspRenderer {
	frame_index: u32,
}

impl Renderer for PspRenderer {
	fn new() -> Self {
		return Self { frame_index: 0 };
	}

	fn init(&mut self) {}

	fn poll_input(&mut self) -> InputState {
		return crate::platform::input::psp::poll();
	}
	fn begin_frame(&mut self) {}

	fn draw_world(&mut self, _world: &GameState) {
		self.frame_index = self.frame_index.wrapping_add(1);
	}

	fn commit(&mut self) {
		// stub: don’t call psp syscalls unless psp crate is enabled for psp builds
		// unsafe { psp::sys::sceDisplayWaitVblankStart(); }
	}
}
