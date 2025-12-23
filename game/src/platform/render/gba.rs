#![cfg(feature = "gba")]

use crate::{
	game::game_state::GameState,
	platform::render::{InputState, Renderer},
};

pub struct GbaRenderer {
	frame_index: u32,
}

impl RenderBackend for GbaRenderer {
	fn screen_size(&self) -> (i32, i32) {
		return (240, 160);
	}

	fn render_scale(&self) -> f32 {
		return 1.0;
	}
	fn new() -> Self {
		return Self { frame_index: 0 };
	}

	fn init(&mut self) {}

	fn poll_input(&mut self) -> InputState {
		return crate::platform::input::gba::poll();
	}

	fn begin_frame(&mut self) {}

	fn draw_world(&mut self, _world: &GameState) {
		self.frame_index = self.frame_index.wrapping_add(1);
	}

	fn commit(&mut self) {
		// stub: don’t call agb unless you’ve got the crate enabled for gba builds
		// agb::wait_for_vblank();
	}
}
