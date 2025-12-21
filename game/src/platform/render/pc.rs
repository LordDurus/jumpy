use std::{
	thread::sleep,
	time::{Duration, Instant},
};

pub struct PcRenderer {
	last_frame_time: Instant,
}

impl PcRenderer {
	pub fn new() -> Self {
		Self { last_frame_time: Instant::now() }
	}
}

impl Renderer for PcRenderer {
	fn init(&mut self) {
		// Initialize PC-specific rendering (e.g., SDL2, winit)
	}

	fn render_frame(&mut self, _world: &mut World) {
		// Draw everything to the screen (e.g., sprites, UI)
		let now = Instant::now();
		let frame_duration = now - self.last_frame_time;
		let target_frame_time = Duration::from_millis(16); // ~60 FPS

		if frame_duration < target_frame_time {
			sleep(target_frame_time - frame_duration);
		}

		self.last_frame_time = Instant::now();
	}
}
